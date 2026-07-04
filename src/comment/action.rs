//! 评论区相关操作 API
//!
//! [参考文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/comment/action)

use serde::{Deserialize, Serialize};

/// 评论区类型枚举（部分示例，需按需求扩展）
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentType {
    Video = 1,    // 视频
    Article = 12, // 专栏
    Dynamic = 17, // 动态
    Unknown = 0,
}

/// 举报原因枚举
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ReportReason {
    Other = 0,
    Ad = 1,
    Porn = 2,
    Spam = 3,
    Flame = 4,
    Spoiler = 5,
    Politics = 6,
    Abuse = 7,
    Irrelevant = 8,
    Illegal = 9,
    Vulgar = 10,
    Phishing = 11,
    Scam = 12,
    Rumor = 13,
    Incitement = 14,
    Privacy = 15,
    FloorSnatching = 16,
    HarmfulToYouth = 17,
}

/// 评论成功返回数据
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CommentData {
    pub rpid: u64,
    pub rpid_str: String,
    pub root: u64,
    pub root_str: String,
    pub parent: u64,
    pub parent_str: String,
    pub dialog: u64,
    pub dialog_str: String,
    pub success_toast: Option<String>,
}
