use serde::{ Deserialize, Serialize };

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SilentUserInfo {
    /// 禁言者uid
    pub tuid: i64,
    /// 禁言者昵称
    pub tname: String,
    /// 发起者uid
    pub uid: i64,
    /// 发起者昵称
    pub name: String,
    /// 禁言时间
    pub ctime: String,
    /// 禁言记录Id
    pub id: i64,
    /// 是否是房主禁言的，0否，1是
    pub is_anchor: i8,
    /// 禁言者头像
    pub face: String,
    /// 禁言理由
    pub msg: String,
    /// 发起者权限
    pub admin_level: i8,
    /// 是否注销
    pub is_mystery: bool,
    /// 禁言结束时间，空代表永久或本场禁言
    pub block_end_time: String,
    /// 禁言模式，0代表永久，1代表正常，2代表本场禁言
    pub r#type: i8,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SilentUserListData {
    /// 禁言列表
    pub data: Vec<SilentUserInfo>,
    /// 禁言观众数量
    pub total: i32,
    /// 页码总数量，只有一页的时候没有
    #[serde(default)]
    pub total_page: i32,
    /// 页码，只有一页的时候没有
    #[serde(default)]
    pub pn: i32,
    /// 上限，只有一页的时候没有
    #[serde(default)]
    pub ps: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BannedUserInfo {
    /// 拉黑者uid
    pub uid: i64,
    /// 拉黑时间
    pub mtime: String,
    /// 拉黑者头像
    pub face: String,
    /// 拉黑者昵称
    pub name: String,
    /// 是否是房主拉黑的
    pub is_anchor: bool,
    /// 发起者昵称
    pub operator_name: String,
    /// 发起者权限
    pub admin_level: i8,
    /// 是否注销
    pub is_mystery: bool,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BannedUserListData {
    /// 拉黑列表
    pub data: Vec<BannedUserInfo>,
    /// 拉黑观众数量
    pub total: i32,
    /// 页码总数量，只有一页的时候没有，由于接口不返回，所以默认0
    #[serde(default)]
    pub total_page: i32,
    /// 上限，只有一页的时候没有，由于接口不返回，所以默认0
    #[serde(default)]
    pub pn: i32,
    /// 页码，只有一页的时候没有，由于接口不返回，所以默认0
    #[serde(default)]
    pub ps: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ShieldKeywordInfo {
    /// 违禁词
    pub keyword: String,
    /// 添加者uid
    pub uid: i64,
    /// 添加者昵称
    pub name: String,
    /// 是否是房主添加的，0否，1是
    pub is_anchor: i8,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ShieldKeywordListData {
    /// 违禁词列表
    pub keyword_list: Vec<ShieldKeywordInfo>,
    /// 数量上限
    pub max_limit: i32,
}

impl BpiClient {
    /// 禁言观众
    /// tuid: 用户uid
    /// hour: -1永久 0本场直播
    /// msg: 禁言理由，一般为禁言的弹幕，选填
    pub async fn live_add_silent_user(
        &self,
        room_id: i64,
        tuid: i64,
        hour: i32,
        msg: Option<String>,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let form = vec![
            ("room_id", room_id.to_string()),
            ("tuid", tuid.to_string()),
            ("msg", msg.unwrap_or(String::new())),
            ("mobile_app", "web".to_string()),
            ("type", if hour == 0 {"2".to_string()} else {"1".to_string()}),
            ("hour", hour.to_string()),
            ("csrf_token", csrf.clone()),
            ("csrf", csrf)
        ];

        // if let Some(msg) = msg {
        //     form.push(("msg", msg.to_string()));
        // }

        self
            .post("https://api.live.bilibili.com/xlive/web-ucenter/v1/banned/AddSilentUser")
            .form(&form)
            .send_bpi("禁言观众").await
    }

    /// 查询直播间禁言列表
    /// ps: 每页数量
    /// pn: 页码，默认1
    pub async fn live_list_silent_users(
        &self,
        room_id: i64,
        ps: i32,
        pn: i32,
    ) -> Result<BpiResponse<SilentUserListData>, BpiError> {
        let csrf = self.csrf()?;

        let form = vec![
            ("room_id", room_id.to_string()),
            ("pn", pn.to_string()),
            ("ps", ps.to_string()),
            ("csrf_token", csrf.clone()),
            ("csrf", csrf)
        ];

        self
            .post("https://api.live.bilibili.com/xlive/web-ucenter/v1/banned/GetSilentUserList")
            .form(&form)
            .send_bpi("查询直播间禁言列表").await
    }

    /// 解除禁言
    ///
    pub async fn live_del_block_user(
        &self,
        roomid: i64,
        tuid: i64
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let form = vec![
            ("room_id", roomid.to_string()),
            ("tuid", tuid.to_string()),
            ("csrf_token", csrf.clone()),
            ("csrf", csrf)
        ];

        self
            .post("https://api.live.bilibili.com/xlive/web-ucenter/v1/banned/DelSilentUser")
            .form(&form)
            .send_bpi("解除禁言").await
    }

    /// 拉黑观众
    /// anchor_id：主播uid
    pub async fn live_add_banned_user(
        &self,
        room_id: i64,
        anchor_id: i64,
        tuid: i64,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let form = vec![
            ("tuid", tuid.to_string()),
            ("anchor_id", anchor_id.to_string()),
            ("spmid", "444.8.0.0".to_string()),
            ("csrf_token", csrf.clone()),
            ("csrf", csrf),
            ("visit_id", "".to_string()),
        ];

        self
            .post("https://api.live.bilibili.com/xlive/app-ucenter/v2/xbanned/banned/AddBlack")
            .header("Referer", format!("https://live.bilibili.com/{}", room_id))
            .form(&form)
            .send_bpi("拉黑观众").await
    }
    
    /// 查询直播间拉黑列表
    /// pn: 页码，默认1
    /// ps: 每页数量
    pub async fn live_list_banned_users(
        &self,
        anchor_id: i64,
        pn: i32,
        ps: i32,
    ) -> Result<BpiResponse<BannedUserListData>, BpiError> {
        let csrf = self.csrf()?;

        let query = vec![
            ("anchor_id", anchor_id.to_string()),
            ("pn", pn.to_string()),
            ("ps", ps.to_string()),
            ("mobi_app", "android".to_string()),
            ("platform", "android".to_string()),
            ("spmid", "444.8.0.0".to_string()),
            ("csrf_token", csrf.clone()),
            ("csrf", csrf),
            ("visit_id", "".to_string()),
        ];

        self
            .get("https://api.live.bilibili.com/xlive/app-ucenter/v2/xbanned/banned/GetBlackList")
            .query(&query)
            .send_bpi("查询直播间拉黑列表").await
    }

    /// 解除拉黑
    /// anchor_id：主播uid
    pub async fn live_del_banned_user(
        &self,
        room_id: i64,
        anchor_id: i64,
        tuid: i64,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let form = vec![
            ("tuid", tuid.to_string()),
            ("anchor_id", anchor_id.to_string()),
            ("spmid", "444.8.0.0".to_string()),
            ("csrf_token", csrf.clone()),
            ("csrf", csrf),
            ("visit_id", "".to_string()),
            ("mobi_app", "android".to_string()),
            ("platform", "android".to_string()),
        ];

        self.post("https://api.live.bilibili.com/xlive/app-ucenter/v2/xbanned/banned/DelBlack")
            .header("Referer", format!("https://live.bilibili.com/{}", room_id))
            .form(&form)
            .send_bpi("解除拉黑")
            .await
    }

    /// 添加屏蔽词
    ///
    pub async fn live_add_shield_keyword(
        &self,
        room_id: i64,
        keyword: String,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let form = vec![
            ("keyword", keyword),
            ("room_id", room_id.to_string()),
            ("spmid", "444.8.0.0".to_string()),
            ("csrf_token", csrf.clone()),
            ("csrf", csrf),
            ("visit_id", "".to_string()),
            ("mobi_app", "android".to_string()),
            ("platform", "android".to_string()),
        ];

        self
            .post("https://api.live.bilibili.com/xlive/app-ucenter/v1/banned/AddShieldKeyword")
            .form(&form)
            .send_bpi("添加屏蔽词").await
    }
    
    /// 查询直播间屏蔽词列表
    /// 没有ps和pn，返回全部屏蔽词
    pub async fn live_list_shield_keyword(
        &self,
        room_id: i64,
    ) -> Result<BpiResponse<ShieldKeywordListData>, BpiError> {
        let csrf = self.csrf()?;

        let form = vec![
            ("room_id", room_id.to_string()),
            ("spmid", "444.8.0.0".to_string()),
            ("csrf_token", csrf.clone()),
            ("csrf", csrf),
            ("visit_id", "".to_string()),
            ("mobi_app", "android".to_string()),
            ("platform", "android".to_string()),
        ];

        self
            .post("https://api.live.bilibili.com/xlive/app-ucenter/v1/banned/GetShieldKeywordList")
            .form(&form)
            .send_bpi("查询直播间屏蔽词列表").await
    }

    /// 删除屏蔽词
    ///
    pub async fn live_del_shield_keyword(
        &self,
        room_id: i64,
        keyword: String,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let form = vec![
            ("keyword", keyword),
            ("room_id", room_id.to_string()),
            ("spmid", "444.8.0.0".to_string()),
            ("csrf_token", csrf.clone()),
            ("csrf", csrf),
            ("visit_id", "".to_string()),
            ("mobi_app", "android".to_string()),
            ("platform", "android".to_string()),
        ];

        self
            .post("https://api.live.bilibili.com/xlive/app-ucenter/v1/banned/DelShieldKeyword")
            .form(&form)
            .send_bpi("删除屏蔽词").await
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_silent_user_list() {
        let bpi = BpiClient::new();
        let resp = bpi.live_list_silent_users(3818081, 1, 10).await.unwrap();
        tracing::info!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_add_silent_user() {
        let bpi = BpiClient::new();
        let resp = bpi.live_add_silent_user(3818081, 316183842, 0, None).await.unwrap();
        tracing::info!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_del_silent_user_list() {
        let bpi = BpiClient::new();
        let resp = bpi.live_del_block_user(3818081, 316183842).await.unwrap();
        tracing::info!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_get_banned_user_list() {
        let bpi = BpiClient::new();
        let resp = bpi.live_list_banned_users(4279370, 1, 10).await.unwrap();
        tracing::info!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_add_banned_user() {
        let bpi = BpiClient::new();
        let resp = bpi.live_add_banned_user(3818081, 4279370, 316183842).await.unwrap();
        tracing::info!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_del_banned_user_list() {
        let bpi = BpiClient::new();
        let resp = bpi.live_del_banned_user(3818081, 4279370, 316183842).await.unwrap();
        tracing::info!("{:?}", resp);
    }
    #[tokio::test]
    async fn test_get_keyword_list() {
        let bpi = BpiClient::new();
        let resp = bpi.live_list_shield_keyword(3818081).await.unwrap();
        tracing::info!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_add_keyword() {
        let bpi = BpiClient::new();
        let resp = bpi.live_add_shield_keyword(3818081, "test keyword".to_string()).await.unwrap();
        tracing::info!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_del_keyword() {
        let bpi = BpiClient::new();
        let resp = bpi.live_del_shield_keyword(3818081, "test keyword".to_string()).await.unwrap();
        tracing::info!("{:?}", resp);
    }
}
