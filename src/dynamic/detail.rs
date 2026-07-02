use crate::models::{Official, Pendant, Vip};
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

use super::{
    DynamicDetailParams, DynamicForwardItemParams, DynamicForwardsParams,
    DynamicLotteryNoticeParams, DynamicPicsParams, DynamicReactionsParams,
};
// --- 动态详情 API 结构体 ---

/// 动态详情响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicDetailData {
    pub item: DynamicDetailItem,
}

/// 动态卡片内容，作为多个 API 的共享结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicDetailItem {
    pub id_str: String,
    pub basic: DynamicBasic,

    pub modules: serde_json::Value,

    /// 当`type`字段的值为DYNAMIC_TYPE_FORWARD（转发动态）时，此字段不为null
    pub orig: Option<Box<DynamicDetailItem>>,

    pub r#type: String,

    pub visible: bool,
}

/// 动态卡片内容，作为多个 API 的共享结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicForwardItem {
    pub desc: Desc,
    pub id_str: String,
    pub pub_time: String,
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Desc {
    pub rich_text_nodes: Vec<RichTextNode>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichTextNode {
    pub orig_text: String,
    pub text: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub face: String,
    pub face_nft: bool,
    pub mid: i64,
    pub name: String,
    pub official: Official,
    pub pendant: Pendant,
    pub vip: Vip,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicBasic {
    pub comment_id_str: String,
    pub comment_type: i64,
    pub editable: Option<bool>,
    pub jump_url: Option<String>,
    pub like_icon: serde_json::Value,
    pub rid_str: String,
}

// --- 动态点赞与转发列表 API 结构体 ---

/// 点赞或转发的用户列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicReactionItem {
    pub action: String,
    /// 1: 对方仅关注了发送者    2: 发送者关注了对方
    pub attend: u8,
    pub desc: String,
    pub face: String,
    pub mid: String,
    pub name: String,
}

/// 动态点赞与转发列表响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicReactionData {
    pub has_more: bool,
    pub items: Vec<DynamicReactionItem>,
    pub offset: String,
    pub total: u64,
}

// --- 动态抽奖详情 API 结构体 ---

/// 抽奖结果
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LotteryResultItem {
    pub uid: u64,
    pub name: String,
    pub face: String,
    pub hongbao_money: Option<f64>,
}

/// 动态抽奖详情响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicLotteryData {
    pub lottery_id: u64,
    pub sender_uid: u64,
    pub business_type: u8,
    pub business_id: u64,
    pub status: u8,
    pub lottery_time: u64,
    pub participants: u64,
    pub first_prize_cmt: String,
    pub second_prize_cmt: Option<String>,
    pub third_prize_cmt: Option<String>,
    pub lottery_result: Option<serde_json::Value>, // 使用 Value 以应对可选的嵌套对象
                                                   // ... 其他字段
}

// --- 动态转发列表 API 结构体 ---

/// 动态转发列表响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicForwardData {
    pub has_more: bool,
    pub items: Vec<DynamicForwardItem>,
    pub offset: String,
    pub total: u64,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicForwardInfoData {
    pub item: DynamicForwardItem,
}

// --- 获取动态图片 API 结构体 ---

/// 动态图片信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicPic {
    pub height: u64,
    pub size: f64,
    pub src: String,
    pub width: u64,
}

/// 动态图片列表响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicPicsData {
    pub data: Vec<DynamicPic>,
}

