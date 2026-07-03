//! 视频流地址相关接口 (web端)
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

use super::params::VideoPlayUrlParams;

pub(crate) const PLAY_URL_ENDPOINT: &str = "https://api.bilibili.com/x/player/wbi/playurl";

// --- 视频流URL相关数据结构体 ---

/// DASH 流信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DashInfo {
    pub video: Vec<DashStream>,
    pub audio: Vec<DashStream>,
    #[serde(rename = "dolby")]
    pub dolby: Option<DashDolby>,
    pub flac: Option<DashFlac>,
    pub duration: u64,
}

/// DASH 流中的 Dolby 音频信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DashDolby {
    pub r#type: u8,
    pub audio: Option<Vec<DashStream>>,
}

/// DASH 流中的 FLAC 音频信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DashFlac {
    pub audio: Vec<DashStream>,
}

/// 单个 DASH 流信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DashStream {
    pub id: u64,
    #[serde(rename = "baseUrl")]
    pub base_url: String,

    #[serde(rename = "backupUrl")]
    pub backup_url: Vec<String>,
    pub bandwidth: u64,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub codecs: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    #[serde(rename = "frameRate")]
    pub frame_rate: Option<String>,
    pub sar: Option<String>,
    pub start_with_sap: Option<u8>,
    pub segment_base: Option<serde_json::Value>,
    pub md5: Option<String>,
    pub size: Option<u64>,
    pub db_type: Option<u8>,
    pub r#type: Option<String>,
    pub stream_name: Option<String>,
    pub orientation: Option<u8>,
}

/// FLV/MP4 视频分段流信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DurlInfo {
    pub order: u32,
    pub length: u64,
    pub size: u64,
    pub ahead: String,
    pub vhead: String,
    pub url: String,
    pub backup_url: Vec<String>,
}

/// 支持的格式详细信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SupportFormat {
    pub quality: u64,
    pub format: String,
    pub new_description: String,
    pub display_desc: String,
    pub superscript: String,
    pub codecs: Option<Vec<String>>,
}

/// 视频流URL响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayUrlResponseData {
    pub from: String,
    pub result: String,
    pub message: String,
    pub quality: u64,
    pub format: String,
    pub timelength: u64,
    pub accept_format: String,
    pub accept_description: Vec<String>,
    pub accept_quality: Vec<u64>,
    pub video_codecid: u8,
    pub seek_param: String,
    pub seek_type: String,
    pub durl: Option<Vec<DurlInfo>>,
    pub dash: Option<DashInfo>,
    pub support_formats: Vec<SupportFormat>,
    pub high_format: Option<serde_json::Value>,
    pub last_play_time: u64,
    pub last_play_cid: u64,
}

// --- API 实现 ---

