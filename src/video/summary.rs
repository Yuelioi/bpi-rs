//! 视频 AI 总结相关接口
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// --- 响应数据结构体 ---

/// AI 总结提纲分段要点
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiSummaryPartOutline {
    /// 要点起始时间，单位为秒
    pub timestamp: u64,
    /// 小结内容
    pub content: String,
}

/// AI 总结提纲
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiSummaryOutline {
    /// 分段标题
    pub title: String,
    /// 分段要点
    pub part_outline: Vec<AiSummaryPartOutline>,
    /// 分段起始时间，单位为秒
    pub timestamp: u64,
}

/// AI 总结摘要内容
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiSummaryModelResult {
    /// 数据类型, 0: 没有摘要, 1: 仅有摘要总结, 2: 有摘要及提纲
    pub result_type: u8,
    /// 视频摘要
    pub summary: String,
    /// 分段提纲
    pub outline: Option<Vec<AiSummaryOutline>>,
}

/// 视频 AI 总结响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AiSummaryResponseData {
    /// 返回值, -1: 不支持 AI 摘要, 0: 有摘要, 1: 无摘要
    pub code: i8,
    /// 摘要内容
    pub model_result: Option<AiSummaryModelResult>,
    /// 摘要 id
    pub stid: Option<String>,
    pub status: Option<u8>,
    /// 点赞数
    pub like_num: u64,
    /// 点踩数
    pub dislike_num: u64,
}

impl BpiClient {
    /// 获取视频 AI 总结内容
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/video/summary.html#获取视频ai总结
    ///
    /// # 参数
    /// | 名称     | 类型         | 说明                 |
    /// | -------- | ------------| -------------------- |
    /// | `aid`    | Option<u64> | 稿件 avid，可选      |
    /// | `bvid`   | Option<&str>| 稿件 bvid，可选      |
    /// | `cid`    | u64         | 视频 cid             |
    /// | `up_mid` | u64         | UP主 mid             |
    ///
    /// `aid` 和 `bvid` 必须提供一个。
    pub async fn video_ai_summary(
        &self,
        aid: Option<u64>,
        bvid: Option<&str>,
        cid: u64,
        up_mid: u64,
    ) -> Result<BpiResponse<AiSummaryResponseData>, BpiError> {
        if aid.is_none() && bvid.is_none() {
            return Err(BpiError::parse("必须提供 aid 或 bvid"));
        }

        let mut params = vec![("cid", cid.to_string()), ("up_mid", up_mid.to_string())];

        if let Some(a) = aid {
            params.push(("aid", a.to_string()));
        }

        if let Some(b) = bvid {
            params.push(("bvid", b.to_string()));
        }

        let wbi_params = self.get_wbi_sign2(params).await?;

        let req = self
            .get("https://api.bilibili.com/x/web-interface/view/conclusion/get")
            .query(&wbi_params);

        req.send_bpi("获取视频 AI 总结内容").await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    const TEST_AID: u64 = 10001;

    const TEST_CID: u64 = 16546;
    const TEST_UP_MID: u64 = 34893;

    #[tokio::test]

    async fn test_video_ai_summary_by_aid() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi
            .video_ai_summary(Some(TEST_AID), None, TEST_CID, TEST_UP_MID)
            .await?;
        let data = resp.into_data()?;

        info!("视频 AI 总结: {:?}", data);

        if data.code == 0 {
            assert!(data.model_result.is_some());
        }

        Ok(())
    }
}
