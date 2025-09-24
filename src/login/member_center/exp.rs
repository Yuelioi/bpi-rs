//! 查询每日投币获得经验数
//!
//! [文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/login/member_center.html#查询每日投币获得经验数)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

impl BpiClient {
    /// 查询每日投币获得经验数
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/login)
    pub async fn member_center_today_coin_exp(&self) -> Result<BpiResponse<u32>, BpiError> {
        self
            .get("https://api.bilibili.com/x/web-interface/coin/today/exp")
            .send_bpi("每日投币经验").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_today_coin_exp() {
        let bpi = BpiClient::new();

        match bpi.member_center_today_coin_exp().await {
            Ok(resp) => {
                if resp.code == 0 {
                    tracing::info!("今日投币获得经验: {:?}", resp.data.unwrap());
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