impl BpiClient {
    /// 获取视频流地址（web端）
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/video/videostream_url.html#获取视频流地址)
    ///
    pub async fn video_playurl(
        &self,
        params: VideoPlayUrlParams,
    ) -> Result<BpiResponse<PlayUrlResponseData>, BpiError> {
        let params = self.get_wbi_sign2(params.query_pairs()).await?;

        self.get(PLAY_URL_ENDPOINT)
            .with_bilibili_headers()
            .query(&params)
            .send_bpi("获取视频流地址")
            .await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{Aid, Cid};
    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;
    use crate::{ApiEnvelope, BpiResult};
    use tracing::info;

    const TEST_AID: u64 = 113898824998659;
    const TEST_CID: u64 = 28104724389;

    fn contract() -> BpiResult<EndpointContract> {
        EndpointContract::from_slice(include_bytes!(
            "../../tests/contracts/video/playurl/play-url/contract.json"
        ))
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_video_playurl_mp4_by_aid() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let params = VideoPlayUrlParams::from_aid(Aid::new(TEST_AID)?, Cid::new(TEST_CID)?)
            .quality(64)
            .format_flags(1);
        let resp = bpi.video_playurl(params).await?;
        let data = resp.into_data()?;

        info!("MP4 视频流信息: {:?}", data);
        assert!(data.durl.is_some());
        assert_eq!(data.quality, 64);

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_video_playurl_4k() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let params = VideoPlayUrlParams::from_aid(Aid::new(TEST_AID)?, Cid::new(TEST_CID)?)
            .quality(120)
            .format_flags(16 | 128)
            .format_version(0)
            .fourk(true);
        let resp = bpi.video_playurl(params).await?;
        let data = resp.into_data()?;

        info!("4K 视频流信息: {:?}", data);
        assert!(data.dash.is_some());
        assert_eq!(data.quality, 120);

        Ok(())
    }

    #[test]
    fn video_play_url_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = contract()?;
        let params = VideoPlayUrlParams::from_bvid("BV1xx411c7mD".parse()?, Cid::new(62131)?)
            .quality(32)
            .format_flags(16)
            .format_version(0);

        assert_eq!(contract.name, "video.play_url");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(contract.request.url.as_str(), PLAY_URL_ENDPOINT);
        assert!(contract.request.auth.requires_wbi());
        assert_eq!(
            contract.request.query.get("cid").map(String::as_str),
            Some("62131")
        );
        assert_eq!(
            contract.request.query.get("bvid").map(String::as_str),
            Some("BV1xx411c7mD")
        );
        assert_eq!(
            contract.request.query.get("qn").map(String::as_str),
            Some("32")
        );
        assert_eq!(
            contract.request.query.get("fnval").map(String::as_str),
            Some("16")
        );
        assert_eq!(
            contract.request.query.get("fnver").map(String::as_str),
            Some("0")
        );
        assert_eq!(
            contract.request.query.get("platform").map(String::as_str),
            Some("pc")
        );
        assert_eq!(
            params.query_pairs(),
            vec![
                ("cid", "62131".to_string()),
                ("bvid", "BV1xx411c7mD".to_string()),
                ("qn", "32".to_string()),
                ("fnval", "16".to_string()),
                ("fnver", "0".to_string()),
                ("platform", "pc".to_string()),
            ]
        );
        assert_eq!(contract.cases.len(), 3);
        assert!(
            contract
                .cases
                .iter()
                .all(|case| case.response.api_code == Some(0))
        );
        assert!(
            contract
                .cases
                .iter()
                .all(|case| case.response.rust_model.as_deref() == Some("PlayUrlResponseData"))
        );
        Ok(())
    }

    #[test]
    fn video_play_url_response_fixtures_parse_declared_model() -> BpiResult<()> {
        let payload = ApiEnvelope::<PlayUrlResponseData>::from_slice(include_bytes!(
            "../../tests/contracts/video/playurl/play-url/responses/success.json"
        ))?
        .into_payload()?;

        assert_eq!(payload.quality, 32);
        assert_eq!(payload.format, "flv480");
        assert_eq!(
            payload
                .dash
                .as_ref()
                .and_then(|dash| dash.video.first())
                .map(|track| track.base_url.as_str()),
            Some("https://example.invalid/bilibili/playurl/redacted.m4s")
        );
        Ok(())
    }

    fn local_probe_body(profile: &str) -> Option<serde_json::Value> {
        let path = format!("target/bpi-probe-runs/video/playurl/play-url/{profile}.response.json");
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("response")
            .and_then(|response| response.get("body"))
            .cloned()
    }

    #[test]
    fn video_play_url_model_matches_local_probe_outputs_when_available() -> BpiResult<()> {
        for profile in ["anonymous", "normal", "vip"] {
            let Some(body) = local_probe_body(profile) else {
                continue;
            };
            let payload =
                serde_json::from_value::<ApiEnvelope<PlayUrlResponseData>>(body)?.into_payload()?;

            assert_eq!(payload.quality, 32);
            assert!(payload.dash.is_some());
        }
        Ok(())
    }
}
