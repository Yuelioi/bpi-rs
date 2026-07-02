use std::time::Duration;

/// Metadata safe to emit in response logs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponseMetadata {
    pub status: u16,
    pub duration: Duration,
    pub api_code: Option<i32>,
}
