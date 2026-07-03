use crate::err::error::BpiError;
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};

/// Crate-wide result type for bpi-rs operations.
pub type BpiResult<T> = Result<T, BpiError>;

/// Canonical Bilibili JSON envelope used by most web API endpoints.
#[derive(Debug, Serialize, Clone)]
pub struct ApiEnvelope<T> {
    /// API return code. `0` means success.
    pub code: i32,

    /// Payload returned by successful endpoints.
    pub data: Option<T>,

    /// API message. Bilibili often returns `"0"` for success.
    pub message: String,

    /// Optional status flag returned by some endpoints.
    pub status: bool,
}

impl<'de, T> Deserialize<'de> for ApiEnvelope<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawEnvelope::<T>::deserialize(deserializer)?;
        Ok(Self {
            code: raw.code.or(raw.errno).unwrap_or_default(),
            data: raw.data,
            message: raw.message.or(raw.msg).or(raw.show_msg).unwrap_or_default(),
            status: raw.status,
        })
    }
}

impl<T> ApiEnvelope<T> {
    /// Parses a JSON envelope from bytes.
    pub fn from_slice(bytes: &[u8]) -> BpiResult<Self>
    where
        T: DeserializeOwned,
    {
        serde_json::from_slice(bytes).map_err(BpiError::from)
    }

    /// Returns this envelope if it represents a successful API response.
    pub fn ensure_success(self) -> BpiResult<Self> {
        if self.code == 0 {
            return Ok(self);
        }

        if self.message.is_empty() || self.message == "0" {
            Err(BpiError::from_code(self.code))
        } else {
            Err(BpiError::from_code_message(self.code, self.message))
        }
    }

    /// Extracts a required payload from a successful response.
    pub fn into_payload(self) -> BpiResult<T> {
        self.ensure_success()?.data.ok_or(BpiError::MissingData)
    }

    /// Extracts an optional payload from a successful response.
    pub fn into_optional_payload(self) -> BpiResult<Option<T>> {
        Ok(self.ensure_success()?.data)
    }

    /// Converts this envelope into the legacy response type used by current endpoints.
    pub fn into_legacy_response(self) -> BpiResponse<T> {
        BpiResponse {
            code: self.code,
            data: self.data,
            message: self.message,
            status: self.status,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct BpiResponse<T> {
    /// 返回值 0：成功
    pub code: i32,

    pub data: Option<T>,

    /// 错误信息，默认为0
    pub message: String,

    /// 状态, 部分接口需要
    pub status: bool,
}

impl<'de, T> Deserialize<'de> for BpiResponse<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawEnvelope::<T>::deserialize(deserializer)?;
        Ok(Self {
            code: raw.code.or(raw.errno).unwrap_or_default(),
            data: raw.data,
            message: raw.message.or(raw.msg).or(raw.show_msg).unwrap_or_default(),
            status: raw.status,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
struct RawEnvelope<T> {
    #[serde(default)]
    code: Option<i32>,
    #[serde(default)]
    errno: Option<i32>,
    #[serde(default, alias = "result")]
    data: Option<T>,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    msg: Option<String>,
    #[serde(default, rename = "showMsg")]
    show_msg: Option<String>,
    #[serde(default)]
    status: bool,
}

impl<T> BpiResponse<T> {
    pub fn into_data(self) -> Result<T, BpiError> {
        self.data.ok_or(BpiError::missing_data())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct FixturePayload {
        title: String,
        aid: u64,
    }

    fn fixture(name: &str) -> &'static [u8] {
        match name {
            "success" => include_bytes!("../tests/fixtures/envelope/success.json"),
            "result-alias" => include_bytes!("../tests/fixtures/envelope/result-alias.json"),
            "api-error" => include_bytes!("../tests/fixtures/envelope/api-error.json"),
            "missing-data" => include_bytes!("../tests/fixtures/envelope/missing-data.json"),
            "no-payload" => include_bytes!("../tests/fixtures/envelope/no-payload.json"),
            _ => unreachable!("unknown fixture"),
        }
    }

    #[test]
    fn api_envelope_extracts_data_payload() -> Result<(), BpiError> {
        let payload =
            ApiEnvelope::<FixturePayload>::from_slice(fixture("success"))?.into_payload()?;

        assert_eq!(payload.title, "fixture video");
        Ok(())
    }

    #[test]
    fn api_envelope_extracts_result_alias_payload() -> Result<(), BpiError> {
        let payload =
            ApiEnvelope::<FixturePayload>::from_slice(fixture("result-alias"))?.into_payload()?;

        assert_eq!(payload.aid, 170002);
        Ok(())
    }

    #[test]
    fn api_envelope_returns_missing_data_for_required_payload() {
        let err = ApiEnvelope::<FixturePayload>::from_slice(fixture("missing-data"))
            .and_then(ApiEnvelope::into_payload)
            .unwrap_err();

        assert!(matches!(err, BpiError::MissingData));
    }

    #[test]
    fn api_envelope_allows_optional_payload() -> Result<(), BpiError> {
        let payload = ApiEnvelope::<FixturePayload>::from_slice(fixture("no-payload"))?
            .into_optional_payload()?;

        assert!(payload.is_none());
        Ok(())
    }

    #[test]
    fn api_envelope_converts_api_error() {
        let err = ApiEnvelope::<FixturePayload>::from_slice(fixture("api-error"))
            .and_then(ApiEnvelope::ensure_success)
            .unwrap_err();

        assert!(matches!(err, BpiError::Api { code: -101, .. }));
        assert_eq!(err.code(), Some(-101));
    }

    #[test]
    fn api_envelope_treats_null_message_as_empty() -> Result<(), BpiError> {
        let payload = ApiEnvelope::<LoginCoinFixture>::from_slice(
            br#"{ "code": 0, "message": null, "data": { "money": 0.0 } }"#,
        )?
        .into_payload()?;

        assert_eq!(payload.money, 0.0);
        Ok(())
    }

    #[test]
    fn api_envelope_maps_errno_login_error() {
        let err = ApiEnvelope::<serde_json::Value>::from_slice(
            br#"{ "errno": 800501007, "msg": "user not login", "showMsg": "user not login" }"#,
        )
        .and_then(ApiEnvelope::ensure_success)
        .unwrap_err();

        assert!(err.requires_login());
        assert_eq!(err.code(), Some(800501007));
    }

    #[test]
    fn api_envelope_returns_decode_error_for_invalid_json() {
        let err = ApiEnvelope::<FixturePayload>::from_slice(b"{not json").unwrap_err();

        assert!(matches!(err, BpiError::Decode { .. }));
    }

    #[derive(Debug, Deserialize)]
    struct LoginCoinFixture {
        money: f64,
    }
}
