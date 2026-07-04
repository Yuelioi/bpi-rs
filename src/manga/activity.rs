//! 漫画任务操作
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/manga/Activity.md)

use crate::BpiResponse;
use serde::{Deserialize, Serialize};

// ================= 数据结构 =================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ShareComicData {
    /// 获取积分
    pub point: i32,
}

pub type ShareComicResponse = BpiResponse<ShareComicData>;

// ================= 实现 =================

#[cfg(test)]
mod tests {}
