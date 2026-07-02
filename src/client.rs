use std::sync::{Arc, Mutex};
use std::time::Duration;

use reqwest::cookie::CookieStore;
use reqwest::header::{COOKIE, HeaderValue, ORIGIN, REFERER, USER_AGENT};
use reqwest::{Client, RequestBuilder, Url};

use crate::BpiError;
use crate::auth::Account;
#[cfg(feature = "login")]
use crate::login::LoginClient;
use crate::session::cookie::{format_cookie_pairs, parse_cookie_header as parse_cookie_pairs};
use crate::sign::wbi::WbiKeyCache;
#[cfg(feature = "user")]
use crate::user::UserClient;
#[cfg(feature = "video")]
use crate::video::VideoClient;

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
const DEFAULT_REFERER: &str = "https://www.bilibili.com/";
const DEFAULT_ORIGIN: &str = "https://www.bilibili.com";
const BILIBILI_URL: &str = "https://www.bilibili.com";
const API_BILIBILI_URL: &str = "https://api.bilibili.com";

/// Configures a [`BpiClient`] before construction.
#[derive(Debug)]
pub struct BpiClientBuilder {
    timeout: Duration,
    connect_timeout: Duration,
    user_agent: String,
    referer: String,
    origin: String,
    no_proxy: bool,
    proxies: Vec<reqwest::Proxy>,
    cookie: Option<String>,
    account: Option<Account>,
    reqwest_client: Option<Client>,
}

impl Default for BpiClientBuilder {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            connect_timeout: Duration::from_secs(10),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            referer: DEFAULT_REFERER.to_string(),
            origin: DEFAULT_ORIGIN.to_string(),
            no_proxy: true,
            proxies: Vec::new(),
            cookie: None,
            account: None,
            reqwest_client: None,
        }
    }
}

impl BpiClientBuilder {
    /// Sets the total request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the TCP connect timeout.
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Sets the default user-agent applied by [`BpiClient::get`] and [`BpiClient::post`].
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Sets the default referer header applied by [`BpiClient::get`] and [`BpiClient::post`].
    pub fn referer(mut self, referer: impl Into<String>) -> Self {
        self.referer = referer.into();
        self
    }

    /// Sets the default origin header applied by [`BpiClient::get`] and [`BpiClient::post`].
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = origin.into();
        self
    }

    /// Controls whether reqwest should bypass system proxies.
    pub fn no_proxy(mut self, enabled: bool) -> Self {
        self.no_proxy = enabled;
        self
    }

    /// Adds an explicit proxy to the reqwest client builder.
    pub fn proxy(mut self, proxy: reqwest::Proxy) -> Self {
        self.proxies.push(proxy);
        self
    }

    /// Seeds the client session from a raw Cookie header string.
    pub fn cookie(mut self, cookie: impl Into<String>) -> Self {
        self.cookie = Some(cookie.into());
        self
    }

    /// Seeds the client session from structured account values.
    pub fn account(mut self, account: Account) -> Self {
        self.account = Some(account);
        self
    }

    /// Uses an externally configured reqwest client.
    pub fn reqwest_client(mut self, client: Client) -> Self {
        self.reqwest_client = Some(client);
        self
    }

    /// Builds a client without reading files, initializing global logging, or using shared state.
    pub fn build(self) -> Result<BpiClient, BpiError> {
        let jar = Arc::new(reqwest::cookie::Jar::default());
        let mut account = self.account;
        let mut cookie_header = None;

        if let Some(cookie) = self.cookie {
            let pairs = parse_cookie_pairs(&cookie)?;
            add_cookie_pairs(&jar, &pairs);
            cookie_header = Some(format_cookie_pairs(&pairs));

            if account.is_none() {
                account = Some(Account::from_cookie_pairs(&pairs));
            }
        }

        if let Some(account) = account.as_ref() {
            let pairs = account.cookie_pairs();
            add_cookie_pairs(&jar, &pairs);
            cookie_header.get_or_insert_with(|| format_cookie_pairs(&pairs));
        }

        let client = match self.reqwest_client {
            Some(client) => client,
            None => {
                let mut builder = Client::builder()
                    .timeout(self.timeout)
                    .connect_timeout(self.connect_timeout)
                    .gzip(true)
                    .deflate(true)
                    .brotli(true)
                    .cookie_provider(jar.clone())
                    .pool_max_idle_per_host(0);

                if self.no_proxy {
                    builder = builder.no_proxy();
                }

                for proxy in self.proxies {
                    builder = builder.proxy(proxy);
                }

                builder.build()?
            }
        };

        Ok(BpiClient {
            client,
            jar,
            account: Mutex::new(account),
            user_agent: validate_header("user_agent", &self.user_agent)?,
            referer: validate_header("referer", &self.referer)?,
            origin: validate_header("origin", &self.origin)?,
            cookie_header: Mutex::new(cookie_header),
            wbi_key_cache: WbiKeyCache::default(),
        })
    }
}

