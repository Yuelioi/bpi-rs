use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- API 结构体 ---

/// 未读消息数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UnreadCountData {
    pub coin: u32,       // 未读投币数
    pub danmu: u32,      // 未读弹幕数
    pub favorite: u32,   // 未读收藏数
    pub recv_like: u32,  // 未读收到喜欢数
    pub recv_reply: u32, // 未读回复
    pub sys_msg: u32,    // 未读系统通知数
    pub up: u32,         // 未读UP主助手信息数
}

/// "回复我的"信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReplyFeedData {
    pub cursor: ReplyCursor,
    pub items: Vec<ReplyItem>,
    pub last_view_at: u64,
}

/// 分页游标
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReplyCursor {
    pub is_end: bool,
    pub id: Option<u64>,
    pub time: Option<u64>,
}

/// 单条回复通知
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReplyItem {
    pub id: u64,
    pub user: ReplyUser,
    pub item: ReplyDetail,
    pub counts: u32,
    pub is_multi: u32,
    pub reply_time: u64,
}

/// 回复者用户信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReplyUser {
    pub mid: u64,
    pub nickname: String,
    pub avatar: String,
    pub follow: bool,
    // 以下字段文档表示固定或不返回，但为了完整性保留
    pub fans: Option<u32>,
    pub mid_link: Option<String>,
}

/// 回复通知详情
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReplyDetail {
    pub subject_id: u64,
    pub root_id: u64,
    pub source_id: u64,
    pub target_id: u64,
    #[serde(rename = "type")]
    pub reply_type: String,
    pub business_id: u32,
    pub business: String,
    pub title: String,
    pub desc: String,
    pub uri: String,
    pub native_uri: String,
    pub root_reply_content: String,
    pub source_content: String,
    pub target_reply_content: String,
    pub at_details: Vec<AtUserDetail>,
    pub hide_reply_button: bool,
    pub hide_like_button: bool,
    pub like_state: u32,
}

/// @的用户详情
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AtUserDetail {
    pub mid: u64,
    pub nickname: String,
    pub avatar: String,
    pub follow: bool,
}

impl BpiClient {
    /// 获取未读消息数。
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/message
    pub async fn message_unread_count(&self) -> Result<BpiResponse<UnreadCountData>, BpiError> {
        self.get("https://api.vc.bilibili.com/x/im/web/msgfeed/unread")
            .query(&[("build", "0"), ("mobi_app", "web")])
            .send_bpi("获取未读消息数")
            .await
    }

    /// 获取"回复我的"信息列表。
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/message
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `start_id` | Option<u64> | 起始 ID（上次返回的 cursor.id） |
    /// | `start_time` | Option<u64> | 起始时间戳（上次返回的 cursor.time） |
    pub async fn message_reply_feed(
        &self,
        start_id: Option<u64>,
        start_time: Option<u64>,
    ) -> Result<BpiResponse<ReplyFeedData>, BpiError> {
        let mut params = HashMap::new();
        params.insert("build", "0".to_string());
        params.insert("mobi_app", "web".to_string());
        params.insert("platform", "web".to_string());
        params.insert("web_location", "".to_string());

        if let Some(id) = start_id {
            params.insert("id", id.to_string());
        }
        if let Some(time) = start_time {
            params.insert("reply_time", time.to_string());
        }

        self.get("https://api.bilibili.com/x/msgfeed/reply")
            .query(&params)
            .send_bpi("获取回复我的信息")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]

    async fn test_get_unread_count() -> Result<(), BpiError> {
        let bpi = BpiClient::new();

        let new_resp = bpi.message_unread_count().await?;
        let new_data = new_resp.into_data()?;
        println!("未读消息数 (新接口): {:?}", new_data);
        Ok(())
    }

    #[tokio::test]

    async fn test_get_reply_feed() -> Result<(), BpiError> {
        let bpi = BpiClient::new();

        let resp = bpi.message_reply_feed(None, None).await?;
        let data = resp.into_data()?;

        println!("最近回复我的信息:");
        println!("  上次查看时间: {}", data.last_view_at);
        println!("  游标信息: {:?}", data.cursor);

        for item in data.items {
            println!("---");
            println!("  回复者: {}", item.user.nickname);
            println!("  回复内容: {}", item.item.source_content);
            println!("  回复时间: {}", item.reply_time);
            println!("  关联视频/动态: {}", item.item.title);
            println!("  根评论: {}", item.item.root_reply_content);
            println!("  跳转链接: {}", item.item.uri);
        }

        if !data.cursor.is_end {
            println!("---");
            println!(
                "还有更多数据，下次请求可使用 id: {:?}, time: {:?}",
                data.cursor.id, data.cursor.time
            );
        }

        Ok(())
    }
}
