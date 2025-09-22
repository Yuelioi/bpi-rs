use serde::{Deserialize, Serialize};

use crate::client::BilibiliRequest;
use crate::{BpiClient, BpiError, BpiResponse};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeetestData {
    #[serde(rename = "type")]
    pub type_field: String,
    pub token: String,
    pub geetest: Geetest,
    pub tencent: Tencent,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geetest {
    pub challenge: String,
    pub gt: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tencent {
    pub appid: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GenerateCaptcha {
    pub token: String,
    pub gt: String,
    pub challenge: String,
}

impl BpiClient {
    pub async fn login_generate_captcha(&self) -> Result<GenerateCaptcha, BpiError> {
        let result: BpiResponse<GeetestData> = self
            .get("https://passport.bilibili.com/x/passport-login/captcha?source=main_web")
            .send_bpi("获取验证码")
            .await?;

        let data = result.into_data()?;

        let token = data.token;
        let geetest = data.geetest;

        Ok(GenerateCaptcha {
            token,
            gt: geetest.gt,
            challenge: geetest.challenge,
        })
    }
}

#[tokio::test]
async fn test_generate_captcha() {
    let bpi = BpiClient::new();
    match bpi.login_generate_captcha().await {
        Ok(captcha) => {
            tracing::info!("验证码请求成功！");
            tracing::info!("Token: {}", captcha.token);
            tracing::info!("GT: {}", captcha.gt);
            tracing::info!("Challenge: {}", captcha.challenge);
        }
        Err(e) => {
            tracing::info!("验证码请求失败: {}", e);
        }
    }
}
