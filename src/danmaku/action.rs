use serde::{Deserialize, Serialize};

use crate::ids::{Aid, Bvid, Cid};
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse, BpiResult};

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

impl BpiClient {
    /// 发送视频弹幕
    ///
    /// 文档: [弹幕相关](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)
    ///
    pub async fn danmaku_send(
        &self,
        params: DanmakuSendParams,
    ) -> Result<BpiResponse<DanmakuPostData>, BpiError> {
        let csrf = self.csrf()?;

        let form = params.form_pairs(csrf);

        // 签名参数加入表单
        let signed_params = self.get_wbi_sign2(form.clone()).await?;

        self.post("https://api.bilibili.com/x/v2/dm/post")
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
        let csrf = self.csrf()?;

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
        let signed_form = self.get_wbi_sign2(form).await?;

        self.post("https://api.bilibili.com/x/v2/dm/post")
            .form(&signed_form)
            .send_bpi("发送视频弹幕")
            .await
    }
}

// -------------------
// 撤回弹幕
// -------------------

impl BpiClient {
    /// 撤回弹幕
    ///
    /// 文档: [弹幕相关](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `cid` | u64 | 视频 cid |
    /// | `dmid` | u64 | 要撤回的弹幕 id（仅能撤回自己两分钟内的弹幕，每天 5 次） |
    ///
    /// 返回中的 `message` 示例："撤回成功，你还有{}次撤回机会"
    pub async fn danmaku_recall(
        &self,
        cid: u64,
        dmid: u64,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        self.post("https://api.bilibili.com/x/dm/recall")
            .form(&[
                ("cid", &cid.to_string()),
                ("dmid", &dmid.to_string()),
                ("type", &"1".to_string()),
                ("csrf", &csrf),
            ])
            .send_bpi("撤回弹幕")
            .await
    }
}

// -------------------
// 购买高级弹幕发送权限
// -------------------

impl BpiClient {
    /// 购买高级弹幕发送权限（一次需要 2 硬币）
    ///
    /// 文档: [弹幕相关](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `cid` | u64 | 视频 cid |
    pub async fn danmaku_buy_adv(
        &self,
        cid: u64,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        self.post("https://api.bilibili.com/x/dm/adv/buy")
            .form(&[
                ("cid", cid.to_string()),
                ("mode", "sp".to_string()),
                ("csrf", csrf),
            ])
            .send_bpi("购买高级弹幕发送权限")
            .await
    }
}

// -------------------
// 检测高级弹幕发送权限
// -------------------

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DanmakuAdvState {
    pub coins: u8,
    pub confirm: u8,
    pub accept: bool,
    pub has_buy: bool,
}

impl BpiClient {
    /// 检测高级弹幕发送权限
    ///
    /// 文档: [弹幕相关](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `cid` | u64 | 视频 cid |
    pub async fn danmaku_adv_state(
        &self,
        cid: u64,
    ) -> Result<BpiResponse<DanmakuAdvState>, BpiError> {
        self.get("https://api.bilibili.com/x/dm/adv/state")
            .query(&[("cid", cid.to_string()), ("mode", "sp".to_string())])
            .send_bpi("检测高级弹幕发送权限")
            .await
    }
}

// -------------------
// 点赞弹幕
// -------------------

impl BpiClient {
    /// 点赞弹幕
    ///
    /// 文档: [弹幕相关](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `oid` | u64 | 视频 cid |
    /// | `dmid` | u64 | 弹幕 id |
    /// | `op` | u8 | 1 点赞，2 取消点赞 |
    pub async fn danmaku_thumbup(
        &self,
        oid: u64,
        dmid: u64,
        op: u8,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let mut form = vec![
            ("oid", oid.to_string()),
            ("dmid", dmid.to_string()),
            ("op", op.to_string()),
            ("csrf", csrf),
        ];

        form.push(("platform", "web_player".to_string()));

        self.post("https://api.bilibili.com/x/v2/dm/thumbup/add")
            .form(&form)
            .send_bpi("点赞弹幕")
            .await
    }
}

// -------------------
// 举报弹幕
// -------------------

impl BpiClient {
    /// 举报弹幕
    ///
    /// 文档: [弹幕相关](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `cid` | u64 | 视频 cid |
    /// | `dmid` | u64 | 弹幕 id |
    /// | `reason` | u8 | 原因代码 |
    /// | `content` | `Option<&str>` | 举报备注（`reason=11` 时有效） |
    pub async fn danmaku_report(
        &self,
        cid: u64,
        dmid: u64,
        reason: u8,
        content: Option<&str>,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let mut form = vec![
            ("cid", cid.to_string()),
            ("dmid", dmid.to_string()),
            ("reason", reason.to_string()),
            ("csrf", csrf),
        ];

        if let Some(c) = content {
            form.push(("content", c.to_string()));
        }

        self.post("https://api.bilibili.com/x/dm/report/add")
            .form(&form)
            .send_bpi("举报弹幕")
            .await
    }
}

// -------------------
// 保护&删除弹幕
// -------------------

