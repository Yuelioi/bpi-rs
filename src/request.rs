#[cfg(feature = "manga")]
use crate::response::ApiEnvelope;
use crate::{
    BpiError,
    transport::{ReqwestTransport, TransportEnvelope, TransportResponse},
};
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

    fn send_bpi_payload<T>(
        self,
        operation_name: &str,
    ) -> impl std::future::Future<Output = Result<T, BpiError>> + Send
    where
        Self: Sized + Send,
        T: DeserializeOwned;

    fn send_bpi_optional_payload<T>(
        self,
        operation_name: &str,
    ) -> impl std::future::Future<Output = Result<Option<T>, BpiError>> + Send
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

    async fn send_bpi_payload<T>(self, operation_name: &str) -> Result<T, BpiError>
    where
        T: DeserializeOwned,
    {
        let start = Instant::now();
        let response =
            ReqwestTransport::send_request_builder(self.log_url(operation_name), operation_name)
                .await?;
        let result = decode_bpi_payload_response(operation_name, &response)?;

        log_success(operation_name, start);
        Ok(result)
    }

    async fn send_bpi_optional_payload<T>(self, operation_name: &str) -> Result<Option<T>, BpiError>
    where
        T: DeserializeOwned,
    {
        let start = Instant::now();
        let response =
            ReqwestTransport::send_request_builder(self.log_url(operation_name), operation_name)
                .await?;
        let result = decode_bpi_optional_payload_response(operation_name, &response)?;

        log_success(operation_name, start);
        Ok(result)
    }

    fn log_url(self, operation_name: &str) -> Self {
        tracing::info!("开始请求 {}", operation_name);

        self
    }
}

#[cfg(feature = "manga")]
pub(crate) async fn send_bpi_envelope<T>(
    request: RequestBuilder,
    operation_name: &str,
) -> Result<ApiEnvelope<T>, BpiError>
where
    T: DeserializeOwned,
{
    let start = Instant::now();
    let response =
        ReqwestTransport::send_request_builder(request.log_url(operation_name), operation_name)
            .await?;
    let result = decode_bpi_envelope_response(operation_name, &response)?;

    log_success(operation_name, start);
    Ok(result)
}

#[cfg(feature = "manga")]
fn decode_bpi_envelope_response<T>(
    operation_name: &str,
    response: &TransportResponse,
) -> Result<ApiEnvelope<T>, BpiError>
where
    T: DeserializeOwned,
{
    decode_bpi_transport_response(operation_name, response, |decoded| {
        decoded.into_api_envelope()
    })
}

fn decode_bpi_payload_response<T>(
    operation_name: &str,
    response: &TransportResponse,
) -> Result<T, BpiError>
where
    T: DeserializeOwned,
{
    decode_bpi_transport_response(operation_name, response, |decoded| {
        decoded.into_payload().map(|payload| payload.payload)
    })
}

fn decode_bpi_optional_payload_response<T>(
    operation_name: &str,
    response: &TransportResponse,
) -> Result<Option<T>, BpiError>
where
    T: DeserializeOwned,
{
    decode_bpi_transport_response(operation_name, response, |decoded| {
        decoded
            .into_optional_payload()
            .map(|payload| payload.payload)
    })
}

fn decode_bpi_transport_response<T, R>(
    operation_name: &str,
    response: &TransportResponse,
    extract: impl FnOnce(TransportEnvelope<T>) -> Result<R, BpiError>,
) -> Result<R, BpiError>
where
    T: DeserializeOwned,
{
    match response.decode_api_envelope::<T>().and_then(extract) {
        Ok(result) => Ok(result),
        Err(err) => {
            if let BpiError::Decode { source } = &err {
                log_decode_error(operation_name, &response.body, source);
            } else {
                tracing::error!("{} API错误: {}", operation_name, err);
            }
            Err(err)
        }
    }
}

fn log_success(operation_name: &str, start: Instant) {
    let duration = start.elapsed();
    tracing::info!("{} 请求成功，耗时: {:.2?}", operation_name, duration);
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
        let _ = bytes;
        tracing::error!("{} JSON解析失败: {}", operation_name, error);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bytes::Bytes;
    use serde::Deserialize;

    use super::*;
    use crate::transport::{ResponseMetadata, TransportResponse};
    use crate::{BpiError, BpiResult};

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Payload {
        value: u64,
    }

    #[test]
    fn decode_bpi_payload_response_returns_required_payload() -> BpiResult<()> {
        let payload = decode_bpi_payload_response::<Payload>(
            "unit",
            &response(br#"{ "code": 0, "data": { "value": 42 } }"#),
        )?;

        assert_eq!(payload.value, 42);
        Ok(())
    }

    #[test]
    fn decode_bpi_payload_response_rejects_missing_required_payload() {
        let err = decode_bpi_payload_response::<Payload>(
            "unit",
            &response(br#"{ "code": 0, "message": "0" }"#),
        )
        .unwrap_err();

        assert!(matches!(err, BpiError::MissingData));
    }

    #[test]
    fn decode_bpi_optional_payload_response_allows_missing_payload() -> BpiResult<()> {
        let payload = decode_bpi_optional_payload_response::<Payload>(
            "unit",
            &response(br#"{ "code": 0, "message": "0" }"#),
        )?;

        assert!(payload.is_none());
        Ok(())
    }

    fn response(body: &'static [u8]) -> TransportResponse {
        TransportResponse {
            metadata: ResponseMetadata {
                status: 200,
                duration: Duration::from_millis(1),
                api_code: None,
            },
            body: Bytes::from_static(body),
        }
    }
}
