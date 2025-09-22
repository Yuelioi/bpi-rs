use serde::{ Deserialize, Serialize };

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

// ================= 数据结构 =================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct EmoticonItem {
    /// 突出展示
    pub bulge_display: i32,
    /// 描述
    pub descript: String,
    /// 触发关键词
    pub emoji: String,
    /// 表情ID
    pub emoticon_id: i64,
    /// 表情唯一标识
    pub emoticon_unique: String,
    /// 表情值类型
    pub emoticon_value_type: i32,
    /// 表情图片高度
    pub height: i32,
    /// 身份限制标识
    pub identity: i32,
    /// 播放器区域内展示
    pub in_player_area: i32,
    /// 是否为动态表情
    pub is_dynamic: i32,
    /// 使用权限
    pub perm: i32,
    /// 解锁需求礼物
    pub unlock_need_gift: i32,
    /// 解锁需求等级
    pub unlock_need_level: i32,
    /// 解锁展示颜色
    pub unlock_show_color: String,
    /// 解锁展示图片
    pub unlock_show_image: String,
    /// 解锁展示文字
    pub unlock_show_text: String,
    /// 表情图片URL
    pub url: String,
    /// 表情图片宽度
    pub width: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TopShowItem {
    /// 图片
    pub image: String,
    /// 文字
    pub text: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct TopShow {
    /// 左上
    pub top_left: TopShowItem,
    /// 右上
    pub top_right: TopShowItem,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct EmoticonPackage {
    /// 封面URL
    pub current_cover: String,
    /// 表情列表
    pub emoticons: Vec<EmoticonItem>,
    /// 文字描述
    pub pkg_descript: String,
    /// 包ID
    pub pkg_id: i64,
    /// 包名称
    pub pkg_name: String,
    /// 使用权限
    pub pkg_perm: i32,
    /// 包类型
    pub pkg_type: i32,
    /// 最近使用的表情
    pub recently_used_emoticons: Vec<serde_json::Value>,
    /// 顶部展示信息
    pub top_show: Option<TopShow>,
    /// 最近使用的顶部展示信息
    pub top_show_recent: Option<TopShow>,
    /// 解锁所需身份标识
    pub unlock_identity: i32,
    /// 解锁所需礼物
    pub unlock_need_gift: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct EmoticonData {
    /// 表情包数据
    pub data: Vec<EmoticonPackage>,
    /// 品牌标识
    pub fans_brand: i32,
    /// 购买链接
    pub purchase_url: Option<String>,
}

pub type EmoticonResponse = BpiResponse<EmoticonData>;

// ================= 实现 =================

impl BpiClient {
    /// 获取直播间的表情包
    ///
    pub async fn live_emoticons(
        &self,
        room_id: i64,
        platform: &str
    ) -> Result<EmoticonResponse, BpiError> {
        let params = [
            ("room_id", room_id.to_string()),
            ("platform", platform.to_string()),
        ];

        let resp: EmoticonResponse = self
            .get("https://api.live.bilibili.com/xlive/web-ucenter/v2/emoticon/GetEmoticons")
            .query(&params)
            .send_bpi("获取直播间表情包").await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_live_emoticons() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let resp = bpi.live_emoticons(14047, "pc").await?;

        let data = resp.data.unwrap();
        assert!(data.data.len() > 0);
        Ok(())
    }
}