impl BpiClient {
    /// 保护或删除弹幕（仅能操作自己的稿件或具备权限的稿件）
    ///
    /// 文档: [弹幕相关](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `oid` | u64 | 视频 oid/cid |
    /// | `dmids` | &`[u64]` | 弹幕 id 列表 |
    /// | `state` | u8 | 1 删除，2 保护，3 取消保护 |
    pub async fn danmaku_edit_state(
        &self,
        oid: u64,
        dmids: &[u64],
        state: u8,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let dmids_str = dmids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.post("https://api.bilibili.com/x/v2/dm/edit/state")
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
}

// -------------------
// 修改字幕池
// -------------------

impl BpiClient {
    /// 修改字幕池（仅能操作自己的稿件或具备权限的稿件）
    ///
    /// 文档: [弹幕相关](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `oid` | u64 | 视频 oid/cid |
    /// | `dmids` | &`[u64]` | 弹幕 id 列表 |
    /// | `pool` | u8 | 弹幕池：0 普通池，1 字幕池，2 特殊池 |
    pub async fn danmaku_edit_pool(
        &self,
        oid: u64,
        dmids: &[u64],
        pool: u8,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let dmids_str = dmids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self.post("https://api.bilibili.com/x/v2/dm/edit/pool")
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
    use crate::ids::{Aid, Bvid, Cid};
    use tracing::info;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_danmaku_post() {
        let bpi = BpiClient::new().expect("client should build");

        let params = DanmakuSendParams::new(Cid::new(413195701).expect("cid is valid"), "测试22")
            .expect("message is valid")
            .aid(Aid::new(590635620).expect("aid is valid"));
        let resp = bpi.danmaku_send(params).await;
        info!("{:#?}", resp);
        assert!(resp.is_ok());
        info!("dmid{}", resp.unwrap().data.unwrap().dmid);
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_danmaku_recall() {
        let bpi = BpiClient::new().expect("client should build");

        let resp = bpi.danmaku_recall(413195701, 1932013422544416768).await;
        info!("{:#?}", resp);
        assert!(resp.is_ok());
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_danmaku_buy_adv() {
        let bpi = BpiClient::new().expect("client should build");

        let resp = bpi.danmaku_buy_adv(413195701).await;
        info!("{:#?}", resp);
        assert!(resp.is_ok());
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_danmaku_get_adv_state() {
        let bpi = BpiClient::new().expect("client should build");

        let resp = bpi.danmaku_adv_state(413195701).await;
        info!("{:#?}", resp);
        assert!(resp.is_ok());
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_danmaku_thumbup() {
        let bpi = BpiClient::new().expect("client should build");

        let resp = bpi.danmaku_thumbup(413195701, 1932011031958944000, 1).await;
        info!("{:#?}", resp);
        assert!(resp.is_ok());
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_danmaku_edit_state() {
        let bpi = BpiClient::new().expect("client should build");

        let dmids = vec![1932011031958944000];
        let resp = bpi.danmaku_edit_state(413195701, &dmids, 1).await;
        info!("{:#?}", resp);
        assert!(resp.is_ok());
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_danmaku_edit_pool() {
        let bpi = BpiClient::new().expect("client should build");

        let dmids = vec![1932011031958944000];
        let resp = bpi.danmaku_edit_pool(413195701, &dmids, 1).await;
        info!("{:#?}", resp);
        assert!(resp.is_ok());
    }

    #[test]
    fn danmaku_send_params_serializes_default_form() -> Result<(), BpiError> {
        let params = DanmakuSendParams::new(Cid::new(413195701)?, "hello")?;

        assert_eq!(
            params.form_pairs("csrf-token"),
            vec![
                ("type", "1".to_string()),
                ("oid", "413195701".to_string()),
                ("msg", "hello".to_string()),
                ("mode", "1".to_string()),
                ("fontsize", "25".to_string()),
                ("color", "16777215".to_string()),
                ("pool", "0".to_string()),
                ("progress", "1878".to_string()),
                ("rnd", "2".to_string()),
                ("plat", "1".to_string()),
                ("csrf", "csrf-token".to_string()),
                ("checkbox_type", "0".to_string()),
                ("colorful", "".to_string()),
                ("gaiasource", "main_web".to_string()),
                ("polaris_app_id", "100".to_string()),
                ("polaris_platform", "5".to_string()),
                ("spmid", "333.788.0.0".to_string()),
                ("from_spmid", "333.788.0.0".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn danmaku_send_params_serializes_optional_fields() -> Result<(), BpiError> {
        let params = DanmakuSendParams::new(Cid::new(413195701)?, "hello")?
            .aid(Aid::new(590635620)?)
            .bvid(Bvid::new("BV1xx411c7mD")?)
            .mode(5)
            .danmaku_type(2)
            .progress(3600)
            .color(0)
            .font_size(36)
            .pool(1);

        let form = params.form_pairs("csrf-token");

        assert_eq!(form.iter().filter(|(key, _)| *key == "mode").count(), 1);
        assert!(form.contains(&("mode", "5".to_string())));
        assert!(form.contains(&("type", "2".to_string())));
        assert!(form.contains(&("progress", "3600".to_string())));
        assert!(form.contains(&("color", "0".to_string())));
        assert!(form.contains(&("fontsize", "36".to_string())));
        assert!(form.contains(&("pool", "1".to_string())));
        assert!(form.contains(&("avid", "590635620".to_string())));
        assert!(form.contains(&("bvid", "BV1xx411c7mD".to_string())));
        Ok(())
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
}
