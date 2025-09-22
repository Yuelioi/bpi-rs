use serde::{ Deserialize, Serialize };

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

/// 生成 QRCode 数据
#[derive(Debug, Deserialize, Serialize)]
pub struct GenerateQrCodeData {
    pub url: String, // 二维码登录url
    pub qrcode_key: String, // 扫码登录标识
}

/// 二维码状态数据
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CheckQrCodeStatusData {
    pub url: String, // 游戏分站跨域登录 url
    pub refresh_token: String, // 刷新令牌
    pub timestamp: u64, // 时间戳
    pub code: i32, // 状态码
    pub message: String, // 扫码状态信息

    /// cookie 仅在扫码成功后写入
    ///
    ///  [("SESSDATA", "xxx")]
    ///
    /// sessdata
    #[serde(default)]
    pub cookies: Vec<(String, String)>,
}

/// 二维码图片数据
#[derive(Serialize, Deserialize, Clone)]
pub struct QrcodeImageData {
    pub qr_image: String, // base64 编码的二维码图片
    pub expires_in: u64, // 过期时间（秒）
}

// ================= 核心实现 =================
impl BpiClient {
    /// 发送二维码请求
    pub async fn login_send_qrcode(&self) -> Result<BpiResponse<GenerateQrCodeData>, BpiError> {
        self
            .get("https://passport.bilibili.com/x/passport-login/web/qrcode/generate")
            .send_bpi("发送二维码").await
    }

    /// 检查二维码状态
    pub async fn login_check_qrcode_status(
        &self,
        qrcode_key: &str
    ) -> Result<BpiResponse<CheckQrCodeStatusData>, BpiError> {
        let response = self
            .get("https://passport.bilibili.com/x/passport-login/web/qrcode/poll")
            .query(&[("qrcode_key", qrcode_key)])
            .send().await?;

        let cookies: Vec<(String, String)> = response
            .cookies()
            .map(|c| (c.name().to_string(), c.value().to_string()))
            .collect();

        let mut qr_response: BpiResponse<CheckQrCodeStatusData> = response
            .json().await
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

// ================= 测试模块 =================
#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_send_qrcode() {
        use tokio::time::{ Duration, sleep };

        tracing::info!("获取二维码...");

        let bpi = BpiClient::new();
        match bpi.login_send_qrcode().await {
            Ok(response) => {
                tracing::info!("Code: {}", response.code);
                tracing::info!("Message: {}", response.message);

                let data = response.data.unwrap();
                tracing::info!("二维码URL: {}", data.url);
                tracing::info!("QR Key: {}", data.qrcode_key);

                for _ in 1..=3 {
                    // 每次循环延迟 5 秒
                    sleep(Duration::from_secs(20)).await;
                    let resp = bpi.login_check_qrcode_status(data.qrcode_key.as_str()).await;

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
