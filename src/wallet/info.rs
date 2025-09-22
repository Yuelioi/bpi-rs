use serde::{ Deserialize, Serialize };
use serde_json::json;

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

/// 用户钱包数据
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserWallet {
    /// 用户 mid
    pub mid: i64,
    /// 总计 B 币
    pub total_bp: f64,
    /// 默认 B 币
    pub default_bp: f64,
    /// iOS B 币
    pub ios_bp: f64,
    /// 优惠券余额
    pub coupon_balance: f64,
    /// 可用 B 币
    pub available_bp: f64,
    /// 不可用 B 币
    pub unavailable_bp: f64,
    /// 不可用原因
    pub unavailable_reason: String,
    /// 提示信息
    pub tip: String,
    /// 需要显示类余额, 1
    pub need_show_class_balance: i64,
}

impl BpiClient {
    /// 获取用户钱包信息
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/wallet/info.html#获取用户钱包信息
    pub async fn wallet_info(&self) -> Result<BpiResponse<UserWallet>, BpiError> {
        let csrf = self.csrf()?;

        let timestamp = chrono::Utc::now().timestamp_millis();

        let body =
            json!({
            "csrf": csrf,
            "platformType": 3,
            "timestamp": timestamp,
            "traceId": timestamp,
            "version": "1.0",
        });

        self
            .post("https://pay.bilibili.com/paywallet/wallet/getUserWallet")
            .json(&body)
            .send_bpi("获取用户钱包").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_get_user_wallet() {
        let bpi = BpiClient::new();
        let resp = bpi.wallet_info().await;
        info!("响应: {:?}", resp);
        assert!(resp.is_ok());
        if let Ok(data) = resp {
            info!("用户mid: {}", data.data.unwrap().mid);
        }
    }
}
