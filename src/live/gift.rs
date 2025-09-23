use serde::{Deserialize, Serialize};

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

// ================= 数据结构 =================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GiftItem {
    /// 礼物id
    pub id: i64,
    /// 礼物名字
    pub name: String,
    /// 价格（该值/1000的单位为元）
    pub price: i64,
    /// 类型
    pub r#type: i32,
    /// 货币类型（一般为gold，即电池）
    pub coin_type: String,
    /// 特效类型
    pub effect: i32,
    /// 礼物展示的时间
    pub stay_time: i32,
    /// 礼物动画帧数
    pub animation_frame_num: i32,
    /// 礼物描述
    pub desc: String,
    /// 礼物图片
    pub img_basic: String,
    /// 礼物gif动画
    pub gif: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GiftConfig {
    /// 礼物列表
    pub list: Vec<GiftItem>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GiftBaseConfig {
    /// 基础配置
    pub base_config: GiftConfig,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct RoomGiftData {
    /// 礼物配置
    pub gift_config: GiftBaseConfig,
}

pub type RoomGiftResponse = BpiResponse<RoomGiftData>;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BlindGiftItem {
    /// 爆出的礼物id
    pub gift_id: i64,
    /// 爆出的礼物价格
    pub price: i64,
    /// 礼物名字
    pub gift_name: String,
    /// 礼物图片
    pub gift_img: String,
    /// 概率
    pub chance: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BlindGiftData {
    /// 描述
    pub note_text: String,
    /// 盲盒价格
    pub blind_price: i64,
    /// 盲盒名字
    pub blind_gift_name: String,
    /// 盲盒礼物列表
    pub gifts: Vec<BlindGiftItem>,
}

pub type BlindGiftResponse = BpiResponse<BlindGiftData>;

// ================= 实现 =================

impl BpiClient {
    /// 获取直播间内礼物
    ///
    /// area_parent_id: 直播分区
    /// area_id: 直播子分区

    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `room_id` | i64 | 直播间 ID |
    /// | `area_parent_id` | Option<i32> | 分区 ID |
    /// | `area_id` | Option<i32> | 子分区 ID |
    pub async fn live_room_gift_list(
        &self,
        room_id: i64,
        area_parent_id: Option<i32>,
        area_id: Option<i32>,
    ) -> Result<RoomGiftResponse, BpiError> {
        let mut params: Vec<(&str, String)> = vec![
            ("room_id", room_id.to_string()),
            ("platform", "web".to_string()),
        ];

        if let Some(area_parent_id) = area_parent_id {
            params.push(("area_parent_id", area_parent_id.to_string()));
        }

        if let Some(area_id) = area_id {
            params.push(("area_id", area_id.to_string()));
        }

        let resp: RoomGiftResponse = self
            .get("https://api.live.bilibili.com/xlive/web-room/v1/giftPanel/roomGiftList")
            .query(&params)
            .send_bpi("获取直播间礼物列表")
            .await?;

        Ok(resp)
    }

    /// 获取盲盒概率
    ///

    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `gift_id` | i64 | 盲盒礼物 ID |
    pub async fn live_blind_gift_info(&self, gift_id: i64) -> Result<BlindGiftResponse, BpiError> {
        let params = [("gift_id", gift_id.to_string())];

        let resp: BlindGiftResponse = self
            .get("https://api.live.bilibili.com/xlive/general-interface/v1/blindFirstWin/getInfo")
            .query(&params)
            .send_bpi("获取盲盒概率")
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_room_gift_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let resp = bpi.live_room_gift_list(23174842, None, None).await?;

        let data = resp.data.unwrap();
        assert!(data.gift_config.base_config.list.len() > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_blind_gift_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let resp = bpi.live_blind_gift_info(32251).await?;

        let data = resp.data.unwrap();
        assert!(data.gifts.len() > 0);
        Ok(())
    }
}
