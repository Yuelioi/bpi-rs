//! 视频流地址相关接口 (web端)
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

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
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/video/videostream_url.html#获取视频流地址
    ///
    /// # 参数
    /// | 名称         | 类型           | 说明                 |
    /// | ------------ | --------------| -------------------- |
    /// | `aid`        | Option<u64>   | 稿件 avid，可选      |
    /// | `bvid`       | Option<&str>  | 稿件 bvid，可选      |
    /// | `cid`        | u64           | 视频 cid             |
    /// | `qn`         | Option<u64>   | 清晰度选择，可选     |
    /// | `fnval`      | Option<u64>   | 流格式标识，可选，默认1(MP4) |
    /// | `fnver`      | Option<u64>   | 流版本标识，可选，默认0 |
    /// | `fourk`      | Option<u8>    | 是否允许4K，可选，默认0 |
    /// | `platform`   | Option<&str>  | 平台标识，可选，默认"pc" |
    /// | `high_quality`| Option<u8>   | 是否高画质，可选     |
    /// | `try_look`   | Option<u8>    | 是否可不登录拉取高画质，可选 |
    ///
    /// `aid` 和 `bvid` 必须提供一个。
    pub async fn video_playurl(
        &self,
        aid: Option<u64>,
        bvid: Option<&str>,
        cid: u64,
        qn: Option<u64>,
        fnval: Option<u64>,
        fnver: Option<u64>,
        fourk: Option<u8>,
        platform: Option<&str>,
        high_quality: Option<u8>,
        try_look: Option<u8>,
    ) -> Result<BpiResponse<PlayUrlResponseData>, BpiError> {
        if aid.is_none() && bvid.is_none() {
            return Err(BpiError::parse("必须提供 aid 或 bvid"));
        }

        let mut params = vec![("cid", cid.to_string())];

        if let Some(a) = aid {
            params.push(("avid", a.to_string()));
        }
        if let Some(b) = bvid {
            params.push(("bvid", b.to_string()));
        }
        if let Some(q) = qn {
            params.push(("qn", q.to_string()));
        }
        if let Some(f) = fnval {
            params.push(("fnval", f.to_string()));
        }
        if let Some(f) = fnver {
            params.push(("fnver", f.to_string()));
        }
        if let Some(f) = fourk {
            params.push(("fourk", f.to_string()));
        }
        if let Some(p) = platform {
            params.push(("platform", p.to_string()));
        } else {
            params.push(("platform", "pc".to_string()));
        }
        if let Some(h) = high_quality {
            params.push(("high_quality", h.to_string()));
        }
        if let Some(t) = try_look {
            params.push(("try_look", t.to_string()));
        }

        // 签名
        let params = self.get_wbi_sign2(params).await?;

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
    use tracing::info;

    const TEST_AID: u64 = 113898824998659;
    const TEST_CID: u64 = 28104724389;

    #[tokio::test]

    async fn test_video_playurl_mp4_by_aid() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        // 请求 MP4 格式，720P
        let resp = bpi
            .video_playurl(
                Some(TEST_AID),
                None,
                TEST_CID,
                Some(64),
                Some(1),
                None,
                None,
                None,
                None,
                None,
            )
            .await?;
        let data = resp.into_data()?;

        info!("MP4 视频流信息: {:?}", data);
        assert!(!data.durl.is_none());
        assert_eq!(data.quality, 64);

        Ok(())
    }

    #[tokio::test]

    async fn test_video_playurl_4k() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        // 请求 4K
        let resp = bpi
            .video_playurl(
                Some(TEST_AID),
                None,
                TEST_CID,
                Some(120),
                Some(16 | 128),
                Some(0),
                Some(1),
                None,
                None,
                None,
            )
            .await?;
        let data = resp.into_data()?;

        info!("4K 视频流信息: {:?}", data);
        assert!(!data.dash.is_none());
        assert_eq!(data.quality, 120);

        Ok(())
    }
}
