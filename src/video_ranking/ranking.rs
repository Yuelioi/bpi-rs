use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

use super::RANKING_LIST_ENDPOINT;
use super::params::VideoRankingListParams;

// --- 获取分区视频排行榜列表 ---

/// 排行榜列表中的单个视频
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingVideoItem {
    // 文档未提供具体字段，通常与视频详细信息相似
    // 这里用 serde_json::Value 代替
    #[serde(flatten)]
    pub inner: serde_json::Value,
}

/// 排行榜列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingListData {
    /// 备注信息
    pub note: String,
    /// 视频列表
    pub list: Vec<RankingVideoItem>,
}

impl BpiClient {
    /// 获取分区视频排行榜列表
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/video_ranking/ranking.html#获取分区视频排行榜列表)
    ///
    /// # 参数
    /// | 名称        | 类型           | 说明                 |
    /// | ----------- | --------------| -------------------- |
    /// | `params` | `VideoRankingListParams` | 分区和榜单类型参数 |
    pub async fn video_ranking_list(
        &self,
        params: VideoRankingListParams,
    ) -> Result<BpiResponse<RankingListData>, BpiError> {
        self.get(RANKING_LIST_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi("获取分区视频排行榜列表")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::params::VideoRankingType;
    use super::*;
    use tracing::info;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_video_ranking_list() {
        let bpi = BpiClient::new().expect("client should build");
        // 获取全站排行榜
        let resp = bpi.video_ranking_list(VideoRankingListParams::new()).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("note: {}", data.note);
            info!("排行榜视频数: {}", data.list.len());
            if let Some(first_item) = data.list.first() {
                // 因为 RankingVideoItem 使用了 serde_json::Value，这里打印原始 JSON
                info!(
                    "first item: {}",
                    serde_json::to_string_pretty(&first_item).unwrap()
                );
            }
        }
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_video_ranking_list_with_rid() {
        let bpi = BpiClient::new().expect("client should build");
        // 获取日常分区排行榜 (rid=21)
        let params = VideoRankingListParams::new()
            .with_rid(21)
            .expect("rid is valid");
        let resp = bpi.video_ranking_list(params).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("note: {}", data.note);
            info!("排行榜视频数: {}", data.list.len());
        }
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_video_ranking_list_with_type() {
        let bpi = BpiClient::new().expect("client should build");
        // 获取新人排行榜
        let params = VideoRankingListParams::new().with_type(VideoRankingType::Rookie);
        let resp = bpi.video_ranking_list(params).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("note: {}", data.note);
            info!("排行榜视频数: {}", data.list.len());
        }
    }
}
