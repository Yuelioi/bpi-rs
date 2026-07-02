//! 弹幕快照（最近产生的几条弹幕，最多20条）
//!
//! [文档入口](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)

use crate::ids::{Aid, Bvid};
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

pub type SnapshotResponse = BpiResponse<Vec<String>>;

/// Parameters for `/x/v2/dm/ajax` danmaku snapshots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DanmakuSnapshotParams {
    target: DanmakuSnapshotTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DanmakuSnapshotTarget {
    Aid(Aid),
    Bvid(Bvid),
}

impl DanmakuSnapshotParams {
    pub fn from_aid(aid: Aid) -> Self {
        Self {
            target: DanmakuSnapshotTarget::Aid(aid),
        }
    }

    pub fn from_bvid(bvid: Bvid) -> Self {
        Self {
            target: DanmakuSnapshotTarget::Bvid(bvid),
        }
    }

    pub fn query_pairs(&self) -> [(&'static str, String); 1] {
        let value = match &self.target {
            DanmakuSnapshotTarget::Aid(aid) => aid.to_string(),
            DanmakuSnapshotTarget::Bvid(bvid) => bvid.to_string(),
        };

        [("aid", value)]
    }
}

impl BpiClient {
    /// 获取弹幕快照（最近产生的若干条，最多20条）
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`DanmakuSnapshotParams`] | 稿件 aid 或 bvid 参数 |
    pub async fn danmaku_snapshot(
        &self,
        params: DanmakuSnapshotParams,
    ) -> Result<SnapshotResponse, BpiError> {
        let query = params.query_pairs();
        let resp: SnapshotResponse = self
            .get("https://api.bilibili.com/x/v2/dm/ajax")
            .query(&query)
            .send_bpi("获取弹幕快照")
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::danmaku::DanmakuSnapshotParams;
    use crate::ids::{Aid, Bvid};

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_danmaku_snapshot() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let bvid: Bvid = "BV1fK4y1t741".parse()?;
        let params = DanmakuSnapshotParams::from_bvid(bvid);
        let result = bpi.danmaku_snapshot(params).await?;

        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        Ok(())
    }

    #[test]
    fn danmaku_snapshot_params_serializes_bvid_query() -> Result<(), BpiError> {
        let bvid: Bvid = "BV1fK4y1t741".parse()?;
        let params = DanmakuSnapshotParams::from_bvid(bvid);

        assert_eq!(params.query_pairs(), [("aid", "BV1fK4y1t741".to_string())]);
        Ok(())
    }

    #[test]
    fn danmaku_snapshot_params_serializes_aid_query() -> Result<(), BpiError> {
        let params = DanmakuSnapshotParams::from_aid(Aid::new(170001)?);

        assert_eq!(params.query_pairs(), [("aid", "170001".to_string())]);
        Ok(())
    }
}
