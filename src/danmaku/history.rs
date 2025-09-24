//! 历史弹幕 API
//!
//! [文档入口](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryDatesResponseData(Vec<String>);

pub type HistoryDatesResponse = BpiResponse<Vec<String>>;

impl BpiClient {
    /// 查询历史弹幕日期
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `oid` | i64 | 视频 cid |
    /// | `month` | &str | 形如 `2006-01` |
    pub async fn danmaku_history_dates(
        &self,
        oid: i64,
        month: &str
    ) -> Result<HistoryDatesResponse, BpiError> {
        let params = vec![
            ("type", "1".to_string()),
            ("oid", oid.to_string()),
            ("month", month.to_string())
        ];

        let resp: HistoryDatesResponse = self
            .get("https://api.bilibili.com/x/v2/dm/history/index")
            .query(&params)
            .send_bpi("查询历史弹幕日期").await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_history_dates() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.danmaku_history_dates(144541892, "2022-01").await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        Ok(())
    }
}
