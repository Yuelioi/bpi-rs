//! 音频状态数
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/audio/status_number.md)
use crate::audio::params::AudioSongParams;
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStatusNumberData {
    pub sid: i64,
    pub play: i64,
    pub collect: i64,
    pub comment: i64,
    pub share: i64,
}

impl BpiClient {
    /// 查询歌曲状态数
    ///
    /// 唯缺投币数2333333
    ///
    /// # 参数
    /// | 名称     | 类型              | 说明           |
    /// | -------- | ----------------- | -------------- |
    /// | `params` | `AudioSongParams` | 音频 auid 参数 |
    ///
    /// # 文档
    /// [歌曲状态数](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/audio/status_number.md#歌曲状态数)
    pub async fn audio_status_number(
        &self,
        params: AudioSongParams,
    ) -> Result<BpiResponse<AudioStatusNumberData>, BpiError> {
        self.get("https://www.bilibili.com/audio/music-service-c/web/stat/song")
            .query(&params.query_pairs())
            .send_bpi("查询歌曲状态数")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::AudioId;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_audio_status_number() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new().expect("client should build");
        let result = bpi
            .audio_status_number(AudioSongParams::new(AudioId::new(15664)?))
            .await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert_eq!(data.sid, 15664);
        assert!(data.play >= 0);
        assert!(data.collect >= 0);
        assert!(data.comment >= 0);
        assert!(data.share >= 0);

        Ok(())
    }
}
