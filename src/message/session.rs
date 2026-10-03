//! 私信会话与消息记录。接口每次只读取一页，不自动确认已读或重试发送。

use serde::{Deserialize, Serialize};

use super::MessageClient;
use crate::{BilibiliRequest, BpiError, BpiResult};

pub(crate) const SESSIONS_ENDPOINT: &str =
    "https://api.vc.bilibili.com/session_svr/v1/session_svr/get_sessions";
pub(crate) const SESSION_MESSAGES_ENDPOINT: &str =
    "https://api.vc.bilibili.com/svr_sync/v1/svr_sync/fetch_session_msgs";

/// 会话分类。
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum MessageSessionListType {
    /// 用户与系统会话，不含未关注用户分类。
    UserAndSystem,
    /// 未关注用户会话。
    Unfollowed,
    /// 全部会话；实测可能忽略 end_ts，调用方必须检查游标前进。
    #[default]
    All,
}

impl MessageSessionListType {
    fn query_value(self) -> &'static str {
        match self {
            Self::UserAndSystem => "1",
            Self::Unfollowed => "2",
            Self::All => "4",
        }
    }
}

/// 会话列表参数，默认包含未关注用户。时间游标单位为微秒。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageSessionsParams {
    session_type: MessageSessionListType,
    size: u32,
    begin_ts: Option<u64>,
    end_ts: Option<u64>,
}

impl Default for MessageSessionsParams {
    fn default() -> Self {
        Self {
            session_type: MessageSessionListType::All,
            size: 100,
            begin_ts: None,
            end_ts: None,
        }
    }
}

impl MessageSessionsParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// 选择会话分类。需要历史分页时可分别查询用户/系统与未关注用户。
    pub fn with_session_type(mut self, session_type: MessageSessionListType) -> Self {
        self.session_type = session_type;
        self
    }

    /// 每页 1 到 100 条会话。
    pub fn with_size(mut self, size: u32) -> BpiResult<Self> {
        self.size = validate_size(size)?;
        Ok(self)
    }

    /// 只读取此时间之后更新的会话，单位为微秒。
    pub fn with_begin_ts(mut self, timestamp: u64) -> BpiResult<Self> {
        self.begin_ts = Some(positive("begin_ts", timestamp)?);
        Ok(self)
    }

    /// 读取更早一页，使用上一页最小 session_ts（微秒），并检查服务端是否推进。
    pub fn with_end_ts(mut self, timestamp: u64) -> BpiResult<Self> {
        self.end_ts = Some(positive("end_ts", timestamp)?);
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![
            ("session_type", self.session_type.query_value().into()),
            ("sort_rule", "2".into()),
            ("size", self.size.to_string()),
            ("mobi_app", "web".into()),
            ("group_fold", "0".into()),
            ("unfollow_fold", "0".into()),
        ];
        if let Some(value) = self.begin_ts {
            pairs.push(("begin_ts", value.to_string()));
        }
        if let Some(value) = self.end_ts {
            pairs.push(("end_ts", value.to_string()));
        }
        pairs
    }
}

/// 单个用户私信记录参数。游标为消息序列号，不是时间戳。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageSessionMessagesParams {
    talker_id: u64,
    size: u32,
    begin_seqno: Option<u64>,
    end_seqno: Option<u64>,
}

impl MessageSessionMessagesParams {
    pub fn new(talker_id: u64) -> BpiResult<Self> {
        Ok(Self {
            talker_id: positive("talker_id", talker_id)?,
            size: 100,
            begin_seqno: None,
            end_seqno: None,
        })
    }

    pub fn with_size(mut self, size: u32) -> BpiResult<Self> {
        self.size = validate_size(size)?;
        Ok(self)
    }

    /// 只读取此序列号之后的消息。
    pub fn with_begin_seqno(mut self, seqno: u64) -> BpiResult<Self> {
        self.begin_seqno = Some(positive("begin_seqno", seqno)?);
        Ok(self)
    }

