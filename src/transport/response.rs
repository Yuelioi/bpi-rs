use std::time::Duration;

use bytes::Bytes;

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
