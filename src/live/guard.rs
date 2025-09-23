use serde::{Deserialize, Serialize};

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserInfo {
    /// 用户UID
    pub uid: i64,
    /// 用户基本信息
    pub base: UserBaseInfo,
    /// 粉丝牌信息
    pub medal: UserMedalInfo,
    /// 财富信息
    pub wealth: Option<serde_json::Value>,
    /// 标题
    pub title: Option<serde_json::Value>,
    /// 大航海信息
    pub guard: UserGuardInfo,
    /// 头像框
    pub uhead_frame: Option<serde_json::Value>,
    /// 大航海队长
    pub guard_leader: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GuardTabInfo {
    /// 大航海总人数
    pub num: i32,
    /// 总页数
    pub page: i32,
    /// 当前页数
    pub now: i32,
    /// 成就等级
    pub achievement_level: i32,
    /// 主播守护成就等级
    pub anchor_guard_achieve_level: i32,
    /// 成就图标
    pub achievement_icon_src: String,
    /// 购买守护图标
    pub buy_guard_icon_src: String,
    /// 规则文档链接
    pub rule_doc_src: String,
    /// 背景图片
    pub ex_background_src: String,
    /// 颜色开始
    pub color_start: String,
    /// 颜色结束
    pub color_end: String,
    /// 标签颜色
    pub tab_color: Vec<String>,
    /// 标题颜色
    pub title_color: Vec<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserOriginInfo {
    /// 用户名
    pub name: String,
    /// 头像
    pub face: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserOfficialInfo {
    /// 角色
    pub role: i32,
    /// 标题
    pub title: String,
    /// 描述
    pub desc: String,
    /// 类型
    pub r#type: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserBaseInfo {
    /// 用户名
    pub name: String,
    /// 头像
    pub face: String,
    /// 名称颜色
    pub name_color: i32,
    /// 是否匿名
    pub is_mystery: bool,
    /// 风险控制信息
    pub risk_ctrl_info: Option<serde_json::Value>,
    /// 原始信息
    pub origin_info: UserOriginInfo,
    /// 官方信息
    pub official_info: UserOfficialInfo,
    /// 名称颜色字符串
    pub name_color_str: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserMedalInfo {
    /// 粉丝牌名称
    pub name: String,
    /// 粉丝牌等级
    pub level: i32,
    /// 颜色开始
    pub color_start: i32,
    /// 颜色结束
    pub color_end: i32,
    /// 边框颜色
    pub color_border: i32,
    /// 颜色
    pub color: i32,
    /// ID
    pub id: i32,
    /// 类型
    pub typ: i32,
    /// 是否点亮
    pub is_light: i32,
    /// 主播UID
    pub ruid: i64,
    /// 大航海等级
    pub guard_level: i32,
    /// 亲密度
    pub score: i32,
    /// 大航海图标
    pub guard_icon: String,
    /// 荣誉图标
    pub honor_icon: String,
    /// V2粉丝牌颜色开始
    pub v2_medal_color_start: String,
    /// V2粉丝牌颜色结束
    pub v2_medal_color_end: String,
    /// V2粉丝牌边框颜色
    pub v2_medal_color_border: String,
    /// V2粉丝牌文本颜色
    pub v2_medal_color_text: String,
    /// V2粉丝牌等级颜色
    pub v2_medal_color_level: String,
    /// 用户接收数量
    pub user_receive_count: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UserGuardInfo {
    /// 大航海等级
    pub level: i32,
    /// 过期时间字符串
    pub expired_str: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GuardMember {
    /// 主播UID
    pub ruid: i64,
    /// 排名
    pub rank: i32,
    /// 陪伴天数
    pub accompany: i32,
    /// 用户信息
    pub uinfo: UserInfo,
    /// 亲密度
    pub score: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GuardListData {
    /// 大航海信息
    pub info: GuardTabInfo,
    /// 前三名
    pub top3: Vec<GuardMember>,
    /// 大航海成员列表
    pub list: Vec<GuardMember>,
}

pub type GuardListResponse = BpiResponse<GuardListData>;

// ================= 实现 =================

impl BpiClient {
    /// 查询大航海成员
    ///
    /// room_id: 直播间号
    /// ruid: 主播id
    /// page: 页
    /// page_size: 10~30 [20]
    /// typ:3,4,5 周/月/总 亲密度

    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `room_id` | i64 | 直播间号 |
    /// | `ruid` | i64 | 主播 id |
    /// | `page` | Option<i32> | 页码，默认 1 |
    /// | `page_size` | Option<i32> | 每页 10~30，默认 20 |
    /// | `typ` | Option<i32> | 3/4/5：周/月/总 亲密度 |
    pub async fn live_guard_list(
        &self,
        room_id: i64,
        ruid: i64,
        page: Option<i32>,
        page_size: Option<i32>,
        typ: Option<i32>,
    ) -> Result<GuardListResponse, BpiError> {
        let params: Vec<(&str, String)> = vec![
            ("roomid", room_id.to_string()),
            ("ruid", ruid.to_string()),
            ("page", page.unwrap_or(1).to_string()),
            ("page_size", page_size.unwrap_or(20).to_string()),
            ("typ", typ.unwrap_or(5).to_string()),
        ];

        let resp: GuardListResponse = self
            .get("https://api.live.bilibili.com/xlive/app-room/v2/guardTab/topListNew")
            .query(&params)
            .send_bpi("查询大航海成员")
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_guard_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let resp = bpi
            .live_guard_list(23174842, 504140200, None, None, None)
            .await?;

        let data = resp.data.unwrap();
        assert!(data.list.len() > 0);
        Ok(())
    }
}
