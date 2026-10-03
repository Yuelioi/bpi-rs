use super::session::{SESSION_MESSAGES_ENDPOINT, SESSIONS_ENDPOINT};
use super::*;
use crate::message::private_msg::SendMsgData;
use crate::probe::endpoint_contract::EndpointContract;
use crate::{ApiEnvelope, BpiClient, BpiError, BpiResult};
use std::collections::BTreeMap;

fn contract(endpoint: &str) -> BpiResult<EndpointContract> {
    let bytes: &[u8] = match endpoint {
        "sessions" => {
            include_bytes!("../../tests/contracts/message/private-read/sessions/contract.json")
        }
        "session-messages" => include_bytes!(
            "../../tests/contracts/message/private-read/session-messages/contract.json"
        ),
        "send" => include_bytes!("../../tests/contracts/message/private-write/send/contract.json"),
        _ => {
            return Err(BpiError::invalid_parameter(
                "endpoint",
                "unknown test endpoint",
            ));
        }
    };
    EndpointContract::from_slice(bytes)
}

#[test]
fn private_read_requests_match_contracts() -> BpiResult<()> {
    let client = BpiClient::new()?;
    for (endpoint, url, query) in [
        (
            "sessions",
            SESSIONS_ENDPOINT,
            MessageSessionsParams::new().query_pairs(),
        ),
        (
            "session-messages",
            SESSION_MESSAGES_ENDPOINT,
            MessageSessionMessagesParams::new(1_000_001)?.query_pairs(),
        ),
    ] {
        let contract = contract(endpoint)?;
        let request = client.get(url).query(&query).build()?;
        assert_eq!(request.method().as_str(), "GET");
        assert_eq!(url, contract.request.url.as_str());
        let actual: BTreeMap<String, String> = request.url().query_pairs().into_owned().collect();
        assert_eq!(actual, contract.request.query);
        for header in &contract.request.required_headers {
            assert!(request.headers().contains_key(header));
        }
    }
    Ok(())
}

#[test]
fn private_read_cursors_preserve_units_and_reject_invalid_inputs() -> BpiResult<()> {
    for (kind, expected) in [
        (MessageSessionListType::UserAndSystem, "1"),
        (MessageSessionListType::Unfollowed, "2"),
        (MessageSessionListType::All, "4"),
    ] {
        let pairs: BTreeMap<_, _> = MessageSessionsParams::new()
            .with_session_type(kind)
            .query_pairs()
            .into_iter()
            .collect();
        assert_eq!(pairs["session_type"], expected);
    }
    let pairs: BTreeMap<_, _> = MessageSessionsParams::new()
        .with_size(1)?
        .with_begin_ts(1_700_000_000_000_000)?
        .with_end_ts(1_700_000_001_000_000)?
        .query_pairs()
        .into_iter()
        .collect();
    assert_eq!(pairs["begin_ts"], "1700000000000000");
    assert_eq!(pairs["end_ts"], "1700000001000000");
    assert_eq!(pairs["size"], "1");
    let pairs: BTreeMap<_, _> = MessageSessionMessagesParams::new(1_000_001)?
        .with_begin_seqno(10)?
        .with_end_seqno(20)?
        .with_size(2)?
        .query_pairs()
        .into_iter()
        .collect();
    assert_eq!(pairs["begin_seqno"], "10");
    assert_eq!(pairs["end_seqno"], "20");
    for size in [0, 101] {
        assert!(MessageSessionsParams::new().with_size(size).is_err());
        assert!(
            MessageSessionMessagesParams::new(1)?
                .with_size(size)
                .is_err()
        );
    }
    assert!(MessageSessionMessagesParams::new(0).is_err());
    assert!(MessageSessionsParams::new().with_begin_ts(0).is_err());
    assert!(MessageSessionsParams::new().with_end_ts(0).is_err());
    assert!(
        MessageSessionMessagesParams::new(1)?
            .with_begin_seqno(0)
            .is_err()
    );
    assert!(
        MessageSessionMessagesParams::new(1)?
            .with_end_seqno(0)
            .is_err()
    );
    Ok(())
}

