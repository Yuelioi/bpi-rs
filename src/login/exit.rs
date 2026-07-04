// 退出登录 (Web端)
//
// [文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/login/exit.html#退出登录-web端)

use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResponse;
use crate::login::LoginClient;
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

impl<'a> LoginClient<'a> {
    /// 退出登录 (Web端)
    ///
    /// # 参数
    /// - `gourl`：成功后跳转的 URL，可选，默认 `javascript:history.go(-1)`
    pub async fn logout_web(
        &self,
        gourl: Option<&str>,
    ) -> Result<BpiResponse<LogoutData>, BpiError> {
        let csrf = self.client.csrf()?;

        let form = vec![
            ("biliCSRF", csrf),
            (
                "gourl",
                gourl.unwrap_or("javascript:history.go(-1)").to_string(),
            ),
        ];

        let result = self
            .client
            .post("https://passport.bilibili.com/login/exit/v2")
            .form(&form)
            .send_bpi("退出登录 (Web端)")
            .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {}
