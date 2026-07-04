use serde::{Deserialize, Serialize};

use crate::ids::{Aid, Bvid, Cid};
use crate::{BpiError, BpiResult};

// -------------------
// 发送视频弹幕
// -------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DanmakuPostData {
    pub colorful_src: Option<serde_json::Value>, // 当请求参数colorful=60001时有效
    pub dmid: u64,
    pub dmid_str: String,
}

/// Parameters for sending a video danmaku.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DanmakuSendParams {
    oid: Cid,
    msg: String,
    aid: Option<Aid>,
    bvid: Option<Bvid>,
    mode: u8,
    typ: u8,
    progress: u32,
    color: u32,
    font_size: u8,
    pool: u8,
}

impl DanmakuSendParams {
    /// Creates parameters with Bilibili web defaults.
    pub fn new(oid: Cid, msg: impl Into<String>) -> BpiResult<Self> {
        let msg = msg.into();
        if msg.trim().is_empty() {
            return Err(BpiError::invalid_parameter(
                "msg",
                "danmaku message cannot be blank",
            ));
        }

        Ok(Self {
            oid,
            msg,
            aid: None,
            bvid: None,
            mode: 1,
            typ: 1,
            progress: 1878,
            color: 16_777_215,
            font_size: 25,
            pool: 0,
        })
    }

    /// Sets the optional AV numeric video ID.
    pub fn aid(mut self, aid: Aid) -> Self {
        self.aid = Some(aid);
        self
    }

    /// Sets the optional BV string video ID.
    pub fn bvid(mut self, bvid: Bvid) -> Self {
        self.bvid = Some(bvid);
        self
    }

    /// Sets the danmaku display mode.
    pub fn mode(mut self, mode: u8) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the danmaku type.
    pub fn danmaku_type(mut self, typ: u8) -> Self {
        self.typ = typ;
        self
    }

    /// Sets the danmaku timestamp in milliseconds.
    pub fn progress(mut self, progress: u32) -> Self {
        self.progress = progress;
        self
    }

    /// Sets the RGB888 color value.
    pub fn color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }

    /// Sets the font size.
    pub fn font_size(mut self, font_size: u8) -> Self {
        self.font_size = font_size;
        self
    }

    /// Sets the danmaku pool.
    pub fn pool(mut self, pool: u8) -> Self {
        self.pool = pool;
        self
    }
}

// -------------------
// 撤回弹幕
// -------------------

// -------------------
// 购买高级弹幕发送权限
// -------------------

// -------------------
// 检测高级弹幕发送权限
// -------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DanmakuAdvState {
    pub coins: u8,
    #[serde(default)]
    pub confirm: u8,
    pub accept: bool,
    #[serde(default)]
    pub has_buy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DanmakuAdvStateParams {
    cid: Cid,
}

impl DanmakuAdvStateParams {
    pub fn new(cid: Cid) -> Self {
        Self { cid }
    }

    pub(crate) fn query_pairs(&self) -> [(&'static str, String); 2] {
        [("cid", self.cid.to_string()), ("mode", "sp".to_string())]
    }
}

// -------------------
// 点赞弹幕
// -------------------

// -------------------
// 举报弹幕
// -------------------

// -------------------
// 保护&删除弹幕
// -------------------

// -------------------
// 修改字幕池
// -------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::Cid;
    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;
    use crate::response::ApiEnvelope;
    use std::collections::BTreeMap;

    const TEST_CID: u64 = 413195701;

    fn adv_state_contract() -> Result<EndpointContract, BpiError> {
        EndpointContract::from_slice(include_bytes!(
            "../../tests/contracts/danmaku/json-read/adv-state/contract.json"
        ))
    }

    fn query_map(params: [(&'static str, String); 2]) -> BTreeMap<String, String> {
        params
            .into_iter()
            .map(|(key, value)| (key.to_string(), value))
            .collect()
    }

    #[test]
    fn danmaku_send_params_rejects_blank_message() {
        let err =
            DanmakuSendParams::new(Cid::new(413195701).expect("cid is valid"), "   ").unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "msg", .. }
        ));
    }

    #[test]
    fn danmaku_adv_state_params_serializes_query() -> Result<(), BpiError> {
        let params = DanmakuAdvStateParams::new(Cid::new(TEST_CID)?);

        assert_eq!(
            params.query_pairs(),
            [("cid", TEST_CID.to_string()), ("mode", "sp".to_string())]
        );
        Ok(())
    }

    #[test]
    fn danmaku_adv_state_contract_matches_endpoint_request() -> Result<(), BpiError> {
        let contract = adv_state_contract()?;
        let params = DanmakuAdvStateParams::new(Cid::new(TEST_CID)?);

        assert_eq!(contract.name, "danmaku.adv.state");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.bilibili.com/x/dm/adv/state"
        );
        assert_eq!(query_map(params.query_pairs()), contract.request.query);
        Ok(())
    }

    #[test]
    fn danmaku_adv_state_response_fixtures_parse_declared_model() -> Result<(), BpiError> {
        let normal = ApiEnvelope::<DanmakuAdvState>::from_slice(include_bytes!(
            "../../tests/contracts/danmaku/json-read/adv-state/responses/normal.success.json"
        ))?
        .into_payload()?;
        assert!(normal.accept);
        assert_eq!(normal.coins, 2);
        assert_eq!(normal.confirm, 0);
        assert!(!normal.has_buy);

        let vip = ApiEnvelope::<DanmakuAdvState>::from_slice(include_bytes!(
            "../../tests/contracts/danmaku/json-read/adv-state/responses/vip.success.json"
        ))?
        .into_payload()?;
        assert!(vip.accept);
        assert_eq!(vip.coins, 2);
        assert_eq!(vip.confirm, 1);
        assert!(vip.has_buy);
        Ok(())
    }

    #[test]
    fn danmaku_adv_state_anonymous_fixture_records_login_error() -> Result<(), BpiError> {
        let err = ApiEnvelope::<serde_json::Value>::from_slice(include_bytes!(
            "../../tests/contracts/danmaku/json-read/adv-state/responses/anonymous.requires_login.json"
        ))
        .and_then(ApiEnvelope::ensure_success)
        .unwrap_err();

        assert!(err.requires_login());
        Ok(())
    }
}
