//! 活动主题信息
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/activity/info.md)
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 活动主题信息数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityInfoData {
    /// 活动 id
    pub id: u64,
    /// 开始时间 UNIX 秒级时间戳
    pub stime: i64,
    /// 结束时间 UNIX 秒级时间戳
    pub etime: i64,
    /// 创建时间 UNIX 秒级时间戳
    pub ctime: i64,
    /// 修改时间 UNIX 秒级时间戳
    pub mtime: i64,
    /// 活动名称
    pub name: String,
    /// 活动链接
    pub act_url: String,
    /// 封面图片
    pub cover: String,
    /// 简介
    pub dic: String,
    /// H5 封面
    pub h5_cover: String,
    /// Android 端活动链接
    pub android_url: String,
    /// iOS 端活动链接
    pub ios_url: String,
    /// 子活动 id?
    pub child_sids: String,
    /// 仅在传入 bvid 时存在
    pub lid: Option<i64>,
}

impl BpiClient {
    /// 获取活动主题信息
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明       |
    /// | ------ | ------ | ---------- |
    /// | `sid`  | u64    | 活动 ID    |
    /// | `bvid` | String | 来源视频号 |
    ///
    /// # 文档
    /// [查看API文档](<https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/activity/info.md#主题信息>)
    pub async fn activity_info(
        &self,
        sid: u64,
        bvid: Option<&str>,
    ) -> Result<BpiResponse<ActivityInfoData>, BpiError> {
        let mut params = vec![("sid", sid.to_string())];

        if let Some(bvid) = bvid {
            params.push(("bvid", bvid.to_string()));
        }

        let result = self
            .get("https://api.bilibili.com/x/activity/subject/info")
            .query(&params)
            .send_bpi("获取活动主题信息")
            .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_activity_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let sid = 4017552;
        let bvid = Some("BV1mKY4e8ELy");

        let result = bpi.activity_info(sid, bvid).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        Ok(())
    }

    #[tokio::test]
    async fn test_activity_info_without_bvid() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let sid = 4017552;

        let result = bpi.activity_info(sid, None).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert_eq!(data.id, sid);

        Ok(())
    }
}
