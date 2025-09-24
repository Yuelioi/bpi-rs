use serde::{ Deserialize, Serialize };

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct QualityDescription {
    /// 画质代码
    pub qn: i32,
    /// 该代码对应的画质名称
    pub desc: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LiveStreamUrl {
    /// 直播流url
    pub url: String,
    /// 服务器线路序号
    pub order: i32,
    /// 作用尚不明确
    pub stream_type: i32,
    /// 作用尚不明确
    pub p2p_type: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LiveStreamData {
    /// 当前画质代码qn
    pub current_quality: i32,
    /// 可选画质数参数
    pub accept_quality: Vec<String>,
    /// 当前画质代码quality
    pub current_qn: i32,
    /// 可选画质参数quality
    pub quality_description: Vec<QualityDescription>,
    /// 直播流url组
    pub durl: Vec<LiveStreamUrl>,
}

impl BpiClient {
    /// 根据真实直播间号获取直播视频流
    ///
    /// # 参数
    /// * `cid` - 目标真实直播间号（必要），直播间的 room_id（非短号）。
    ///
    /// * `platform` - 播放平台（非必要，默认 `web` 即 http-flv 方式）。
    ///   - `"h5"`  → HLS 方式
    ///   - `"web"` → HTTP-FLV 方式
    ///
    /// * `quality` - 画质（非必要，与 `qn` 二选一）。
    ///   - `2` → 流畅
    ///   - `3` → 高清
    ///   - `4` → 原画
    ///
    /// * `qn` - 画质（非必要，与 `quality` 二选一）。
    ///   - `80` → 流畅
    ///   - `150` → 高清
    ///   - `250` → 超清
    ///   - `400` → 蓝光
    ///   - `10000` → 原画
    ///   - `20000` → 4K
    ///   - `25000` → 默认
    ///   - `30000` → 杜比
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live)
    pub async fn live_stream(
        &self,
        cid: i64,
        platform: Option<&str>,
        quality: Option<i32>,
        qn: Option<i32>
    ) -> Result<BpiResponse<LiveStreamData>, BpiError> {
        let mut query = vec![("cid", cid.to_string())];

        if let Some(platform) = platform {
            query.push(("platform", platform.to_string()));
        }

        if let Some(quality) = quality {
            query.push(("quality", quality.to_string()));
        }

        if let Some(qn) = qn {
            query.push(("qn", qn.to_string()));
        }

        self
            .get("https://api.live.bilibili.com/room/v1/Room/playUrl")
            .query(&query)
            .send_bpi("获取直播视频流").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_live_stream() {
        let bpi = BpiClient::new();
        let resp = bpi.live_stream(14073662, Some("web"), None, Some(10000)).await.unwrap();
        tracing::info!("{:?}", resp);
    }
}
