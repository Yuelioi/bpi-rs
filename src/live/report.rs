use serde::{ Deserialize, Serialize };

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use base64::{ Engine as _, engine::general_purpose };

// ================= 数据结构 =================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct HeartBeatData {
    /// 下次心跳间隔
    pub next_interval: i32,
}

pub type HeartBeatResponse = BpiResponse<HeartBeatData>;

// ================= 实现 =================

impl BpiClient {
    /// 直播心跳 (Web端)
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live)
    pub async fn live_web_heart_beat(
        &self,
        room_id: i64,
        next_interval: Option<i32>,
        platform: Option<&str>
    ) -> Result<HeartBeatResponse, BpiError> {
        // 构建心跳数据
        let interval = next_interval.unwrap_or(60);
        let heart_beat_data = format!("{interval}|{room_id}|1|0");

        // Base64编码
        let encoded_hb = general_purpose::STANDARD.encode(heart_beat_data);

        let mut params: Vec<(&str, String)> = vec![("hb", encoded_hb)];

        if let Some(platform) = platform {
            params.push(("pf", platform.to_string()));
        } else {
            params.push(("pf", "web".to_string()));
        }

        let resp: HeartBeatResponse = self
            .get("https://live-trace.bilibili.com/xlive/rdata-interface/v1/heartbeat/webHeartBeat")
            .query(&params)
            .send_bpi("直播心跳上报").await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_web_heart_beat() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let resp = bpi.live_web_heart_beat(23174842, None, None).await?;

        let data = resp.data.unwrap();

        assert!(data.next_interval > 0);
        Ok(())
    }
}
