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

    pub fn from_value(value: serde_json::Value) -> BpiResult<Self> {
        let raw: RawApiContract = serde_json::from_value(value)?;
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
#[serde(rename_all = "UPPERCASE")]
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

    pub fn validate_expectations(&self, contract: &ApiContract) -> BpiResult<()> {
        if let Some(expected_code) = contract.expect.get("api_code") {
            let actual_code = self
                .response
                .body
                .get("code")
                .ok_or_else(|| BpiError::unsupported_response("probe response missing code"))?;
            if actual_code != expected_code {
                return Err(BpiError::unsupported_response(format!(
                    "probe api_code mismatch: expected {expected_code}, got {actual_code}"
                )));
            }
        }

        if let Some(expected_vip_active) = contract.expect.get("vip_active") {
            let expected_vip_active = expected_vip_active.as_bool().ok_or_else(|| {
                BpiError::invalid_parameter("vip_active", "vip_active expectation must be boolean")
            })?;
            let data = self
                .response
                .body
                .get("data")
                .ok_or_else(|| BpiError::unsupported_response("probe response missing data"))?;
            let vip_status = data
                .get("vip_status")
                .and_then(serde_json::Value::as_u64)
                .ok_or_else(|| {
                    BpiError::unsupported_response("probe response missing data.vip_status")
                })?;
            let vip_due_date = data
                .get("vip_due_date")
                .and_then(serde_json::Value::as_u64)
                .ok_or_else(|| {
                    BpiError::unsupported_response("probe response missing data.vip_due_date")
                })?;
            let actual_vip_active = vip_status == 1 && vip_due_date > 0;

            if actual_vip_active != expected_vip_active {
                return Err(BpiError::unsupported_response(format!(
                    "probe vip_active mismatch: expected {expected_vip_active}, got {actual_vip_active}"
                )));
            }
        }

        Ok(())
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

    fn vip_info_contract(
        profile: &str,
        requires_cookie: bool,
        expect: serde_json::Value,
    ) -> BpiResult<ApiContract> {
        let required_headers = if requires_cookie {
            serde_json::json!(["user-agent", "referer", "origin", "cookie"])
        } else {
            serde_json::json!(["user-agent", "referer", "origin"])
        };
        let auth = if requires_cookie {
            serde_json::json!({ "profile": profile, "requires": ["cookie"] })
        } else {
            serde_json::json!({ "requires": [] })
        };

        ApiContract::from_value(serde_json::json!({
            "name": format!("login.vip_info.{profile}"),
            "request": {
                "method": "GET",
                "url": "https://api.bilibili.com/x/vip/web/user/info",
                "query": {},
                "required_headers": required_headers,
                "headers": {},
                "auth": auth
            },
            "expect": expect
        }))
    }

    #[test]
    fn contract_deserializes_get_cookie_request() -> Result<(), BpiError> {
        let contract = vip_info_contract("vip", true, serde_json::json!({ "api_code": 0 }))?;

        assert_eq!(contract.name, "login.vip_info.vip");
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
        let contract = vip_info_contract(
            "normal",
            true,
            serde_json::json!({ "api_code": 0, "vip_active": false }),
        )?;

        assert_eq!(contract.name, "login.vip_info.normal");
        assert_eq!(contract.request.auth.profile.as_deref(), Some("normal"));
        assert_eq!(contract.expect["vip_active"], false);
        Ok(())
    }

    #[test]
    fn contract_deserializes_anonymous_variant() -> Result<(), BpiError> {
        let contract =
            vip_info_contract("anonymous", false, serde_json::json!({ "api_code": -101 }))?;

        assert_eq!(contract.name, "login.vip_info.anonymous");
        assert_eq!(contract.request.auth.profile, None);
        assert!(!contract.request.auth.requires_cookie());
        assert_eq!(contract.expect["api_code"], -101);
        Ok(())
    }

    #[test]
    fn captured_request_serializes_method_as_uppercase() -> Result<(), BpiError> {
        let captured = CapturedRequest {
            method: HttpMethod::Get,
            url: "https://api.bilibili.com/x/vip/web/user/info".to_string(),
            headers: Default::default(),
            query: Default::default(),
            body: None,
        };

        let value = serde_json::to_value(captured)?;

        assert_eq!(value["method"], "GET");
        Ok(())
    }

    #[test]
    fn probe_result_validates_expected_api_code() -> Result<(), BpiError> {
        let contract =
            vip_info_contract("anonymous", false, serde_json::json!({ "api_code": -101 }))?;
        let result = ProbeResult {
            contract: contract.name.clone(),
            request: CapturedRequest {
                method: HttpMethod::Get,
                url: contract.request.url.as_str().to_string(),
                headers: Default::default(),
                query: Default::default(),
                body: None,
            },
            response: ProbeResponse {
                status: 200,
                headers: Default::default(),
                body: serde_json::json!({
                    "code": -101,
                    "message": "账号未登录"
                }),
            },
        };

        result.validate_expectations(&contract)?;
        Ok(())
    }

    #[test]
    fn probe_result_rejects_mismatched_vip_active_expectation() -> Result<(), BpiError> {
        let contract = vip_info_contract(
            "vip",
            true,
            serde_json::json!({ "api_code": 0, "vip_active": true }),
        )?;
        let result = ProbeResult {
            contract: contract.name.clone(),
            request: CapturedRequest {
                method: HttpMethod::Get,
                url: contract.request.url.as_str().to_string(),
                headers: Default::default(),
                query: Default::default(),
                body: None,
            },
            response: ProbeResponse {
                status: 200,
                headers: Default::default(),
                body: serde_json::json!({
                    "code": 0,
                    "data": {
                        "mid": 1,
                        "vip_type": 0,
                        "vip_status": 0,
                        "vip_due_date": 0,
                        "vip_pay_type": 0,
                        "theme_type": 0
                    },
                    "message": "0"
                }),
            },
        };

        let err = result.validate_expectations(&contract).unwrap_err();

        assert!(matches!(
            err,
            BpiError::UnsupportedResponse { .. } | BpiError::InvalidParameter { .. }
        ));
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
            contract: "login.vip_info.vip".to_string(),
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
