//! 修改签名
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/login/member_center.md)

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

impl BpiClient {
    /// 修改个人签名
    ///
    /// # 参数
    /// * `user_sign` - 要设置的签名内容，最多70个字符。留空表示删除签名
    pub async fn member_center_update_user_sign(
        &self,
        user_sign: &str,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        if user_sign.len() > 70 {
            return Err(BpiError::InvalidParameter {
                field: "user_sign",
                message: "签名长度不能超过70个字符",
            });
        }

        let csrf = self.csrf()?;
        let result = self
            .post("https://api.bilibili.com/x/member/web/sign/update")
            .form(&[("user_sign", user_sign.to_string()), ("csrf", csrf)])
            .send_bpi("设置个人签名")
            .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    /// 更新签名测试
    async fn test_update_user_sign() -> Result<(), BpiError> {
        let Some(bpi) = live_mutating_client_or_skip()? else {
            return Ok(());
        };

        let test_sign = "这是一个测试签名 - Powered by Rust";
        let _result = bpi.member_center_update_user_sign(test_sign).await?;

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_sign_length_validation() {
        let bpi = BpiClient::new().expect("client should build");

        // 测试超长签名
        let long_sign = "a".repeat(71);
        let result = bpi.member_center_update_user_sign(&long_sign).await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("签名长度不能超过70个字符")
        );
    }

    fn live_mutating_client_or_skip() -> Result<Option<BpiClient>, BpiError> {
        if std::env::var("BPI_LIVE_TEST").ok().as_deref() != Some("1")
            || std::env::var("BPI_MUTATING_TEST").ok().as_deref() != Some("1")
        {
            return Ok(None);
        }

        let Some(cookie) = std::env::var("BPI_COOKIE")
            .ok()
            .filter(|value| !value.is_empty())
        else {
            return Ok(None);
        };

        BpiClient::builder().cookie(cookie).build().map(Some)
    }
}
