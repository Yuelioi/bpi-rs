use crate::{ BpiError };
use reqwest::RequestBuilder;
use reqwest::cookie::CookieStore;
use reqwest::{ Client, Url, cookie::Jar };
use std::sync::{ Arc, Mutex };
use tracing;

use super::auth::Account;
use super::request::BilibiliRequest;

/// 使用示例：
///
///
/// ```rust
/// use bpi_rs::{ Account, BpiClient };
///
/// #[tokio::main]
/// async fn main() {
///     let bpi = BpiClient::new();
///     bpi.set_account(Account {
///         dede_user_id: "".to_string(),
///         dede_user_id_ckmd5: "".to_string(),
///         sessdata: "".to_string(),
///         bili_jct: "".to_string(),
///         buvid3: "".to_string(),
///     });
///
///     // bpi.set_account_from_cookie_str("dede_user_id=123;bili_jct=456...");
///
///     let result = bpi.bangumi_info(28220978).await;
///     match result {
///         Ok(result) => {
///             tracing::info!("{:#?}", result.data);
///         }
///         Err(e) => {
///             tracing::error!("{:#?}", e);
///         }
///     }
/// }

/// ```
pub struct BpiClient {
    client: Client,
    jar: Arc<Jar>,
    account: Mutex<Option<Account>>,
}

impl BpiClient {
    /// 创建client
    pub fn new() -> &'static Self {
        static INSTANCE: std::sync::OnceLock<BpiClient> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(|| {
            let jar = Arc::new(Jar::default());
            let client = Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .gzip(true) // 启用gzip自动解压缩
                .deflate(true) // 启用deflate解压缩
                .brotli(true) // 启用brotli解压缩
                .no_proxy()
                .cookie_provider(jar.clone())
                .pool_max_idle_per_host(0)
                .build()
                .unwrap();

            let instance = Self {
                client,
                jar,
                account: Mutex::new(None),
            };

            // 在 debug 模式下自动从account.toml加载测试账号
            #[cfg(any(test, debug_assertions))]
            {
                use super::log::init_log;

                init_log();
                if let Ok(test_account) = Account::load_test_account() {
                    instance.set_account(test_account);
                    tracing::info!("已自动加载测试账号");
                } else {
                    tracing::warn!("无法加载测试账号，使用默认配置");
                }
            }

            instance
        })
    }

    /// 设置账号信息
    pub fn set_account(&self, account: Account) {
        if account.is_complete() {
            self.load_cookies_from_account(&account);
            let mut acc = self.account.lock().unwrap();
            *acc = Some(account);
            tracing::info!("设置账号信息完成，使用[登录]模式");
        } else {
            tracing::warn!("账号信息不完整，使用[游客]模式");
        }
    }

    /// 从账号信息设置登录 cookies
    fn load_cookies_from_account(&self, account: &Account) {
        tracing::info!("开始从账号信息加载cookies...");

        let cookies = vec![
            ("DedeUserID", account.dede_user_id.clone()),
            ("DedeUserID__ckMd5", account.dede_user_id_ckmd5.clone()),
            ("SESSDATA", account.sessdata.clone()),
            ("bili_jct", account.bili_jct.clone()),
            ("buvid3", account.buvid3.clone())
        ];
        self.add_cookies(cookies);
        tracing::info!("从账号信息加载登录 cookies 完成");
    }

    /// 清除账号信息
    pub fn clear_account(&self) {
        let mut acc = self.account.lock().unwrap();
        *acc = None;
        self.clear_cookies();
        tracing::info!("清除账号信息完成");
    }

    fn add_cookie_pair(&self, key: &str, value: &str) {
        let url = Url::parse("https://www.bilibili.com").unwrap();
        let cookie = format!("{}={}; Domain=.bilibili.com; Path=/", key, value);
        self.jar.add_cookie_str(&cookie, &url);
        tracing::debug!("添加 cookie: {} = {}", key, value);
    }

    /// 批量添加 cookies
    fn add_cookies<I, K, V>(&self, cookies: I)
        where I: IntoIterator<Item = (K, V)>, K: ToString, V: ToString
    {
        for (key, value) in cookies {
            self.add_cookie_pair(&key.to_string(), &value.to_string());
        }
    }

    /// 清空所有 cookies
    /// todo
    fn clear_cookies(&self) {
        // 注意：reqwest 的 Jar 没有直接的 clear 方法
        // 这里需要重新创建 jar，但由于 Arc 的限制，需要在上层重置整个 Bpi
        tracing::info!("清空 cookies（需要重置整个客户端）");
    }

    pub fn set_account_from_cookie_str(&self, cookie_str: &str) {
        // 先解析成 map
        let mut map = std::collections::HashMap::new();
        for kv in cookie_str.split(';') {
            let kv = kv.trim();
            if let Some(pos) = kv.find('=') {
                let (key, value) = kv.split_at(pos);
                map.insert(key.trim().to_string(), value[1..].trim().to_string());
            }
        }

        let account = Account {
            dede_user_id: map.get("DedeUserID").cloned().unwrap_or_default(),
            dede_user_id_ckmd5: map.get("DedeUserID__ckMd5").cloned().unwrap_or_default(),
            sessdata: map.get("SESSDATA").cloned().unwrap_or_default(),
            bili_jct: map.get("bili_jct").cloned().unwrap_or_default(),
            buvid3: map.get("buvid3").cloned().unwrap_or_default(),
        };

        self.set_account(account);
    }

    /// 检查是否有登录 cookies
    pub fn has_login_cookies(&self) -> bool {
        let url = Url::parse("https://api.bilibili.com").unwrap();
        self.jar.cookies(&url).is_some()
    }

    /// 获取当前账号信息
    pub fn get_account(&self) -> Option<Account> {
        self.account.lock().unwrap().clone()
    }

    /// 从账号信息获取 CSRF token
    pub fn csrf(&self) -> Result<String, BpiError> {
        let account = self.account.lock().unwrap();
        account
            .as_ref()
            .filter(|acc| !acc.bili_jct.is_empty())
            .map(|acc| acc.bili_jct.clone())
            .ok_or_else(BpiError::missing_csrf)
    }

    /// reqwest的get请求包装, 自带user_agent
    pub fn get(&self, url: &str) -> RequestBuilder {
        self.client.get(url).with_user_agent()
    }
    /// reqwest的post请求包装, 自带user_agent
    pub fn post(&self, url: &str) -> RequestBuilder {
        self.client.post(url).with_user_agent()
    }
}

impl BpiClient {
    /// 从配置创建Client
    pub fn from_config(config: &Account) -> &Self {
        let bpi = Self::new();

        if
            !config.dede_user_id.is_empty() &&
            !config.sessdata.is_empty() &&
            !config.bili_jct.is_empty() &&
            !config.buvid3.is_empty()
        {
            let account = Account::new(
                config.dede_user_id.clone(),
                config.dede_user_id_ckmd5.clone(),
                config.sessdata.clone(),
                config.bili_jct.clone(),
                config.buvid3.clone()
            );
            bpi.set_account(account);
        }

        bpi
    }
}
