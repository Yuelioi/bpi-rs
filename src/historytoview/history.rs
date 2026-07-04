#[cfg(test)]
use crate::historytoview::params::HistoryListParams;
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 历史记录列表的页面信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryCursor {
    /// 最后一项目标 ID
    pub max: u64,
    /// 最后一项时间节点 (时间戳)
    pub view_at: u64,
    /// 最后一项业务类型
    pub business: String,
    /// 每页项数
    pub ps: u32,
}

/// 历史记录筛选类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryTab {
    /// 类型
    #[serde(rename = "type")]
    pub type_name: String,
    /// 类型名
    pub name: String,
}

/// 历史记录封面图组
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HistoryCovers {
    /// 专栏的封面图数组
    Array(Vec<String>),
    /// 其他条目的单封面图
    String(String),
}

/// 历史记录中的详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryDetail {
    /// 目标 ID，如稿件 avid、直播间 ID 等
    pub oid: u64,
    /// 剧集 epid，仅用于剧集
    pub epid: Option<u64>,
    /// 稿件 bvid，仅用于稿件视频
    pub bvid: Option<String>,
    /// 观看到的视频分P数，仅用于稿件视频
    pub page: Option<u32>,
    /// 观看到的对象 ID，如视频 cid、文章 cvid
    pub cid: Option<u64>,
    /// 观看到的视频分 P 标题，仅用于稿件视频
    pub part: Option<String>,
    /// 业务类型
    pub business: String,
    /// 记录查看的平台代码
    pub dt: u32,
}

/// 单个历史记录条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryListItem {
    /// 条目标题
    pub title: String,
    /// 角标文案
    pub badge: Option<String>,
    /// 条目副标题
    pub long_title: Option<String>,
    /// 条目封面图 URL，用于专栏以外的条目
    pub cover: Option<String>,
    /// 条目封面图组，仅用于专栏
    pub covers: Option<Vec<String>>,
    /// 重定向 URL，仅用于剧集和直播
    pub uri: Option<String>,
    /// 条目详细信息
    pub history: HistoryDetail,
    /// 视频分 P 数目，仅用于稿件视频
    pub videos: Option<u32>,
    /// UP 主昵称
    pub author_name: Option<String>,
    /// UP 主头像 URL
    pub author_face: Option<String>,
    /// UP 主 mid
    pub author_mid: Option<u64>,
    /// 查看时间 (时间戳)
    pub view_at: u64,
    /// 视频观看进度 (秒)
    pub progress: i32,
    /// 分 P 标题，用于稿件视频或剧集
    pub show_title: Option<String>,
    /// 视频总时长，用于稿件视频或剧集
    pub duration: Option<u32>,
    /// 备注
    pub current: Option<String>,
    /// 总计分集数，仅用于剧集
    pub total: Option<u32>,
    /// 最新一话 / 最新一 P 标识
    pub new_desc: Option<String>,
    /// 是否已完结，仅用于剧集
    pub is_finish: Option<u8>,
    /// 是否收藏
    pub is_fav: u8,
    /// 条目目标 id
    pub kid: u64,
    /// 子分区名，用于稿件视频和直播
    pub tag_name: Option<String>,
    /// 直播状态，仅用于直播
    pub live_status: Option<u8>,
}

/// 历史记录列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryListData {
    /// 历史记录页面信息
    pub cursor: HistoryCursor,
    /// 历史记录筛选类型
    pub tab: Vec<HistoryTab>,
    /// 分段历史记录列表
    pub list: Vec<HistoryListItem>,
}

