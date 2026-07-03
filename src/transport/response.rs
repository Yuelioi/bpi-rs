use std::time::Duration;

use bytes::Bytes;
use serde::de::DeserializeOwned;

use crate::BpiResult;
use crate::response::ApiEnvelope;

/// Metadata safe to emit in response logs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponseMetadata {
    pub status: u16,
    pub duration: Duration,
    pub api_code: Option<i32>,
}

/// Raw HTTP response bytes plus safe response metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportResponse {
    pub metadata: ResponseMetadata,
    pub body: Bytes,
}

/// Decoded JSON API envelope plus the response metadata observed before parsing.
#[derive(Debug, Clone)]
pub struct TransportEnvelope<T> {
    pub metadata: ResponseMetadata,
    pub envelope: ApiEnvelope<T>,
}

impl TransportResponse {
    /// Decodes this response as a Bilibili JSON API envelope.
    pub fn decode_api_envelope<T>(&self) -> BpiResult<TransportEnvelope<T>>
    where
        T: DeserializeOwned,
    {
        let envelope = ApiEnvelope::<T>::from_slice(&self.body)?;
        let mut metadata = self.metadata.clone();
        metadata.api_code = Some(envelope.code);

        Ok(TransportEnvelope { metadata, envelope })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Payload {
        value: u64,
    }

    #[test]
    fn decode_api_envelope_preserves_response_metadata_and_api_code() -> BpiResult<()> {
        let response = TransportResponse {
            metadata: ResponseMetadata {
                status: 200,
                duration: Duration::from_millis(12),
                api_code: None,
            },
            body: Bytes::from_static(br#"{ "code": 0, "data": { "value": 42 } }"#),
        };

        let decoded = response.decode_api_envelope::<Payload>()?;

        assert_eq!(decoded.metadata.status, 200);
        assert_eq!(decoded.metadata.api_code, Some(0));
        assert_eq!(decoded.envelope.data.unwrap().value, 42);
        Ok(())
    }
}