#[test]
fn private_read_models_parse_sanitized_and_null_pages() -> BpiResult<()> {
    let sessions = ApiEnvelope::<MessageSessionsData>::from_slice(include_bytes!(
        "../../tests/contracts/message/private-read/sessions/responses/vip.success.json"
    ))?
    .into_payload()?;
    assert_eq!(sessions.session_list.as_ref().map(Vec::len), Some(2));
    let empty = ApiEnvelope::<MessageSessionsData>::from_slice(include_bytes!(
        "../../tests/contracts/message/private-read/sessions/responses/vip.empty.json"
    ))?
    .into_payload()?;
    assert!(empty.session_list.is_none());
    let records = ApiEnvelope::<MessageSessionMessagesData>::from_slice(include_bytes!(
        "../../tests/contracts/message/private-read/session-messages/responses/vip.success.json"
    ))?
    .into_payload()?;
    let messages = records.messages.unwrap_or_default();
    assert!(messages.iter().any(|item| item.new_face_version.is_none()));
    let mut value = serde_json::to_value(&messages[0])?;
    value["msg_type"] = 999.into();
    let unknown: MessagePrivateMessage = serde_json::from_value(value)?;
    assert_eq!(unknown.msg_type, 999);
    let empty = ApiEnvelope::<MessageSessionMessagesData>::from_slice(include_bytes!(
        "../../tests/contracts/message/private-read/session-messages/responses/vip.empty.json"
    ))?
    .into_payload()?;
    assert!(empty.messages.is_none());
    Ok(())
}

#[test]
fn private_sessions_without_last_message_parse() -> BpiResult<()> {
    let body = include_bytes!(
        "../../tests/contracts/message/private-read/sessions/responses/vip.no-last-message.json"
    );
    let data = ApiEnvelope::<MessageSessionsData>::from_slice(body)?.into_payload()?;
    assert!(data.session_list.as_ref().unwrap()[0].last_msg.is_none());
    let mut value: serde_json::Value = serde_json::from_slice(body)?;
    value["data"]["session_list"][0]
        .as_object_mut()
        .unwrap()
        .remove("last_msg");
    let data = ApiEnvelope::<MessageSessionsData>::from_slice(&serde_json::to_vec(&value)?)?
        .into_payload()?;
    assert!(data.session_list.as_ref().unwrap()[0].last_msg.is_none());
    Ok(())
}

#[test]
fn private_read_models_match_local_probe_bodies_when_available() -> BpiResult<()> {
    for endpoint in ["sessions", "session-messages"] {
        for profile in [
            "anonymous",
            "normal",
            "vip",
            "vip-empty",
            "vip-page1",
            "vip-page2",
            "vip-type1-full",
            "vip-type1-full-page8",
            "vip-type1-page1",
            "vip-type1-page2",
            "vip-type2-page1",
            "vip-type2-page2",
        ] {
            let path = format!(
                "target/bpi-probe-runs/message/private-read/{endpoint}/{profile}.response.json"
            );
            let Ok(bytes) = std::fs::read(path) else {
                continue;
            };
            let value: serde_json::Value = serde_json::from_slice(&bytes)?;
            let body = serde_json::to_vec(&value["response"]["body"])?;
            let envelope = ApiEnvelope::<serde_json::Value>::from_slice(&body)?;
            if envelope.code != 0 {
                assert!(envelope.ensure_success().unwrap_err().requires_login());
                continue;
            }
            if endpoint == "sessions" {
                ApiEnvelope::<MessageSessionsData>::from_slice(&body)?.into_payload()?;
            } else {
                ApiEnvelope::<MessageSessionMessagesData>::from_slice(&body)?.into_payload()?;
            }
        }
    }
    Ok(())
}

