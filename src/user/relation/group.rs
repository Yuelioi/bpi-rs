//! B站用户分组相关接口
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
use serde::{Deserialize, Serialize};

// --- 响应数据结构体 ---

/// 创建分组响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateTagResponseData {
    /// 创建的分组的 ID
    pub tagid: i64,
}

// --- API 实现 ---

// --- 测试模块 ---

#[cfg(test)]
mod tests {}
