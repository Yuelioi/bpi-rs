use std::collections::BTreeMap;

use reqwest::RequestBuilder;

use crate::probe::account::RawProbeConfig;
use crate::probe::contract::{
    ApiContract, CapturedRequest, HttpMethod, ProbeResponse, ProbeResult,
};
use crate::{BpiClient, BpiError, BpiResult};

pub async fn execute_contract(
    contract: &ApiContract,
    accounts: &RawProbeConfig,
) -> BpiResult<ProbeResult> {
    let client = client_for_contract(contract, accounts)?;
    let request = build_request(&client, contract)?;
    let captured_request = capture_request(&request)?;
    let response = request.send().await?;
    let status = response.status().as_u16();
    let headers = collect_headers(response.headers());
    let body = response_body(response).await?;

    Ok(ProbeResult {
        contract: contract.name.clone(),
        request: captured_request,
        response: ProbeResponse {
            status,
            headers,
            body,
        },
    }
    .sanitized())
}

fn client_for_contract(contract: &ApiContract, accounts: &RawProbeConfig) -> BpiResult<BpiClient> {
    let mut builder = BpiClient::builder();

    if let Some(profile) = contract.request.auth.profile.as_deref() {
        let account = accounts
            .account(profile)
            .ok_or_else(|| BpiError::invalid_parameter("profile", "account profile not found"))?;
        builder = builder.account(account);
    } else if contract.request.auth.requires_cookie() {
        return Err(BpiError::invalid_parameter(
            "profile",
            "cookie-authenticated contracts must name an account profile",
        ));
    }

    builder.build()
}

fn build_request(client: &BpiClient, contract: &ApiContract) -> BpiResult<RequestBuilder> {
    let mut request = match contract.request.method {
        HttpMethod::Get => client.get(contract.request.url.as_str()),
        HttpMethod::Post => client.post(contract.request.url.as_str()),
    };

    if !contract.request.query.is_empty() {
        request = request.query(&contract.request.query);
    }

    for (name, value) in &contract.request.headers {
        request = request.header(name, value);
    }

    if let Some(body) = contract.request.body.as_ref() {
        request = request.json(body);
    }

    Ok(request)
}

fn capture_request(request: &RequestBuilder) -> BpiResult<CapturedRequest> {
    let request = request
        .try_clone()
        .ok_or_else(|| BpiError::invalid_parameter("request", "request cannot be cloned"))?
        .build()?;
    let query = request
        .url()
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();

    Ok(CapturedRequest {
        method: match *request.method() {
            reqwest::Method::GET => HttpMethod::Get,
            reqwest::Method::POST => HttpMethod::Post,
            _ => {
                return Err(BpiError::invalid_parameter(
                    "method",
                    "supported methods are GET and POST",
                ));
            }
        },
        url: request.url().to_string(),
        headers: collect_headers(request.headers()),
        query,
        body: None,
    })
}

fn collect_headers(headers: &reqwest::header::HeaderMap) -> BTreeMap<String, String> {
    headers
        .iter()
        .map(|(name, value)| {
            let value = value
                .to_str()
                .map(str::to_owned)
                .unwrap_or_else(|_| "<non-utf8>".to_string());
            (name.as_str().to_string(), value)
        })
        .collect()
}

async fn response_body(response: reqwest::Response) -> BpiResult<serde_json::Value> {
    let bytes = response.bytes().await?;
    match serde_json::from_slice(&bytes) {
        Ok(value) => Ok(value),
        Err(_) => Ok(serde_json::Value::String(
            String::from_utf8_lossy(&bytes).into_owned(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::BpiError;
    use crate::probe::account::{ProbeAccountConfig, ProbeAccountProfile, RawProbeConfig};
    use crate::probe::contract::ApiContract;

    #[tokio::test]
    async fn execute_rejects_cookie_contract_without_named_profile() {
        let contract = ApiContract::from_slice(
            br#"{
                "name": "login.vip_info.missing_profile",
                "request": {
                    "method": "GET",
                    "url": "https://api.bilibili.com/x/vip/web/user/info",
                    "auth": { "requires": ["cookie"] }
                }
            }"#,
        )
        .expect("contract should parse");

        let err = execute_contract(&contract, &RawProbeConfig::default())
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "profile",
                ..
            }
        ));
    }

    #[test]
    fn captured_request_includes_default_cookie_but_sanitized_output_redacts_it()
    -> Result<(), BpiError> {
        let contract = ApiContract::from_slice(
            br#"{
                "name": "login.vip_info.active",
                "request": {
                    "method": "GET",
                    "url": "https://api.bilibili.com/x/vip/web/user/info",
                    "auth": {
                        "profile": "vip",
                        "requires": ["cookie"]
                    }
                }
            }"#,
        )?;
        let config = RawProbeConfig {
            probe: ProbeAccountConfig {
                accounts: HashMap::from([(
                    "vip".to_string(),
                    ProbeAccountProfile {
                        bili_jct: "csrf".to_string(),
                        dede_user_id: "42".to_string(),
                        dede_user_id_ckmd5: "ck".to_string(),
                        sessdata: "session".to_string(),
                        buvid3: "buvid".to_string(),
                    },
                )]),
            },
        };

        let client = client_for_contract(&contract, &config)?;
        let request = build_request(&client, &contract)?;
        let captured = capture_request(&request)?;

        assert!(captured.headers.contains_key("cookie"));

        let sanitized = captured.sanitized();
        assert_eq!(sanitized.headers["cookie"], "<redacted>");
        Ok(())
    }
}
