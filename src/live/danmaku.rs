use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use chrono::Utc;
use reqwest::multipart::Form;
use serde::{ Deserialize, Serialize };

// --- 弹幕发送响应数据结构体 ---

/// 弹幕发送响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendDanmuData {
    pub mode_info: Option<serde_json::Value>,
    pub dm_v2: Option<serde_json::Value>,
}

/// 直播弹幕 WebSocket 接入点（`getDanmuInfo` 返回的 `host_list` 一项）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiveDanmuInfoHost {
    pub host: String,
    #[serde(default)]
    pub port: u32,
    #[serde(default)]
    pub wss_port: u32,
    #[serde(default)]
    pub ws_port: u32,
}

/// 直播弹幕服务器信息（WebSocket token / 接入 host）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiveDanmuInfoData {
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub host_list: Vec<LiveDanmuInfoHost>,
}

impl BpiClient {
    /// 获取直播弹幕服务器信息（WebSocket `token`、接入 `host` 等）
    ///
    /// `GET https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo`
    ///
    /// 内部已对查询参数执行 WBI 签名（`get_wbi_sign2`）；缺少签名时接口常见返回码为 `-352`。
    ///
    /// # 参数
    /// - `room_id`: 直播间长号（真实房间号）
    /// - `info_type`: 与官方接口 `type` 一致，一般为 `0`
    pub async fn live_get_danmu_info(
        &self,
        room_id: u64,
        info_type: u8
    ) -> Result<BpiResponse<LiveDanmuInfoData>, BpiError> {
        let signed = self
            .get_wbi_sign2(vec![("id", room_id.to_string()), ("type", info_type.to_string())])
            .await?;

        self
            .get("https://api.live.bilibili.com/xlive/web-room/v1/index/getDanmuInfo")
            .with_bilibili_headers()
            .query(&signed)
            .send_bpi("直播 getDanmuInfo")
            .await
    }

    /// 发送直播间弹幕
    ///
    /// # 参数
    /// * `room_id` - 直播间 ID
    /// * `message` - 弹幕内容
    /// * `color` - 十进制颜色值，默认 16777215 (白色)
    /// * `font_size` - 字体大小，默认 25
    pub async fn live_send_danmu(
        &self,
        room_id: u64,
        message: &str,
        color: Option<u32>,
        font_size: Option<u32>
    ) -> Result<BpiResponse<SendDanmuData>, BpiError> {
        let csrf = self.csrf()?;
        let now = Utc::now().timestamp();

        // 使用 Form 构建 application/x-www-form-urlencoded 请求体
        let mut form = Form::new()
            .text("csrf", csrf.clone())
            .text("roomid", room_id.to_string())
            .text("msg", message.to_string())
            .text("rnd", now.to_string())
            .text("bubble", "0")
            .text("mode", "1")
            .text("statistics", r#"{"appId":100,"platform":5}"#)
            .text("csrf_token", csrf); // 文档中提到 csrf_token 和 csrf 相同

        if let Some(c) = color {
            form = form.text("color", c.to_string());
        } else {
            form = form.text("color", "16777215"); // 默认白色
        }

        if let Some(s) = font_size {
            form = form.text("fontsize", s.to_string());
        } else {
            form = form.text("fontsize", "25"); // 默认 25
        }

        self
            .post("https://api.live.bilibili.com/msg/send")
            .multipart(form)
            .send_bpi("发送直播弹幕").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_live_get_danmu_info() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let room_id = 21733448;

        let resp = bpi.live_get_danmu_info(room_id, 0).await?;
        assert_eq!(resp.code, 0);
        let data = resp.into_data()?;

        assert!(!data.token.is_empty(), "token 不应为空");
        assert!(!data.host_list.is_empty(), "host_list 不应为空");
        info!("token: {}..., host_list 数量: {}", &data.token[..20], data.host_list.len());

        Ok(())
    }

    #[tokio::test]
    async fn test_send_live_danmu() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        // 替换为实际的直播间 ID，这是一个公开的直播间 ID
        let room_id = 21733448;
        let message = "牛";

        let resp = bpi.live_send_danmu(room_id, &message, None, None).await?;
        assert_eq!(resp.code, 0);
        let data = resp.into_data()?;

        info!("弹幕发送成功！返回数据: {:?}", data);

        Ok(())
    }
}
