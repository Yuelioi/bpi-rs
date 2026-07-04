use serde::{Deserialize, Serialize};
use serde_json::Value;

// --- 直播间管理 API 结构体 ---

/// 开通直播间响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateRoomData {
    #[serde(rename = "roomID")]
    pub room_id: Option<String>,
}

/// 直播间信息更新响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateRoomData {
    pub sub_session_key: String,
    pub audit_info: Option<AuditInfo>,
}

/// 审核信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditInfo {
    pub audit_title_reason: String,
    pub audit_title_status: u8,
    pub audit_title: Option<String>,
    pub update_title: Option<String>,
}

/// RTMP 推流地址信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RtmpInfo {
    pub addr: String,
    pub code: String,
}

/// 开始直播响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StartLiveData {
    pub change: u8,
    pub status: String,
    pub rtmp: RtmpInfo,
    pub live_key: String,
    pub sub_session_key: String,
    pub need_face_auth: bool,
    // 其他不明确的字段都使用 Value
    pub room_type: Value,
    pub protocols: Value,
    pub notice: Value,
    pub qr: Value,
    pub service_source: String,
    pub rtmp_backup: Value,
    pub up_stream_extra: Value,
}

/// 关闭直播响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StopLiveData {
    pub change: u8,
    pub status: String,
}

/// 预更新直播间信息响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdatePreLiveInfoData {
    pub audit_info: Option<AuditInfo>,
}

/// PC直播姬版本号响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PcLiveVersionData {
    pub curr_version: String,
    pub build: u64,
    pub instruction: String,
    pub file_size: String,
    pub file_md5: String,
    pub content: String,
    pub download_url: String,
    pub hdiffpatch_switch: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;
    use crate::{ApiEnvelope, BpiResult};

    fn version_contract() -> BpiResult<EndpointContract> {
        EndpointContract::from_slice(include_bytes!(
            "../../tests/contracts/live/public-core/version/contract.json"
        ))
    }

    #[test]
    fn live_version_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = version_contract()?;

        assert_eq!(contract.name, "live.version");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.live.bilibili.com/xlive/app-blink/v1/liveVersionInfo/getHomePageLiveVersion"
        );
        assert_eq!(
            contract
                .request
                .query
                .get("system_version")
                .map(String::as_str),
            Some("2")
        );
        assert_eq!(contract.cases.len(), 3);
        assert_eq!(
            contract.cases[0].response.rust_model.as_deref(),
            Some("PcLiveVersionData")
        );
        Ok(())
    }

    #[test]
    fn live_version_response_fixture_parses_declared_model() -> BpiResult<()> {
        let payload = ApiEnvelope::<PcLiveVersionData>::from_slice(include_bytes!(
            "../../tests/contracts/live/public-core/version/responses/success.json"
        ))?
        .into_payload()?;

        assert_eq!(payload.curr_version, "7.61.0.10694");
        Ok(())
    }

    fn local_probe_body(profile: &str) -> Option<serde_json::Value> {
        let path =
            format!("target/bpi-probe-runs/live/public-core/version/{profile}.response.json");
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("response")
            .and_then(|response| response.get("body"))
            .cloned()
    }

    #[test]
    fn live_version_model_matches_local_probe_outputs_when_available() -> BpiResult<()> {
        for profile in ["anonymous", "normal", "vip"] {
            if let Some(body) = local_probe_body(profile) {
                let payload = serde_json::from_value::<ApiEnvelope<PcLiveVersionData>>(body)?
                    .into_payload()?;
                assert!(!payload.curr_version.is_empty());
            }
        }
        Ok(())
    }
}
