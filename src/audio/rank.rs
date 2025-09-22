//! 音频榜单
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/rank.md
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankPeriodData {
    pub list: std::collections::HashMap<String, Vec<AudioRankPeriod>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankPeriod {
    #[serde(rename = "ID")]
    pub id: u64,
    pub priod: u64,
    pub publish_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankDetailData {
    pub listen_fid: u64,
    pub all_fid: u64,
    pub fav_mid: u64,
    pub cover_url: String,
    pub is_subscribe: bool,
    pub listen_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankMusicListData {
    pub list: Vec<AudioRankMusicItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankMusicItem {
    pub music_id: String,
    pub music_title: String,
    pub singer: String,
    pub album: String,
    pub mv_aid: u64,
    pub mv_bvid: String,
    pub mv_cover: String,
    pub heat: u64,
    pub rank: u64,
    pub can_listen: bool,
    pub recommendation: String,
    pub creation_aid: u64,
    pub creation_bvid: String,
    pub creation_cover: String,
    pub creation_title: String,
    pub creation_up: u64,
    pub creation_nickname: String,
    pub creation_duration: u64,
    pub creation_play: u64,
    pub creation_reason: String,
    pub achievements: Vec<String>,
    pub material_id: u64,
    pub material_use_num: u64,
    pub material_duration: u64,
    pub material_show: u64,
    pub song_type: u64,
}

impl BpiClient {
    /// 获取音频榜单每期列表
    ///
    /// # 参数
    /// | 名称       | 类型          | 说明       |
    /// | ---------- | ------------- | ---------- |
    /// | `list_type`| u32  | 榜单类型 1:hot 2:origin   |
    ///
    /// # 文档
    /// [获取音频榜单每期列表](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/rank.md#获取音频榜单每期列表)
    pub async fn audio_rank_period(
        &self,
        list_type: u32,
    ) -> Result<BpiResponse<AudioRankPeriodData>, BpiError> {
        let csrf = self.csrf()?;
        let params = vec![("list_type", list_type.to_string()), ("csrf", csrf)];

        self.get("https://api.bilibili.com/x/copyright-music-publicity/toplist/all_period")
            .query(&params)
            .send_bpi("获取音频榜单每期列表")
            .await
    }

    /// 查询音频榜单单期信息
    ///
    /// # 参数
    /// | 名称      | 类型  | 说明    |
    /// | --------- | ----- | ------- |
    /// | `list_id` | u64   | 榜单 id |
    ///
    /// # 文档
    /// [查询音频榜单单期信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/rank.md#查询音频榜单单期信息)
    pub async fn audio_rank_detail(
        &self,
        list_id: u64,
    ) -> Result<BpiResponse<AudioRankDetailData>, BpiError> {
        let csrf = self.csrf()?;
        let params = vec![("list_id", list_id.to_string()), ("csrf", csrf)];

        self.get("https://api.bilibili.com/x/copyright-music-publicity/toplist/detail")
            .query(&params)
            .send_bpi("查询音频榜单单期信息")
            .await
    }

    /// 获取音频榜单单期内容
    ///
    /// # 参数
    /// | 名称      | 类型  | 说明                 |
    /// | --------- | ----- | ------------------- |
    /// | `list_id` | u64   | 榜单 id             |
    /// | `csrf`    | String| CSRF Token（可选）   |
    ///
    /// # 文档
    /// [获取音频榜单单期内容](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/rank.md#获取音频榜单单期内容)
    pub async fn audio_rank_music_list(
        &self,
        list_id: u64,
    ) -> Result<BpiResponse<AudioRankMusicListData>, BpiError> {
        let csrf = self.csrf()?;
        let params = vec![("list_id", list_id.to_string()), ("csrf", csrf)];

        self.get("https://api.bilibili.com/x/copyright-music-publicity/toplist/music_list")
            .query(&params)
            .send_bpi("获取音频榜单单期内容")
            .await
    }

    /// 订阅或退订榜单
    ///
    /// # 参数
    /// | 名称      | 类型           | 说明                       |
    /// | --------- | -------------- | -------------------------- |
    /// | `state`   | u32            | 操作代码（1：订阅，2：退订）|
    /// | `list_id` | Option<u64>    | 榜单 id（可选）            |
    ///
    /// # 文档
    /// [订阅或退订榜单](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/rank.md#订阅或退订榜单)
    pub async fn audio_rank_subscribe(
        &self,
        state: u32,
        list_id: Option<u64>,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let mut params = vec![("state", state.to_string()), ("csrf", csrf.to_string())];
        if let Some(id) = list_id {
            params.push(("list_id", id.to_string()));
        }

        self.post("https://api.bilibili.com/x/copyright-music-publicity/toplist/subscribe/update")
            .form(&params)
            .send_bpi("订阅或退订榜单")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LIST_ID: u64 = 76;

    #[tokio::test]
    async fn test_audio_rank_period() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.audio_rank_period(2).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        // 检查年份数据
        for (year, periods) in data.list {
            assert!(!year.is_empty());
            assert!(!periods.is_empty());
            for period in periods {
                assert!(period.id > 0);
                assert!(period.priod > 0);
                assert!(period.publish_time > 0);
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_audio_rank_detail() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.audio_rank_detail(TEST_LIST_ID).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert!(data.listen_fid > 0);
        assert!(data.all_fid > 0);
        assert!(data.fav_mid > 0);
        assert!(!data.cover_url.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_audio_rank_music_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.audio_rank_music_list(TEST_LIST_ID).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        for item in &data.list {
            assert!(!item.music_id.is_empty());
            assert!(!item.music_title.is_empty());
            assert!(!item.singer.is_empty());
            assert!(!item.achievements.is_empty());
        }

        Ok(())
    }

    #[tokio::test]

    async fn test_update_audio_rank_subscribe() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        bpi.audio_rank_subscribe(1, Some(76)).await?;

        Ok(())
    }
}
