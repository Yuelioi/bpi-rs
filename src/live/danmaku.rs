use serde::{Deserialize, Serialize};

// --- 弹幕发送响应数据结构体 ---

/// 弹幕发送响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendDanmuData {
    pub mode_info: Option<serde_json::Value>,
    pub dm_v2: Option<serde_json::Value>,
}

/// 直播弹幕 WebSocket 接入点（`getDanmuInfo` 返回的 `host_list` 一项）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiveDanmuInfoHost {
    pub host: String,
    #[serde(default)]
    pub port: u32,
    #[serde(default)]
    pub wss_port: u32,
    #[serde(default)]
    pub ws_port: u32,
}

/// 直播弹幕服务器信息（WebSocket token / 接入 host）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiveDanmuInfoData {
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub host_list: Vec<LiveDanmuInfoHost>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;
    use crate::{ApiEnvelope, BpiResult};

    fn contract() -> BpiResult<EndpointContract> {
        EndpointContract::from_slice(include_bytes!(
            "../../tests/contracts/live/room-interaction-read/danmu-info/contract.json"
        ))
    }

    #[test]
    fn live_danmu_info_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = contract()?;

        assert_eq!(contract.name, "live.danmu_info");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo"
        );
        assert_eq!(
            contract.request.query.get("id").map(String::as_str),
            Some("21733448")
        );
        assert_eq!(
            contract.request.query.get("type").map(String::as_str),
            Some("0")
        );
        assert!(contract.request.auth.requires_wbi());
        assert_eq!(contract.cases.len(), 3);
        assert_eq!(
            contract.cases[0].response.error.as_deref(),
            Some("wbi_risk_control")
        );
        assert_eq!(
            contract.cases[1].response.rust_model.as_deref(),
            Some("LiveDanmuInfoData")
        );
        Ok(())
    }

    #[test]
    fn live_danmu_info_response_fixtures_parse_declared_model() -> BpiResult<()> {
        let err = ApiEnvelope::<serde_json::Value>::from_slice(include_bytes!(
            "../../tests/contracts/live/room-interaction-read/danmu-info/responses/anonymous.error.json"
        ))?
        .ensure_success()
        .unwrap_err();
        assert_eq!(err.code(), Some(-352));

        let payload = ApiEnvelope::<LiveDanmuInfoData>::from_slice(include_bytes!(
            "../../tests/contracts/live/room-interaction-read/danmu-info/responses/authenticated.success.json"
        ))?
        .into_payload()?;
        assert_eq!(payload.token, "<redacted>");
        assert_eq!(payload.host_list.len(), 1);
        Ok(())
    }

    fn local_probe_body(profile: &str) -> Option<serde_json::Value> {
        let path = format!(
            "target/bpi-probe-runs/live/room-interaction-read/danmu-info/{profile}.response.json"
        );
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("response")
            .and_then(|response| response.get("body"))
            .cloned()
    }

    #[test]
    fn live_danmu_info_model_matches_local_probe_outputs_when_available() -> BpiResult<()> {
        for profile in ["anonymous", "normal", "vip"] {
            let Some(body) = local_probe_body(profile) else {
                continue;
            };
            let envelope = serde_json::from_value::<ApiEnvelope<LiveDanmuInfoData>>(body)?;

            if profile == "anonymous" {
                assert_eq!(envelope.ensure_success().unwrap_err().code(), Some(-352));
            } else {
                let payload = envelope.into_payload()?;
                assert!(!payload.token.is_empty());
                assert!(!payload.host_list.is_empty());
            }
        }
        Ok(())
    }
}
