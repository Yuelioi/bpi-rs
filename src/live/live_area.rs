use serde::{Deserialize, Serialize};

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

// ================= 数据结构 =================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LiveSubArea {
    /// 子分区id
    pub id: String,
    /// 父分区id
    pub parent_id: String,
    /// 旧分区id
    pub old_area_id: String,
    /// 子分区名
    pub name: String,
    /// 活动id
    pub act_id: String,
    /// pk状态
    pub pk_status: String,
    /// 是否为热门分区
    pub hot_status: i32,
    /// 锁定状态
    pub lock_status: String,
    /// 子分区标志图片url
    pub pic: String,
    /// 父分区名
    pub parent_name: String,
    /// 区域类型
    pub area_type: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LiveParentArea {
    /// 父分区id
    pub id: i32,
    /// 父分区名
    pub name: String,
    /// 子分区列表
    pub list: Vec<LiveSubArea>,
}

impl BpiClient {
    /// 获取全部直播间分区列表
    ///
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live)
    pub async fn live_area_list(&self) -> Result<BpiResponse<Vec<LiveParentArea>>, BpiError> {
        let resp = self
            .get("https://api.live.bilibili.com/room/v1/Area/getList")
            .send_bpi("获取全部直播间分区列表")
            .await?;

        Ok(resp)
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
            "../../tests/contracts/live/public-core/area-list/contract.json"
        ))
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_live_area_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi.live_area_list().await?;

        let data = resp.data.unwrap();
        assert!(!data.is_empty());
        Ok(())
    }

    #[test]
    fn live_area_list_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = contract()?;

        assert_eq!(contract.name, "live.area_list");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.live.bilibili.com/room/v1/Area/getList"
        );
        assert!(contract.request.query.is_empty());
        assert_eq!(contract.cases.len(), 3);
        assert_eq!(
            contract.cases[0].response.rust_model.as_deref(),
            Some("Vec<LiveParentArea>")
        );
        Ok(())
    }

    #[test]
    fn live_area_list_response_fixture_parses_declared_model() -> BpiResult<()> {
        let payload = ApiEnvelope::<Vec<LiveParentArea>>::from_slice(include_bytes!(
            "../../tests/contracts/live/public-core/area-list/responses/success.json"
        ))?
        .into_payload()?;

        assert_eq!(payload.len(), 1);
        assert_eq!(payload[0].list.len(), 1);
        Ok(())
    }

    fn local_probe_body(profile: &str) -> Option<serde_json::Value> {
        let path =
            format!("target/bpi-probe-runs/live/public-core/area-list/{profile}.response.json");
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("response")
            .and_then(|response| response.get("body"))
            .cloned()
    }

    #[test]
    fn live_area_list_model_matches_local_probe_outputs_when_available() -> BpiResult<()> {
        for profile in ["anonymous", "normal", "vip"] {
            if let Some(body) = local_probe_body(profile) {
                let payload = serde_json::from_value::<ApiEnvelope<Vec<LiveParentArea>>>(body)?
                    .into_payload()?;
                assert!(!payload.is_empty());
            }
        }
        Ok(())
    }
}
