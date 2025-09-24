use crate::{ BpiClient, BpiResponse, BpiError, BilibiliRequest };

impl BpiClient {
    /// 大会员签到
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/vip/clockin.html#大会员签到)
    ///
    pub async fn vip_sign(&self) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        self
            .post("https://api.bilibili.com/pgc/activity/score/task/sign")
            .form(&[("csrf", csrf)])

            .header("referer", "https://www.bilibili.com")
            .send_bpi("大会员签到").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_vip_sign_cookie() {
        let bpi = BpiClient::new();

        let resp = bpi.vip_sign().await;
        assert!(resp.is_ok());
        if let Ok(r) = resp {
            info!("大会员签到: {:?}", r);
        }
    }
}
