//! 弹幕快照（最近产生的几条弹幕，最多20条）
//!
//! 文档入口: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

pub type SnapshotResponse = BpiResponse<Vec<String>>;

impl BpiClient {
    /// 获取弹幕快照（最近产生的若干条，最多20条）
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `aid_or_bvid` | &str | 可传 `avid` (数字) 或 `bvid` (如 `BV...`) |
    pub async fn danmaku_snapshot(&self, aid_or_bvid: &str) -> Result<SnapshotResponse, BpiError> {
        let resp: SnapshotResponse = self
            .get("https://api.bilibili.com/x/v2/dm/ajax")
            .query(&[("aid", aid_or_bvid.to_string())])
            .send_bpi("获取弹幕快照")
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_danmaku_snapshot() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.danmaku_snapshot("BV1fK4y1t741").await?;

        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        Ok(())
    }
}
