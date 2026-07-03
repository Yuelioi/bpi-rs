//! 查询每日奖励状态
//!
//! [文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/login/member_center.html#查询每日奖励状态)

use crate::login::LoginDailyReward;
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

const DAILY_REWARD_ENDPOINT: &str = "https://api.bilibili.com/x/member/web/exp/reward";

/// Legacy member-center daily reward type.
pub type DailyReward = LoginDailyReward;

impl BpiClient {
    /// 查询每日奖励状态
    pub async fn member_center_daily_reward(&self) -> Result<BpiResponse<DailyReward>, BpiError> {
        self.get(DAILY_REWARD_ENDPOINT)
            .send_bpi("查询每日奖励状态")
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
    fn member_center_daily_reward_matches_login_contract() -> Result<(), BpiError> {
        let contract = EndpointContract::from_slice(include_bytes!(
            "../../../tests/contracts/login/daily-reward/contract.json"
        ))?;

        assert_eq!(contract.name, "login.daily_reward");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(contract.request.url.as_str(), DAILY_REWARD_ENDPOINT);
        assert_eq!(contract.cases.len(), 3);
        Ok(())
    }

    #[test]
    fn member_center_daily_reward_fixture_parses_legacy_alias() -> Result<(), BpiError> {
        let reward = ApiEnvelope::<DailyReward>::from_slice(include_bytes!(
            "../../../tests/contracts/login/daily-reward/responses/normal.success.json"
        ))?
        .into_payload()?;

        assert!(reward.coins <= 50);
        Ok(())
    }
}
