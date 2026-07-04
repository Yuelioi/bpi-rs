//! 追番相关
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/bangumi/follow.md)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiFollowResult {
    pub fmid: i64,
    pub relation: bool,
    pub status: i32,
    pub toast: String,
}

#[cfg(test)]
mod tests {}
