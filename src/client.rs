use crate::{ BpiError, response::BpiResponse };
use reqwest::RequestBuilder;
use reqwest::cookie::CookieStore;
use reqwest::{ Client, Url, cookie::Jar };
use serde::de::DeserializeOwned;
use std::sync::{ Arc, Mutex };
use tokio::time::Instant;
use tracing;

use super::auth::Account;

pub trait BilibiliRequest {
    fn with_bilibili_headers(self) -> Self;
    fn with_user_agent(self) -> Self;

    fn send_request(
        self,
        operation_name: &str
    ) -> impl std::future::Future<Output = Result<bytes::Bytes, BpiError>> + Send;

    fn send_bpi<T>(
        self,
        operation_name: &str
    )
        -> impl std::future::Future<Output = Result<BpiResponse<T>, BpiError>> + Send
        where Self: Sized + Send, T: DeserializeOwned;

    fn log_url(self, operation_name: &str) -> Self;
}

impl BilibiliRequest for RequestBuilder {
    /// UserAgent + Referer + Origin
    fn with_bilibili_headers(self) -> Self {
        self.with_user_agent()
            .header("Referer", "https://www.bilibili.com/")
            .header("Origin", "https://www.bilibili.com")
    }

    fn with_user_agent(self) -> Self {
        self.header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        )
    }

    async fn send_request(self, operation_name: &str) -> Result<bytes::Bytes, BpiError> {
        // 发送请求
        let response = self.send().await.map_err(|e| {
            tracing::error!("{} 请求失败: {}", operation_name, e);
            BpiError::from(e) // 使用 From trait 自动转换
        })?;

        // 检查响应状态
        let status = response.status();
        if !status.is_success() {
            let err = BpiError::http(status.as_u16());
            tracing::error!("{} HTTP错误: {}", operation_name, err);
            return Err(err);
        }

        // 获取响应体
        response.bytes().await.map_err(|e| {
            tracing::error!("{} 获取响应体失败: {}", operation_name, e);
            BpiError::network(format!("获取响应体失败: {}", e))
        })
    }

    async fn send_bpi<T>(self, operation_name: &str) -> Result<BpiResponse<T>, BpiError>
        where T: DeserializeOwned
    {
        // 开始计时
        let start = Instant::now();
        // 请求拿到响应 bytes
        let bytes = self.log_url(operation_name).send_request(operation_name).await?;

        // 解析JSON响应
        let result: BpiResponse<T> = serde_json::from_slice(&bytes).map_err(|e| {
            #[cfg(any(test, debug_assertions))]
            {
                let json_str = String::from_utf8_lossy(&bytes);
                let error_pos = e.column().saturating_sub(1);
                let start = error_pos.saturating_sub(25);
                let end = (error_pos + 25).min(json_str.len());
                let context = &json_str[start..end];

                tracing::error!(
                    "{} JSON解析失败 (行:{} 列:{}): {}",
                    operation_name,
                    e.line(),
                    e.column(),
                    e
                );
                tracing::error!(
                    "错误位置: ...{}... ({}^)",
                    context,
                    " ".repeat(error_pos.saturating_sub(start))
                );
            }
            #[cfg(not(any(test, debug_assertions)))]
            {
                tracing::error!("{} JSON解析失败: {}", operation_name, e);
            }
            BpiError::from(e)
        })?;

        // 处理API业务错误
        if result.code != 0 {
            let err = if result.message.is_empty() || result.message == "0" {
                BpiError::from_code(result.code)
            } else {
                BpiError::from_code_message(result.code, result.message.clone())
            };

            tracing::error!("{} API错误: {}", operation_name, err);
            return Err(err);
        }

        let duration = start.elapsed();
        tracing::info!("{} 请求成功，耗时: {:.2?}", operation_name, duration);
        Ok(result)
    }

    fn log_url(self, operation_name: &str) -> Self {
        let url = self
            .try_clone() // 注意：这里用不到也行，直接 build 也可以
            .and_then(|rb| rb.build().ok())
            .map(|req| req.url().to_string())
            .unwrap_or_else(|| "未知URL".to_string());

        tracing::info!("开始请求 {}: {}", operation_name, url);

        self
    }
}

pub struct BpiClient {
    client: Client,
    jar: Arc<Jar>,
    account: Mutex<Option<Account>>,
}

impl BpiClient {
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

            // 在 debug 模式下自动加载测试账号
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

    /// 获取 CSRF token
    pub fn csrf(&self) -> Result<String, BpiError> {
        let account = self.account.lock().unwrap();
        account
            .as_ref()
            .filter(|acc| !acc.bili_jct.is_empty())
            .map(|acc| acc.bili_jct.clone())
            .ok_or_else(BpiError::missing_csrf)
    }

    pub fn get(&self, url: &str) -> RequestBuilder {
        self.client.get(url).with_user_agent()
    }

    pub fn post(&self, url: &str) -> RequestBuilder {
        self.client.post(url).with_user_agent()
    }
}

impl BpiClient {
    /// 从配置创建（如果你仍然需要从外部配置加载）
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