/// Bilibili API client.
pub struct BpiClient {
    client: Client,
    jar: Arc<reqwest::cookie::Jar>,
    account: Mutex<Option<Account>>,
    user_agent: HeaderValue,
    referer: HeaderValue,
    origin: HeaderValue,
    cookie_header: Mutex<Option<String>>,
    wbi_key_cache: WbiKeyCache,
}

impl BpiClient {
    /// Creates an owned client with default configuration.
    pub fn new() -> Result<Self, BpiError> {
        Self::builder().build()
    }

    /// Starts configuring a client.
    pub fn builder() -> BpiClientBuilder {
        BpiClientBuilder::default()
    }

    /// Sets account information and updates this client's cookie state.
    pub fn set_account(&self, account: Account) {
        if account.is_complete() {
            let pairs = account.cookie_pairs();
            add_cookie_pairs(&self.jar, &pairs);
            *self
                .cookie_header
                .lock()
                .expect("cookie header mutex poisoned") = Some(format_cookie_pairs(&pairs));
            *self.account.lock().expect("account mutex poisoned") = Some(account);
            tracing::info!("Bilibili account configured");
        } else {
            tracing::warn!("Bilibili account is incomplete; continuing as guest");
        }
    }

    /// Clears account information from this client.
    pub fn clear_account(&self) {
        *self.account.lock().expect("account mutex poisoned") = None;
        *self
            .cookie_header
            .lock()
            .expect("cookie header mutex poisoned") = None;
        tracing::info!("Bilibili account cleared");
    }

    /// Sets account information from a raw Cookie header string.
    pub fn set_account_from_cookie_str(&self, cookie_str: &str) -> Result<(), BpiError> {
        let pairs = parse_cookie_pairs(cookie_str)?;
        add_cookie_pairs(&self.jar, &pairs);
        *self
            .cookie_header
            .lock()
            .expect("cookie header mutex poisoned") = Some(format_cookie_pairs(&pairs));
        *self.account.lock().expect("account mutex poisoned") =
            Some(Account::from_cookie_pairs(&pairs));
        Ok(())
    }

    /// Checks whether this client has login cookies.
    pub fn has_login_cookies(&self) -> bool {
        if self
            .cookie_header
            .lock()
            .expect("cookie header mutex poisoned")
            .is_some()
        {
            return true;
        }

        let url = Url::parse(API_BILIBILI_URL).expect("static Bilibili API URL is valid");
        self.jar.cookies(&url).is_some()
    }

    /// Returns the current account information.
    pub fn get_account(&self) -> Option<Account> {
        self.account.lock().expect("account mutex poisoned").clone()
    }

    /// Gets the current CSRF token from account information.
    pub fn csrf(&self) -> Result<String, BpiError> {
        let account = self.account.lock().expect("account mutex poisoned");
        let account = account.as_ref().ok_or_else(BpiError::auth_required)?;

        account.csrf().map(str::to_owned)
    }

    /// Creates a GET request with this client's default Bilibili headers.
    pub fn get(&self, url: &str) -> RequestBuilder {
        self.apply_default_headers(url, self.client.get(url))
    }

    /// Creates a POST request with this client's default Bilibili headers.
    pub fn post(&self, url: &str) -> RequestBuilder {
        self.apply_default_headers(url, self.client.post(url))
    }

    fn apply_default_headers(&self, url: &str, builder: RequestBuilder) -> RequestBuilder {
        let builder = builder
            .header(USER_AGENT, self.user_agent.clone())
            .header(REFERER, self.referer.clone())
            .header(ORIGIN, self.origin.clone());

        if !is_bilibili_url(url) {
            return builder;
        }

        match self
            .cookie_header
            .lock()
            .expect("cookie header mutex poisoned")
            .as_ref()
        {
            Some(cookie_header) => builder.header(COOKIE, cookie_header),
            None => builder,
        }
    }

    pub(crate) fn wbi_key_cache(&self) -> &WbiKeyCache {
        &self.wbi_key_cache
    }

    #[cfg(test)]
    fn cookie_header_for_test(&self) -> Option<String> {
        self.cookie_header
            .lock()
            .expect("cookie header mutex poisoned")
            .clone()
    }

