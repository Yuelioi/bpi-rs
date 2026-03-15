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
    /// 不明
    pub is_anchor: i32,
    /// 禁言者头像
    pub face: String,
    /// 发起者权限
    pub admin_level: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SilentUserListData {
    /// 禁言列表
    pub data: Vec<SilentUserInfo>,
    /// 禁言观众数量
    pub total: i32,
    /// 页码总数量
    pub total_page: i32,
}

impl BpiClient {
    /// 禁言观众
    /// tuid: 用户uid
    /// hour: -1永久 0本场直播
    pub async fn live_add_silent_user(
        &self,
        room_id: i64,
        tuid: i64,
        hour: i32
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let block_type = if hour == 0 {"2".to_string()} else {"1".to_string()};
        
        let form = vec![
            ("room_id", room_id.to_string()),
            ("tuid", tuid.to_string()),
            ("mobile_app", "web".to_string()),
            ("type", block_type),
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
    ///
    pub async fn live_list_silent_users(
        &self,
        room_id: i64,
        ps: i32
    ) -> Result<BpiResponse<SilentUserListData>, BpiError> {
        let csrf = self.csrf()?;

        let form = vec![
            ("room_id", room_id.to_string()),
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
        id: i64
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let form = vec![
            ("room_id", roomid.to_string()),
            ("tuid", id.to_string()),
            ("csrf_token", csrf.clone()),
            ("csrf", csrf)
        ];

        self
            .post("https://api.live.bilibili.com/xlive/web-ucenter/v1/banned/DelSilentUser")
            .form(&form)
            .send_bpi("解除禁言").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_silent_user_list() {
        let bpi = BpiClient::new();
        let resp = bpi.live_list_silent_users(3818081, 1).await.unwrap();
        tracing::info!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_add_silent_user() {
        let bpi = BpiClient::new();
        let resp = bpi.live_add_silent_user(3818081, 316183842, 0).await.unwrap();
        tracing::info!("{:?}", resp);
    }

    #[tokio::test]
    async fn test_del_silent_user_list() {
        let bpi = BpiClient::new();
        let resp = bpi.live_del_block_user(3818081, 316183842).await.unwrap();
        tracing::info!("{:?}", resp);
    }
}
