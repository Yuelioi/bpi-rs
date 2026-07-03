//! 获取我的信息
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/login/member_center.md)

use crate::login::LoginAccountInfo;
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

const ACCOUNT_INFO_ENDPOINT: &str = "https://api.bilibili.com/x/member/web/account";

/// Legacy member-center account info type.
pub type AccountInfo = LoginAccountInfo;

impl BpiClient {
    /// 获取我的账号信息
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/login)
    pub async fn member_center_account_info(&self) -> Result<BpiResponse<AccountInfo>, BpiError> {
        self.get(ACCOUNT_INFO_ENDPOINT)
            .send_bpi("获取我的信息")
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
    fn member_center_account_info_matches_login_account_contract() -> Result<(), BpiError> {
        let contract = EndpointContract::from_slice(include_bytes!(
            "../../../tests/contracts/login/account-info/contract.json"
        ))?;

        assert_eq!(contract.name, "login.account_info");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(contract.request.url.as_str(), ACCOUNT_INFO_ENDPOINT);
        Ok(())
    }

    #[test]
    fn member_center_account_info_fixture_parses_legacy_alias() -> Result<(), BpiError> {
        let data = ApiEnvelope::<AccountInfo>::from_slice(include_bytes!(
            "../../../tests/contracts/login/account-info/responses/normal.success.json"
        ))?
        .into_payload()?;

        assert!(data.mid.get() > 0);
        Ok(())
    }
}
