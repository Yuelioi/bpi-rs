use serde::{Deserialize, Serialize};

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

// ================= 数据结构 =================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct WatchedShow {
    /// 开关
    pub switch: bool,
    /// 看过人数
    pub num: i32,
    /// 小文本
    pub text_small: String,
    /// 大文本
    pub text_large: String,
    /// 图标URL
    pub icon: String,
    /// 图标位置
    pub icon_location: i32,
    /// Web端图标URL
    pub icon_web: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct RecommendRoom {
    /// 头像框
    pub head_box: Option<serde_json::Value>,
    /// 分区ID
    pub area_v2_id: i32,
    /// 父分区ID
    pub area_v2_parent_id: i32,
    /// 分区名称
    pub area_v2_name: String,
    /// 父分区名称
    pub area_v2_parent_name: String,
    /// 广播类型
    pub broadcast_type: i32,
    /// 封面URL
    pub cover: String,
    /// 直播间链接
    pub link: String,
    /// 观看人数
    pub online: i32,
    /// 挂件信息
    #[serde(rename = "pendant_Info")]
    pub pendant_info: serde_json::Value,
    /// 直播间ID
    pub roomid: i64,
    /// 直播间标题
    pub title: String,
    /// 主播用户名
    pub uname: String,
    /// 主播头像URL
    pub face: String,
    /// 认证信息
    pub verify: serde_json::Value,
    /// 主播用户mid
    pub uid: i64,
    /// 关键帧URL
    pub keyframe: String,
    /// 是否自动播放
    pub is_auto_play: i32,
    /// 头像框类型
    pub head_box_type: i32,
    /// 标记
    pub flag: i32,
    /// 会话ID
    pub session_id: String,
    /// 展示回调URL
    pub show_callback: String,
    /// 点击回调URL
    pub click_callback: String,
    /// 特殊ID
    pub special_id: i32,
    /// 观看展示
    pub watched_show: WatchedShow,
    /// 是否为NFT头像
    pub is_nft: i32,
    /// NFT标记
    pub nft_dmark: String,
    /// 是否为广告
    pub is_ad: bool,
    /// 广告透明内容
    pub ad_transparent_content: Option<serde_json::Value>,
    /// 显示广告图标
    pub show_ad_icon: bool,
    /// 状态
    pub status: bool,
    /// 关注者数量
    pub followers: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct RecommendData {
    /// 推荐房间列表
    pub recommend_room_list: Vec<RecommendRoom>,
    /// 置顶直播间号
    pub top_room_id: i64,
}

impl BpiClient {
    /// 主页获取直播推荐
    ///

    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live
    pub async fn live_recommend(&self) -> Result<BpiResponse<RecommendData>, BpiError> {
        let params = [("platform", "web"), ("web_location", "333.1007")];

        let resp = self
            .get("https://api.live.bilibili.com/xlive/web-interface/v1/webMain/getMoreRecList")
            .query(&params)
            .send_bpi("主页获取直播推荐")
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_live_recommend() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let resp = bpi.live_recommend().await?;

        let data = resp.data.unwrap();

        assert!(data.recommend_room_list.len() > 0);
        Ok(())
    }
}
