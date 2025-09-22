use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 动态首页公告栏响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicBannerData {
    /// 横幅列表
    pub banners: Vec<DynamicBanner>,
}

/// 动态横幅数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicBanner {
    /// 横幅 ID
    pub banner_id: u64,
    /// 结束时间（UNIX 秒级时间戳）
    pub end_time: u64,
    /// 图片 URL
    pub img_url: String,
    /// 跳转链接
    pub link: String,
    /// 平台
    pub platform: u64,
    /// 位置
    pub position: String,
    /// 开始时间（UNIX 秒级时间戳）
    pub start_time: u64,
    /// 标题
    pub title: String,
    /// 权重
    pub weight: u64,
}

impl BpiClient {
    /// 获取动态首页公告栏
    ///
    /// # 参数
    /// * `platform` - 平台，默认为 1
    pub async fn dynamic_feed_banner(&self) -> Result<BpiResponse<DynamicBannerData>, BpiError> {
        let req = self
            .get("https://api.bilibili.com/x/dynamic/feed/dyn/banner")
            .query(&[
                ("platform", "1"),
                ("position", "web动态"),
                ("web_location", "333.1365"),
            ]);

        req.send_bpi("获取动态首页公告栏").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_dynamic_feed_banner() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.dynamic_feed_banner().await?;
        let data = resp.into_data()?;

        info!("成功获取到 {} 条公告", data.banners.len());
        assert!(!data.banners.is_empty());

        Ok(())
    }
}
