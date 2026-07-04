// -------------------
// 发送视频弹幕
// -------------------

use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResponse;
use crate::BpiResult;
use crate::danmaku::DanmakuClient;
use crate::ids::{Aid, Bvid, Cid};
use serde::{Deserialize, Serialize};

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

    fn form_pairs(&self, csrf: impl Into<String>) -> Vec<(&'static str, String)> {
        let mut form = vec![
            ("type", self.typ.to_string()),
            ("oid", self.oid.to_string()),
            ("msg", self.msg.clone()),
            ("mode", self.mode.to_string()),
            ("fontsize", self.font_size.to_string()),
            ("color", self.color.to_string()),
            ("pool", self.pool.to_string()),
            ("progress", self.progress.to_string()),
            ("rnd", "2".to_string()),
            ("plat", "1".to_string()),
            ("csrf", csrf.into()),
            ("checkbox_type", "0".to_string()),
            ("colorful", "".to_string()),
            ("gaiasource", "main_web".to_string()),
            ("polaris_app_id", "100".to_string()),
            ("polaris_platform", "5".to_string()),
            ("spmid", "333.788.0.0".to_string()),
            ("from_spmid", "333.788.0.0".to_string()),
        ];

        if let Some(aid) = self.aid {
            form.push(("avid", aid.to_string()));
        }
        if let Some(bvid) = self.bvid.as_ref() {
            form.push(("bvid", bvid.to_string()));
        }

        form
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

impl<'a> DanmakuClient<'a> {
    /// 发送视频弹幕
    ///
    /// 文档: [弹幕相关](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)
    ///
    pub async fn danmaku_send(
        &self,
        params: DanmakuSendParams,
    ) -> Result<BpiResponse<DanmakuPostData>, BpiError> {
        let csrf = self.client.csrf()?;

        let form = params.form_pairs(csrf);

        // 签名参数加入表单
        let signed_params = self.client.get_wbi_sign2(form.clone()).await?;

        self.client
            .post("https://api.bilibili.com/x/v2/dm/post")
            .form(&signed_params)
            .send_bpi("发送视频弹幕")
            .await
    }

    /// 发送视频弹幕（精简参数版本）
    ///
    /// 文档: [弹幕相关](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `oid` | u64 | 视频 cid |
    /// | `msg` | &str | 弹幕内容 |
    /// | `avid` | `Option<u64>` | 稿件 aid（`avid` 与 `bvid` 二选一） |
    /// | `bvid` | `Option<&str>` | 稿件 bvid（`avid` 与 `bvid` 二选一） |
    pub async fn danmaku_send_default(
        &self,
        oid: u64,
        msg: &str,
        avid: Option<u64>,
        bvid: Option<&str>,
    ) -> Result<BpiResponse<DanmakuPostData>, BpiError> {
        let csrf = self.client.csrf()?;

        let mut form = vec![
            ("type", "1".to_string()),
            ("oid", oid.to_string()),
            ("msg", msg.to_string()),
            ("mode", "1".to_string()),
            ("csrf", csrf),
        ];

        if let Some(b) = bvid {
            form.push(("bvid", b.to_string()));
        }
        if let Some(a) = avid {
            form.push(("avid", a.to_string()));
        }

        // 使用 get_wbi_sign2 自动生成 w_rid / wts
        let signed_form = self.client.get_wbi_sign2(form).await?;

        self.client
            .post("https://api.bilibili.com/x/v2/dm/post")
            .form(&signed_form)
            .send_bpi("发送视频弹幕")
            .await
    }

    /// 撤回弹幕。
    pub async fn danmaku_recall(
        &self,
        cid: u64,
        dmid: u64,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;
        self.client
            .post("https://api.bilibili.com/x/dm/recall")
            .form(&[
                ("cid", &cid.to_string()),
                ("dmid", &dmid.to_string()),
                ("type", &"1".to_string()),
                ("csrf", &csrf),
            ])
            .send_bpi("撤回弹幕")
            .await
    }

    /// 购买高级弹幕发送权限。
    pub async fn danmaku_buy_adv(
        &self,
        cid: u64,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;
        self.client
            .post("https://api.bilibili.com/x/dm/adv/buy")
            .form(&[
                ("cid", cid.to_string()),
                ("mode", "sp".to_string()),
                ("csrf", csrf),
            ])
            .send_bpi("购买高级弹幕发送权限")
            .await
    }

    /// 点赞弹幕。
    pub async fn danmaku_thumbup(
        &self,
        oid: u64,
        dmid: u64,
        op: u8,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;
        let mut form = vec![
            ("oid", oid.to_string()),
            ("dmid", dmid.to_string()),
            ("op", op.to_string()),
            ("csrf", csrf),
        ];
        form.push(("platform", "web_player".to_string()));

        self.client
            .post("https://api.bilibili.com/x/v2/dm/thumbup/add")
            .form(&form)
            .send_bpi("点赞弹幕")
            .await
    }

    /// 举报弹幕。
    pub async fn danmaku_report(
        &self,
        cid: u64,
        dmid: u64,
        reason: u8,
        content: Option<&str>,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;
        let mut form = vec![
            ("cid", cid.to_string()),
            ("dmid", dmid.to_string()),
            ("reason", reason.to_string()),
            ("csrf", csrf),
        ];
        if let Some(content) = content {
            form.push(("content", content.to_string()));
        }

        self.client
            .post("https://api.bilibili.com/x/dm/report/add")
            .form(&form)
            .send_bpi("举报弹幕")
            .await
    }

    /// 保护或删除弹幕。
    pub async fn danmaku_edit_state(
        &self,
        oid: u64,
        dmids: &[u64],
        state: u8,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;
        let dmids_str = dmids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.client
            .post("https://api.bilibili.com/x/v2/dm/edit/state")
            .form(&[
                ("type", "1"),
                ("oid", &oid.to_string()),
                ("dmids", &dmids_str),
                ("state", &state.to_string()),
                ("csrf", &csrf),
            ])
            .send_bpi("保护&删除弹幕")
            .await
    }

    /// 修改字幕池。
    pub async fn danmaku_edit_pool(
        &self,
        oid: u64,
        dmids: &[u64],
        pool: u8,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;
        let dmids_str = dmids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.client
            .post("https://api.bilibili.com/x/v2/dm/edit/pool")
            .form(&[
                ("type", "1"),
                ("oid", &oid.to_string()),
                ("dmids", &dmids_str),
                ("pool", &pool.to_string()),
                ("csrf", &csrf),
            ])
            .send_bpi("修改字幕池")
            .await
    }
}

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
