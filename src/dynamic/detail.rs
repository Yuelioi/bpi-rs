use crate::models::{ Official, Pendant, Vip };
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };
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
    pub editable: bool,
    pub jump_url: String,
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
    /// | `id` | &str | 动态 ID |
    /// | `features` | `Option<&str>` | 功能特性，如 `itemOpusStyle,opusBigCover,onlyfansVote...` |
    pub async fn dynamic_detail(
        &self,
        id: &str,
        features: Option<&str>
    ) -> Result<BpiResponse<DynamicDetailData>, BpiError> {
        let mut req = self
            .get("https://api.bilibili.com/x/polymer/web-dynamic/v1/detail")
            .query(&[("id", id)]);

        if let Some(f) = features {
            req = req.query(&[("features", f)]);
        } else {
            // 默认值处理
            req = req.query(&[("features", "htmlNewStyle,itemOpusStyle,decorationCard")]);
        }

        req.send_bpi("获取动态详情").await
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
    /// | `id` | &str | 动态 ID |
    /// | `offset` | `Option<&str>` | 偏移量，用于翻页 |
    pub async fn dynamic_reactions(
        &self,
        id: &str,
        offset: Option<&str>
    ) -> Result<BpiResponse<DynamicReactionData>, BpiError> {
        let mut req = self
            .get("https://api.bilibili.com/x/polymer/web-dynamic/v1/detail/reaction")
            .query(&[("id", id)]);

        if let Some(o) = offset {
            req = req.query(&[("offset", o)]);
        }

        req.send_bpi("获取动态点赞与转发列表").await
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
    /// | `business_id` | &str | 动态 ID |
    pub async fn dynamic_lottery_notice(
        &self,
        business_id: &str
    ) -> Result<BpiResponse<DynamicLotteryData>, BpiError> {
        let csrf = self.csrf()?;
        self
            .get("https://api.vc.bilibili.com/lottery_svr/v1/lottery_svr/lottery_notice")
            .query(
                &[
                    ("business_id", business_id),
                    ("business_type", "1"),
                    ("csrf", &csrf),
                ]
            )
            .send_bpi("获取动态抽奖详情").await
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
    /// | `id` | &str | 动态 ID |
    /// | `offset` | `Option<&str>` | 偏移量，用于翻页 |
    pub async fn dynamic_forwards(
        &self,
        id: &str,
        offset: Option<&str>
    ) -> Result<BpiResponse<DynamicForwardData>, BpiError> {
        let mut req = self
            .get("https://api.bilibili.com/x/polymer/web-dynamic/v1/detail/forward")
            .query(&[("id", id)]);

        if let Some(o) = offset {
            req = req.query(&[("offset", o)]);
        }

        req.send_bpi("获取动态转发列表").await
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
    /// | `id` | &str | 动态 ID |
    pub async fn dynamic_pics(&self, id: &str) -> Result<BpiResponse<Vec<DynamicPic>>, BpiError> {
        self
            .get("https://api.bilibili.com/x/polymer/web-dynamic/v1/detail/pic")
            .query(&[("id", id)])
            .send_bpi("获取动态图片列表").await
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
    /// | `id` | &str | 动态 ID |
    pub async fn dynamic_forward_item(
        &self,
        id: &str
    ) -> Result<BpiResponse<DynamicForwardInfoData>, BpiError> {
        self
            .get("https://api.bilibili.com/x/polymer/web-dynamic/v1/detail/forward/item")
            .query(&[("id", id)])
            .send_bpi("获取转发动态信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_get_dynamic_detail() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let dynamic_id = "1099138163191840776";
        let resp = bpi.dynamic_detail(dynamic_id, None).await?;
        let data = resp.into_data()?;

        info!("动态详情: {:?}", data.item);
        assert_eq!(data.item.id_str, dynamic_id);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_dynamic_reactions() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let dynamic_id = "1099138163191840776";
        let resp = bpi.dynamic_reactions(dynamic_id, None).await?;
        let data = resp.into_data()?;

        info!("点赞/转发总数: {}", data.total);
        assert!(!data.items.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_lottery_notice() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let dynamic_id = "969916293954142214";
        let resp = bpi.dynamic_lottery_notice(dynamic_id).await?;
        let data = resp.into_data()?;

        info!("抽奖状态: {}", data.status);
        assert_eq!(data.business_id.to_string(), dynamic_id);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_dynamic_forwards() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let dynamic_id = "1099138163191840776";
        let resp = bpi.dynamic_forwards(dynamic_id, None).await?;
        let data = resp.into_data()?;

        info!("转发总数: {}", data.total);
        assert!(!data.items.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_dynamic_pics() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let dynamic_id = "1099138163191840776";
        let resp = bpi.dynamic_pics(dynamic_id).await?;
        let data = resp.into_data()?;

        info!("图片数量: {}", data.len());
        assert!(!data.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_forward_item() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let dynamic_id = "1110902525317349376";
        let resp = bpi.dynamic_forward_item(dynamic_id).await?;
        let data = resp.into_data()?;

        info!("转发动态详情: {:?}", data.item);
        assert_eq!(data.item.id_str, dynamic_id);

        Ok(())
    }
}
