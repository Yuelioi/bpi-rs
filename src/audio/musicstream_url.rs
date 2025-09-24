//! 音频流URL
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/musicstream_url.md)
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

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
    /// | 名称   | 类型  | 说明         |
    /// | ------ | ----- | ------------ |
    /// | `sid`  | u64   | 音频 auid    |
    ///
    /// # 文档
    /// [获取音频流URL(web端)](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/musicstream_url.md#获取音频流urlweb端)
    pub async fn audio_stream_url_web(
        &self,
        sid: u64
    ) -> Result<BpiResponse<AudioStreamUrlWebData>, BpiError> {
        let params = [
            ("sid", sid.to_string()),
            ("quality", "2".to_string()),
            ("privilege", "2".to_string()),
        ];

        self
            .get("https://www.bilibili.com/audio/music-service-c/web/url")
            .query(&params)
            .send_bpi("获取音频流URL(web端)").await
    }

    /// 获取音频流 URL（可获取付费音频）
    ///
    /// 注：付费音乐需要有带大会员或音乐包的账号登录（Cookie或 APP），否则为试听片段
    /// 无损音质需要登录的用户为会员
    ///
    /// # 参数
    /// | 名称     | 类型          | 说明               |
    /// | -------- | ------------- | ----------------- |
    /// | `songid` | u64           | 音频 auid         |
    /// | `quality`| AudioQuality  | 音质代码           |
    ///
    /// # 文档
    /// [获取音频流URL（可获取付费音频）](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/musicstream_url.md#获取音频流url可获取付费音频)
    pub async fn audio_stream_url(
        &self,
        songid: u64,
        quality: AudioQuality
    ) -> Result<BpiResponse<AudioStreamUrlData>, BpiError> {
        self
            .get("https://api.bilibili.com/audio/music-service-c/url")
            .with_bilibili_headers()
            .query(
                &[
                    ("songid", songid.to_string()),
                    ("quality", quality.as_u32().to_string()),
                    ("privilege", "2".to_string()),
                    ("mid", "2".to_string()),
                    ("platform", "android".to_string()),
                ]
            )
            .send_bpi("获取音频流URL").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SID: u64 = 13603;

    #[tokio::test]
    async fn test_audio_stream_url_web() {
        let bpi = BpiClient::new();
        let result = bpi.audio_stream_url_web(TEST_SID).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.code, 0);
        let data = response.data.unwrap();

        assert_eq!(data.sid, TEST_SID);
        assert!(data.timeout > 0);
        assert!(!data.cdns.is_empty());
    }

    #[tokio::test]
    async fn test_audio_stream_url() {
        let bpi = BpiClient::new();
        let result = bpi.audio_stream_url(15664, AudioQuality::HighQuality).await;
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
