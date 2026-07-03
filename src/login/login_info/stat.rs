//! 登录用户状态数（双端）
//!
//! [查看 API 文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/login/login_info.html#登录用户状态数-双端)

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

const STAT_ENDPOINT: &str = "https://api.bilibili.com/x/web-interface/nav/stat";

/// 登录用户状态数 - 信息体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStat {
    /// 当前关注数
    pub following: u64,
    /// 当前粉丝数
    pub follower: u64,
    /// 发布的动态数
    pub dynamic_count: u64,
}

impl BpiClient {
    /// 获取登录用户状态数（关注/粉丝/动态）
    pub async fn login_info_user_stat(&self) -> Result<BpiResponse<UserStat>, BpiError> {
        let result = self.get(STAT_ENDPOINT).send_bpi("获取登录用户状态").await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;
    use crate::{ApiEnvelope, BpiResult};

    fn contract() -> BpiResult<EndpointContract> {
        EndpointContract::from_slice(include_bytes!(
            "../../../tests/contracts/login/stat/contract.json"
        ))
    }

    fn live_login_tests_enabled() -> bool {
        std::env::var("BPI_LIVE_TEST").ok().as_deref() == Some("1")
    }

    fn live_client() -> Result<BpiClient, BpiError> {
        match std::env::var("BPI_COOKIE") {
            Ok(cookie) if !cookie.trim().is_empty() => BpiClient::builder().cookie(cookie).build(),
            _ => BpiClient::new(),
        }
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_user_stat() -> Result<(), BpiError> {
        if !live_login_tests_enabled() {
            return Ok(());
        }

        let bpi = live_client()?;

        match bpi.login_info_user_stat().await {
            Ok(resp) => {
                if resp.code == 0 {
                    let Some(data) = resp.data else {
                        return Ok(());
                    };

                    tracing::info!(
                        "关注数: {}, 粉丝数: {}, 动态数: {}",
                        data.following,
                        data.follower,
                        data.dynamic_count
                    );
                } else {
                    tracing::info!("请求失败: code={}, message={}", resp.code, resp.message);
                }
            }
            Err(err) => {
                return Err(err);
            }
        }

        Ok(())
    }

    #[test]
    fn legacy_login_info_stat_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = contract()?;

        assert_eq!(contract.name, "login.stat");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(contract.request.url.as_str(), STAT_ENDPOINT);
        assert!(contract.request.query.is_empty());
        assert_eq!(contract.cases.len(), 3);
        assert_eq!(contract.cases[0].response.api_code, Some(-101));
        assert_eq!(contract.cases[1].response.api_code, Some(0));
        assert_eq!(contract.cases[2].response.api_code, Some(0));
        Ok(())
    }

    #[test]
    fn legacy_login_info_stat_fixtures_parse_promoted_contract_models() -> BpiResult<()> {
        for (bytes, expected) in [
            (
                include_bytes!("../../../tests/contracts/login/stat/responses/normal.success.json")
                    .as_slice(),
                (1, 2, 3),
            ),
            (
                include_bytes!("../../../tests/contracts/login/stat/responses/vip.success.json")
                    .as_slice(),
                (10, 20, 30),
            ),
        ] {
            let payload = ApiEnvelope::<UserStat>::from_slice(bytes)?.into_payload()?;
            assert_eq!(
                (payload.following, payload.follower, payload.dynamic_count),
                expected
            );
        }

        let err = ApiEnvelope::<serde_json::Value>::from_slice(include_bytes!(
            "../../../tests/contracts/login/stat/responses/anonymous.error.json"
        ))?
        .ensure_success()
        .unwrap_err();
        assert!(err.requires_login());
        Ok(())
    }
}
