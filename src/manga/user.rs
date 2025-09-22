//! 漫画用户操作 API
//!
//! 文档: src/doc/manga/User.md

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 漫读券信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserCoupon {
    /// 漫读券id
    #[serde(rename = "ID")]
    pub id: i32,
    /// 漫读券剩余数
    pub remain_amount: i32,
    /// 漫读券总数
    pub total_amount: u32,
}

/// 漫读券信息数据
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CouponsData {
    /// 总剩余数量
    pub total_remain_amount: i32,
    /// 用户漫读券列表
    pub user_coupons: Vec<UserCoupon>,
    /// 漫读券信息
    pub coupon_info: CouponInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CouponInfo {
    /// 拥有的漫读券数量
    pub remain_coupon: i64,
    /// 拥有的通用券数量
    pub remain_silver: i64,
    /// 拥有的商城优惠券数量
    pub remain_shop_coupon: i64,
}

/// 获取漫读券列表请求参数
#[derive(Debug, Clone, Serialize)]
pub struct GetCouponsRequest {
    /// 页数
    #[serde(rename = "pageNum")]
    pub page_num: i32,

    /// 分页大小，默认20，取值范围[1,100]
    #[serde(rename = "pageSize")]
    pub page_size: i32,

    /// 是否未过期
    #[serde(rename = "notExpired", skip_serializing_if = "Option::is_none")]
    pub not_expired: Option<bool>,

    /// 标签类型
    #[serde(rename = "tabType", skip_serializing_if = "Option::is_none")]
    pub tab_type: Option<i32>,

    /// 类型
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
}

// ================= 实现 =================

impl BpiClient {
    /// 获取拥有的漫读券列表
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/manga
    ///
    /// 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `page_num` | i32 | 页码 |
    /// | `page_size` | i32 | 分页大小，默认 20，[1,100] |
    pub async fn manga_coupons(
        &self,
        page_num: i32,
        page_size: i32,
    ) -> Result<BpiResponse<CouponsData>, BpiError> {
        let params = GetCouponsRequest {
            page_num: page_num,
            page_size: page_size,
            not_expired: Some(true),
            tab_type: Some(1),
            r#type: Some(0),
        };

        self.post("https://manga.bilibili.com/twirp/user.v1.User/GetCoupons")
            .json(&params)
            .send_bpi("获取漫读券列表")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]

    async fn test_get_manga_coupons() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let result = bpi.manga_coupons(1, 20).await?;

        tracing::info!("{:#?}", result.data.unwrap());

        Ok(())
    }
}
