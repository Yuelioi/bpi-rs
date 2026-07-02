use serde::{Deserialize, Serialize};

use crate::login::LoginQrPollParams;
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

const QR_GENERATE_ENDPOINT: &str =
    "https://passport.bilibili.com/x/passport-login/web/qrcode/generate";
const QR_POLL_ENDPOINT: &str = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll";

/// 生成 QRCode 数据
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GenerateQrCodeData {
    /// QR login URL rendered by callers.
    pub url: String,
    /// Temporary key used to poll QR login state.
    pub qrcode_key: String,
}

/// 二维码状态数据
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct CheckQrCodeStatusData {
    pub url: String,           // 游戏分站跨域登录 url
    pub refresh_token: String, // 刷新令牌
    pub timestamp: u64,        // 时间戳
    pub code: i32,             // 状态码
    pub message: String,       // 扫码状态信息

    /// Cookies returned only after a successful QR login.
    #[serde(default)]
    pub cookies: Vec<(String, String)>,
}

/// 二维码图片数据
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QrcodeImageData {
    pub qr_image: String, // base64 编码的二维码图片
    pub expires_in: u64,  // 过期时间（秒）
}

impl BpiClient {
    /// 发送二维码请求
    pub async fn login_send_qrcode(&self) -> Result<BpiResponse<GenerateQrCodeData>, BpiError> {
        self.get(QR_GENERATE_ENDPOINT).send_bpi("发送二维码").await
    }

    /// 检查二维码状态
    pub async fn login_check_qrcode_status(
        &self,
        params: LoginQrPollParams,
    ) -> Result<BpiResponse<CheckQrCodeStatusData>, BpiError> {
        let response = self
            .get(QR_POLL_ENDPOINT)
            .query(&params.query_pairs())
            .send()
            .await?;

        let cookies: Vec<(String, String)> = response
            .cookies()
            .map(|c| (c.name().to_string(), c.value().to_string()))
            .collect();

        let mut qr_response: BpiResponse<CheckQrCodeStatusData> = response
            .json()
            .await
            .map_err(|e| BpiError::parse(e.to_string()))?;

        if qr_response.code == 0 {
            if let Some(ref mut data) = qr_response.data {
                if data.code == 0 {
                    data.cookies = cookies;
                    Ok(qr_response)
                } else {
                    Err(BpiError::from_code_message(data.code, data.message.clone()))
                }
            } else {
                Err(BpiError::missing_data())
            }
        } else {
            Err(BpiError::from_code(qr_response.code))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiEnvelope;
    use crate::probe::contract::{ApiContract, HttpMethod};
    use tokio;

    fn local_qr_probe_body(endpoint: &str) -> Option<serde_json::Value> {
        let path = format!("target/bpi-probe-runs/login/qr/{endpoint}/anonymous.response.json");
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("response")
            .and_then(|response| response.get("body"))
            .cloned()
    }

    #[test]
    fn qr_generate_contract_matches_endpoint_request() -> Result<(), BpiError> {
        let contract = ApiContract::from_slice(include_bytes!(
            "../../../tests/contracts/login/qr/generate/anonymous.request.json"
        ))?;

        assert_eq!(contract.name, "login.qr_generate.anonymous");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(contract.request.url.as_str(), QR_GENERATE_ENDPOINT);
        assert!(contract.request.query.is_empty());
        assert_eq!(contract.expect["api_code"], 0);
        assert!(!contract.request.auth.requires_cookie());
        Ok(())
    }

    #[test]
    fn qr_generate_model_matches_local_probe_output_when_available() -> Result<(), BpiError> {
        let Some(body) = local_qr_probe_body("generate") else {
            return Ok(());
        };

        let data =
            serde_json::from_value::<ApiEnvelope<GenerateQrCodeData>>(body)?.into_payload()?;

        assert!(data.url.starts_with("https://"));
        assert!(!data.qrcode_key.trim().is_empty());
        Ok(())
    }

    #[test]
    fn qr_poll_model_matches_local_probe_output_when_available() -> Result<(), BpiError> {
        let Some(body) = local_qr_probe_body("poll") else {
            return Ok(());
        };

        let response = serde_json::from_value::<BpiResponse<CheckQrCodeStatusData>>(body)?;
        let data = response.into_data()?;

        assert_eq!(data.code, 86101);
        assert!(!data.message.trim().is_empty());
        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_send_qrcode() {
        use tokio::time::{Duration, sleep};

        tracing::info!("获取二维码...");

        let bpi = BpiClient::new().expect("client should build");
        match bpi.login_send_qrcode().await {
            Ok(response) => {
                tracing::info!("Code: {}", response.code);
                tracing::info!("Message: {}", response.message);

                let data = response.data.unwrap();
                tracing::info!("二维码URL: {}", data.url);
                tracing::info!("已获取二维码轮询 key");

                for _ in 1..=3 {
                    // 每次循环延迟 5 秒
                    sleep(Duration::from_secs(20)).await;
                    let params = LoginQrPollParams::new(data.qrcode_key.as_str()).unwrap();
                    let resp = bpi.login_check_qrcode_status(params).await;

                    if resp.is_ok() {
                        tracing::info!("扫码成功{:?}", resp);
                    } else if let Err(error) = resp {
                        tracing::error!("扫码失败: {:?}", error);
                    }
                }
            }
            Err(e) => {
                tracing::info!("二维码请求失败: {}", e);
            }
        }
    }
}
