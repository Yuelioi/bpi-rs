use serde::Deserialize;
use std::collections::HashMap;

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

/// 分区当日投稿稿件数信息
/// 使用 `HashMap<u64, u64>` 存储，键为分区 ID，值为当日投稿数。
#[derive(Debug, Clone, Deserialize)]
pub struct OnlineRegionCount(pub HashMap<String, u64>);

/// 分区当日投稿数数据
#[derive(Debug, Clone, Deserialize)]
pub struct OnlineData {
    pub region_count: OnlineRegionCount,
}

impl BpiClient {
    /// 获取分区当日投稿稿件数

    pub async fn web_widget_online(&self) -> Result<BpiResponse<OnlineData>, BpiError> {
        self
            .get("https://api.bilibili.com/x/web-interface/online")
            .send_bpi("获取分区当日投稿数").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_get_online() {
        let bpi = BpiClient::new();
        let resp = bpi.web_widget_online().await;
        info!("响应: {:?}", resp);
        assert!(resp.is_ok());

        if let Ok(data) = resp {
            if let Some(counts) = data.data {
                for (region_id, count) in counts.region_count.0 {
                    info!("分区ID: {}, 投稿数: {}", region_id, count);
                }
            }
        }
    }
}
