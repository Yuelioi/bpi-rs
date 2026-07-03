use crate::{BpiError, response::BpiResponse, transport::ReqwestTransport};
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use tokio::time::Instant;
use tracing;

/// 找到不超过 index 的最近合法 UTF-8 字符边界
#[cfg(any(test, debug_assertions))]
fn floor_char_boundary(s: &str, index: usize) -> usize {
    if index >= s.len() {
        return s.len();
    }
    let mut i = index;
    while i > 0 && !s.is_char_boundary(i) {
        i -= 1;
    }
    i
}

pub trait BilibiliRequest {
    fn with_bilibili_headers(self) -> Self;
    fn with_user_agent(self) -> Self;

    fn send_request(
        self,
        operation_name: &str,
    ) -> impl std::future::Future<Output = Result<bytes::Bytes, BpiError>> + Send;

    fn send_bpi<T>(
        self,
        operation_name: &str,
    ) -> impl std::future::Future<Output = Result<BpiResponse<T>, BpiError>> + Send
    where
        Self: Sized + Send,
        T: DeserializeOwned;

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
        ReqwestTransport::send_request_builder(self, operation_name)
            .await
            .map(|response| response.body)
    }

    async fn send_bpi<T>(self, operation_name: &str) -> Result<BpiResponse<T>, BpiError>
    where
        T: DeserializeOwned,
    {
        let start = Instant::now();
        let response =
            ReqwestTransport::send_request_builder(self.log_url(operation_name), operation_name)
                .await?;

        let result = match response
            .decode_api_envelope::<T>()
            .and_then(|decoded| decoded.envelope.ensure_success())
        {
            Ok(envelope) => envelope.into_legacy_response(),
            Err(err) => {
                if let BpiError::Decode { source } = &err {
                    log_decode_error(operation_name, &response.body, source);
                } else {
                    tracing::error!("{} API错误: {}", operation_name, err);
                }
                return Err(err);
            }
        };

        let duration = start.elapsed();
        tracing::info!("{} 请求成功，耗时: {:.2?}", operation_name, duration);
        Ok(result)
    }

    fn log_url(self, operation_name: &str) -> Self {
        tracing::info!("开始请求 {}", operation_name);

        self
    }
}

fn log_decode_error(operation_name: &str, bytes: &[u8], error: &serde_json::Error) {
    #[cfg(any(test, debug_assertions))]
    {
        let json_str = String::from_utf8_lossy(bytes);
        let error_pos = error.column().saturating_sub(1);
        let start = floor_char_boundary(&json_str, error_pos.saturating_sub(25));
        let end = floor_char_boundary(&json_str, (error_pos + 25).min(json_str.len()));
        let context = &json_str[start..end];
        tracing::error!(
            "{} JSON解析失败 (行:{} 列:{}): {}",
            operation_name,
            error.line(),
            error.column(),
            error
        );
        tracing::error!(
            "错误位置: ...{}... ({}^)",
            context,
            " ".repeat(error_pos.saturating_sub(start))
        );
    }
    #[cfg(not(any(test, debug_assertions)))]
    {
        tracing::error!("{} JSON解析失败: {}", operation_name, error);
    }
}
