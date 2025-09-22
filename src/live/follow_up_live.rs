use serde::{Deserialize, Serialize};

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FollowUpLiveItem {
    /// 房间号
    pub roomid: i64,
    /// 主播uid
    pub uid: i64,
    /// 主播名
    pub uname: String,
    /// 直播标题
    pub title: String,
    /// 主播头像
    pub face: String,
    /// 是否正在直播
    pub live_status: i32,
    /// 主播上一次直播结束的时间戳
    pub record_live_time: i64,
    /// 频道的名称
    pub area_name_v2: String,
    /// 房间公告
    pub room_news: String,
    /// 作用尚不明确，当主播正在直播时，为在线人数(可能)
    pub text_small: String,
    /// 房间封面图片的URL
    pub room_cover: String,
    /// 父分区id
    pub parent_area_id: i32,
    /// 分区id
    pub area_id: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FollowUpLiveData {
    /// 标题
    pub title: String,

    /// 每页的数据数量
    #[serde(rename = "pageSize")]
    pub page_size: i32,

    /// 分页数量
    #[serde(rename = "totalPage")]
    pub total_page: i32,

    /// UP直播情况列表
    pub list: Vec<FollowUpLiveItem>,

    /// 曾直播过的UP数量
    pub count: i32,

    /// 未直播过的UP数量
    pub never_lived_count: i32,

    /// 正在直播的UP数量
    pub live_count: i32,

    /// 作用尚不明确
    pub never_lived_faces: Vec<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LiveRoom {
    /// 直播间标题
    pub title: String,
    /// 直播间真实id
    pub room_id: i64,
    /// 目标用户mid
    pub uid: i64,
    /// 观看人数
    pub online: i32,
    /// 已经直播的时长（单位为秒）
    pub live_time: i64,
    /// 开播状态
    pub live_status: i32,
    /// 直播间短id
    pub short_id: i32,
    /// 分区id
    pub area: i32,
    /// 分区名称
    pub area_name: String,
    /// 二级分区id
    pub area_v2_id: i32,
    /// 二级分区名
    pub area_v2_name: String,
    /// 二级父分区名
    pub area_v2_parent_name: String,
    /// 二级父分区id
    pub area_v2_parent_id: i32,
    /// 用户名
    pub uname: String,
    /// 用户头像图片链接
    pub face: String,
    /// 标签名
    pub tag_name: String,
    /// 标签列表
    pub tags: String,
    /// 直播间封面图片链接
    pub cover_from_user: String,
    /// 关键帧图片链接
    pub keyframe: String,
    /// 未知
    pub lock_till: String,
    /// 未知
    pub hidden_till: String,
    /// 广播类型
    pub broadcast_type: i32,
    /// 直播间是否加密
    pub is_encrypt: bool,
    /// 直播间链接
    pub link: String,
    /// 用户昵称
    pub nickname: String,
    /// 直播间名称
    pub roomname: String,
    /// 直播间真实id
    pub roomid: i64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LiveWebListData {
    /// 正在直播的房间列表
    pub rooms: Vec<LiveRoom>,
    /// 正在直播的房间列表
    pub list: Vec<LiveRoom>,
    /// 关注列表中正在直播的人数
    pub count: i32,
    /// 关注列表中未开播的人数
    pub not_living_num: i32,
}

impl BpiClient {
    /// 获取用户关注的所有UP的直播情况
    ///

    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live
    pub async fn live_follow_up_list(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
        ignore_record: Option<i32>,
        hit_ab: Option<bool>,
    ) -> Result<BpiResponse<FollowUpLiveData>, BpiError> {
        let mut query = Vec::new();

        if let Some(page) = page {
            query.push(("page", page.to_string()));
        }

        if let Some(page_size) = page_size {
            query.push(("page_size", page_size.to_string()));
        }

        if let Some(ignore_record) = ignore_record {
            query.push(("ignoreRecord", ignore_record.to_string()));
        }

        if let Some(hit_ab) = hit_ab {
            query.push(("hit_ab", hit_ab.to_string()));
        }

        self.get("https://api.live.bilibili.com/xlive/web-ucenter/user/following")
            .query(&query)
            .send_bpi("获取用户关注的所有UP的直播情况")
            .await
    }

    /// 获取用户关注的所有UP且正在直播的列表（PC端）
    ///

    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live
    pub async fn live_follow_up_web_list(
        &self,
        hit_ab: Option<bool>,
    ) -> Result<BpiResponse<LiveWebListData>, BpiError> {
        let mut query = Vec::new();

        if let Some(hit_ab) = hit_ab {
            query.push(("hit_ab", hit_ab.to_string()));
        }

        self.get("https://api.live.bilibili.com/xlive/web-ucenter/v1/xfetter/GetWebList")
            .query(&query)
            .send_bpi("获取用户关注的所有UP且正在直播的列表")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_follow_up_live_list() {
        let bpi = BpiClient::new();
        let resp = bpi
            .live_follow_up_list(Some(1), Some(2), Some(1), Some(true))
            .await
            .unwrap();
        tracing::info!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_get_follow_up_live_web_list() {
        let bpi = BpiClient::new();
        let resp = bpi.live_follow_up_web_list(Some(false)).await.unwrap();
        tracing::info!("{:?}", resp);
    }
}
