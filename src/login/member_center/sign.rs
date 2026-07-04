// 修改签名
//
// [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/login/member_center.md)

use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResponse;
use crate::login::LoginClient;

impl<'a> LoginClient<'a> {
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

        let csrf = self.client.csrf()?;
        let result = self
            .client
            .post("https://api.bilibili.com/x/member/web/sign/update")
            .form(&[("user_sign", user_sign.to_string()), ("csrf", csrf)])
            .send_bpi("设置个人签名")
            .await?;

        Ok(result)
    }
}
