//! 视频流地址相关接口 (web端)
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

use super::params::VideoPlayUrlParams;

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

        self.get("https://api.bilibili.com/x/player/wbi/playurl")
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
    use tracing::info;

    const TEST_AID: u64 = 113898824998659;
    const TEST_CID: u64 = 28104724389;

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
}
