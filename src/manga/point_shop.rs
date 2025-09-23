//! 积分商城
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/manga/point_shop.md

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// ================= 数据结构 =================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserPointData {
    /// 用户当前持有的点数
    pub point: String,
}

pub type UserPointResponse = BpiResponse<UserPointData>;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ProductLimit {
    /// 限制类型
    #[serde(rename = "type")]
    pub limit_type: i32,
    /// 限制ID
    pub id: i64,
    /// 限制标题
    pub title: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Product {
    /// 物品ID
    pub id: i64,
    /// 物品类型
    pub r#type: i32,
    /// 物品名称
    pub title: String,
    /// 显示的图像
    pub image: String,
    /// 库存总量
    pub amount: i32,
    /// 兑换所需点数（原价）
    pub cost: i32,
    /// 兑换所需点数（现价）
    pub real_cost: i32,
    /// 库存剩余数
    pub remain_amount: i32,
    /// 相关漫画ID
    pub comic_id: i64,
    /// 限定使用范围（漫画）
    pub limits: Vec<ProductLimit>,
    /// 折扣
    pub discount: i32,
    /// 产品类型
    pub product_type: i32,
    /// 挂件URL
    pub pendant_url: String,
    /// 挂件过期时间
    pub pendant_expire: i32,
    /// 兑换次数限制
    pub exchange_limit: i32,
    /// 地址截止时间
    pub address_deadline: String,
    /// 活动类型
    pub act_type: i32,
    /// 是否兑换过该物品
    pub has_exchanged: bool,
    /// 主优惠券截止时间
    pub main_coupon_deadline: String,
    /// 截止时间
    pub deadline: String,
    /// 点数
    pub point: String,
}

pub type ProductListResponse = BpiResponse<Vec<Product>>;

#[derive(Debug, Clone, Serialize)]
pub struct ExchangeRequest {
    /// 物品ID
    pub product_id: String,
    /// 兑换个数
    pub product_num: i32,
    /// 物品所需点数（现价）
    pub point: i32,
}

pub type ExchangeResponse = BpiResponse<serde_json::Value>;

// ================= 实现 =================

impl BpiClient {
    /// 获取当前持有点数
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/manga
    pub async fn manga_user_point(&self) -> Result<UserPointResponse, BpiError> {
        self
            .post("https://manga.bilibili.com/twirp/pointshop.v1.Pointshop/GetUserPoint")
            .send_bpi("获取当前持有点数").await
    }

    /// 获取兑换奖品列表
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/manga
    pub async fn manga_point_products(&self) -> Result<ProductListResponse, BpiError> {
        self
            .post("https://manga.bilibili.com/twirp/pointshop.v1.Pointshop/ListProduct")
            .send_bpi("获取兑换奖品列表").await
    }

    /// 兑换物品
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/manga
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `product_id` | i64 | 物品 ID |
    /// | `product_num` | i32 | 兑换数量 |
    /// | `point` | i32 | 现价所需点数 |
    pub async fn manga_point_exchange(
        &self,
        product_id: i64,
        product_num: i32,
        point: i32
    ) -> Result<ExchangeResponse, BpiError> {
        let req = ExchangeRequest {
            product_id: product_id.to_string(),
            product_num,
            point,
        };

        self
            .post("https://manga.bilibili.com/twirp/pointshop.v1.Pointshop/Exchange")
            .form(&req)
            .send_bpi("兑换物品").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_product() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let resp = bpi.manga_point_products().await?;

        let data = resp.into_data()?;
        assert!(!data.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_user_point() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let resp = bpi.manga_user_point().await?;
        let data = resp.into_data()?;

        tracing::info!("User point: {}", data.point);
        Ok(())
    }
}
