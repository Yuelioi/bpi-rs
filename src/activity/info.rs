//! 活动主题信息
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/activity/info.md)
use crate::ids::Bvid;
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse, BpiResult};
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

/// Parameters for fetching activity subject information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivityInfoParams {
    sid: u64,
    bvid: Option<Bvid>,
}

impl ActivityInfoParams {
    /// Creates activity subject parameters from a non-zero activity ID.
    pub fn new(sid: u64) -> BpiResult<Self> {
        if sid == 0 {
            return Err(BpiError::invalid_parameter("sid", "sid must be non-zero"));
        }

        Ok(Self { sid, bvid: None })
    }

    /// Sets the optional source video ID.
    pub fn with_bvid(mut self, bvid: Bvid) -> Self {
        self.bvid = Some(bvid);
        self
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut params = vec![("sid", self.sid.to_string())];

        if let Some(bvid) = self.bvid.as_ref() {
            params.push(("bvid", bvid.to_string()));
        }

        params
    }
}

impl BpiClient {
    /// 获取活动主题信息
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明       |
    /// | ------ | ------ | ---------- |
    /// | `params` | `ActivityInfoParams` | 活动主题参数 |
    ///
    /// # 文档
    /// [查看API文档](<https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/activity/info.md#主题信息>)
    pub async fn activity_info(
        &self,
        params: ActivityInfoParams,
    ) -> Result<BpiResponse<ActivityInfoData>, BpiError> {
        let result = self
            .get("https://api.bilibili.com/x/activity/subject/info")
            .query(&params.query_pairs())
            .send_bpi("获取活动主题信息")
            .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_activity_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new().expect("client should build");
        let params = ActivityInfoParams::new(4017552)?
            .with_bvid("BV1mKY4e8ELy".parse().expect("bvid should be valid"));

        let result = bpi.activity_info(params).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_activity_info_without_bvid() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new().expect("client should build");
        let sid = 4017552;
        let params = ActivityInfoParams::new(sid)?;

        let result = bpi.activity_info(params).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert_eq!(data.id, sid);

        Ok(())
    }

    #[test]
    fn activity_info_params_serializes_required_query() -> Result<(), BpiError> {
        let params = ActivityInfoParams::new(4017552)?;

        assert_eq!(params.query_pairs(), vec![("sid", "4017552".to_string())]);
        Ok(())
    }

    #[test]
    fn activity_info_params_serializes_bvid_query() -> Result<(), BpiError> {
        let params = ActivityInfoParams::new(4017552)?.with_bvid("BV1mKY4e8ELy".parse()?);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("sid", "4017552".to_string()),
                ("bvid", "BV1mKY4e8ELy".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn activity_info_params_rejects_zero_sid() {
        let err = ActivityInfoParams::new(0).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "sid", .. }
        ));
    }
}
