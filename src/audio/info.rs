//! 歌曲基本信息
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/info.md

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 歌曲基本信息数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioInfoData {
    /// 音频auid
    pub id: i64,
    /// UP主mid
    pub uid: i64,
    /// UP主昵称
    pub uname: String,
    /// 作者名
    pub author: String,
    /// 歌曲标题
    pub title: String,
    /// 封面图片url
    pub cover: String,
    /// 歌曲简介
    pub intro: String,
    /// lrc歌词url
    pub lyric: String,
    /// 1 作用尚不明确
    pub crtype: i32,
    /// 歌曲时间长度 单位为秒
    pub duration: i64,
    /// 歌曲发布时间 时间戳
    pub passtime: i64,
    /// 当前请求时间 时间戳
    pub curtime: i64,
    /// 关联稿件avid 无为0
    pub aid: i64,
    /// 关联稿件bvid 无为空
    pub bvid: String,
    /// 关联视频cid 无为0
    pub cid: i64,
    /// 0 作用尚不明确
    pub msid: i64,
    /// 0 作用尚不明确
    pub attr: i64,
    /// 0 作用尚不明确
    pub limit: i64,
    /// 0 作用尚不明确
    #[serde(rename = "activityId")]
    pub activity_id: i64,
    pub limitdesc: String,
    /// null 作用尚不明确
    pub ctime: Option<serde_json::Value>,
    /// 状态数
    pub statistic: AudioStatistic,
    /// UP主会员状态
    #[serde(rename = "vipInfo")]
    pub vip_info: AudioVipInfo,
    /// 歌曲所在的收藏夹mlid 需要登录(SESSDATA)
    #[serde(rename = "collectIds")]
    pub collect_ids: Vec<i64>,
    /// 投币数
    pub coin_num: i64,
}

/// 音频状态数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStatistic {
    /// 音频auid
    pub sid: i64,
    /// 播放次数
    pub play: i64,
    /// 收藏数
    pub collect: i64,
    /// 评论数
    pub comment: i64,
    /// 分享数
    pub share: i64,
}

/// UP主会员状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioVipInfo {
    /// 会员类型 0：无 1：月会员 2：年会员
    pub r#type: i32,
    /// 会员状态 0：无 1：有
    pub status: i32,
    /// 会员到期时间 时间戳 毫秒
    pub due_date: i64,
    /// 会员开通状态 0：无 1：有
    pub vip_pay_type: i32,
}

/// 歌曲TAG
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTag {
    /// song 作用尚不明确
    pub r#type: String,
    /// ？？？ 作用尚不明确
    pub subtype: i32,
    /// TAG id？？ 作用尚不明确
    pub key: i32,
    /// TAG名
    pub info: String,
}

/// 歌曲创作成员响应类型
pub type AudioMemberResponse = BpiResponse<Vec<AudioMemberType>>;

/// 歌曲创作成员类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMemberType {
    /// 成员列表
    pub list: Vec<AudioMember>,
    /// 成员类型代码 1：歌手 2：作词 3：作曲 4：编曲 5：后期/混音 7：封面制作 8：音源 9：调音 10：演奏 11：乐器 127：UP主
    pub r#type: i32,
}

/// 歌曲创作成员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMember {
    /// 0 作用尚不明确
    pub mid: i64,
    /// 成员名
    pub name: String,
    /// 成员id？？ 作用尚不明确
    pub member_id: i64,
}

impl BpiClient {
    /// 查询歌曲基本信息
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明              |
    /// | ------ | ------ | ----------------- |
    /// | `sid`  | u64    | 音频 auid (必要)  |
    ///
    /// # 文档
    /// [查询歌曲基本信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/info.md#查询歌曲基本信息)
    pub async fn audio_info(&self, sid: u64) -> Result<BpiResponse<AudioInfoData>, BpiError> {
        self.get("https://www.bilibili.com/audio/music-service-c/web/song/info")
            .query(&[("sid", sid.to_string())])
            .send_bpi("查询歌曲基本信息")
            .await
    }

    /// 查询歌曲 TAG
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明              |
    /// | ------ | ------ | ----------------- |
    /// | `sid`  | u64    | 音频 auid (必要)  |
    ///
    /// # 文档
    /// [查询歌曲 TAG](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/info.md#查询歌曲tag)
    pub async fn audio_tags(&self, sid: u64) -> Result<BpiResponse<Vec<AudioTag>>, BpiError> {
        self.get("https://www.bilibili.com/audio/music-service-c/web/tag/song")
            .query(&[("sid", sid.to_string())])
            .send_bpi("查询歌曲TAG")
            .await
    }

    /// 查询歌曲创作成员列表
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明              |
    /// | ------ | ------ | ----------------- |
    /// | `sid`  | u64    | 音频 auid (必要)  |
    ///
    /// # 文档
    /// [查询歌曲创作成员列表](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/info.md#查询歌曲创作成员列表)
    pub async fn audio_members(&self, sid: u64) -> Result<AudioMemberResponse, BpiError> {
        self.get("https://www.bilibili.com/audio/music-service-c/web/member/song")
            .query(&[("sid", sid.to_string())])
            .send_bpi("查询歌曲创作成员列表")
            .await
    }

    /// 获取歌曲歌词
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明              |
    /// | ------ | ------ | ----------------- |
    /// | `sid`  | u64    | 音频 auid (必要)  |
    ///
    /// # 文档
    /// [获取歌曲歌词](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/info.md#获取歌曲歌词)
    pub async fn audio_lyric(&self, sid: u64) -> Result<BpiResponse<String>, BpiError> {
        self.get("https://www.bilibili.com/audio/music-service-c/web/song/lyric")
            .query(&[("sid", sid.to_string())])
            .send_bpi("获取歌曲歌词")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_SID: u64 = 13603;
    #[tokio::test]
    async fn test_audio_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.audio_info(TEST_SID).await?;
        let data = result.data.unwrap();
        assert!(!data.title.is_empty());
        assert!(!data.author.is_empty());
        assert!(data.duration > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_audio_tags() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.audio_tags(TEST_SID).await?;
        let data = result.into_data()?;

        tracing::info!("{:#?}", data);

        Ok(())
    }

    #[tokio::test]
    async fn test_audio_members() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.audio_members(TEST_SID).await?;
        let data = result.into_data()?;

        tracing::info!("{:#?}", data);

        Ok(())
    }

    #[tokio::test]
    async fn test_audio_lyric() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let result = bpi.audio_lyric(TEST_SID).await?;

        let data = result.into_data()?;

        tracing::info!("{:#?}", data);

        Ok(())
    }

    #[tokio::test]
    async fn test_audio_info_fields() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let result = bpi.audio_info(13598).await?;

        let data = &result.data.unwrap();
        assert!(data.id > 0);
        assert!(data.uid > 0);
        assert!(!data.uname.is_empty());
        assert!(!data.title.is_empty());
        assert!(data.duration > 0);
        assert!(data.passtime > 0);

        let stats = &data.statistic;
        assert!(stats.sid > 0);
        assert!(stats.play >= 0);
        assert!(stats.collect >= 0);

        Ok(())
    }
}
