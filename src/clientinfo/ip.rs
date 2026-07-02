//! IP 地址归属地查询 API
//!
//! [参考文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/clientinfo/ip.md)

use crate::clientinfo::params::ClientInfoIpParams;
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// ==========================
// 数据结构
// ==========================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpInfo {
    /// 国家
    pub country: Option<String>,
    /// 省份
    pub province: Option<String>,
    /// 城市
    pub city: Option<String>,
    /// ISP 运营商
    pub isp: Option<String>,
    /// IP 地址
    pub addr: Option<String>,
}

// ==========================
// API 封装
// ==========================

impl BpiClient {
    /// 查询 IP 地址归属地
    ///
    /// 查询指定 IP 地址的地理位置信息，包括国家、省份、城市和 ISP 运营商。
    /// 如果不提供 IP 参数，将返回请求方 IP 的地理信息。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `ClientInfoIpParams` | IP 归属地查询参数。如果留空，返回请求方 IP 信息 |
    ///
    /// # 文档
    /// [IP 地址归属地查询](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/clientinfo/ip.md)
    pub async fn clientinfo_ip(
        &self,
        params: ClientInfoIpParams,
    ) -> Result<BpiResponse<IpInfo>, BpiError> {
        self.get("https://api.live.bilibili.com/ip_service/v1/ip_service/get_ip_addr")
            .query(&params.query_pairs())
            .send_bpi("查询 IP 地址归属地")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probe::contract::{ApiContract, HttpMethod};
    use crate::{ApiEnvelope, BpiResult};

    const TEST_IP: &str = "8.8.8.8";

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_clientinfo_ip() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new().expect("client should build");

        let params = ClientInfoIpParams::new().with_ip_str(TEST_IP)?;
        let resp = bpi.clientinfo_ip(params).await?;
        if resp.code == 0 {
            if let Some(data) = resp.data {
                tracing::info!(
                    "IP 地址: {}, 省份: {:?}, 城市: {:?}, ISP: {:?}",
                    data.addr.unwrap_or_default(),
                    data.province,
                    data.city,
                    data.isp
                );
            }
        } else {
            tracing::error!("请求失败: code={}, message={}", resp.code, resp.message);
        }

        Ok(())
    }

    fn contract(profile: &str) -> BpiResult<ApiContract> {
        let bytes = match profile {
            "anonymous" => {
                include_bytes!("../../tests/contracts/clientinfo/ip/anonymous.request.json")
                    .as_slice()
            }
            "normal" => {
                include_bytes!("../../tests/contracts/clientinfo/ip/normal.request.json").as_slice()
            }
            "vip" => {
                include_bytes!("../../tests/contracts/clientinfo/ip/vip.request.json").as_slice()
            }
            _ => unreachable!("unknown clientinfo ip contract profile"),
        };

        ApiContract::from_slice(bytes)
    }

    #[test]
    fn clientinfo_ip_contracts_match_endpoint_request() -> BpiResult<()> {
        for profile in ["anonymous", "normal", "vip"] {
            let contract = contract(profile)?;

            assert_eq!(contract.request.method, HttpMethod::Get);
            assert_eq!(
                contract.request.url.as_str(),
                "https://api.live.bilibili.com/ip_service/v1/ip_service/get_ip_addr"
            );
            assert_eq!(contract.request.query.get("ip"), Some(&TEST_IP.to_string()));
            assert_eq!(contract.expect["api_code"], 0);
        }
        Ok(())
    }

    fn local_probe_body(profile: &str) -> Option<serde_json::Value> {
        let path = format!("target/bpi-probe-runs/clientinfo/ip/ip/{profile}.response.json");
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("response")
            .and_then(|response| response.get("body"))
            .cloned()
    }

    #[test]
    fn clientinfo_ip_model_matches_local_probe_outputs_when_available() -> BpiResult<()> {
        for profile in ["anonymous", "normal", "vip"] {
            let Some(body) = local_probe_body(profile) else {
                continue;
            };

            let payload = serde_json::from_value::<ApiEnvelope<IpInfo>>(body)?.into_payload()?;

            assert_eq!(payload.addr.as_deref(), Some(TEST_IP));
        }
        Ok(())
    }
}
