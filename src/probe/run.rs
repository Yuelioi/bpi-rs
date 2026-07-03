use std::collections::BTreeMap;

use base64::{Engine as _, engine::general_purpose};
use reqwest::RequestBuilder;

use crate::probe::account::RawProbeConfig;
use crate::probe::contract::{
    ApiContract, CapturedRequest, HttpMethod, ProbeResponse, ProbeResult, ResponseDecoding,
};
use crate::{BpiClient, BpiError, BpiResult};

pub async fn execute_contract(
    contract: &ApiContract,
    accounts: &RawProbeConfig,
) -> BpiResult<ProbeResult> {
    let client = client_for_contract(contract, accounts)?;
    let request = build_request(&client, contract).await?;
    let captured_request = capture_request(&request)?;
    let response = request.send().await?;
    let status = response.status().as_u16();
    let headers = collect_headers(response.headers());
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let body = response_body(response, content_type).await?;

    let result = ProbeResult {
        contract: contract.name.clone(),
        request: captured_request,
        response: ProbeResponse {
            status,
            headers,
            body,
        },
    }
    .sanitized();

    result.validate_expectations(contract)?;

    Ok(result)
}

fn client_for_contract(contract: &ApiContract, accounts: &RawProbeConfig) -> BpiResult<BpiClient> {
    let mut builder = BpiClient::builder();

    if matches!(
        contract.request.response_decoding,
        ResponseDecoding::Disabled
    ) {
        builder = builder.reqwest_client(raw_response_client()?);
    }

    if let Some(profile) = contract.request.auth.profile.as_deref() {
        let account = accounts
            .account(profile)?
            .ok_or_else(|| {
                BpiError::invalid_parameter(
                    "profile",
                    "account profile not found; configure [probe.vip]/[probe.normal] or *_vip/*_normal fields",
                )
            })?;
        builder = builder.account(account);
    } else if contract.request.auth.requires_cookie() || contract.request.auth.requires_csrf() {
        return Err(BpiError::invalid_parameter(
            "profile",
            "authenticated contracts must name an account profile",
        ));
    }

    builder.build()
}

fn raw_response_client() -> BpiResult<reqwest::Client> {
    reqwest::Client::builder()
        .no_gzip()
        .no_brotli()
        .no_deflate()
        .no_proxy()
        .pool_max_idle_per_host(0)
        .build()
        .map_err(BpiError::from)
}

async fn build_request(client: &BpiClient, contract: &ApiContract) -> BpiResult<RequestBuilder> {
    let mut request = match contract.request.method {
        HttpMethod::Get => client.get(contract.request.url.as_str()),
        HttpMethod::Post => client.post(contract.request.url.as_str()),
    };
    let variables = template_variables(client, contract)?;
    let query = render_string_map(&contract.request.query, &variables)?;

    let query = if contract.request.auth.requires_wbi() {
        client.get_wbi_sign2(query.iter()).await?
    } else {
        query.into_iter().collect()
    };

    if !query.is_empty() {
        request = request.query(&query);
    }

    let headers = render_string_map(&contract.request.headers, &variables)?;
    for (name, value) in headers {
        request = request.header(name, value);
    }

    if let Some(body) = contract.request.body.as_ref() {
        let body = render_value(body, &variables)?;
        request = request.json(&body);
    }

    Ok(request)
}

fn template_variables(
    client: &BpiClient,
    contract: &ApiContract,
) -> BpiResult<BTreeMap<String, String>> {
    let mut variables = BTreeMap::new();

    if contract.request.auth.requires_csrf() {
        variables.insert("csrf".to_string(), client.csrf()?);
    }

    Ok(variables)
}

fn render_string_map(
    values: &BTreeMap<String, String>,
    variables: &BTreeMap<String, String>,
) -> BpiResult<BTreeMap<String, String>> {
    values
        .iter()
        .map(|(key, value)| Ok((key.clone(), render_string(value, variables)?)))
        .collect()
}

fn render_value(
    value: &serde_json::Value,
    variables: &BTreeMap<String, String>,
) -> BpiResult<serde_json::Value> {
    match value {
        serde_json::Value::String(value) => {
            Ok(serde_json::Value::String(render_string(value, variables)?))
        }
        serde_json::Value::Array(values) => values
            .iter()
            .map(|value| render_value(value, variables))
            .collect::<BpiResult<Vec<_>>>()
            .map(serde_json::Value::Array),
        serde_json::Value::Object(values) => values
            .iter()
            .map(|(key, value)| Ok((key.clone(), render_value(value, variables)?)))
            .collect::<BpiResult<serde_json::Map<_, _>>>()
            .map(serde_json::Value::Object),
        value => Ok(value.clone()),
    }
}

