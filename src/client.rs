use std::sync::{Arc, Mutex};
use std::time::Duration;

use reqwest::cookie::CookieStore;
use reqwest::header::{COOKIE, HeaderValue, ORIGIN, REFERER, USER_AGENT};
use reqwest::{Client, RequestBuilder, Url};

use crate::BpiError;
#[cfg(feature = "activity")]
use crate::activity::ActivityClient;
#[cfg(feature = "article")]
use crate::article::ArticleClient;
use crate::auth::Account;
#[cfg(feature = "clientinfo")]
use crate::clientinfo::ClientInfoClient;
#[cfg(feature = "comment")]
use crate::comment::CommentClient;
#[cfg(feature = "fav")]
use crate::fav::FavClient;
#[cfg(feature = "historytoview")]
use crate::historytoview::HistoryToViewClient;
#[cfg(feature = "login")]
use crate::login::LoginClient;
#[cfg(feature = "message")]
use crate::message::MessageClient;
#[cfg(feature = "misc")]
use crate::misc::MiscClient;
#[cfg(feature = "note")]
use crate::note::NoteClient;
#[cfg(feature = "opus")]
use crate::opus::OpusClient;
use crate::session::cookie::{format_cookie_pairs, parse_cookie_header as parse_cookie_pairs};
use crate::sign::wbi::WbiKeyCache;
#[cfg(feature = "user")]
use crate::user::UserClient;
#[cfg(feature = "video")]
use crate::video::VideoClient;
#[cfg(feature = "vip")]
use crate::vip::VipClient;
#[cfg(feature = "wallet")]
use crate::wallet::WalletClient;
#[cfg(feature = "web_widget")]
use crate::web_widget::WebWidgetClient;

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
const DEFAULT_REFERER: &str = "https://www.bilibili.com/";
const DEFAULT_ORIGIN: &str = "https://www.bilibili.com";
const BILIBILI_URL: &str = "https://www.bilibili.com";
const API_BILIBILI_URL: &str = "https://api.bilibili.com";
const AUTH_COOKIE_NAMES: &[&str] = &[
    "DedeUserID",
    "DedeUserID__ckMd5",
    "SESSDATA",
    "bili_jct",
    "buvid3",
];

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
        let mut account = None;
        let mut cookie_header = None;

        if let Some(cookie) = self.cookie {
            let pairs = parse_cookie_pairs(&cookie)?;
            add_cookie_pairs(&jar, &pairs);
            cookie_header = Some(format_cookie_pairs(&pairs));

            let cookie_account = Account::from_cookie_pairs(&pairs);
            if cookie_account.is_complete() {
                account = Some(cookie_account);
            }
        }

        if let Some(configured_account) = self.account {
            configured_account.validate_complete()?;
            let pairs = configured_account.cookie_pairs();
            add_cookie_pairs(&jar, &pairs);
            cookie_header = Some(format_cookie_pairs(&pairs));
            account = Some(configured_account);
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
    pub fn set_account(&self, account: Account) -> Result<(), BpiError> {
        account.validate_complete()?;

        let pairs = account.cookie_pairs();
        add_cookie_pairs(&self.jar, &pairs);
        *self
            .cookie_header
            .lock()
            .expect("cookie header mutex poisoned") = Some(format_cookie_pairs(&pairs));
        *self.account.lock().expect("account mutex poisoned") = Some(account);
        tracing::info!("Bilibili account configured");
        Ok(())
    }

    /// Clears account information from this client.
    pub fn clear_account(&self) {
        *self.account.lock().expect("account mutex poisoned") = None;
        *self
            .cookie_header
            .lock()
            .expect("cookie header mutex poisoned") = None;
        expire_auth_cookies(&self.jar);
        tracing::info!("Bilibili account cleared");
    }

    /// Sets account information from a raw Cookie header string.
    pub fn set_account_from_cookie_str(&self, cookie_str: &str) -> Result<(), BpiError> {
        let pairs = parse_cookie_pairs(cookie_str)?;
        let account = Account::from_cookie_pairs(&pairs);
        account.validate_complete()?;

        add_cookie_pairs(&self.jar, &pairs);
        *self
            .cookie_header
            .lock()
            .expect("cookie header mutex poisoned") = Some(format_cookie_pairs(&pairs));
        *self.account.lock().expect("account mutex poisoned") = Some(account);
        Ok(())
    }

    /// Checks whether this client has login cookies.
    pub fn has_login_cookies(&self) -> bool {
        if self
            .cookie_header
            .lock()
            .expect("cookie header mutex poisoned")
            .as_deref()
            .is_some_and(contains_login_cookie)
        {
            return true;
        }

        let url = Url::parse(API_BILIBILI_URL).expect("static Bilibili API URL is valid");
        self.jar
            .cookies(&url)
            .and_then(|cookies| cookies.to_str().ok().map(contains_login_cookie))
            .unwrap_or(false)
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

    /// Creates a GET request that preserves raw compressed response bytes.
    pub(crate) fn get_without_response_decoding(
        &self,
        url: &str,
    ) -> Result<RequestBuilder, BpiError> {
        let client = Client::builder()
            .no_gzip()
            .no_brotli()
            .no_deflate()
            .no_proxy()
            .cookie_provider(self.jar.clone())
            .pool_max_idle_per_host(0)
            .build()?;

        Ok(self.apply_default_headers(url, client.get(url)))
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
    /// Creates an activity domain client.
    #[cfg(feature = "activity")]
    pub fn activity(&self) -> ActivityClient<'_> {
        ActivityClient::new(self)
    }

    /// Creates an article domain client.
    #[cfg(feature = "article")]
    pub fn article(&self) -> ArticleClient<'_> {
        ArticleClient::new(self)
    }

    /// Creates a client info domain client.
    #[cfg(feature = "clientinfo")]
    pub fn clientinfo(&self) -> ClientInfoClient<'_> {
        ClientInfoClient::new(self)
    }

    /// Creates a comment domain client.
    #[cfg(feature = "comment")]
    pub fn comment(&self) -> CommentClient<'_> {
        CommentClient::new(self)
    }

    /// Creates a favorite domain client.
    #[cfg(feature = "fav")]
    pub fn fav(&self) -> FavClient<'_> {
        FavClient::new(self)
    }

    /// Creates a history and to-view domain client.
    #[cfg(feature = "historytoview")]
    pub fn historytoview(&self) -> HistoryToViewClient<'_> {
        HistoryToViewClient::new(self)
    }

    /// Creates a login domain client.
    #[cfg(feature = "login")]
    pub fn login(&self) -> LoginClient<'_> {
        LoginClient::new(self)
    }

    /// Creates a misc domain client.
    #[cfg(feature = "misc")]
    pub fn misc(&self) -> MiscClient<'_> {
        MiscClient::new(self)
    }

    /// Creates a message domain client.
    #[cfg(feature = "message")]
    pub fn message(&self) -> MessageClient<'_> {
        MessageClient::new(self)
    }

    /// Creates a note domain client.
    #[cfg(feature = "note")]
    pub fn note(&self) -> NoteClient<'_> {
        NoteClient::new(self)
    }

    /// Creates an opus domain client.
    #[cfg(feature = "opus")]
    pub fn opus(&self) -> OpusClient<'_> {
        OpusClient::new(self)
    }

    /// Creates a video domain client.
    #[cfg(feature = "video")]
    pub fn video(&self) -> VideoClient<'_> {
        VideoClient::new(self)
    }

    /// Creates a VIP domain client.
    #[cfg(feature = "vip")]
    pub fn vip(&self) -> VipClient<'_> {
        VipClient::new(self)
    }

    /// Creates a wallet domain client.
    #[cfg(feature = "wallet")]
    pub fn wallet(&self) -> WalletClient<'_> {
        WalletClient::new(self)
    }

    /// Creates a user domain client.
    #[cfg(feature = "user")]
    pub fn user(&self) -> UserClient<'_> {
        UserClient::new(self)
    }

    /// Creates a web widget domain client.
    #[cfg(feature = "web_widget")]
    pub fn web_widget(&self) -> WebWidgetClient<'_> {
        WebWidgetClient::new(self)
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

