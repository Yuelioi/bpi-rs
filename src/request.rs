use crate::{ BpiError, response::BpiResponse };
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use tokio::time::Instant;
use tracing;

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
