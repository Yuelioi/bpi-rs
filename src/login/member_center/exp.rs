//! 查询每日投币获得经验数
//!
//! [文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/login/member_center.html#查询每日投币获得经验数)

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

const TODAY_COIN_EXP_ENDPOINT: &str = "https://api.bilibili.com/x/web-interface/coin/today/exp";

impl BpiClient {
    /// 查询每日投币获得经验数
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/login)
    pub async fn member_center_today_coin_exp(&self) -> Result<BpiResponse<u32>, BpiError> {
        self.get(TODAY_COIN_EXP_ENDPOINT)
            .send_bpi("每日投币经验")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ApiEnvelope;
    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;

    #[test]
    fn member_center_today_coin_exp_matches_login_contract() -> Result<(), BpiError> {
        let contract = EndpointContract::from_slice(include_bytes!(
            "../../../tests/contracts/login/today-coin-exp/contract.json"
        ))?;

        assert_eq!(contract.name, "login.today_coin_exp");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(contract.request.url.as_str(), TODAY_COIN_EXP_ENDPOINT);
        Ok(())
    }

    #[test]
    fn member_center_today_coin_exp_fixture_parses_legacy_u32() -> Result<(), BpiError> {
        let value = ApiEnvelope::<u32>::from_slice(include_bytes!(
            "../../../tests/contracts/login/today-coin-exp/responses/normal.success.json"
        ))?
        .into_payload()?;

        assert!(value <= 50);
        Ok(())
    }
}
