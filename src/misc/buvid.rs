//! 获取 buvid3 (Web端)
//!
//! 文档：https://api.bilibili.com/x/web-frontend/getbuvid

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 获取 buvid3 - 响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buvid3Data {
    /// buvid3，需要手动存放至 Cookie 中
    pub buvid: String,
}

impl BpiClient {
    /// 获取 buvid3
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/misc
    pub async fn misc_buvid3(&self) -> Result<BpiResponse<Buvid3Data>, BpiError> {
        self.get("https://api.bilibili.com/x/web-frontend/getbuvid")
            .send_bpi("获取 buvid3")
            .await
    }
}

/// 获取 buvid3/4 - 响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuvidData {
    /// buvid3，需要手动存放至 Cookie 中
    #[serde(rename = "b_3")]
    pub buvid3: String,

    /// buvid4，需要手动存放至 Cookie 中
    #[serde(rename = "b_4")]
    pub buvid4: String,
}

impl BpiClient {
    /// 获取 buvid3 / buvid4
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/misc
    pub async fn misc_buvid(&self) -> Result<BpiResponse<BuvidData>, BpiError> {
        self.get("https://api.bilibili.com/x/frontend/finger/spi")
            .send_bpi("获取 buvid3/4")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_buvid3() {
        let bpi = BpiClient::new();

        match bpi.misc_buvid3().await {
            Ok(resp) => {
                if resp.code == 0 {
                    let data = resp.data.unwrap();
                    tracing::info!("获取 buvid3 成功: {}", data.buvid);
                } else {
                    tracing::info!("请求失败: code={}, message={}", resp.code, resp.message);
                }
            }
            Err(err) => {
                panic!("请求出错: {}", err);
            }
        }
    }

    #[tokio::test]
    async fn test_get_buvid() {
        let bpi = BpiClient::new();

        match bpi.misc_buvid().await {
            Ok(resp) => {
                if resp.code == 0 {
                    let data = resp.data.unwrap();
                    tracing::info!("获取 buvid3 成功: {}", data.buvid3);
                    tracing::info!("获取 buvid4 成功: {}", data.buvid4);
                } else {
                    tracing::info!("请求失败: code={}, message={}", resp.code, resp.message);
                }
            }
            Err(err) => {
                panic!("请求出错: {}", err);
            }
        }
    }
}
