//! 创建合集 API
//!
//! [参考文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/creativecenter/season.md)

use serde::{Deserialize, Serialize};

/// 合集视频条目（添加用）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpisodeAdd {
    pub title: String,
    pub aid: u64,
    pub cid: u64,

    #[serde(default)]
    pub charging_pay: i64,
    #[serde(default)]
    pub member_first: i64,
    #[serde(default)]
    pub limited_free: bool,
}

#[cfg(test)]
mod tests {}
