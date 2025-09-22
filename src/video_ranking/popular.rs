use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// --- 获取当前热门视频列表 ---

/// 热门视频列表的页面信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PopularListData {
    /// 视频列表
    pub list: Vec<serde_json::Value>, // 视频内容复杂，这里用Value代替
    /// 是否有更多数据
    pub no_more: bool,
}

// --- 每周必看全部列表 ---

/// 每周必看列表中的单个必看
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PopularSeriesItem {
    /// 期数
    pub number: u32,
    /// 主题
    pub subject: String,
    /// 状态，2: 已结束
    pub status: u8,
    /// 名称，如 "yyyy第n期 MM.dd - MM.dd"
    pub name: String,
}

/// 每周必看全部列表数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PopularSeriesListData {
    /// 全部信息列表
    pub list: Vec<PopularSeriesItem>,
}

// --- 每周必看选期详细信息 ---

/// 每周必看选期信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PopularSeriesConfig {
    /// 选期 ID
    pub id: u64,
    /// 选期类型
    #[serde(rename = "type")]
    pub type_name: String,
    /// 期数
    pub number: u32,
    /// 主题
    pub subject: String,
    /// 开始时间
    pub stime: u64,
    /// 结束时间
    pub etime: u64,
    /// 状态，2: 已结束
    pub status: u8,
    /// 名称
    pub name: String,
    /// 标题
    pub label: String,
    /// 提示
    pub hint: String,
    /// 颜色
    pub color: u32,
    /// 封面
    pub cover: String,
    /// 分享标题
    pub share_title: String,
    /// 分享副标题
    pub share_subtitle: String,
    /// 媒体 ID
    pub media_id: u64,
}

/// 每周必看选期详细信息数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PopularSeriesOneData {
    /// 选期信息
    pub config: PopularSeriesConfig,
    /// 提醒
    pub reminder: Option<String>,
    /// 选期视频列表
    pub list: Vec<serde_json::Value>,
}

impl BpiClient {
    /// 获取当前热门视频列表
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/video_ranking/popular.html#获取当前热门视频列表
    ///
    /// # 参数
    /// | 名称 | 类型         | 说明                 |
    /// | ---- | ------------| -------------------- |
    /// | `pn` | Option<u32> | 页码，可选           |
    /// | `ps` | Option<u32> | 每页数量，可选       |
    pub async fn video_popular_list(
        &self,
        pn: Option<u32>,
        ps: Option<u32>,
    ) -> Result<BpiResponse<PopularListData>, BpiError> {
        let mut request = self.get("https://api.bilibili.com/x/web-interface/popular");

        if let Some(pn) = pn {
            request = request.query(&[("pn", pn)]);
        }
        if let Some(ps) = ps {
            request = request.query(&[("ps", ps)]);
        }

        request.send_bpi("获取当前热门视频列表").await
    }

    /// 获取每周必看全部列表
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/video_ranking/popular.html#获取每周必看全部列表
    ///
    pub async fn video_popular_series_list(
        &self,
    ) -> Result<BpiResponse<PopularSeriesListData>, BpiError> {
        self.get("https://api.bilibili.com/x/web-interface/popular/series/list")
            .send_bpi("获取每周必看全部列表")
            .await
    }

    /// 获取每周必看选期详细信息
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/video_ranking/popular.html#获取每周必看选期详细信息
    ///
    /// # 参数
    /// | 名称     | 类型     | 说明         |
    /// | -------- | --------| ------------|
    /// | `number` | u32     | 期数         |
    pub async fn video_popular_series_one(
        &self,
        number: u32,
    ) -> Result<BpiResponse<PopularSeriesOneData>, BpiError> {
        self.get("https://api.bilibili.com/x/web-interface/popular/series/one")
            .query(&[("number", number)])
            .send_bpi("获取每周必看选期详细信息")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_video_popular_list() {
        let bpi = BpiClient::new();
        let resp = bpi.video_popular_list(Some(1), Some(2)).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("no_more: {}", data.no_more);
            info!("first item: {:?}", data.list.first());
        }
    }

    #[tokio::test]
    async fn test_video_popular_series_list() {
        let bpi = BpiClient::new();
        let resp = bpi.video_popular_series_list().await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("first series: {:?}", data.list.first());
        }
    }

    #[tokio::test]
    async fn test_video_popular_series_one() {
        let bpi = BpiClient::new();
        let resp = bpi.video_popular_series_one(1).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("config: {:?}", data.config);
            info!("first video: {:?}", data.list.first());
        }
    }
}
