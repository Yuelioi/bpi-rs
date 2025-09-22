use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// --- 获取入站必刷视频 ---

/// 入站必刷视频列表中的单个视频
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PreciousVideoData {
    /// 标题
    pub title: String,
    /// media_id
    pub media_id: u64,
    /// 解释（概括）
    pub explain: String,
    /// 视频列表
    pub list: Vec<serde_json::Value>, // 视频内容复杂，这里用Value代替
}

impl BpiClient {
    /// 获取入站必刷视频
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/video_ranking/precious_videos.html#获取入站必刷视频
    ///
    pub async fn video_popular_precious(&self) -> Result<BpiResponse<PreciousVideoData>, BpiError> {
        self.get("https://api.bilibili.com/x/web-interface/popular/precious")
            .send_bpi("获取入站必刷视频")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_video_popular_precious() {
        let bpi = BpiClient::new();
        let resp = bpi.video_popular_precious().await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("total lists: {}", data.list.len());
            info!("first list item: {:?}", data.list.first());
        }
    }
}
