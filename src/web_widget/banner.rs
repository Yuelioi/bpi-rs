//! B站分区轮播图相关接口
//!
//! 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/web_widget/banner.html
use crate::video::video_zone_v2::VideoPartitionV2;
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 轮播图对象
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegionBanner {
    pub image: String, // 封面资源路径
    pub title: String, // 封面标题
    pub sub_title: String, // 封面子标题
    pub url: String, // 点击后的跳转链接
    pub rid: i64, // 分区 ID
}

/// 轮播图响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RegionBannerData {
    pub region_banner_list: Vec<RegionBanner>, // 轮播图列表
}

impl BpiClient {
    /// 获取各分区的轮播图（Web端）
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/web_widget/banner.html#获取各分区的轮播图
    ///
    /// # 参数
    /// | 名称        | 类型                | 说明         |
    /// | ----------- | -------------------| ------------|
    /// | `region_id` | VideoPartitionV2    | 分区 ID      |
    pub async fn web_widget_region_banner(
        &self,
        region_id: VideoPartitionV2
    ) -> Result<BpiResponse<RegionBannerData>, BpiError> {
        let query = vec![("region_id", region_id.tid().to_string())];

        self
            .get("https://api.bilibili.com/x/web-show/region/banner")
            .query(&query)
            .send_bpi("获取各分区的轮播图").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::video::video_zone_v2::{ Douga, VideoPartitionV2 };

    use tracing::info;

    #[tokio::test]
    async fn test_get_region_banner() {
        let bpi = BpiClient::new();
        // 例如 region_id = 1 (动画)
        let resp = bpi.web_widget_region_banner(VideoPartitionV2::Douga(Douga::Douga)).await;
        info!("响应: {:?}", resp);
        assert!(resp.is_ok());

        let data = resp.unwrap().data.unwrap();
        info!("分区轮播图: {:?}", data);
    }
}
