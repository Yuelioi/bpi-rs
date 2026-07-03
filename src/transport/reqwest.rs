use tokio::time::Instant;

use reqwest::{Client, RequestBuilder};

use crate::{BpiError, BpiResult};

use super::{RequestMetadata, ResponseMetadata, TransportResponse};

/// Reqwest-backed transport placeholder used while endpoint modules are migrated.
#[derive(Debug, Clone)]
pub struct ReqwestTransport {
    client: Client,
}

impl ReqwestTransport {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub async fn send_request_builder(
        builder: RequestBuilder,
        endpoint: &str,
    ) -> BpiResult<TransportResponse> {
        let request_metadata = RequestMetadata::from_builder(&builder, endpoint);
        if let Some(metadata) = &request_metadata {
            tracing::info!(
                endpoint = metadata.endpoint.as_str(),
                method = %metadata.method,
                url = metadata.sanitized_url.as_str(),
                "sending Bilibili request"
            );
        } else {
            tracing::info!(endpoint, "sending Bilibili request");
        }

        let start = Instant::now();
        let response = builder.send().await.map_err(BpiError::from)?;
        let status = response.status();

        if !status.is_success() {
            tracing::error!(
                endpoint,
                status = status.as_u16(),
                "Bilibili request returned HTTP error"
            );
            return Err(BpiError::http(status.as_u16()));
        }

        let body = response.bytes().await.map_err(BpiError::from)?;
        let duration = start.elapsed();
        tracing::info!(
            endpoint,
            status = status.as_u16(),
            duration_ms = duration.as_millis(),
            "Bilibili request completed"
        );

        Ok(TransportResponse {
            metadata: ResponseMetadata {
                status: status.as_u16(),
                duration,
                api_code: None,
            },
            body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reqwest_transport_exposes_inner_client() {
        let client = Client::new();
        let transport = ReqwestTransport::new(client);

        let _ = transport.client();
    }
}
