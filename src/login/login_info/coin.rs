//! 获取硬币数
//!
//! https://socialsisteryi.github.io/bilibili-API-collect/docs/login/login_info_info.html#获取硬币数

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 获取硬币数 - 响应结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinInfo {
    /// 当前硬币数
    pub money: f64,
}

impl BpiClient {
    /// 获取账号硬币数
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/login
    pub async fn login_info_coin(&self) -> Result<BpiResponse<CoinInfo>, BpiError> {
        self.get("https://account.bilibili.com/site/getCoin").send_bpi("获取硬币数").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_coin() {
        let bpi = BpiClient::new();

        match bpi.login_info_coin().await {
            Ok(resp) => {
                if resp.code == 0 {
                    tracing::info!("获取硬币数成功: {:?}", resp.data.unwrap().money);
                } else {
                    tracing::info!("请求失败: code={}", resp.code);
                }
            }
            Err(err) => {
                panic!("请求出错: {}", err);
            }
        }
    }
}
