//! 退出登录 (Web端)
//!
//! [文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/login/exit.html#退出登录-web端)

use serde::{Deserialize, Serialize};

/// 退出登录成功后的数据体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutData {
    /// 重定向 URL
    #[serde(rename = "redirectUrl")]
    pub redirect_url: String,
}

/// 退出登录响应结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutResponse {
    /// 返回码
    /// - 0：成功
    /// - 2202：csrf 请求非法
    pub code: i32,
    /// 返回状态，成功为 true
    pub status: bool,
    /// 时间戳
    pub ts: u64,
    /// 错误信息（成功时不存在）
    pub message: Option<String>,
    /// 有效时的 data
    pub data: Option<LogoutData>,
}

#[cfg(test)]
mod tests {}