impl BpiClient {
    /// 删除历史记录
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/historytoview)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `kid` | &str | 记录目标 id |
    pub async fn history_delete(
        &self,
        kid: &str,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let payload = [("kid", kid), ("csrf", &csrf)];

        self.post("https://api.bilibili.com/x/v2/history/delete")
            .form(&payload)
            .send_bpi("删除历史记录")
            .await
    }

    /// 清空历史记录
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/historytoview)
    pub async fn history_clear(&self) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let payload = [("csrf", &csrf)];

        self.post("https://api.bilibili.com/x/v2/history/clear")
            .form(&payload)
            .send_bpi("清空历史记录")
            .await
    }

    /// 停用历史记录
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/historytoview)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `switch` | bool | 是否停用 |
    pub async fn history_shadow_set(
        &self,
        switch: bool,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let payload = [("switch", switch.to_string()), ("csrf", csrf)];

        self.post("https://api.bilibili.com/x/v2/history/shadow/set")
            .form(&payload)
            .send_bpi("停用历史记录")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;
    use crate::{ApiEnvelope, BpiResult};
    use tracing::info;

    fn contract(endpoint: &str) -> BpiResult<EndpointContract> {
        let bytes = match endpoint {
            "history-list" => include_bytes!(
                "../../tests/contracts/historytoview/read/history-list/contract.json"
            )
            .as_slice(),
            "history-shadow" => include_bytes!(
                "../../tests/contracts/historytoview/read/history-shadow/contract.json"
            )
            .as_slice(),
            _ => unreachable!("unknown historytoview history contract endpoint"),
        };

        EndpointContract::from_slice(bytes)
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_history_get_list() {
        let bpi = BpiClient::new().expect("client should build");
        let params = HistoryListParams::new()
            .with_page_size(10)
            .expect("fixture page size should be valid");
        let resp = bpi.historytoview().history_list(params).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let data = resp.unwrap();
        info!("cursor: {:?}", data.cursor);
        info!("tabs: {:?}", data.tab);
        info!("first item: {:?}", data.list.first());
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_history_shadow_set_and_get() {
        let bpi = BpiClient::new().expect("client should build");

        // 获取当前状态
        let current_status = bpi.historytoview().history_shadow().await.unwrap();

        // 切换状态
        let new_status = !current_status;
        let set_resp = bpi.history_shadow_set(new_status).await;
        info!("Set status to {}: {:?}", new_status, set_resp);
        assert!(set_resp.is_ok());

        // 再次获取状态，验证是否已切换
        let new_status_resp = bpi.historytoview().history_shadow().await;
        info!("New status: {:?}", new_status_resp);
        assert!(new_status_resp.is_ok());
        assert_eq!(new_status_resp.unwrap(), new_status);

        // 恢复原始状态
        let set_back_resp = bpi.history_shadow_set(current_status).await;
        info!(
            "Set back to original status {}: {:?}",
            current_status, set_back_resp
        );
        assert!(set_back_resp.is_ok());
    }

    #[test]
    fn history_list_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = contract("history-list")?;

        assert_eq!(contract.name, "historytoview.history_list");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.bilibili.com/x/web-interface/history/cursor"
        );
        assert_eq!(
            contract.request.query.get("ps").map(String::as_str),
            Some("5")
        );
        assert_eq!(contract.cases.len(), 3);
        assert_eq!(contract.cases[0].response.api_code, Some(-101));
        assert_eq!(
            contract.cases[1].response.rust_model.as_deref(),
            Some("HistoryListData")
        );
        Ok(())
    }

    #[test]
    fn history_shadow_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = contract("history-shadow")?;

        assert_eq!(contract.name, "historytoview.history_shadow");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.bilibili.com/x/v2/history/shadow"
        );
        assert!(contract.request.query.is_empty());
        assert_eq!(contract.cases.len(), 3);
        assert_eq!(contract.cases[0].response.api_code, Some(-101));
        assert_eq!(
            contract.cases[1].response.rust_model.as_deref(),
            Some("bool")
        );
        Ok(())
    }

    #[test]
    fn history_response_fixtures_parse_declared_models() -> BpiResult<()> {
        let err = ApiEnvelope::<serde_json::Value>::from_slice(include_bytes!(
            "../../tests/contracts/historytoview/read/history-list/responses/anonymous.requires_login.json"
        ))?
        .ensure_success()
        .unwrap_err();
        assert!(err.requires_login());

        let list = ApiEnvelope::<HistoryListData>::from_slice(include_bytes!(
            "../../tests/contracts/historytoview/read/history-list/responses/authenticated.success.json"
        ))?
        .into_payload()?;
        assert_eq!(list.cursor.ps, 5);
        assert_eq!(list.list.len(), 1);

        let err = ApiEnvelope::<serde_json::Value>::from_slice(include_bytes!(
            "../../tests/contracts/historytoview/read/history-shadow/responses/anonymous.requires_login.json"
        ))?
        .ensure_success()
        .unwrap_err();
        assert!(err.requires_login());

        let shadow = ApiEnvelope::<bool>::from_slice(include_bytes!(
            "../../tests/contracts/historytoview/read/history-shadow/responses/authenticated.success.json"
        ))?
        .into_payload()?;
        assert!(!shadow);
        Ok(())
    }

    fn local_probe_body(endpoint: &str, profile: &str) -> Option<serde_json::Value> {
        let path =
            format!("target/bpi-probe-runs/historytoview/read/{endpoint}/{profile}.response.json");
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("response")
            .and_then(|response| response.get("body"))
            .cloned()
    }

    #[test]
    fn history_models_match_local_probe_outputs_when_available() -> BpiResult<()> {
        for profile in ["anonymous", "normal", "vip"] {
            if let Some(body) = local_probe_body("history-list", profile) {
                let envelope = serde_json::from_value::<ApiEnvelope<HistoryListData>>(body)?;
                if profile == "anonymous" {
                    let err = envelope.ensure_success().unwrap_err();
                    assert!(err.requires_login());
                } else {
                    let payload = envelope.into_payload()?;
                    assert!(payload.cursor.ps > 0);
                    assert!(payload.cursor.ps as usize >= payload.list.len());
                }
            }

            if let Some(body) = local_probe_body("history-shadow", profile) {
                let envelope = serde_json::from_value::<ApiEnvelope<bool>>(body)?;
                if profile == "anonymous" {
                    let err = envelope.ensure_success().unwrap_err();
                    assert!(err.requires_login());
                } else {
                    let _ = envelope.into_payload()?;
                }
            }
        }
        Ok(())
    }
}
