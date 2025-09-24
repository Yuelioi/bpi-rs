//! 视频 TAG 相关接口
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// --- 响应数据结构体 ---

/// 视频 TAG 信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VideoTag {
    /// tag ID, 当 tag_type 不为 bgm 时有效
    pub tag_id: Option<u64>,
    /// TAG 名称
    pub tag_name: String,
    /// 背景音乐 ID, 当 tag_type 为 bgm 时有效
    pub music_id: Option<String>,
    /// TAG 类型, old_channel: 普通标签, topic: 话题, bgm: 背景音乐
    pub tag_type: String,
    /// 跳转 url, 当 tag_type 为 topic 或 bgm 时有效
    pub jump_url: Option<String>,
}

impl BpiClient {
    /// 获取视频 TAG 信息（新版）
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/video/tags.html#获取视频tag信息)
    ///
    /// # 参数
    /// | 名称   | 类型         | 说明                 |
    /// | ------ | ------------| -------------------- |
    /// | `aid`  | `Option<u64>` | 稿件 avid，可选      |
    /// | `bvid` | `Option<&str>`| 稿件 bvid，可选      |
    /// | `cid`  | `Option<u64>` | 分P cid，可选        |
    ///
    /// `aid` 和 `bvid` 必须提供一个。
    pub async fn video_tags(
        &self,
        aid: Option<u64>,
        bvid: Option<&str>,
        cid: Option<u64>
    ) -> Result<BpiResponse<Vec<VideoTag>>, BpiError> {
        if aid.is_none() && bvid.is_none() {
            return Err(BpiError::parse("必须提供 aid 或 bvid"));
        }

        let mut req = self.get("https://api.bilibili.com/x/web-interface/view/detail/tag");

        if let Some(a) = aid {
            req = req.query(&[("aid", &a.to_string())]);
        }
        if let Some(b) = bvid {
            req = req.query(&[("bvid", b)]);
        }
        if let Some(c) = cid {
            req = req.query(&[("cid", &c.to_string())]);
        }

        req.send_bpi("获取视频 TAG 信息").await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    const TEST_AID: u64 = 89772773;
    const TEST_BVID: &str = "BV1M741177Kg";
    const TEST_CID: u64 = 153322313;

    #[tokio::test]
    async fn test_video_tags_by_aid() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.video_tags(Some(TEST_AID), None, Some(TEST_CID)).await?;
        let data = resp.into_data()?;

        info!("视频 TAG 列表: {:?}", data);

        assert!(!data.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_video_tags_by_bvid() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.video_tags(None, Some(TEST_BVID), None).await?;
        let data = resp.into_data()?;

        info!("视频 TAG 列表: {:?}", data);

        assert!(!data.is_empty());

        Ok(())
    }
}
