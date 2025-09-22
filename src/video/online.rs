//! 视频在线人数相关接口
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// --- 响应数据结构体 ---

/// 在线人数数据控制
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OnlineTotalShowSwitch {
    /// 展示所有终端总计人数
    pub total: bool,
    /// 展示web端实时在线人数
    pub count: bool,
}

/// 视频在线人数响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OnlineTotalResponseData {
    /// 所有终端总计人数
    pub total: String,
    /// web端实时在线人数
    pub count: String,
    /// 数据显示控制
    pub show_switch: OnlineTotalShowSwitch,
}

impl BpiClient {
    /// 获取视频在线人数（web端）
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/video/online.md
    ///
    /// # 参数
    /// | 名称    | 类型         | 说明                 |
    /// | ------- | ------------| -------------------- |
    /// | `aid`   | Option<u64> | 稿件 avid，可选      |
    /// | `bvid`  | Option<&str>| 稿件 bvid，可选      |
    /// | `cid`   | u64         | 视频 cid             |
    ///
    /// `aid` 和 `bvid` 必须提供一个。
    pub async fn video_online_total(
        &self,
        aid: Option<u64>,
        bvid: Option<&str>,
        cid: u64,
    ) -> Result<BpiResponse<OnlineTotalResponseData>, BpiError> {
        if aid.is_none() && bvid.is_none() {
            return Err(BpiError::parse("必须提供 aid 或 bvid"));
        }

        let mut req = self
            .get("https://api.bilibili.com/x/player/online/total")
            .query(&[("cid", &cid.to_string())]);

        if let Some(a) = aid {
            req = req.query(&[("aid", &a.to_string())]);
        }
        if let Some(b) = bvid {
            req = req.query(&[("bvid", b)]);
        }

        req.send_bpi("获取视频在线人数").await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    // 假设这是一个已知的视频
    const TEST_AID: u64 = 759949922;
    const TEST_CID: u64 = 392402545;
    const TEST_BVID: &str = "BV1y64y1q757";

    #[tokio::test]
    async fn test_video_online_total_by_aid() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi
            .video_online_total(Some(TEST_AID), None, TEST_CID)
            .await?;

        let data = resp.into_data()?;

        info!("视频在线人数: {:?}", data);
        assert!(!data.count.is_empty());
        assert!(!data.total.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_video_online_total_by_bvid() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi
            .video_online_total(None, Some(TEST_BVID), TEST_CID)
            .await?;

        let data = resp.into_data()?;

        info!("视频在线人数: {:?}", data);

        assert!(!data.count.is_empty());
        assert!(!data.total.is_empty());

        Ok(())
    }
}