#[test]
fn private_read_preserves_api_errors_and_reports_success_schema_drift() {
    fn decode(body: &[u8]) -> BpiResult<MessageSessionMessagesData> {
        let response = crate::transport::TransportResponse {
            metadata: crate::transport::ResponseMetadata {
                status: 200,
                duration: std::time::Duration::ZERO,
                api_code: None,
            },
            body: bytes::Bytes::copy_from_slice(body),
        };
        response
            .decode_api_envelope::<MessageSessionMessagesData>()?
            .into_payload()
            .map(|result| result.payload)
    }
    let body = br#"{"code":-101,"data":{"messages":"private-marker"}}"#;
    assert!(decode(body).unwrap_err().requires_login());
    let body = br#"{"code":0,"data":{"messages":"private-marker"}}"#;
    let err = decode(body).unwrap_err();
    assert_eq!(err.response_body(), Some(body.as_slice()));
    assert!(!format!("{err:?} {err}").contains("private-marker"));
    assert!(matches!(
        decode(br#"{"code":0,"data":null}"#).unwrap_err(),
        BpiError::MissingData
    ));
}

#[test]
fn private_send_form_matches_contract_without_network() -> BpiResult<()> {
    let contract = contract("send")?;
    let dev_id = "00000000-0000-4000-8000-000000000000";
    let parts = MessageSendParams::text(1_000_001, "<redacted>")?.request_parts(
        "1000001",
        "fixture-csrf",
        dev_id,
        1_700_000_000,
    )?;
    assert_eq!(
        parts
            .query
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<BTreeMap<_, _>>(),
        contract.request.query
    );
    let mut expected = contract.request.form.unwrap_or_default();
    expected.insert("csrf".into(), "fixture-csrf".into());
    expected.insert("csrf_token".into(), "fixture-csrf".into());
    expected.insert("msg[timestamp]".into(), "1700000000".into());
    let client = BpiClient::new()?;
    let request = client
        .post(contract.request.url.as_str())
        .form(&parts.form)
        .build()?;
    assert_eq!(request.method().as_str(), "POST");
    let actual = request
        .body()
        .and_then(reqwest::Body::as_bytes)
        .unwrap_or_default();
    let actual: BTreeMap<_, _> = reqwest::Url::parse(&format!(
        "https://example.invalid/?{}",
        String::from_utf8_lossy(actual)
    ))
    .expect("encoded request body must form a valid URL query")
    .query_pairs()
    .into_owned()
    .collect();
    assert_eq!(actual, expected);
    let result = ApiEnvelope::<SendMsgData>::from_slice(include_bytes!(
        "../../tests/contracts/message/private-write/send/responses/synthetic.success.json"
    ))?
    .into_payload()?;
    assert_eq!(result.msg_key, Some(1_000_001));
    Ok(())
}

#[test]
fn private_send_validates_encoded_text_and_preserves_content() -> BpiResult<()> {
    assert!(MessageSendParams::text(0, "x").is_err());
    assert!(MessageSendParams::text(1, " \n ").is_err());
    assert!(MessageSendParams::text(1, "中".repeat(663)).is_err());
    assert!(MessageSendParams::text(1, "\"".repeat(1000)).is_err());
    assert!(MessageSendParams::text(1, "a".repeat(1986)).is_ok());
    let parts = MessageSendParams::text(1, " 收到\n第二行 ")?.request_parts("1", "x", "x", 1)?;
    let form: BTreeMap<_, _> = parts.form.into_iter().collect();
    let content: serde_json::Value = serde_json::from_str(&form["msg[content]"])?;
    assert_eq!(content["content"], " 收到\n第二行 ");
    Ok(())
}

#[test]
fn private_send_requires_auth_before_signing_or_network() -> BpiResult<()> {
    let client = BpiClient::new()?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("test runtime");
    let err = runtime
        .block_on(client.message().send(MessageSendParams::text(1, "x")?))
        .unwrap_err();
    assert!(matches!(err, BpiError::Auth { .. }));
    Ok(())
}

#[ignore = "真实发送：需显式开启并指定收件人与正文"]
#[tokio::test]
async fn live_private_send_requires_explicit_target_and_gate() -> BpiResult<()> {
    if std::env::var("BPI_MUTATING_TEST").as_deref() != Ok("1") {
        return Err(BpiError::invalid_parameter(
            "BPI_MUTATING_TEST",
            "set to 1 to send a real message",
        ));
    }
    let receiver = std::env::var("BPI_MESSAGE_RECEIVER_ID")
        .ok()
        .and_then(|v| v.parse().ok())
        .ok_or_else(|| {
            BpiError::invalid_parameter("BPI_MESSAGE_RECEIVER_ID", "explicit receiver required")
        })?;
    let text = std::env::var("BPI_MESSAGE_TEXT")
        .map_err(|_| BpiError::invalid_parameter("BPI_MESSAGE_TEXT", "explicit text required"))?;
    let params = MessageSendParams::text(receiver, text)?;
    let path = std::env::var("BPI_ACCOUNT_FILE").unwrap_or_else(|_| "account.toml".into());
    let profile = std::env::var("BPI_ACCOUNT_PROFILE").unwrap_or_else(|_| "vip".into());
    let account = crate::probe::account::RawProbeConfig::load(path)?
        .account(&profile)?
        .ok_or_else(BpiError::auth_required)?;
    let client = BpiClient::builder().account(account).build()?;
    let result = client.message().send(params).await?;
    assert!(result.msg_key.is_some_and(|key| key > 0));
    Ok(())
}
