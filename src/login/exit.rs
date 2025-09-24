//! 退出登录 (Web端)
//!
//! [文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/login/exit.html#退出登录-web端)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

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

impl BpiClient {
    /// 退出登录 (Web端)
    ///
    /// # 参数
    /// - `gourl`：成功后跳转的 URL，可选，默认 `javascript:history.go(-1)`
    pub async fn logout_web(
        &self,
        gourl: Option<&str>
    ) -> Result<BpiResponse<LogoutData>, BpiError> {
        let csrf = self.csrf()?;

        let form = vec![
            ("biliCSRF", csrf),
            ("gourl", gourl.unwrap_or("javascript:history.go(-1)").to_string())
        ];

        let result = self
            .post("https://passport.bilibili.com/login/exit/v2")
            .form(&form)
            .send_bpi("退出登录 (Web端)").await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_logout_web() -> Result<(), Box<BpiError>> {
        // let bpi = BpiClient::new();
        //
        // match bpi.logout_web(None).await {
        //     Ok(resp) => {
        //         if resp.code == 0 {
        //             let data = resp.data.unwrap();
        //             tracing::info!("退出登录成功，重定向 URL: {}", data.redirect_url);
        //         } else {
        //             tracing::info!(
        //                 "退出登录失败: code={}, message={:?}",
        //                 resp.code,
        //                 resp.message
        //             );
        //         }
        //     }
        //     Err(err) => {
        //         panic!("请求出错: {}", err);
        //     }
        // }
        Ok(())
    }
}
