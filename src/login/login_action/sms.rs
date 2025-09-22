use reqwest::header::SET_COOKIE;

use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct SMSSendData {
    captcha_key: String, // 短信登录 token
}

#[derive(Debug, Deserialize, Serialize)]
struct SMSLoginData {
    is_new: bool, // 是否为新用户
    status: i32,  // 0:成功
    url: String,  // 跳转url
}

impl BpiClient {
    /// 发送短信验证码（Web端）
    ///
    /// # 参数
    /// * `cid` - 国际冠字码
    /// * `tel` - 手机号码
    /// * `source` - 登录来源 "main_web" 或 "main_mini"
    /// * `token` - 登录 API token
    /// * `challenge` - 极验 challenge
    /// * `validate` - 极验 result
    /// * `seccode` - 极验 result + "|jordan"
    pub async fn login_send_sms_code(
        &self,
        cid: u32,
        tel: u32,
        source: &str,
        token: &str,
        challenge: &str,
        validate: &str,
        seccode: &str,
    ) -> Result<BpiResponse<SMSSendData>, BpiError> {
        // 构建表单
        let form = vec![
            ("cid", cid.to_string()),
            ("tel", tel.to_string()),
            ("source", source.to_string()),
            ("token", token.to_string()),
            ("challenge", challenge.to_string()),
            ("validate", validate.to_string()),
            ("seccode", seccode.to_string()),
        ];

        // 发送请求
        let result = self
            .post("https://passport.bilibili.com/x/passport-login/web/sms/send")
            .form(&form)
            .send_bpi("发送短信验证码")
            .await?;

        Ok(result)
    }

    /// 短信登录
    ///
    /// * `cid` - 国际冠字码
    /// * `tel` - 手机号码
    /// * `captcha_key` - 短信登录 token(基于login_send_sms_code)
    /// * `code` - 短信验证码 5min过期
    pub async fn login_with_sms(
        &self,
        cid: u32,
        tel: u32,
        captcha_key: &str,
        code: &str,
    ) -> Result<(), String> {
        let form = vec![
            ("cid", cid.to_string()),
            ("tel", tel.to_string()),
            ("code", code.to_string()),
            ("source", "main_web".to_string()),
            ("captcha_key", captcha_key.to_string()),
            ("go_url", "https://www.bilibili.com".to_string()),
            ("keep", true.to_string()),
        ];

        let response = BpiClient::new()
            .post("https://passport.bilibili.com/x/passport-login/web/login/sms")
            .form(&form)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if let Some(cookies) = response
            .headers()
            .get_all(SET_COOKIE)
            .iter()
            .map(|v| v.to_str().unwrap_or(""))
            .collect::<Vec<_>>()
            .join("; ")
            .into()
        {
            info!("登录返回的 Cookie: {}", cookies);
        }

        let resp = response
            .json::<BpiResponse<SMSLoginData>>()
            .await
            .map_err(|e| {
                error!("解析短信登录响应失败: {:?}", e);
                e.to_string()
            })?;

        if resp.code != 0 {
            error!("短信登录失败: code={}, message={}", resp.code, resp.message);
            return Err(resp.message);
        }

        match resp.code {
            0 => {
                info!("短信登录成功");
                Ok(())
            }
            code => {
                error!("验证码发送失败: code={}, message={}", code, resp.message);
                let msg = match code {
                    -400 => "请求错误".to_string(),
                    1006 => "请输入正确的短信验证码".to_string(),
                    1007 => "短信验证码已过期".to_string(),

                    _ => resp.message,
                };
                Err(msg)
            }
        }
    }
}
