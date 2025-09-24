use serde::{ Deserialize, Serialize };

use crate::models::{ LevelInfo, Official, Pendant, Vip };
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct DynamicCardData {
    pub card: DynamicCard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicCard {
    pub desc: Desc,
    pub card: String,
    pub extend_json: String,
    pub display: serde_json::Value,
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
    pub bvid: String,
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

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct RecentUpData {
    /// 直播用户（暂不明确，可能为 null）
    pub live_users: Option<serde_json::Value>,
    /// 我的信息
    pub my_info: Option<MyInfo>,
    /// 最近更新的 UP 主列表
    pub up_list: Vec<UpUser>,
}

/// 我的信息对象
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct MyInfo {
    /// 个人动态数
    pub dyns: i32,
    /// 头像地址
    pub face: String,
    /// 粉丝数
    pub follower: String,
    /// 我的关注数
    pub following: i32,
    /// 等级信息
    pub level_info: LevelInfo,
    /// 用户 mid
    pub mid: i64,
    /// 用户昵称
    pub name: String,
    /// 认证信息
    #[serde(rename = "official")]
    pub official: Official,
    /// 个人空间背景图
    pub space_bg: String,
    /// 会员信息
    pub vip: Vip,
}

/// 最近更新的 UP 主
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UpUser {
    /// 头像
    pub face: String,
    /// 是否有更新
    pub has_update: bool,
    /// 作用不明
    pub is_reserve_recall: bool,
    /// 用户 mid
    pub mid: i64,
    /// 用户昵称
    pub uname: String,
}

impl BpiClient {
    /// 获取特定动态卡片信息
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `dynamic_id` | &str | 动态 ID |
    pub async fn dynamic_card_detail(
        &self,
        dynamic_id: &str
    ) -> Result<BpiResponse<DynamicCardData>, BpiError> {
        self
            .get("https://api.vc.bilibili.com/dynamic_svr/v1/dynamic_svr/get_dynamic_detail")
            .query(&[("dynamic_id", dynamic_id)])
            .send_bpi("获取特定动态卡片信息").await
    }

    /// 获取最近更新 UP 主列表
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    pub async fn dynamic_recent_up_list(&self) -> Result<BpiResponse<RecentUpData>, BpiError> {
        self
            .get("https://api.bilibili.com/x/polymer/web-dynamic/v1/portal")
            .send_bpi("获取最近更新 UP 主列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dynamic_get_card_detail() {
        let bpi = BpiClient::new();
        let resp = bpi.dynamic_card_detail("1099138163191840776").await;
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn test_dynamic_recent_up_list() {
        let bpi = BpiClient::new();
        let resp = bpi.dynamic_recent_up_list().await;
        assert!(resp.is_ok());
        if let Ok(res) = resp {
            tracing::info!("{:#?}", res.data.unwrap().up_list.len());
        }
    }
}