fn expire_auth_cookies(jar: &reqwest::cookie::Jar) {
    let url = Url::parse(BILIBILI_URL).expect("static Bilibili URL is valid");
    for key in AUTH_COOKIE_NAMES {
        let cookie = format!(
            "{key}=; Max-Age=0; Expires=Thu, 01 Jan 1970 00:00:00 GMT; Domain=.bilibili.com; Path=/"
        );
        jar.add_cookie_str(&cookie, &url);
    }
}

fn contains_login_cookie(cookie_header: &str) -> bool {
    parse_cookie_pairs(cookie_header)
        .map(|pairs| {
            pairs
                .iter()
                .any(|(key, value)| key.eq_ignore_ascii_case("SESSDATA") && !value.is_empty())
        })
        .unwrap_or(false)
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
    fn clear_account_removes_login_cookie_state() -> Result<(), BpiError> {
        let client = BpiClient::builder()
            .cookie("DedeUserID=42; SESSDATA=session; bili_jct=csrf; buvid3=buvid")
            .build()?;
        assert!(client.has_login_cookies());

        client.clear_account();

        assert!(!client.has_login_cookies());
        assert!(client.cookie_header_for_test().is_none());
        assert!(client.get_account().is_none());
        Ok(())
    }

    #[test]
    fn has_login_cookies_requires_sessdata_cookie() -> Result<(), BpiError> {
        let client = BpiClient::builder().cookie("buvid3=buvid").build()?;

        assert!(!client.has_login_cookies());
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
    fn builder_rejects_incomplete_structured_account() {
        let result = BpiClient::builder().account(Account::default()).build();

        assert!(matches!(
            result,
            Err(BpiError::InvalidParameter {
                field: "account",
                ..
            })
        ));
    }

    #[test]
    fn set_account_rejects_incomplete_account_without_replacing_existing_state()
    -> Result<(), BpiError> {
        let client = BpiClient::builder()
            .cookie("DedeUserID=42; SESSDATA=session; bili_jct=csrf; buvid3=buvid")
            .build()?;

        let err = client.set_account(Account::default()).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "account",
                ..
            }
        ));
        assert_eq!(client.csrf()?, "csrf");
        assert!(client.has_login_cookies());
        Ok(())
    }

    #[test]
    fn set_account_from_cookie_str_rejects_incomplete_login_cookie_without_replacing_existing_state()
    -> Result<(), BpiError> {
        let client = BpiClient::builder()
            .cookie("DedeUserID=42; SESSDATA=session; bili_jct=csrf; buvid3=buvid")
            .build()?;

        let err = client
            .set_account_from_cookie_str("buvid3=guest-buvid")
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "account",
                ..
            }
        ));
        assert_eq!(client.csrf()?, "csrf");
        assert!(client.has_login_cookies());
        Ok(())
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
    fn raw_response_request_keeps_default_headers_and_cookie() -> Result<(), BpiError> {
        let client = BpiClient::builder()
            .user_agent("test-agent")
            .cookie("DedeUserID=42; SESSDATA=session; bili_jct=csrf; buvid3=buvid")
            .build()?;

        let request = client
            .get_without_response_decoding("https://api.bilibili.com/x/v2/dm/history")?
            .build()?;

        assert_eq!(request.headers()[USER_AGENT], "test-agent");
        assert!(
            request
                .headers()
                .get(COOKIE)
                .and_then(|value| value.to_str().ok())
                .is_some_and(|value| value.contains("SESSDATA=session"))
        );
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

    #[cfg(feature = "web_widget")]
    #[test]
    fn web_widget_domain_client_can_be_created() -> Result<(), BpiError> {
        let client = BpiClient::new()?;

        let _web_widget = client.web_widget();

        Ok(())
    }

    #[cfg(feature = "activity")]
    #[test]
    fn activity_domain_client_can_be_created() -> Result<(), BpiError> {
        let client = BpiClient::new()?;

        let _activity = client.activity();

        Ok(())
    }

    #[cfg(feature = "article")]
    #[test]
    fn article_domain_client_can_be_created() -> Result<(), BpiError> {
        let client = BpiClient::new()?;

        let _article = client.article();

        Ok(())
    }

    #[cfg(feature = "wallet")]
    #[test]
    fn wallet_domain_client_can_be_created() -> Result<(), BpiError> {
        let client = BpiClient::new()?;

        let _wallet = client.wallet();

        Ok(())
    }

    #[cfg(feature = "opus")]
    #[test]
    fn opus_domain_client_can_be_created() -> Result<(), BpiError> {
        let client = BpiClient::new()?;

        let _opus = client.opus();

        Ok(())
    }

    #[cfg(feature = "misc")]
    #[test]
    fn misc_domain_client_can_be_created() -> Result<(), BpiError> {
        let client = BpiClient::new()?;

        let _misc = client.misc();

        Ok(())
    }

    #[cfg(feature = "message")]
    #[test]
    fn message_domain_client_can_be_created() -> Result<(), BpiError> {
        let client = BpiClient::new()?;

        let _message = client.message();

        Ok(())
    }

    #[cfg(feature = "vip")]
    #[test]
    fn vip_domain_client_can_be_created() -> Result<(), BpiError> {
        let client = BpiClient::new()?;

        let _vip = client.vip();

        Ok(())
    }

    #[cfg(feature = "comment")]
    #[test]
    fn comment_domain_client_can_be_created() -> Result<(), BpiError> {
        let client = BpiClient::new()?;

        let _comment = client.comment();

        Ok(())
    }

    #[cfg(feature = "fav")]
    #[test]
    fn fav_domain_client_can_be_created() -> Result<(), BpiError> {
        let client = BpiClient::new()?;

        let _fav = client.fav();

        Ok(())
    }

    #[cfg(feature = "historytoview")]
    #[test]
    fn historytoview_domain_client_can_be_created() -> Result<(), BpiError> {
        let client = BpiClient::new()?;

        let _historytoview = client.historytoview();

        Ok(())
    }

    #[cfg(feature = "note")]
    #[test]
    fn note_domain_client_can_be_created() -> Result<(), BpiError> {
        let client = BpiClient::new()?;

        let _note = client.note();

        Ok(())
    }
}
