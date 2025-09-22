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

impl BpiClient {
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
