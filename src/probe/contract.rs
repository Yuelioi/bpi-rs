use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{BpiError, BpiResult};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiContract {
    pub name: String,
    pub request: ContractRequest,
    #[serde(default)]
    pub expect: serde_json::Value,
}

impl ApiContract {
    pub fn from_slice(bytes: &[u8]) -> BpiResult<Self> {
        let raw: RawApiContract = serde_json::from_slice(bytes)?;
        raw.try_into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContractRequest {
    pub method: HttpMethod,
    pub url: ContractUrl,
    pub query: BTreeMap<String, String>,
    pub required_headers: Vec<String>,
    pub headers: BTreeMap<String, String>,
    pub auth: ContractAuth,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContractUrl(String);

impl ContractUrl {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum HttpMethod {
    Get,
    Post,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ContractAuth {
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub requires: Vec<AuthRequirement>,
}

impl ContractAuth {
    pub fn requires_cookie(&self) -> bool {
        self.requires
            .iter()
            .any(|requirement| matches!(requirement, AuthRequirement::Cookie))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthRequirement {
    Cookie,
    Csrf,
    Wbi,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CapturedRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: BTreeMap<String, String>,
    pub query: BTreeMap<String, String>,
    pub body: Option<serde_json::Value>,
}

impl CapturedRequest {
    pub fn sanitized(&self) -> Self {
        let mut output = self.clone();
        redact_headers(&mut output.headers);
        output
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProbeResponse {
    pub status: u16,
    pub headers: BTreeMap<String, String>,
    pub body: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProbeResult {
    pub contract: String,
    pub request: CapturedRequest,
    pub response: ProbeResponse,
}

impl ProbeResult {
    pub fn sanitized(&self) -> Self {
        let mut output = self.clone();
        redact_headers(&mut output.request.headers);
        redact_headers(&mut output.response.headers);
        output
    }
}

fn redact_headers(headers: &mut BTreeMap<String, String>) {
    for (key, value) in headers.iter_mut() {
        let normalized = key.to_ascii_lowercase();
        if matches!(
            normalized.as_str(),
            "cookie" | "authorization" | "set-cookie" | "x-csrf-token"
        ) {
            *value = "<redacted>".to_string();
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawApiContract {
    name: String,
    request: RawContractRequest,
    #[serde(default)]
    expect: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct RawContractRequest {
    method: String,
    url: String,
    #[serde(default)]
    query: BTreeMap<String, String>,
    #[serde(default)]
    required_headers: Vec<String>,
    #[serde(default)]
    headers: BTreeMap<String, String>,
    #[serde(default)]
    auth: ContractAuth,
    #[serde(default)]
    body: Option<serde_json::Value>,
}

impl TryFrom<RawApiContract> for ApiContract {
    type Error = BpiError;

    fn try_from(raw: RawApiContract) -> Result<Self, Self::Error> {
        let method = parse_method(&raw.request.method)?;
        let url = parse_url(raw.request.url)?;

        Ok(Self {
            name: raw.name,
            request: ContractRequest {
                method,
                url,
                query: raw.request.query,
                required_headers: raw.request.required_headers,
                headers: raw.request.headers,
                auth: raw.request.auth,
                body: raw.request.body,
            },
            expect: raw.expect,
        })
    }
}

fn parse_method(method: &str) -> BpiResult<HttpMethod> {
    match method {
        "GET" => Ok(HttpMethod::Get),
        "POST" => Ok(HttpMethod::Post),
        _ => Err(BpiError::invalid_parameter(
            "method",
            "supported methods are GET and POST",
        )),
    }
}

fn parse_url(url: String) -> BpiResult<ContractUrl> {
    reqwest::Url::parse(&url)
        .map(|_| ContractUrl(url))
        .map_err(|_| BpiError::invalid_parameter("url", "invalid URL"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BpiError;

    #[test]
    fn contract_deserializes_get_cookie_request() -> Result<(), BpiError> {
        let contract = ApiContract::from_slice(include_bytes!(
            "../../tests/contracts/login/vip-info/active.request.json"
        ))?;

        assert_eq!(contract.name, "login.vip_info.active");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.bilibili.com/x/vip/web/user/info"
        );
        assert_eq!(contract.request.auth.profile.as_deref(), Some("vip"));
        assert_eq!(
            contract.request.required_headers,
            ["user-agent", "referer", "origin", "cookie"]
        );
        assert!(contract.request.auth.requires_cookie());
        Ok(())
    }

    #[test]
    fn contract_deserializes_normal_account_variant() -> Result<(), BpiError> {
        let contract = ApiContract::from_slice(include_bytes!(
            "../../tests/contracts/login/vip-info/normal.request.json"
        ))?;

        assert_eq!(contract.name, "login.vip_info.normal");
        assert_eq!(contract.request.auth.profile.as_deref(), Some("normal"));
        assert_eq!(contract.expect["vip_active"], false);
        Ok(())
    }

    #[test]
    fn contract_rejects_unsupported_method() {
        let err = ApiContract::from_slice(
            br#"{
                "name": "bad",
                "request": {
                    "method": "TRACE",
                    "url": "https://api.bilibili.com/x/test"
                }
            }"#,
        )
        .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "method",
                ..
            }
        ));
    }

    #[test]
    fn probe_result_redacts_sensitive_headers() -> Result<(), BpiError> {
        let result = ProbeResult {
            contract: "login.vip_info.active".to_string(),
            request: CapturedRequest {
                method: HttpMethod::Get,
                url: "https://api.bilibili.com/x/vip/web/user/info".to_string(),
                headers: [
                    (
                        "cookie".to_string(),
                        "SESSDATA=secret; bili_jct=csrf".to_string(),
                    ),
                    ("user-agent".to_string(), "bpi-probe-test".to_string()),
                ]
                .into_iter()
                .collect(),
                query: Default::default(),
                body: None,
            },
            response: ProbeResponse {
                status: 200,
                headers: Default::default(),
                body: serde_json::json!({ "code": 0 }),
            },
        };

        let sanitized = result.sanitized();

        assert_eq!(sanitized.request.headers["cookie"], "<redacted>");
        assert_eq!(sanitized.request.headers["user-agent"], "bpi-probe-test");
        Ok(())
    }
}
