use serde::{Deserialize, Serialize};

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

// ================= 数据结构 =================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GiftTypeItem {
    /// 礼物id
    pub gift_id: i64,
    /// 礼物名称
    pub gift_name: String,
    /// 瓜子数量（电池礼物为金瓜子数量，银瓜子礼物为银瓜子数量）
    #[serde(default)]
    pub price: i64,
}

impl BpiClient {
    /// 获取所有礼物列表
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live)
    pub async fn live_gift_types(&self) -> Result<BpiResponse<Vec<GiftTypeItem>>, BpiError> {
        let resp = self
            .get("https://api.live.bilibili.com/gift/v1/master/getGiftTypes")
            .send_bpi("获取所有礼物列表")
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_gift_types() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi.live_gift_types().await?;

        assert_eq!(resp.code, 0);
        Ok(())
    }
}
