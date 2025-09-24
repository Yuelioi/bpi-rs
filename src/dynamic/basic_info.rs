use crate::models::Vip;
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };
use serde_json::Value;
// 以下结构体为API文档中未完全列出的部分，根据描述进行了推断和简化。
// 如果您有完整的文档，请替换这些结构体。

/// 动态转发列表中的转发项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepostItem {
    pub desc: Desc,
    pub card: String,
    pub extend_json: String,
    pub display: Display,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Desc {
    pub uid: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub rid: i64,
    pub acl: i64,
    pub view: i64,
    pub repost: i64,
    pub comment: i64,
    pub like: i64,
    pub is_liked: i64,
    pub dynamic_id: i64,
    pub timestamp: i64,
    pub pre_dy_id: i64,
    pub orig_dy_id: i64,
    pub orig_type: i64,
    pub user_profile: UserProfile,
    pub spec_type: i64,
    pub uid_type: i64,
    pub stype: i64,
    pub r_type: i64,
    pub inner_id: i64,
    pub status: i64,
    pub dynamic_id_str: String,
    pub pre_dy_id_str: String,
    pub orig_dy_id_str: String,
    pub rid_str: String,
    pub origin: Origin,
    pub bvid: String,
    pub previous: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub info: Info,
    pub card: Card,
    pub vip: Vip,
    pub pendant: Pendant,
    pub rank: String,
    pub sign: String,
    pub level_info: LevelInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub uid: i64,
    pub uname: String,
    pub face: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub official_verify: OfficialVerify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficialVerify {
    #[serde(rename = "type")]
    pub type_field: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub path: String,
    pub text: String,
    pub label_theme: String,
    pub text_color: String,
    pub bg_style: i64,
    pub bg_color: String,
    pub border_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pendant {
    pub pid: i64,
    pub name: String,
    pub image: String,
    pub expire: i64,
    pub image_enhance: String,
    pub image_enhance_frame: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelInfo {
    pub current_level: i64,
    pub current_min: i64,
    pub current_exp: i64,
    pub next_exp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Origin {
    pub uid: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub rid: i64,
    pub acl: i64,
    pub view: i64,
    pub repost: i64,
    pub comment: i64,
    pub like: i64,
    pub is_liked: i64,
    pub dynamic_id: i64,
    pub timestamp: i64,
    pub pre_dy_id: i64,
    pub orig_dy_id: i64,
    pub orig_type: i64,
    pub user_profile: Value,
    pub spec_type: i64,
    pub uid_type: i64,
    pub stype: i64,
    pub r_type: i64,
    pub inner_id: i64,
    pub status: i64,
    pub dynamic_id_str: String,
    pub pre_dy_id_str: String,
    pub orig_dy_id_str: String,
    pub rid_str: String,
    pub origin: Value,
    pub bvid: String,
    pub previous: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Display {
    pub origin: Origin2,
    pub usr_action_txt: String,
    pub relation: Relation2,
    pub live_info: Value,
    pub emoji_info: EmojiInfo2,
    pub highlight: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Origin2 {
    pub origin: Value,
    pub usr_action_txt: String,
    pub relation: Relation,
    pub live_info: Value,
    pub emoji_info: EmojiInfo,
    pub highlight: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub status: i64,
    pub is_follow: i64,
    pub is_followed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiInfo {
    pub emoji_details: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation2 {
    pub status: i64,
    pub is_follow: i64,
    pub is_followed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiInfo2 {
    pub emoji_details: Value,
}

/// 动态点赞列表中的点赞项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LikeItem {
    // 由于API文档未详细列出字段，这里作为占位符。
    // 请根据实际API响应填充此结构体。
}

/// 纯文本动态内容
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlainTextRequest {
    // 假设纯文本动态内容有一个名为 `content` 的字段。
    pub content: String,
    // 其他字段...
}

/// 获取草稿列表中的单项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Draft {
    /// 草稿id
    pub draft_id: String,
    /// 定时发送的秒级时间戳
    pub publish_time: u64,
    /// 动态类型
    #[serde(rename = "type")]
    pub type_num: u8,
    /// 自己的mid
    pub uid: u64,
    /// 自己的用户信息
    pub user_profile: UserProfile,
    /// 动态内容
    pub request: String,
}

/// 动态转发列表响应数据结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepostDetailResponseData {
    /// 是否还有下一页
    pub has_more: Option<bool>,
    /// 总计
    pub total: u64,
    /// 转发列表
    pub items: Vec<RepostItem>,
}

/// 动态点赞列表响应数据结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpecItemLikesResponseData {
    /// 点赞信息列表主体
    pub item_likes: Vec<LikeItem>,
    /// 是否还有下一页
    pub has_more: bool,
    /// 总计点赞数
    pub total_count: u64,
    /// 作用尚不明确
    pub gt: u64,
}

/// 获取草稿列表响应数据结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetDraftsResponseData {
    /// 草稿列表
    pub drafts: Vec<Draft>,
}

impl BpiClient {
    /// 动态转发列表
    ///
    /// 获取指定动态的转发列表。
    ///
    /// # 参数
    /// * `dynamic_id` - 动态ID
    /// * `offset` - 偏移量，非必要
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `dynamic_id` | &str | 动态 ID |
    /// | `offset` | `Option<&str>` | 偏移量 |
    pub async fn dynamic_repost_detail(
        &self,
        dynamic_id: &str,
        offset: Option<&str>
    ) -> Result<BpiResponse<RepostDetailResponseData>, BpiError> {
        let mut req = self
            .get("https://api.vc.bilibili.com/dynamic_repost/v1/dynamic_repost/repost_detail")
            .query(&[("dynamic_id", dynamic_id)]);

        if let Some(o) = offset {
            req = req.query(&[("offset", o)]);
        }

        req.send_bpi("获取动态转发列表").await
    }

    /// 动态点赞列表
    ///
    /// 获取指定动态的点赞列表。赞列表总计超过25K部分继续获取可能被限制
    ///
    /// # 参数
    /// * `dynamic_id` - 动态ID
    /// * `pn` - 页码，非必要，默认1
    /// * `ps` - 每页数量，非必要，默认20
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `dynamic_id` | u64 | 动态 ID |
    /// | `pn` | `Option<u64>` | 页码，默认 1 |
    /// | `ps` | `Option<u64>` | 页大小，默认 20 |
    pub async fn dynamic_spec_item_likes(
        &self,
        dynamic_id: u64,
        pn: Option<u64>,
        ps: Option<u64>
    ) -> Result<BpiResponse<SpecItemLikesResponseData>, BpiError> {
        let pn_val = pn.unwrap_or(1);
        let ps_val = ps.unwrap_or(20);

        let req = self
            .get("https://api.vc.bilibili.com/dynamic_like/v1/dynamic_like/spec_item_likes")
            .query(
                &[
                    ("dynamic_id", &dynamic_id.to_string()),
                    ("pn", &pn_val.to_string()),
                    ("ps", &ps_val.to_string()),
                ]
            );

        req.send_bpi("获取动态点赞列表").await
    }

    /// 获取草稿列表 (已失效?)
    ///
    /// 获取用户已保存的动态草稿列表。需要登录认证。
    #[allow(dead_code)]
    async fn get_drafts(&self) -> Result<BpiResponse<GetDraftsResponseData>, BpiError> {
        let req = self.get("https://api.vc.bilibili.com/dynamic_draft/v1/dynamic_draft/get_drafts");

        req.send_bpi("获取草稿列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_get_repost_detail() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        // 替换为有效的动态ID进行测试
        let dynamic_id = "1099138163191840776";
        let resp = bpi.dynamic_repost_detail(dynamic_id, None).await?;
        let data = resp.into_data()?;

        info!("动态转发列表测试结果: {:?}", data);
        assert!(!data.items.iter().len() > 0);

        Ok(())
    }
}
