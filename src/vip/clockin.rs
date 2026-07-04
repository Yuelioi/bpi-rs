


use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResponse;
use crate::vip::VipClient;


impl<'a> VipClient<'a> {
    /// 大会员签到
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/vip/clockin.html#大会员签到)
    ///
    pub async fn vip_sign(&self) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;
        self
            .client.post("https://api.bilibili.com/pgc/activity/score/task/sign")
            .form(&[("csrf", csrf)])
            .header("referer", "https://www.bilibili.com")
            .send_bpi("大会员签到").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

}