    #[cfg(test)]
    fn insert_wbi_keys_for_test(
        &self,
        bucket: impl Into<String>,
        keys: crate::sign::wbi::WbiKeys,
    ) -> Result<(), BpiError> {
        self.wbi_key_cache.insert(bucket, keys)
    }

    #[cfg(test)]
    fn wbi_keys_for_test(
        &self,
        bucket: &str,
    ) -> Result<Option<crate::sign::wbi::WbiKeys>, BpiError> {
        self.wbi_key_cache.get(bucket)
    }
}

impl BpiClient {
    /// Creates a login domain client.
    #[cfg(feature = "login")]
    pub fn login(&self) -> LoginClient<'_> {
        LoginClient::new(self)
    }

    /// Creates a video domain client.
    #[cfg(feature = "video")]
    pub fn video(&self) -> VideoClient<'_> {
        VideoClient::new(self)
    }

    /// Creates a user domain client.
    #[cfg(feature = "user")]
    pub fn user(&self) -> UserClient<'_> {
        UserClient::new(self)
    }

    /// Creates a client from structured account configuration.
    pub fn from_config(config: &Account) -> Result<Self, BpiError> {
        Self::builder().account(config.clone()).build()
    }
}

fn add_cookie_pairs(jar: &reqwest::cookie::Jar, pairs: &[(String, String)]) {
    let url = Url::parse(BILIBILI_URL).expect("static Bilibili URL is valid");
    for (key, value) in pairs {
        let cookie = format!("{key}={value}; Domain=.bilibili.com; Path=/");
        jar.add_cookie_str(&cookie, &url);
    }
}

fn validate_header(field: &'static str, value: &str) -> Result<HeaderValue, BpiError> {
    HeaderValue::from_str(value)
        .map_err(|_| BpiError::invalid_parameter(field, "invalid header value"))
}

fn is_bilibili_url(url: &str) -> bool {
    Url::parse(url)
        .ok()
        .and_then(|url| url.host_str().map(|host| host.ends_with("bilibili.com")))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_creates_owned_client_without_account_side_effects() -> Result<(), BpiError> {
        let client = BpiClient::builder().build()?;

        assert!(client.get_account().is_none());
        Ok(())
    }

    #[test]
    fn builder_keeps_cookie_state_isolated_between_clients() -> Result<(), BpiError> {
        let first = BpiClient::builder().cookie("SESSDATA=first").build()?;
        let second = BpiClient::builder().cookie("SESSDATA=second").build()?;

        assert_ne!(
            first.cookie_header_for_test(),
            second.cookie_header_for_test()
        );
        Ok(())
    }

    #[test]
    fn builder_rejects_cookie_strings_without_pairs() {
        let result = BpiClient::builder().cookie("not-a-cookie").build();

        assert!(matches!(
            result,
            Err(BpiError::InvalidParameter {
                field: "cookie",
                ..
            })
        ));
    }

    #[test]
    fn builder_applies_default_headers_to_requests() -> Result<(), BpiError> {
        let client = BpiClient::builder()
            .user_agent("test-agent")
            .referer("https://example.com/referer")
            .origin("https://example.com")
            .build()?;

        let request = client.get("https://api.bilibili.com/x/test").build()?;

        assert_eq!(request.headers()[USER_AGENT], "test-agent");
        assert_eq!(request.headers()[REFERER], "https://example.com/referer");
        assert_eq!(request.headers()[ORIGIN], "https://example.com");
        Ok(())
    }

    #[test]
    fn builder_accepts_explicit_proxy_configuration() -> Result<(), BpiError> {
        let proxy = reqwest::Proxy::http("http://127.0.0.1:8080")?;

        let client = BpiClient::builder().no_proxy(false).proxy(proxy).build()?;

        assert!(client.get_account().is_none());
        Ok(())
    }

    #[test]
    fn builder_keeps_wbi_key_cache_isolated_between_clients() -> Result<(), BpiError> {
        let first = BpiClient::new()?;
        let second = BpiClient::new()?;

        first.insert_wbi_keys_for_test(
            "2026-07-02T10",
            crate::sign::wbi::WbiKeys::new("abcdefghijklmnopqrstuvwxyz123456", "sub-key-a")?,
        )?;
        second.insert_wbi_keys_for_test(
            "2026-07-02T10",
            crate::sign::wbi::WbiKeys::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ654321", "sub-key-b")?,
        )?;

        assert_ne!(
            first.wbi_keys_for_test("2026-07-02T10")?,
            second.wbi_keys_for_test("2026-07-02T10")?
        );
        Ok(())
    }
}
