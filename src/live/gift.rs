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
    pub gift_config: Option<GiftBaseConfig>,
    /// 新版礼物面板数据
    pub gift_data: Option<serde_json::Value>,
    /// 全局配置
    pub global_config: Option<serde_json::Value>,
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
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `room_id` | i64 | 直播间 ID |
    /// | `area_parent_id` | `Option<i32>` | 分区 ID |
    /// | `area_id` | `Option<i32>` | 子分区 ID |
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
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live)
    ///
    /// # 参数
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
    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;
    use crate::{ApiEnvelope, BpiResult};

    fn contract(endpoint: &str) -> BpiResult<EndpointContract> {
        let bytes = match endpoint {
            "room-gift-list" => {
                include_bytes!("../../tests/contracts/live/gift-read/room-gift-list/contract.json")
                    .as_slice()
            }
            "blind-gift-info" => {
                include_bytes!("../../tests/contracts/live/gift-read/blind-gift-info/contract.json")
                    .as_slice()
            }
            _ => unreachable!("unknown live gift contract endpoint"),
        };

        EndpointContract::from_slice(bytes)
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_room_gift_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi.live_room_gift_list(23174842, None, None).await?;

        let data = resp.data.unwrap();
        if let Some(gift_config) = data.gift_config {
            assert!(!gift_config.base_config.list.is_empty());
        } else {
            assert!(data.gift_data.is_some());
        }
        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_blind_gift_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi.live_blind_gift_info(32251).await?;

        let data = resp.data.unwrap();
        assert!(!data.gifts.is_empty());
        Ok(())
    }

    #[test]
    fn live_room_gift_list_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = contract("room-gift-list")?;

        assert_eq!(contract.name, "live.room_gift_list");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.live.bilibili.com/xlive/web-room/v1/giftPanel/roomGiftList"
        );
        assert_eq!(
            contract.request.query.get("room_id").map(String::as_str),
            Some("23174842")
        );
        assert_eq!(
            contract.request.query.get("platform").map(String::as_str),
            Some("web")
        );
        assert_eq!(contract.cases.len(), 3);
        assert_eq!(
            contract.cases[0].response.rust_model.as_deref(),
            Some("RoomGiftData")
        );
        Ok(())
    }

    #[test]
    fn live_blind_gift_info_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = contract("blind-gift-info")?;

        assert_eq!(contract.name, "live.blind_gift_info");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.live.bilibili.com/xlive/general-interface/v1/blindFirstWin/getInfo"
        );
        assert_eq!(
            contract.request.query.get("gift_id").map(String::as_str),
            Some("32251")
        );
        assert_eq!(contract.cases.len(), 3);
        assert_eq!(contract.cases[0].response.api_code, -101);
        assert_eq!(
            contract.cases[1].response.rust_model.as_deref(),
            Some("BlindGiftData")
        );
        Ok(())
    }

    #[test]
    fn live_gift_response_fixtures_parse_declared_models() -> BpiResult<()> {
        let room_gift = ApiEnvelope::<RoomGiftData>::from_slice(include_bytes!(
            "../../tests/contracts/live/gift-read/room-gift-list/responses/success.json"
        ))?
        .into_payload()?;
        assert!(room_gift.gift_config.is_none());
        assert!(room_gift.gift_data.is_some());

        let err = ApiEnvelope::<serde_json::Value>::from_slice(include_bytes!(
            "../../tests/contracts/live/gift-read/blind-gift-info/responses/anonymous.requires_login.json"
        ))?
        .ensure_success()
        .unwrap_err();
        assert!(err.requires_login());

        let blind = ApiEnvelope::<BlindGiftData>::from_slice(include_bytes!(
            "../../tests/contracts/live/gift-read/blind-gift-info/responses/authenticated.success.json"
        ))?
        .into_payload()?;
        assert_eq!(blind.gifts.len(), 1);
        Ok(())
    }

    fn local_probe_body(endpoint: &str, profile: &str) -> Option<serde_json::Value> {
        let path =
            format!("target/bpi-probe-runs/live/gift-read/{endpoint}/{profile}.response.json");
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("response")
            .and_then(|response| response.get("body"))
            .cloned()
    }

    #[test]
    fn live_gift_models_match_local_probe_outputs_when_available() -> BpiResult<()> {
        for profile in ["anonymous", "normal", "vip"] {
            if let Some(body) = local_probe_body("room-gift-list", profile) {
                let payload =
                    serde_json::from_value::<ApiEnvelope<RoomGiftData>>(body)?.into_payload()?;
                assert!(payload.gift_config.is_some() || payload.gift_data.is_some());
            }

            if let Some(body) = local_probe_body("blind-gift-info", profile) {
                let envelope = serde_json::from_value::<ApiEnvelope<BlindGiftData>>(body)?;
                if profile == "anonymous" {
                    let err = envelope.ensure_success().unwrap_err();
                    assert!(err.requires_login());
                } else {
                    let payload = envelope.into_payload()?;
                    assert!(!payload.gifts.is_empty());
                }
            }
        }
        Ok(())
    }
}
