use crate::request::BilibiliRequest;
use crate::{BpiClient, BpiResult};

use super::model::{
    LoginAccountInfo, LoginCoinBalance, LoginDailyReward, LoginNav, LoginStats, LoginTodayCoinExp,
    LoginVipInfo,
};

const NAV_ENDPOINT: &str = "https://api.bilibili.com/x/web-interface/nav";
const STAT_ENDPOINT: &str = "https://api.bilibili.com/x/web-interface/nav/stat";
const COIN_ENDPOINT: &str = "https://account.bilibili.com/site/getCoin";
const TODAY_COIN_EXP_ENDPOINT: &str = "https://api.bilibili.com/x/web-interface/coin/today/exp";
const DAILY_REWARD_ENDPOINT: &str = "https://api.bilibili.com/x/member/web/exp/reward";
const ACCOUNT_INFO_ENDPOINT: &str = "https://api.bilibili.com/x/member/web/account";
const VIP_INFO_ENDPOINT: &str = "https://api.bilibili.com/x/vip/web/user/info";

/// Login domain API client.
#[derive(Clone, Copy)]
pub struct LoginClient<'a> {
    client: &'a BpiClient,
}

impl<'a> LoginClient<'a> {
    pub(crate) fn new(client: &'a BpiClient) -> Self {
        Self { client }
    }

    #[cfg(test)]
    pub(crate) fn nav_endpoint(&self) -> &'static str {
        NAV_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn stat_endpoint(&self) -> &'static str {
        STAT_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn coin_endpoint(&self) -> &'static str {
        COIN_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn today_coin_exp_endpoint(&self) -> &'static str {
        TODAY_COIN_EXP_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn daily_reward_endpoint(&self) -> &'static str {
        DAILY_REWARD_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn account_info_endpoint(&self) -> &'static str {
        ACCOUNT_INFO_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn vip_info_endpoint(&self) -> &'static str {
        VIP_INFO_ENDPOINT
    }

    /// Fetches the current session's navigation/login state.
    pub async fn nav(&self) -> BpiResult<LoginNav> {
        self.client
            .get(NAV_ENDPOINT)
            .send_bpi::<LoginNav>("login.nav")
            .await?
            .into_data()
    }

    /// Fetches the current authenticated user's following, follower, and dynamic counts.
    pub async fn stat(&self) -> BpiResult<LoginStats> {
        self.client
            .get(STAT_ENDPOINT)
            .send_bpi::<LoginStats>("login.stat")
            .await?
            .into_data()
    }

    /// Fetches the current authenticated account's coin balance.
    pub async fn coin(&self) -> BpiResult<LoginCoinBalance> {
        self.client
            .get(COIN_ENDPOINT)
            .send_bpi::<LoginCoinBalance>("login.coin")
            .await?
            .into_data()
    }

    /// Fetches today's experience gained from coin operations.
    pub async fn today_coin_exp(&self) -> BpiResult<LoginTodayCoinExp> {
        self.client
            .get(TODAY_COIN_EXP_ENDPOINT)
            .send_bpi::<LoginTodayCoinExp>("login.today_coin_exp")
            .await?
            .into_data()
    }

    /// Fetches the current authenticated account's daily reward completion state.
    pub async fn daily_reward(&self) -> BpiResult<LoginDailyReward> {
        self.client
            .get(DAILY_REWARD_ENDPOINT)
            .send_bpi::<LoginDailyReward>("login.daily_reward")
            .await?
            .into_data()
    }

    /// Fetches the current authenticated account's profile.
    pub async fn account_info(&self) -> BpiResult<LoginAccountInfo> {
        self.client
            .get(ACCOUNT_INFO_ENDPOINT)
            .send_bpi::<LoginAccountInfo>("login.account_info")
            .await?
            .into_data()
    }

    /// Fetches the current authenticated account's VIP state.
    pub async fn vip_info(&self) -> BpiResult<LoginVipInfo> {
        self.client
            .get(VIP_INFO_ENDPOINT)
            .send_bpi::<LoginVipInfo>("login.vip_info")
            .await?
            .into_data()
    }
}

#[cfg(test)]
mod tests {
    use crate::BpiClient;

    #[test]
    fn login_client_borrows_root_client() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let login = client.login();

        assert_eq!(
            login.nav_endpoint(),
            "https://api.bilibili.com/x/web-interface/nav"
        );
        Ok(())
    }

    #[test]
    fn login_client_exposes_stat_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let login = client.login();

        assert_eq!(
            login.stat_endpoint(),
            "https://api.bilibili.com/x/web-interface/nav/stat"
        );
        Ok(())
    }

    #[test]
    fn login_client_exposes_coin_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let login = client.login();

        assert_eq!(
            login.coin_endpoint(),
            "https://account.bilibili.com/site/getCoin"
        );
        Ok(())
    }

    #[test]
    fn login_client_exposes_today_coin_exp_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let login = client.login();

        assert_eq!(
            login.today_coin_exp_endpoint(),
            "https://api.bilibili.com/x/web-interface/coin/today/exp"
        );
        Ok(())
    }

    #[test]
    fn login_client_exposes_daily_reward_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let login = client.login();

        assert_eq!(
            login.daily_reward_endpoint(),
            "https://api.bilibili.com/x/member/web/exp/reward"
        );
        Ok(())
    }

    #[test]
    fn login_client_exposes_account_info_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let login = client.login();

        assert_eq!(
            login.account_info_endpoint(),
            "https://api.bilibili.com/x/member/web/account"
        );
        Ok(())
    }

    #[test]
    fn login_client_exposes_vip_info_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let login = client.login();

        assert_eq!(
            login.vip_info_endpoint(),
            "https://api.bilibili.com/x/vip/web/user/info"
        );
        Ok(())
    }
}
