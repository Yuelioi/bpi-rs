use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use reqwest::multipart::Form;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// --- 直播间管理 API 结构体 ---

/// 开通直播间响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateRoomData {
    #[serde(rename = "roomID")]
    pub room_id: Option<String>,
}

/// 直播间信息更新响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateRoomData {
    pub sub_session_key: String,
    pub audit_info: Option<AuditInfo>,
}

/// 审核信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditInfo {
    pub audit_title_reason: String,
    pub audit_title_status: u8,
    pub audit_title: Option<String>,
    pub update_title: Option<String>,
}

/// RTMP 推流地址信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RtmpInfo {
    pub addr: String,
    pub code: String,
}

/// 开始直播响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StartLiveData {
    pub change: u8,
    pub status: String,
    pub rtmp: RtmpInfo,
    pub live_key: String,
    pub sub_session_key: String,
    pub need_face_auth: bool,
    // 其他不明确的字段都使用 Value
    pub room_type: Value,
    pub protocols: Value,
    pub notice: Value,
    pub qr: Value,
    pub service_source: String,
    pub rtmp_backup: Value,
    pub up_stream_extra: Value,
}

/// 关闭直播响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StopLiveData {
    pub change: u8,
    pub status: String,
}

/// 预更新直播间信息响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdatePreLiveInfoData {
    pub audit_info: Option<AuditInfo>,
}

/// PC直播姬版本号响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PcLiveVersionData {
    pub curr_version: String,
    pub build: u64,
    pub instruction: String,
    pub file_size: String,
    pub file_md5: String,
    pub content: String,
    pub download_url: String,
    pub hdiffpatch_switch: u8,
}

impl BpiClient {
    /// 开通直播间
    pub async fn live_create_room(&self) -> Result<BpiResponse<CreateRoomData>, BpiError> {
        let csrf = self.csrf()?;
        let form = Form::new()
            .text("platform", "web")
            .text("visit_id", "")
            .text("csrf", csrf.clone())
            .text("csrf_token", csrf);

        self.post("https://api.live.bilibili.com/xlive/app-blink/v1/preLive/CreateRoom")
            .multipart(form)
            .send_bpi("开通直播间")
            .await
    }

    /// 更新直播间信息
    ///
    /// # 参数
    /// * `room_id` - 直播间 ID
    /// * `title` - 标题，可选
    /// * `area_id` - 分区 ID，可选
    /// * `add_tag` - 要添加的标签，可选
    /// * `del_tag` - 要删除的标签，可选
    pub async fn live_update_room_info(
        &self,
        room_id: u64,
        title: Option<&str>,
        area_id: Option<u64>,
        add_tag: Option<&str>,
        del_tag: Option<&str>,
    ) -> Result<BpiResponse<UpdateRoomData>, BpiError> {
        let csrf = self.csrf()?;
        let mut form = Form::new()
            .text("room_id", room_id.to_string())
            .text("csrf", csrf.clone())
            .text("csrf_token", csrf);

        if let Some(t) = title {
            form = form.text("title", t.to_string());
        }
        if let Some(a) = area_id {
            form = form.text("area_id", a.to_string());
        }
        if let Some(a_tag) = add_tag {
            form = form.text("add_tag", a_tag.to_string());
        }
        if let Some(d_tag) = del_tag {
            form = form.text("del_tag", d_tag.to_string());
        }

        self.post("https://api.live.bilibili.com/room/v1/Room/update")
            .multipart(form)
            .send_bpi("更新直播间信息")
            .await
    }

    /// 开始直播 (目前仅支持直播姬开播)
    ///
    /// # 参数
    /// * `room_id` - 直播间 ID
    /// * `area_v2` - 直播分区 ID
    /// * `platform` - 直播平台，如 "pc"
    #[allow(dead_code)]
    async fn live_start(
        &self,
        room_id: u64,
        area_v2: u64,
        platform: &str,
    ) -> Result<BpiResponse<StartLiveData>, BpiError> {
        let csrf = self.csrf()?;
        let form = Form::new()
            .text("room_id", room_id.to_string())
            .text("area_v2", area_v2.to_string())
            .text("platform", platform.to_string())
            .text("csrf", csrf.clone())
            .text("csrf_token", csrf);

        self.post("https://api.live.bilibili.com/room/v1/Room/startLive")
            .multipart(form)
            .send_bpi("开始直播")
            .await
    }

