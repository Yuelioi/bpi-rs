//! 购买漫画章节
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/manga/Comic.md)

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

#[cfg(test)]
mod tests {}
