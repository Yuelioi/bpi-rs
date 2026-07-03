use serde::{Deserialize, Serialize};

use crate::probe::contract::{ApiContract, ContractAuth, ContractRequest};
use crate::{BpiError, BpiResult};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EndpointContract {
    pub name: String,
    pub request: ContractRequest,
    pub cases: Vec<EndpointCase>,
}

impl EndpointContract {
    pub fn from_slice(bytes: &[u8]) -> BpiResult<Self> {
        let raw: RawEndpointContract = serde_json::from_slice(bytes)?;
        let contract: EndpointContract = raw.try_into()?;
        if contract.cases.is_empty() {
            return Err(BpiError::invalid_parameter(
                "cases",
                "endpoint contract must contain at least one response case",
            ));
        }
        Ok(contract)
    }
}

#[derive(Debug, Deserialize)]
struct RawEndpointContract {
    name: String,
    request: serde_json::Value,
    cases: Vec<EndpointCase>,
}

impl TryFrom<RawEndpointContract> for EndpointContract {
    type Error = BpiError;

    fn try_from(raw: RawEndpointContract) -> Result<Self, Self::Error> {
        let request_contract = ApiContract::from_value(serde_json::json!({
            "name": raw.name,
            "request": raw.request,
            "expect": {}
        }))?;

        Ok(Self {
            name: request_contract.name,
            request: request_contract.request,
            cases: raw.cases,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointCase {
    pub name: String,
    #[serde(default)]
    pub profile: Option<String>,
    #[serde(default)]
    pub auth: ContractAuth,
    pub response: EndpointResponse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointResponse {
    #[serde(default)]
    pub http_status: Option<u16>,
    #[serde(default)]
    pub api_code: Option<i32>,
    #[serde(default)]
    pub api_code_text: Option<String>,
    #[serde(default)]
    pub fixture: Option<String>,
    #[serde(default)]
    pub fixture_kind: Option<String>,
    #[serde(default)]
    pub rust_model: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

#[cfg(all(test, feature = "login"))]
mod tests {
    use super::*;
    use crate::login::LoginVipInfo;
    use crate::response::ApiEnvelope;

    #[test]
    fn endpoint_contract_groups_profiles_under_one_request() -> Result<(), BpiError> {
        let contract = EndpointContract::from_slice(include_bytes!(
            "../../tests/contracts/login/vip-info/contract.json"
        ))?;

        assert_eq!(contract.name, "login.vip_info");
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.bilibili.com/x/vip/web/user/info"
        );
        assert_eq!(contract.cases.len(), 3);
        assert_eq!(contract.cases[0].name, "anonymous");
        assert_eq!(contract.cases[0].response.api_code, Some(-101));
        assert_eq!(
            contract.cases[2].response.fixture.as_deref(),
            Some("responses/vip.success.json")
        );
        Ok(())
    }

    #[test]
    fn endpoint_contract_success_fixture_parses_declared_rust_model() -> Result<(), BpiError> {
        let contract = EndpointContract::from_slice(include_bytes!(
            "../../tests/contracts/login/vip-info/contract.json"
        ))?;
        let vip_case = contract
            .cases
            .iter()
            .find(|case| case.name == "vip")
            .ok_or_else(|| BpiError::unsupported_response("missing vip contract case"))?;

        assert_eq!(
            vip_case.response.rust_model.as_deref(),
            Some("LoginVipInfo")
        );

        let payload = ApiEnvelope::<LoginVipInfo>::from_slice(include_bytes!(
            "../../tests/contracts/login/vip-info/responses/vip.success.json"
        ))?
        .into_payload()?;

        assert!(payload.is_active());
        Ok(())
    }

    #[test]
    fn endpoint_contract_supports_non_numeric_api_code_response() -> Result<(), BpiError> {
        let contract = EndpointContract::from_slice(
            br#"{
                "name": "manga.coupons",
                "request": {
                    "method": "POST",
                    "url": "https://manga.bilibili.com/twirp/user.v1.User/GetCoupons",
                    "auth": { "requires": [] }
                },
                "cases": [
                    {
                        "name": "anonymous",
                        "profile": "anonymous",
                        "auth": { "requires": [] },
                        "response": {
                            "http_status": 401,
                            "api_code_text": "unauthenticated",
                            "fixture": "responses/anonymous.requires_login.json",
                            "fixture_kind": "probe_error_body",
                            "error": "requires_login"
                        }
                    }
                ]
            }"#,
        )?;

        let response = &contract.cases[0].response;

        assert_eq!(response.http_status, Some(401));
        assert_eq!(response.api_code, None);
        assert_eq!(response.api_code_text.as_deref(), Some("unauthenticated"));
        Ok(())
    }
}