    /// 关闭直播
    ///
    /// # 参数
    /// * `room_id` - 直播间 ID
    /// * `platform` - 直播平台，如 "pc_link"
    pub async fn live_stop(
        &self,
        room_id: u64,
        platform: &str,
    ) -> Result<BpiResponse<StopLiveData>, BpiError> {
        let csrf = self.csrf()?;
        let form = Form::new()
            .text("platform", platform.to_string())
            .text("room_id", room_id.to_string())
            .text("csrf", csrf.clone())
            .text("csrf_token", csrf);

        self.post("https://api.live.bilibili.com/room/v1/Room/stopLive")
            .multipart(form)
            .send_bpi("关闭直播")
            .await
    }

    /// 预更新直播间信息
    ///
    /// # 参数
    /// * `title` - 标题，可选
    /// * `cover` - 封面 URL，可选
    pub async fn live_update_pre_live_info(
        &self,
        title: Option<&str>,
        cover: Option<&str>,
    ) -> Result<BpiResponse<UpdatePreLiveInfoData>, BpiError> {
        let csrf = self.csrf()?;
        let mut form = Form::new()
            .text("platform", "web")
            .text("mobi_app", "web")
            .text("build", "1")
            .text("csrf", csrf.clone())
            .text("csrf_token", csrf);

        if let Some(t) = title {
            form = form.text("title", t.to_string());
        }
        if let Some(c) = cover {
            form = form.text("cover", c.to_string());
        }

        self.post("https://api.live.bilibili.com/xlive/app-blink/v1/preLive/UpdatePreLiveInfo")
            .multipart(form)
            .send_bpi("预更新直播间信息")
            .await
    }

    /// 更新直播间公告
    ///
    /// # 参数
    /// * `room_id` - 直播间 ID
    /// * `uid` - 用户ID
    /// * `content` - 公告内容
    pub async fn live_update_room_news(
        &self,
        room_id: u64,
        uid: u64,
        content: &str,
    ) -> Result<BpiResponse<Value>, BpiError> {
        let csrf = self.csrf()?;
        let form = Form::new()
            .text("room_id", room_id.to_string())
            .text("uid", uid.to_string())
            .text("content", content.to_string())
            .text("csrf", csrf.clone())
            .text("csrf_token", csrf);

        self.post("https://api.live.bilibili.com/xlive/app-blink/v1/index/updateRoomNews")
            .multipart(form)
            .send_bpi("更新直播间公告")
            .await
    }

    /// 获取 PC 直播姬版本号
    pub async fn live_version(&self) -> Result<BpiResponse<PcLiveVersionData>, BpiError> {
        self.get("https://api.live.bilibili.com/xlive/app-blink/v1/liveVersionInfo/getHomePageLiveVersion")
            .query(&[("system_version", "2")])
            .send_bpi("获取 PC 直播姬版本号")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]

    async fn test_live_create_room() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        match bpi.live_create_room().await {
            Ok(resp) => resp,
            Err(err) => {
                // 已经创建直播间
                if let Some(code) = err.code() {
                    if code == 1531193016 {
                        tracing::warn!("allowed special code: {}", code);
                        return Ok(());
                    }
                }
                return Err(err);
            }
        };
        Ok(())
    }

    #[tokio::test]

    async fn test_live_update_room_info() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let room_id = 3818081;
        let resp = bpi
            .live_update_room_info(room_id, Some("测试新标题"), None, None, None)
            .await?;

        assert_eq!(resp.code, 0);
        let data = resp.into_data()?;

        info!("更新直播间信息返回：{:?}", data);

        Ok(())
    }

    #[tokio::test]

    async fn test_live_stop() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        // 替换为您的直播间 ID
        let room_id = 3818081;
        let resp = bpi.live_stop(room_id, "pc_link").await?;
        assert_eq!(resp.code, 0);
        let data = resp.into_data()?;

        info!("关闭直播返回：{:?}", data);

        Ok(())
    }

    #[tokio::test]

    async fn test_live_update_pre_live_info() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi
            .live_update_pre_live_info(Some("测试预更新标题"), None)
            .await?;
        assert_eq!(resp.code, 0);

        let data = resp.into_data()?;

        info!("预更新直播间信息返回：{:?}", data);

        Ok(())
    }

    #[tokio::test]

    async fn test_live_update_room_news() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        // 替换为您的直播间 ID
        let room_id = 3818081;
        let uid = 4279370;
        let content = "Rust 指南测试公告";
        let resp = bpi.live_update_room_news(room_id, uid, content).await?;
        assert_eq!(resp.code, 0);

        info!("更新直播间公告返回：{:?}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_live_version() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.live_version().await?;
        assert_eq!(resp.code, 0);
        let data = resp.into_data()?;

        info!("PC直播姬版本号: {}", data.curr_version);

        Ok(())
    }
}
