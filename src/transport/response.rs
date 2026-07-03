use std::time::Duration;

use bytes::Bytes;
use serde::de::DeserializeOwned;

use crate::{BpiResult, response::ApiEnvelope};

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

impl<T> TransportEnvelope<T> {
    /// Returns this envelope if it represents a successful API response.
    pub fn ensure_success(self) -> BpiResult<Self> {
        Ok(Self {
            metadata: self.metadata,
            envelope: self.envelope.ensure_success()?,
        })
    }

    /// Extracts a required payload from a successful API response.
    pub fn into_payload(self) -> BpiResult<TransportPayload<T>> {
        let Self { metadata, envelope } = self.ensure_success()?;
        let payload = envelope.data.ok_or(crate::BpiError::MissingData)?;

        Ok(TransportPayload { metadata, payload })
    }

    /// Extracts an optional payload from a successful API response.
    pub fn into_optional_payload(self) -> BpiResult<TransportOptionalPayload<T>> {
        let Self { metadata, envelope } = self.ensure_success()?;

        Ok(TransportOptionalPayload {
            metadata,
            payload: envelope.data,
        })
    }

    /// Converts the decoded transport envelope into the legacy response type.
    pub fn into_legacy_response(self) -> BpiResult<crate::response::BpiResponse<T>> {
        Ok(self.ensure_success()?.envelope.into_legacy_response())
    }
}

/// Required API payload plus the response metadata observed before extraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportPayload<T> {
    pub metadata: ResponseMetadata,
    pub payload: T,
}

/// Optional API payload plus the response metadata observed before extraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportOptionalPayload<T> {
    pub metadata: ResponseMetadata,
    pub payload: Option<T>,
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

    #[test]
    fn transport_envelope_into_payload_returns_payload_with_metadata() -> BpiResult<()> {
        let decoded = success_response().decode_api_envelope::<Payload>()?;

        let payload = decoded.into_payload()?;

        assert_eq!(payload.metadata.api_code, Some(0));
        assert_eq!(payload.payload.value, 42);
        Ok(())
    }

    #[test]
    fn transport_envelope_into_payload_returns_api_error() {
        let response = TransportResponse {
            metadata: ResponseMetadata {
                status: 200,
                duration: Duration::from_millis(12),
                api_code: None,
            },
            body: Bytes::from_static(br#"{ "code": -101, "message": "not logged in" }"#),
        };

        let err = response
            .decode_api_envelope::<Payload>()
            .and_then(TransportEnvelope::into_payload)
            .unwrap_err();

        assert!(err.requires_login());
    }

    #[test]
    fn transport_envelope_into_optional_payload_allows_empty_success() -> BpiResult<()> {
        let response = TransportResponse {
            metadata: ResponseMetadata {
                status: 200,
                duration: Duration::from_millis(12),
                api_code: None,
            },
            body: Bytes::from_static(br#"{ "code": 0, "message": "0" }"#),
        };

        let payload = response
            .decode_api_envelope::<Payload>()?
            .into_optional_payload()?;

        assert_eq!(payload.metadata.api_code, Some(0));
        assert!(payload.payload.is_none());
        Ok(())
    }

    fn success_response() -> TransportResponse {
        TransportResponse {
            metadata: ResponseMetadata {
                status: 200,
                duration: Duration::from_millis(12),
                api_code: None,
            },
            body: Bytes::from_static(br#"{ "code": 0, "data": { "value": 42 } }"#),
        }
    }
}
