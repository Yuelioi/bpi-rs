//! B站用户粉丝勋章相关接口
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 粉丝勋章响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MedalWallData {
    pub list: Vec<MedalWallItem>,
    pub count: u32,
    pub close_space_medal: u32,
    pub only_show_wearing: u32,
    pub name: String,
    pub icon: String,
    pub uid: u64,
    pub level: u32,
}

/// 勋章项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MedalWallItem {
    pub medal_info: MedalInfo,
    pub target_name: String,
    pub target_icon: String,
    pub link: String,
    pub live_status: u32,
    pub offical: Option<u32>, // 部分用户可能没有认证
    pub uinfo_medal: Option<UinfoMedal>,
}

/// 勋章信息（主播相关）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MedalInfo {
    pub target_id: u64,
    pub level: u32,
    pub medal_name: String,
    pub medal_color_start: u32,
    pub medal_color_end: u32,
    pub medal_color_border: u32,
    pub guard_level: u32,
    pub wearing_status: u32,
    pub medal_id: u64,
    pub intimacy: u64,
    pub next_intimacy: u64,
    pub today_feed: u64,
    pub day_limit: u64,
    pub guard_icon: Option<String>,
    pub honor_icon: Option<String>,
}

/// 用户勋章信息（佩戴者视角）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UinfoMedal {
    pub name: String,
    pub level: u32,
    pub color_start: u32,
    pub color_end: u32,
    pub color_border: u32,
    pub color: u32,
    pub id: u64,
    pub typ: u32,
    pub is_light: u32,
    pub ruid: u64,
    pub guard_level: u32,
    pub score: u64,
    pub guard_icon: Option<String>,
    pub honor_icon: Option<String>,
    pub v2_medal_color_start: Option<String>,
    pub v2_medal_color_end: Option<String>,
    pub v2_medal_color_border: Option<String>,
    pub v2_medal_color_text: Option<String>,
    pub v2_medal_color_level: Option<String>,
    pub user_receive_count: Option<u32>,
}

impl BpiClient {
    /// 获取指定用户的所有粉丝勋章
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user
    pub async fn user_medal_wall(
        &self,
        target_id: u64,
    ) -> Result<BpiResponse<MedalWallData>, BpiError> {
        self.get("https://api.live.bilibili.com/xlive/web-ucenter/user/MedalWall")
            .query(&[("target_id", target_id.to_string())])
            .send_bpi("获取用户粉丝勋章")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_user_medal_wall() {
        let bpi = BpiClient::new();
        let resp = bpi.user_medal_wall(2).await.unwrap(); // UID=2: 碧诗
        info!("粉丝勋章墙: {:?}", resp.data);
    }
}
