use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResponse;
use crate::electric::ElectricClient;
use serde::{Deserialize, Serialize};

/// 发送充电留言的请求体

#[derive(Debug, Clone, Serialize)]
pub struct SendElecMessageBody<'a> {
    pub order_id: &'a str,
    pub message: &'a str,
    pub csrf: &'a str,
}

/// 充电留言列表分页信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ElecRemarkPager {
    /// 当前页数
    pub current: u64,
    /// 当前分页大小
    pub size: u64,
    /// 记录总数
    pub total: u64,
}

/// 充电留言列表中的单条留言
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ElecRemarkRecord {
    pub aid: u64,
    pub bvid: String,
    pub id: u64,
    pub mid: u64,
    pub reply_mid: u64,
    pub elec_num: u64,
    /// UP是否已经回复这条留言 0: 未回复 1: 已回复
    pub state: u8,
    /// 留言信息
    pub msg: String,
    pub aname: String,
    pub uname: String,
    pub avator: String,
    pub reply_name: String,
    pub reply_avator: String,
    pub reply_msg: String,
    /// 留言时间毫秒级时间戳
    pub ctime: u64,
    pub reply_time: u64,
}

/// 充电留言列表数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ElecRemarkList {
    pub list: Vec<ElecRemarkRecord>,
    pub pager: ElecRemarkPager,
}

/// 充电留言详情数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ElecRemarkDetail {
    pub aid: u64,
    pub bvid: String,
    pub id: u64,
    /// 留言者mid（充电用户）
    pub mid: u64,
    /// UP主mid
    pub reply_mid: u64,
    pub elec_num: u64,
    /// UP是否已经回复这条留言 0: 未回复 1: 已回复
    pub state: u8,
    /// 留言内容
    pub msg: String,
    pub aname: String,
    /// 留言者用户名
    pub uname: String,
    /// 留言者头像
    pub avator: String,
    /// UP主用户名
    pub reply_name: String,
    /// UP主头像
    pub reply_avator: String,
    /// 回复内容
    pub reply_msg: String,
    /// 留言时间毫秒级时间戳
    pub ctime: u64,
    /// 回复时间毫秒级时间戳
    pub reply_time: u64,
}

