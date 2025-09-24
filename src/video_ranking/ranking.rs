use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

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
    /// | `rid`       | `Option<u32>`   | 目标分区 tid，默认0(全站) |
    /// | `type_name` | `Option<&str>`  | 榜单类型 all/rookie/origin，可选 |
    pub async fn video_ranking_list(
        &self,
        rid: Option<u32>,
        type_name: Option<&str>
    ) -> Result<BpiResponse<RankingListData>, BpiError> {
        let mut request = self.get("https://api.bilibili.com/x/web-interface/ranking/v2");

        if let Some(r) = rid {
            request = request.query(&[("rid", r)]);
        }
        if let Some(t) = type_name {
            request = request.query(&[("type", t)]);
        }

        // 添加 User-Agent 以通过鉴权
        request = request.header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36"
        );

        // WBI签名相关参数文档未给出完整说明，忽略
        // request = request.query(&[("w_rid", "wbi_signature"), ("wts", "wbi_timestamp")]);

        request.send_bpi("获取分区视频排行榜列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_video_ranking_list() {
        let bpi = BpiClient::new();
        // 获取全站排行榜
        let resp = bpi.video_ranking_list(None, None).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("note: {}", data.note);
            info!("排行榜视频数: {}", data.list.len());
            if let Some(first_item) = data.list.first() {
                // 因为 RankingVideoItem 使用了 serde_json::Value，这里打印原始 JSON
                info!("first item: {}", serde_json::to_string_pretty(&first_item).unwrap());
            }
        }
    }

    #[tokio::test]
    async fn test_video_ranking_list_with_rid() {
        let bpi = BpiClient::new();
        // 获取日常分区排行榜 (rid=21)
        let resp = bpi.video_ranking_list(Some(21), None).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("note: {}", data.note);
            info!("排行榜视频数: {}", data.list.len());
        }
    }

    #[tokio::test]
    async fn test_video_ranking_list_with_type() {
        let bpi = BpiClient::new();
        // 获取新人排行榜
        let resp = bpi.video_ranking_list(None, Some("rookie")).await;

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