    /// 读取更早一页，使用上一页最小序列号。
    pub fn with_end_seqno(mut self, seqno: u64) -> BpiResult<Self> {
        self.end_seqno = Some(positive("end_seqno", seqno)?);
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![
            ("talker_id", self.talker_id.to_string()),
            ("session_type", "1".into()),
            ("size", self.size.to_string()),
            ("mobi_app", "web".into()),
        ];
        if let Some(value) = self.begin_seqno {
            pairs.push(("begin_seqno", value.to_string()));
        }
        if let Some(value) = self.end_seqno {
            pairs.push(("end_seqno", value.to_string()));
        }
        pairs
    }
}

fn positive(field: &'static str, value: u64) -> BpiResult<u64> {
    if value == 0 {
        return Err(BpiError::invalid_parameter(field, "value must be non-zero"));
    }
    Ok(value)
}

fn validate_size(size: u32) -> BpiResult<u32> {
    if !(1..=100).contains(&size) {
        return Err(BpiError::invalid_parameter(
            "size",
            "value must be between 1 and 100",
        ));
    }
    Ok(size)
}

/// 会话列表。空列表在实测响应中为 null。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageSessionsData {
    pub session_list: Option<Vec<MessageSession>>,
    pub has_more: u32,
    pub is_address_list_empty: u32,
    pub anti_disturb_cleaning: bool,
    pub show_level: bool,
}

/// 会话的稳定私信字段；账号标签和特殊系统卡片仍由服务端扩展。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageSession {
    pub talker_id: u64,
    pub session_type: u32,
    /// 微秒时间戳，供下一页 end_ts 使用。
    pub session_ts: u64,
    pub top_ts: u64,
    pub ack_seqno: u64,
    pub ack_ts: u64,
    pub max_seqno: u64,
    pub unread_count: u32,
    pub system_msg_type: u32,
    pub is_follow: u32,
    pub is_dnd: u32,
    /// 无最近消息的会话可返回 null。
    pub last_msg: Option<MessagePrivateMessage>,
}

/// 消息记录。空页 messages 为 null，保留服务端分页边界。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageSessionMessagesData {
    pub messages: Option<Vec<MessagePrivateMessage>>,
    pub has_more: u32,
    pub min_seqno: u64,
    pub max_seqno: u64,
}

/// 私信消息。content 是嵌套 JSON 字符串，未知 msg_type 原样保留。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessagePrivateMessage {
    pub sender_uid: u64,
    pub receiver_id: u64,
    pub receiver_type: u32,
    pub msg_type: u32,
    pub content: String,
    pub msg_seqno: u64,
    pub msg_key: u64,
    /// 秒时间戳，与会话 session_ts 的微秒单位不同。
    pub timestamp: u64,
    pub msg_status: u32,
    pub msg_source: u32,
    pub at_uids: Option<Vec<u64>>,
    /// 历史消息中此字段缺失。
    pub new_face_version: Option<u32>,
    pub notify_code: String,
}

impl MessageClient<'_> {
    /// 获取一页全部会话（private-read），不会确认已读。
    ///
    /// ```no_run
    /// # async fn example(client: &bpi_rs::BpiClient) -> bpi_rs::BpiResult<()> {
    /// let page = client.message().sessions(bpi_rs::message::MessageSessionsParams::new()).await?;
    /// for session in page.session_list.unwrap_or_default() {
    ///     let params = bpi_rs::message::MessageSessionMessagesParams::new(session.talker_id)?;
    ///     let _messages = client.message().session_messages(params).await?;
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn sessions(&self, params: MessageSessionsParams) -> BpiResult<MessageSessionsData> {
        self.client
            .get(SESSIONS_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi_payload("message.sessions")
            .await
    }

    /// 获取一页用户私信（private-read），保持服务端顺序，不自动翻页或标记已读。
    pub async fn session_messages(
        &self,
        params: MessageSessionMessagesParams,
    ) -> BpiResult<MessageSessionMessagesData> {
        self.client
            .get(SESSION_MESSAGES_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi_payload("message.session_messages")
            .await
    }
}