impl BpiClient {
    /// 获取动态详情
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`DynamicDetailParams`] | 动态 ID 和详情特性参数 |
    pub async fn dynamic_detail(
        &self,
        params: DynamicDetailParams,
    ) -> Result<BpiResponse<DynamicDetailData>, BpiError> {
        let query = params.query_pairs();

        self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/detail")
            .query(&query)
            .send_bpi("获取动态详情")
            .await
    }

    /// 获取动态点赞与转发列表
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`DynamicReactionsParams`] | 动态 ID 和翻页参数 |
    pub async fn dynamic_reactions(
        &self,
        params: DynamicReactionsParams,
    ) -> Result<BpiResponse<DynamicReactionData>, BpiError> {
        let query = params.query_pairs();

        self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/detail/reaction")
            .query(&query)
            .send_bpi("获取动态点赞与转发列表")
            .await
    }

    /// 获取动态抽奖详情
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`DynamicLotteryNoticeParams`] | 动态抽奖业务参数 |
    pub async fn dynamic_lottery_notice(
        &self,
        params: DynamicLotteryNoticeParams,
    ) -> Result<BpiResponse<DynamicLotteryData>, BpiError> {
        let csrf = self.csrf()?;
        let query = params.query_pairs(&csrf);
        self.get("https://api.vc.bilibili.com/lottery_svr/v1/lottery_svr/lottery_notice")
            .query(&query)
            .send_bpi("获取动态抽奖详情")
            .await
    }

    /// 获取动态转发列表
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`DynamicForwardsParams`] | 动态 ID 和翻页参数 |
    pub async fn dynamic_forwards(
        &self,
        params: DynamicForwardsParams,
    ) -> Result<BpiResponse<DynamicForwardData>, BpiError> {
        let query = params.query_pairs();

        self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/detail/forward")
            .query(&query)
            .send_bpi("获取动态转发列表")
            .await
    }

    /// 获取动态中图片列表
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`DynamicPicsParams`] | 动态 ID 参数 |
    pub async fn dynamic_pics(
        &self,
        params: DynamicPicsParams,
    ) -> Result<BpiResponse<Vec<DynamicPic>>, BpiError> {
        let query = params.query_pairs();

        self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/detail/pic")
            .query(&query)
            .send_bpi("获取动态图片列表")
            .await
    }

    /// 获取转发动态信息
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`DynamicForwardItemParams`] | 动态 ID 参数 |
    pub async fn dynamic_forward_item(
        &self,
        params: DynamicForwardItemParams,
    ) -> Result<BpiResponse<DynamicForwardInfoData>, BpiError> {
        let query = params.query_pairs();

        self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/detail/forward/item")
            .query(&query)
            .send_bpi("获取转发动态信息")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dynamic::{
        DynamicDetailParams, DynamicForwardItemParams, DynamicForwardsParams,
        DynamicLotteryNoticeParams, DynamicPicsParams, DynamicReactionsParams,
    };
    use crate::ids::DynamicId;
    use tracing::info;

    fn parse_dynamic_id(value: &str) -> Result<DynamicId, BpiError> {
        value.parse()
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_dynamic_detail() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let dynamic_id = "1099138163191840776";
        let resp = bpi
            .dynamic_detail(DynamicDetailParams::new(parse_dynamic_id(dynamic_id)?))
            .await?;
        let data = resp.into_data()?;

        info!("动态详情: {:?}", data.item);
        assert_eq!(data.item.id_str, dynamic_id);

        let dynamic_id = "1152614216889270274"; // 此动态为陈叔叔的一条转发动态
        let resp = bpi
            .dynamic_detail(DynamicDetailParams::new(parse_dynamic_id(dynamic_id)?))
            .await?;
        let data = resp.into_data()?;
        info!("动态详情: {:?}", data.item);
        assert_eq!(data.item.id_str, dynamic_id);
        assert!(data.item.orig.is_some());

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_dynamic_reactions() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let dynamic_id = "1099138163191840776";
        let resp = bpi
            .dynamic_reactions(DynamicReactionsParams::new(parse_dynamic_id(dynamic_id)?))
            .await?;
        let data = resp.into_data()?;

        info!("点赞/转发总数: {}", data.total);
        assert!(!data.items.is_empty());

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_lottery_notice() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let dynamic_id = "969916293954142214";
        let resp = bpi
            .dynamic_lottery_notice(DynamicLotteryNoticeParams::new(parse_dynamic_id(
                dynamic_id,
            )?))
            .await?;
        let data = resp.into_data()?;

        info!("抽奖状态: {}", data.status);
        assert_eq!(data.business_id.to_string(), dynamic_id);

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_dynamic_forwards() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let dynamic_id = "1099138163191840776";
        let resp = bpi
            .dynamic_forwards(DynamicForwardsParams::new(parse_dynamic_id(dynamic_id)?))
            .await?;
        let data = resp.into_data()?;

        info!("转发总数: {}", data.total);
        assert!(!data.items.is_empty());

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_dynamic_pics() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let dynamic_id = "1099138163191840776";
        let resp = bpi
            .dynamic_pics(DynamicPicsParams::new(parse_dynamic_id(dynamic_id)?))
            .await?;
        let data = resp.into_data()?;

        info!("图片数量: {}", data.len());
        assert!(!data.is_empty());

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_forward_item() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let dynamic_id = "1110902525317349376";
        let resp = bpi
            .dynamic_forward_item(DynamicForwardItemParams::new(parse_dynamic_id(dynamic_id)?))
            .await?;
        let data = resp.into_data()?;

        info!("转发动态详情: {:?}", data.item);
        assert_eq!(data.item.id_str, dynamic_id);

        Ok(())
    }

    #[test]
    fn dynamic_detail_params_serializes_default_features() -> Result<(), BpiError> {
        let params = DynamicDetailParams::new(parse_dynamic_id("1099138163191840776")?);

        assert_eq!(
            params.query_pairs(),
            [
                ("id", "1099138163191840776".to_string()),
                (
                    "features",
                    "htmlNewStyle,itemOpusStyle,decorationCard".to_string()
                ),
            ]
        );
        Ok(())
    }

    #[test]
    fn dynamic_detail_params_serializes_custom_features() -> Result<(), BpiError> {
        let params = DynamicDetailParams::new(parse_dynamic_id("1099138163191840776")?)
            .with_features("itemOpusStyle,opusBigCover")?;

        assert_eq!(
            params.query_pairs(),
            [
                ("id", "1099138163191840776".to_string()),
                ("features", "itemOpusStyle,opusBigCover".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn dynamic_reactions_params_serializes_offset() -> Result<(), BpiError> {
        let params = DynamicReactionsParams::new(parse_dynamic_id("1099138163191840776")?)
            .with_offset("offset-token")?;

        assert_eq!(
            params.query_pairs(),
            [
                ("id", "1099138163191840776".to_string()),
                ("offset", "offset-token".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn dynamic_lottery_notice_params_serializes_csrf_query() -> Result<(), BpiError> {
        let params = DynamicLotteryNoticeParams::new(parse_dynamic_id("969916293954142214")?);

        assert_eq!(
            params.query_pairs("csrf-token"),
            [
                ("business_id", "969916293954142214".to_string()),
                ("business_type", "1".to_string()),
                ("csrf", "csrf-token".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn dynamic_pics_params_serializes_query() -> Result<(), BpiError> {
        let params = DynamicPicsParams::new(parse_dynamic_id("1099138163191840776")?);

        assert_eq!(
            params.query_pairs(),
            [("id", "1099138163191840776".to_string())]
        );
        Ok(())
    }

    #[test]
    fn dynamic_forward_item_params_serializes_query() -> Result<(), BpiError> {
        let params = DynamicForwardItemParams::new(parse_dynamic_id("1110902525317349376")?);

        assert_eq!(
            params.query_pairs(),
            [("id", "1110902525317349376".to_string())]
        );
        Ok(())
    }

    #[test]
    fn dynamic_forwards_params_rejects_blank_offset() -> Result<(), BpiError> {
        let err = DynamicForwardsParams::new(parse_dynamic_id("1099138163191840776")?)
            .with_offset("   ")
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "offset",
                ..
            }
        ));
        Ok(())
    }
}
