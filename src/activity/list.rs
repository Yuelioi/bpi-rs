//! 活动列表
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/activity/list.md)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 活动列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityListData {
    /// 活动列表
    pub list: Vec<ActivityItem>,
    /// 当前页码
    pub num: i32,
    /// 每页条数
    pub size: i32,
    /// 总条数
    pub total: i32,
}

/// 活动项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityItem {
    /// 活动 ID
    pub id: i32,
    /// 固定值 1
    pub state: i32,
    /// 开始时间 UNIX 秒级时间戳
    pub stime: i64,
    /// 结束时间 UNIX 秒级时间戳
    pub etime: i64,
    /// 创建时间? UNIX 秒级时间戳, 可能为 0
    pub ctime: i64,
    /// 修改时间? UNIX 秒级时间戳, 可能为 0
    pub mtime: i64,
    /// 活动名称
    pub name: String,
    /// 活动链接
    pub h5_url: String,
    /// 活动封面
    pub h5_cover: String,
    /// 页面名称
    pub page_name: String,
    /// 活动平台类型? 即 URL 中 `plat` 参数
    pub plat: i32,
    /// 活动描述
    pub desc: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActivityListParams {
    /// 活动平台类型，可选范围 [1, 3]，以半角逗号分隔
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plat: Option<String>,

    /// 固定值 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mold: Option<i32>,

    /// 固定值 3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<i32>,

    /// 目标页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pn: Option<i32>,

    /// 每页条数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps: Option<i32>,
}

impl Default for ActivityListParams {
    fn default() -> Self {
        Self {
            plat: Some("1,3".to_string()),
            mold: Some(0),
            http: Some(3),
            pn: Some(1),
            ps: Some(15),
        }
    }
}

impl BpiClient {
    /// 获取活动列表
    ///
    /// # 参数
    /// | 名称    | 类型   | 说明                                               |
    /// | ------- | ------ | -------------------------------------------------- |
    /// | `plat`  | u32    | 活动平台类型，可选范围 ``[1,3]``，以半角逗号分隔，默认 `1,3` |
    /// | `mold`  | u32    | 固定值 `0` (可选)                                  |
    /// | `http`  | u32    | 固定值 `3` (可选)                                  |
    /// | `pn`    | u32    | 目标页码 (可选，默认为 `1`)                        |
    /// | `ps`    | u32    | 每页条数 (可选，默认为 `15`)                       |
    ///
    /// # 文档
    /// [获取活动列表](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/activity/list.md#获取活动列表)

    pub async fn activity_list(
        &self,
        plat: Option<&str>,
        mold: Option<i32>,
        http: Option<i32>,
        pn: Option<i32>,
        ps: Option<i32>
    ) -> Result<BpiResponse<ActivityListData>, BpiError> {
        let params = ActivityListParams {
            plat: plat.map(|s| s.to_string()).or_else(|| Some("1,3".to_string())),
            mold: mold.or(Some(0)),
            http: http.or(Some(3)),
            pn: pn.or(Some(1)),
            ps: ps.or(Some(15)),
        };

        let result = self
            .get("https://api.bilibili.com/x/activity/page/list")
            .query(&params)
            .send_bpi("获取活动列表").await?;

        Ok(result)
    }

    /// 获取活动列表（简化版本，使用默认参数）
    ///
    /// # 文档
    /// [获取活动列表](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/activity/list.md#获取活动列表)
    pub async fn activity_list_default(&self) -> Result<BpiResponse<ActivityListData>, BpiError> {
        self.activity_list(Some("1,3"), None, None, Some(1), Some(15)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_activity_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        // 测试获取活动列表
        let result = bpi.activity_list(Some("1,3"), None, None, Some(1), Some(4)).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert!(!data.list.is_empty());
        assert_eq!(data.num, 1);
        assert_eq!(data.size, 4);
        assert!(data.total > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_activity_list_simple() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        // 测试简化版本获取活动列表
        let result = bpi.activity_list_default().await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert!(!data.list.is_empty());
        assert_eq!(data.num, 1);
        assert_eq!(data.size, 15);

        Ok(())
    }

    #[tokio::test]
    async fn test_activity_item_fields() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let result = bpi.activity_list(Some("1,3"), None, None, Some(1), Some(1)).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        if let Some(activity) = data.list.first() {
            assert!(activity.id > 0);
            assert_eq!(activity.state, 1);
            assert!(!activity.name.is_empty());
            assert!(!activity.page_name.is_empty());
        }

        Ok(())
    }
}
