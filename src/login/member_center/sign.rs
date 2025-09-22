//! 修改签名
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/login/member_center.md

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

impl BpiClient {
    /// 修改个人签名
    ///
    /// # 参数
    /// * `user_sign` - 要设置的签名内容，最多70个字符。留空表示删除签名
    pub async fn member_center_update_user_sign(
        &self,
        user_sign: &str
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        // 验证签名长度
        if user_sign.len() > 70 {
            return Err(BpiError::InvalidParameter {
                field: "user_sign",
                message: "签名长度不能超过70个字符",
            });
        }

        // 发送POST请求
        let result = self
            .post("https://api.bilibili.com/x/member/web/sign/update")
            .form(
                &[
                    ("user_sign", user_sign.to_string()),
                    ("csrf", self.csrf().unwrap_or_default()),
                ]
            )
            .send_bpi("设置个人签名").await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    /// 更新签名测试
    async fn test_update_user_sign() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        // 测试修改签名
        let test_sign = "这是一个测试签名 - Powered by Rust";
        let _result = bpi.member_center_update_user_sign(test_sign).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_sign_length_validation() {
        let bpi = BpiClient::new();

        // 测试超长签名
        let long_sign = "a".repeat(71);
        let result = bpi.member_center_update_user_sign(&long_sign).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("签名长度不能超过70个字符"));
    }
}