fn render_string(input: &str, variables: &BTreeMap<String, String>) -> BpiResult<String> {
    let mut output = String::new();
    let mut rest = input;

    while let Some(start) = rest.find("${") {
        output.push_str(&rest[..start]);
        let after_open = &rest[start + 2..];
        let end = after_open.find('}').ok_or_else(|| {
            BpiError::invalid_parameter("template", "missing closing brace in contract variable")
        })?;
        let name = &after_open[..end];
        let value = variables.get(name).ok_or_else(|| {
            BpiError::invalid_parameter("template", "contract variable is not defined")
        })?;
        output.push_str(value);
        rest = &after_open[end + 1..];
    }

    output.push_str(rest);
    Ok(output)
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
        body: captured_body(request.body()),
    })
}

fn captured_body(body: Option<&reqwest::Body>) -> Option<serde_json::Value> {
    let bytes = body?.as_bytes()?;
    serde_json::from_slice(bytes).ok().or_else(|| {
        std::str::from_utf8(bytes)
            .ok()
            .map(|body| serde_json::Value::String(body.to_string()))
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

async fn response_body(
    response: reqwest::Response,
    content_type: Option<String>,
) -> BpiResult<serde_json::Value> {
    let bytes = response.bytes().await?;
    match serde_json::from_slice(&bytes) {
        Ok(value) => Ok(value),
        Err(_) => Ok(binary_response_body(&bytes, content_type)),
    }
}

fn binary_response_body(bytes: &[u8], content_type: Option<String>) -> serde_json::Value {
    serde_json::json!({
        "kind": "binary",
        "encoding": "base64",
        "content_type": content_type,
        "length": bytes.len(),
        "body_base64": general_purpose::STANDARD.encode(bytes),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BpiError;
    use crate::probe::account::{ProbeAccountConfig, ProbeAccountProfile, RawProbeConfig};
    use crate::probe::contract::ApiContract;
    use crate::sign::wbi::WbiKeys;
    use crate::sign::wbi_client::current_wbi_cache_bucket;

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

    #[tokio::test]
    async fn captured_request_includes_default_cookie_but_sanitized_output_redacts_it()
    -> Result<(), BpiError> {
        let contract = ApiContract::from_slice(
            br#"{
                "name": "login.vip_info.vip",
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
                vip: Some(ProbeAccountProfile {
                    bili_jct: "csrf".to_string(),
                    dede_user_id: "42".to_string(),
                    dede_user_id_ckmd5: "ck".to_string(),
                    sessdata: "session".to_string(),
                    buvid3: "buvid".to_string(),
                }),
                normal: None,
            },
            ..RawProbeConfig::default()
        };

        let client = client_for_contract(&contract, &config)?;
        let request = build_request(&client, &contract).await?;
        let captured = capture_request(&request)?;

        assert!(captured.headers.contains_key("cookie"));

        let sanitized = captured.sanitized();
        assert_eq!(sanitized.headers["cookie"], "<redacted>");
        Ok(())
    }

    #[tokio::test]
    async fn captured_request_records_json_body() -> Result<(), BpiError> {
        let contract = ApiContract::from_slice(
            br#"{
                "name": "manga.coupons.normal",
                "request": {
                    "method": "POST",
                    "url": "https://manga.bilibili.com/twirp/user.v1.User/GetCoupons",
                    "body": {
                        "pageNum": 1,
                        "pageSize": 20,
                        "notExpired": true,
                        "tabType": 1,
                        "type": 0
                    }
                }
            }"#,
        )?;
        let client = BpiClient::new()?;
        let request = build_request(&client, &contract).await?;
        let captured = capture_request(&request)?;

        let body = captured.body.expect("json body should be captured");
        assert_eq!(body["pageNum"], 1);
        assert_eq!(body["pageSize"], 20);
        assert_eq!(body["notExpired"], true);
        assert_eq!(body["tabType"], 1);
        assert_eq!(body["type"], 0);
        Ok(())
    }

    #[tokio::test]
    async fn build_request_injects_profile_csrf_into_json_body() -> Result<(), BpiError> {
        let contract = ApiContract::from_slice(
            br#"{
                "name": "wallet.info.normal",
                "request": {
                    "method": "POST",
                    "url": "https://pay.bilibili.com/paywallet/wallet/getUserWallet",
                    "auth": {
                        "profile": "normal",
                        "requires": ["cookie", "csrf"]
                    },
                    "body": {
                        "csrf": "${csrf}",
                        "platformType": 3,
                        "timestamp": 1700000000000,
                        "traceId": 1700000000000,
                        "version": "1.0"
                    }
                }
            }"#,
        )?;
        let config = RawProbeConfig {
            probe: ProbeAccountConfig {
                normal: Some(ProbeAccountProfile {
                    bili_jct: "normal-csrf".to_string(),
                    dede_user_id: "43".to_string(),
                    dede_user_id_ckmd5: "ck-normal".to_string(),
                    sessdata: "session-normal".to_string(),
                    buvid3: "buvid-normal".to_string(),
                }),
                vip: None,
            },
            ..RawProbeConfig::default()
        };
        let client = client_for_contract(&contract, &config)?;
        let request = build_request(&client, &contract).await?;
        let captured = capture_request(&request)?;

        assert_eq!(
            captured
                .body
                .as_ref()
                .and_then(|body| body["csrf"].as_str()),
            Some("normal-csrf")
        );
        assert_eq!(
            captured.sanitized().body.as_ref().map(|body| &body["csrf"]),
            Some(&serde_json::Value::String("<redacted>".to_string()))
        );
        Ok(())
    }

    #[test]
    fn csrf_contract_requires_named_profile() -> Result<(), BpiError> {
        let contract = ApiContract::from_slice(
            br#"{
                "name": "wallet.info.missing_profile",
                "request": {
                    "method": "POST",
                    "url": "https://pay.bilibili.com/paywallet/wallet/getUserWallet",
                    "auth": {
                        "requires": ["csrf"]
                    },
                    "body": {
                        "csrf": "${csrf}"
                    }
                }
            }"#,
        )?;

        let err = match client_for_contract(&contract, &RawProbeConfig::default()) {
            Ok(_) => {
                return Err(BpiError::unsupported_response(
                    "csrf contract without profile should be rejected",
                ));
            }
            Err(err) => err,
        };

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "profile",
                ..
            }
        ));
        Ok(())
    }

    #[tokio::test]
    async fn build_request_adds_wbi_signature_when_required() -> Result<(), BpiError> {
        let contract = ApiContract::from_slice(
            br#"{
                "name": "article.view.anonymous",
                "request": {
                    "method": "GET",
                    "url": "https://api.bilibili.com/x/article/view",
                    "query": {
                        "id": "2",
                        "gaia_source": "main_web"
                    },
                    "auth": { "requires": ["wbi"] }
                }
            }"#,
        )?;
        let client = BpiClient::new()?;
        client.wbi_key_cache().insert(
            current_wbi_cache_bucket(),
            WbiKeys::new(
                "abcdefghijklmnopqrstuvwxyz123456",
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ654321",
            )?,
        )?;

        let request = build_request(&client, &contract).await?;
        let captured = capture_request(&request)?;

        assert_eq!(captured.query.get("id").map(String::as_str), Some("2"));
        assert_eq!(
            captured.query.get("gaia_source").map(String::as_str),
            Some("main_web")
        );
        assert!(captured.query.contains_key("wts"));
        assert!(captured.query.contains_key("w_rid"));
        Ok(())
    }

    #[test]
    fn non_json_response_body_records_lossless_base64() -> Result<(), BpiError> {
        let bytes = b"\x00\x9fprotobuf";
        let body = binary_response_body(bytes, Some("application/octet-stream".to_string()));

        assert_eq!(body["kind"], "binary");
        assert_eq!(body["encoding"], "base64");
        assert_eq!(body["content_type"], "application/octet-stream");
        assert_eq!(body["length"], bytes.len());
        let encoded = body["body_base64"]
            .as_str()
            .ok_or_else(|| BpiError::unsupported_response("missing binary body"))?;
        let decoded = general_purpose::STANDARD
            .decode(encoded)
            .map_err(|err| BpiError::parse(err.to_string()))?;
        assert_eq!(decoded, bytes);
        Ok(())
    }
}
