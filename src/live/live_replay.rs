use serde::{ Deserialize, Serialize };

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LiveInfo {
    /// 直播标题
    pub title: String,
    /// 直播封面
    pub cover: String,
    /// 直播时间
    pub live_time: i64,
    /// 直播类型
    pub live_type: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct VideoInfo {
    /// 回放状态
    pub replay_status: i32,
    /// 直播回放合成结束时间
    pub estimated_time: String,
    /// 直播时长（秒）
    pub duration: i32,
    /// 下载链接片段
    pub download_url: Option<String>,
    /// 快速检查警告代码
    pub alert_code: Option<i32>,
    /// 快速检查警告信息
    pub alert_message: Option<String>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct AlarmInfo {
    /// 回放合成警报代码
    pub code: i32,
    /// 回放合成错误信息
    pub message: String,
    /// 当前时间戳
    pub cur_time: i64,
    /// 是否禁止发布
    pub is_ban_publish: bool,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ReplayInfo {
    /// 直播回放id
    pub replay_id: i64,
    /// 直播信息
    pub live_info: LiveInfo,
    /// 回放视频信息
    pub video_info: VideoInfo,
    /// 警报信息
    pub alarm_info: AlarmInfo,
    /// 直播间id
    pub room_id: i64,
    /// 标记直播场次的key
    pub live_key: String,
    /// 直播开始秒时间戳
    pub start_time: i64,
    /// 直播结束秒时间戳
    pub end_time: i64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Pagination {
    /// 请求的页码
    pub page: i32,
    /// 内容数量
    pub page_size: i32,
    /// 总计内容数量
    pub total: Option<i32>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ReplayListData {
    /// 回放信息列表
    pub replay_info: Option<Vec<ReplayInfo>>,
    /// 分页信息
    pub pagination: Pagination,
}

impl BpiClient {
    /// 获取直播回放列表
    ///

    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live)
    pub async fn live_replay_list(
        &self,
        page: Option<i32>,
        page_size: Option<i32>
    ) -> Result<BpiResponse<ReplayListData>, BpiError> {
        let mut query = Vec::new();

        if let Some(page) = page {
            query.push(("page", page.to_string()));
        }

        if let Some(page_size) = page_size {
            query.push(("page_size", page_size.to_string()));
        }

        self
            .get("https://api.live.bilibili.com/xlive/app-blink/v1/anchorVideo/AnchorGetReplayList")
            .query(&query)
            .send_bpi("获取直播回放列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_live_replay_list() {
        let bpi = BpiClient::new();
        let resp = bpi.live_replay_list(Some(1), Some(2)).await.unwrap();
        tracing::info!("{:?}", resp);
    }
}
