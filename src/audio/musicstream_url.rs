//! 音频流URL
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/audio/musicstream_url.md)
use crate::audio::params::{AudioStreamUrlParams, AudioStreamUrlWebParams};
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 音质参数定义
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioQuality {
    /// 流畅 128K
    Smooth = 0,
    /// 标准 192K
    Standard = 1,
    /// 高品质 320K
    HighQuality = 2,
    /// 无损 FLAC （大会员）
    Lossless = 3,
}

impl AudioQuality {
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStreamUrlWebData {
    pub sid: u64,
    pub r#type: u32,
    pub info: String,
    pub timeout: u64,
    pub size: u64,
    pub cdns: Vec<String>,
    pub qualities: Option<serde_json::Value>,
    pub title: String,
    pub cover: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStreamUrlData {
    pub sid: u64,
    pub r#type: u32,
    pub info: String,
    pub timeout: u64,
    pub size: u64,
    pub cdns: Vec<String>,
    pub qualities: Vec<AudioQualityInfo>,
    pub title: String,
    pub cover: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioQualityInfo {
    pub r#type: u32,
    pub desc: String,
    pub size: u64,
    pub bps: String,
    pub tag: String,
    pub require: u32,
    pub requiredesc: String,
}

impl BpiClient {
    /// 获取音频流 URL (web端)
    ///
    /// 注：web端无法播放完整付费歌曲，付费歌曲为 30s 试听片段
    /// 本接口仅能获取 192K 音质的音频
    ///
    /// # 参数
    /// | 名称     | 类型                      | 说明          |
    /// | -------- | ------------------------- | ------------- |
    /// | `params` | `AudioStreamUrlWebParams` | 音频流 URL 参数 |
    ///
    /// # 文档
    /// [获取音频流URL(web端)](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/audio/musicstream_url.md#获取音频流urlweb端)
    pub async fn audio_stream_url_web(
        &self,
        params: AudioStreamUrlWebParams,
    ) -> Result<BpiResponse<AudioStreamUrlWebData>, BpiError> {
        self.get("https://www.bilibili.com/audio/music-service-c/web/url")
            .query(&params.query_pairs())
            .send_bpi("获取音频流URL(web端)")
            .await
    }

    /// 获取音频流 URL（可获取付费音频）
    ///
    /// 注：付费音乐需要有带大会员或音乐包的账号登录（Cookie或 APP），否则为试听片段
    /// 无损音质需要登录的用户为会员
    ///
    /// # 参数
    /// | 名称     | 类型                   | 说明             |
    /// | -------- | ---------------------- | ---------------- |
    /// | `params` | `AudioStreamUrlParams` | 音频流 URL 参数  |
    ///
    /// # 文档
    /// [获取音频流URL（可获取付费音频）](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/audio/musicstream_url.md#获取音频流url可获取付费音频)
    pub async fn audio_stream_url(
        &self,
        params: AudioStreamUrlParams,
    ) -> Result<BpiResponse<AudioStreamUrlData>, BpiError> {
        self.get("https://api.bilibili.com/audio/music-service-c/url")
            .with_bilibili_headers()
            .query(&params.query_pairs())
            .send_bpi("获取音频流URL")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::AudioId;

    const TEST_SID: u64 = 13603;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_audio_stream_url_web() {
        let bpi = BpiClient::new().expect("client should build");
        let result = bpi
            .audio_stream_url_web(AudioStreamUrlWebParams::new(
                AudioId::new(TEST_SID).expect("valid audio id"),
            ))
            .await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.code, 0);
        let data = response.data.unwrap();

        assert_eq!(data.sid, TEST_SID);
        assert!(data.timeout > 0);
        assert!(!data.cdns.is_empty());
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_audio_stream_url() {
        let bpi = BpiClient::new().expect("client should build");
        let result = bpi
            .audio_stream_url(AudioStreamUrlParams::new(
                AudioId::new(15664).expect("valid audio id"),
                AudioQuality::HighQuality,
            ))
            .await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.code, 0);
        let data = response.data.unwrap();
        assert_eq!(data.sid, 15664);
        assert!(data.timeout > 0);
        assert!(!data.cdns.is_empty());
        assert!(!data.qualities.is_empty());
    }
}