impl<'a> ElectricClient<'a> {
    /// 发送充电留言
    ///
    /// 注意: 此接口需要登录态 (Cookie: SESSDATA)
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/electric)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `order_id` | &str | 留言 token |
    /// | `message` | &str | 留言内容 |
    pub async fn electric_message_send(
        &self,
        order_id: &str,
        message: &str,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;

        let body = [
            ("order_id", order_id),
            ("message", message),
            ("csrf", &csrf),
        ];

        self.client
            .post("https://api.bilibili.com/x/ugcpay/trade/elec/message")
            .form(&body)
            .send_bpi("发送充电留言")
            .await
    }

    /// 回复充电留言
    ///
    /// 注意: 此接口需要登录态 (Cookie: SESSDATA)
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/electric)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `id` | u64 | 留言 id |
    /// | `msg` | &str | 回复内容 |
    pub async fn electric_remark_reply(
        &self,
        id: u64,
        msg: &str,
    ) -> Result<BpiResponse<u64>, BpiError> {
        let csrf = self.client.csrf()?;

        let body = [
            ("id", id.to_string()),
            ("msg", msg.to_string()),
            ("csrf", csrf.to_string()),
        ];

        self.client
            .post("https://member.bilibili.com/x/web/elec/remark/reply")
            .form(&body)
            .send_bpi("回复充电留言")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;
    use crate::probe::flow::ProbeFlow;
    use crate::{ApiEnvelope, BpiResult};

    fn remark_list_contract() -> BpiResult<EndpointContract> {
        EndpointContract::from_slice(include_bytes!(
            "../../tests/contracts/electric/private-read/remark-list/contract.json"
        ))
    }

    fn remark_detail_contract() -> BpiResult<EndpointContract> {
        EndpointContract::from_slice(include_bytes!(
            "../../tests/contracts/electric/private-read/remark-detail/contract.json"
        ))
    }

    fn normal_remark_detail_flow_contract() -> BpiResult<ProbeFlow> {
        ProbeFlow::from_slice(include_bytes!(
            "../../tests/contracts/electric/private-read/remark-detail/flow/normal.contract.json"
        ))
    }

    fn vip_remark_detail_flow_contract() -> BpiResult<ProbeFlow> {
        ProbeFlow::from_slice(include_bytes!(
            "../../tests/contracts/electric/private-read/remark-detail/flow/vip.contract.json"
        ))
    }

    #[test]
    fn electric_remark_list_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = remark_list_contract()?;

        assert_eq!(contract.name, "electric.remark_list");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://member.bilibili.com/x/web/elec/remark/list"
        );
        assert_eq!(
            contract.request.query.get("pn").map(String::as_str),
            Some("1")
        );
        assert_eq!(
            contract.request.query.get("ps").map(String::as_str),
            Some("10")
        );
        assert_eq!(contract.cases.len(), 3);
        assert_eq!(
            contract.cases[1].response.rust_model.as_deref(),
            Some("ElecRemarkList")
        );
        Ok(())
    }

    #[test]
    fn electric_remark_detail_contract_matches_anonymous_endpoint_request() -> BpiResult<()> {
        let contract = remark_detail_contract()?;

        assert_eq!(contract.name, "electric.remark_detail");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://member.bilibili.com/x/web/elec/remark/detail"
        );
        assert_eq!(
            contract.request.query.get("id").map(String::as_str),
            Some("1")
        );
        assert_eq!(contract.cases.len(), 1);
        assert_eq!(
            contract.cases[0].response.error.as_deref(),
            Some("requires_login")
        );
        Ok(())
    }

    #[test]
    fn electric_remark_detail_flow_contracts_use_list_id_placeholder() -> BpiResult<()> {
        for flow in [
            normal_remark_detail_flow_contract()?,
            vip_remark_detail_flow_contract()?,
        ] {
            assert!(matches!(
                flow.name.as_str(),
                "electric.remark_detail.normal.flow" | "electric.remark_detail.vip.flow"
            ));
            assert_eq!(flow.steps.len(), 2);
            assert_eq!(flow.steps[0].name, "remark-list");
            assert_eq!(
                flow.steps[0].extract.get("remark_id").map(String::as_str),
                Some("/response/body/data/list/0/id")
            );
            assert_eq!(flow.steps[1].name, "remark-detail");
            assert_eq!(
                flow.steps[1].contract["request"]["query"]["id"],
                "${remark_id}"
            );
        }

        Ok(())
    }

    #[test]
    fn electric_remark_list_response_fixtures_parse_declared_models() -> BpiResult<()> {
        let err = ApiEnvelope::<serde_json::Value>::from_slice(include_bytes!(
            "../../tests/contracts/electric/private-read/remark-list/responses/anonymous.requires_login.json"
        ))?
        .ensure_success()
        .unwrap_err();
        assert!(err.requires_login());

        let list = ApiEnvelope::<ElecRemarkList>::from_slice(include_bytes!(
            "../../tests/contracts/electric/private-read/remark-list/responses/authenticated.success.json"
        ))?
        .into_payload()?;
        assert_eq!(list.list.len(), 1);
        Ok(())
    }

    #[test]
    fn electric_remark_detail_response_fixtures_parse_declared_models() -> BpiResult<()> {
        let err = ApiEnvelope::<serde_json::Value>::from_slice(include_bytes!(
            "../../tests/contracts/electric/private-read/remark-detail/responses/anonymous.requires_login.json"
        ))?
        .ensure_success()
        .unwrap_err();
        assert!(err.requires_login());

        let detail = ApiEnvelope::<ElecRemarkDetail>::from_slice(include_bytes!(
            "../../tests/contracts/electric/private-read/remark-detail/responses/authenticated.success.json"
        ))?
        .into_payload()?;
        assert_eq!(detail.id, 1);
        assert_eq!(detail.msg, "<redacted>");
        Ok(())
    }

    fn local_probe_body(endpoint: &str, profile: &str) -> Option<serde_json::Value> {
        let path = format!(
            "target/bpi-probe-runs/electric/private-read/{endpoint}/{profile}.response.json"
        );
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("response")
            .and_then(|response| response.get("body"))
            .cloned()
    }

    fn local_probe_flow_step_body(profile: &str, step: &str) -> Option<serde_json::Value> {
        let path = format!(
            "target/bpi-probe-runs/electric/private-read/remark-detail-flow/{profile}.response.json"
        );
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("steps")?
            .as_array()?
            .iter()
            .find(|entry| entry.get("step").and_then(serde_json::Value::as_str) == Some(step))?
            .get("result")
            .and_then(|result| result.get("response"))
            .and_then(|response| response.get("body"))
            .cloned()
    }

    #[test]
    fn electric_remark_list_model_matches_local_probe_outputs_when_available() -> BpiResult<()> {
        for profile in ["anonymous", "normal", "vip"] {
            if let Some(body) = local_probe_body("remark-list", profile) {
                let envelope = serde_json::from_value::<ApiEnvelope<ElecRemarkList>>(body)?;
                if profile == "anonymous" {
                    let err = envelope.ensure_success().unwrap_err();
                    assert!(err.requires_login());
                } else {
                    let payload = envelope.into_payload()?;
                    assert!(payload.pager.total >= payload.list.len() as u64);
                }
            }
        }
        Ok(())
    }

    #[test]
    fn electric_remark_detail_model_matches_local_probe_outputs_when_available() -> BpiResult<()> {
        if let Some(body) = local_probe_body("remark-detail", "anonymous") {
            let err = serde_json::from_value::<ApiEnvelope<serde_json::Value>>(body)?
                .ensure_success()
                .unwrap_err();
            assert!(err.requires_login());
        }

        for profile in ["normal", "vip"] {
            if let Some(body) = local_probe_flow_step_body(profile, "remark-detail") {
                let detail = serde_json::from_value::<ApiEnvelope<ElecRemarkDetail>>(body)?
                    .into_payload()?;
                assert!(detail.id > 0);
            }
        }
        Ok(())
    }
}
