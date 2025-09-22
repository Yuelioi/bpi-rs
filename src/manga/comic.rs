//! 漫画操作 API
//!
//! 文档: src/doc/manga/Comic.md

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::Serialize;

// ================= 数据结构 =================

/// 购买漫画章节请求参数
#[derive(Debug, Clone, Serialize)]
pub struct BuyEpisodeRequest {
    /// 章节id
    #[serde(rename = "epId")]
    pub ep_id: i32,

    /// 购买方式
    /// 2：漫读券
    /// 4：新人等免
    /// 5：通用券
    #[serde(rename = "buyMethod")]
    pub buy_method: i32,

    /// 漫读券id
    #[serde(rename = "couponId")]
    pub coupon_id: i32,

    /// 漫画id，buyMethod=4时必填
    #[serde(rename = "comicId", skip_serializing_if = "Option::is_none")]
    pub comic_id: Option<i32>,

    /// 自动支付状态，buyMethod=2,5时必填
    #[serde(rename = "autoPayGoldStatus", skip_serializing_if = "Option::is_none")]
    pub auto_pay_gold_status: Option<i32>,

    /// 是否预售，buyMethod=2,5时必填
    #[serde(rename = "isPresale", skip_serializing_if = "Option::is_none")]
    pub is_presale: Option<i32>,

    /// 支付金额，buyMethod=5时必填
    #[serde(rename = "payAmount", skip_serializing_if = "Option::is_none")]
    pub pay_amount: Option<i32>,
}

// ================= 实现 =================

impl BpiClient {
    /// 购买漫画章节
    ///
    /// 对应: https://manga.bilibili.com/twirp/comic.v1.Comic/BuyEpisode
    pub async fn manga_buy_episode(
        &self,
        request: BuyEpisodeRequest
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let result = self
            .post("https://manga.bilibili.com/twirp/comic.v1.Comic/BuyEpisode?platform=web")
            .json(&request)
            .send_bpi("购买漫画章节").await?;

        Ok(result)
    }

    /// 使用漫读券购买漫画章节
    ///
    /// 对应: https://manga.bilibili.com/twirp/comic.v1.Comic/BuyEpisode
    pub async fn manga_buy_episode_with_coupon(
        &self,
        ep_id: i32,
        coupon_id: i32
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let request = BuyEpisodeRequest {
            ep_id: ep_id,
            buy_method: 2,
            coupon_id: coupon_id,
            comic_id: None,
            auto_pay_gold_status: Some(2),
            is_presale: Some(0),
            pay_amount: None,
        };

        self.manga_buy_episode(request).await
    }

    /// 使用新人等免购买漫画章节
    ///
    /// 对应: https://manga.bilibili.com/twirp/comic.v1.Comic/BuyEpisode
    pub async fn manga_buy_episode_with_free(
        &self,
        comic_id: i32,
        ep_id: i32
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let request = BuyEpisodeRequest {
            ep_id: ep_id,
            buy_method: 4,
            coupon_id: 0,
            comic_id: Some(comic_id),
            auto_pay_gold_status: None,
            is_presale: None,
            pay_amount: None,
        };

        self.manga_buy_episode(request).await
    }

    /// 使用通用券购买漫画章节
    ///
    /// 对应: https://manga.bilibili.com/twirp/comic.v1.Comic/BuyEpisode
    pub async fn manga_buy_episode_with_general_coupon(
        &self,
        ep_id: i32,
        pay_amount: i32
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let request = BuyEpisodeRequest {
            ep_id: ep_id,
            buy_method: 5,
            coupon_id: 0,
            comic_id: None,
            auto_pay_gold_status: Some(2),
            is_presale: Some(0),
            pay_amount: Some(pay_amount),
        };

        self.manga_buy_episode(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manga_buy_episode_with_coupon() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let coupon_id = 12553634;
        let ep_id = 484360;

        let _result = bpi.manga_buy_episode_with_coupon(ep_id, coupon_id).await?;

        Ok(())
    }
}
