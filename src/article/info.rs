//! 专栏基本信息
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/article/info.md

use crate::article::models::ArticleStats;
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 专栏基本信息数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleInfoData {
    /// 是否点赞 0：未点赞 1：已点赞 需要登录(Cookie) 未登录为0
    pub like: i32,
    /// 是否关注文章作者 false：未关注 true：已关注 需要登录(Cookie) 未登录为false
    pub attention: bool,
    /// 是否收藏 false：未收藏 true：已收藏 需要登录(Cookie) 未登录为false
    pub favorite: bool,
    /// 为文章投币数
    pub coin: i32,
    /// 状态数信息
    pub stats: ArticleStats,
    /// 文章标题
    pub title: String,
    /// 文章头图url
    pub banner_url: String,
    /// 文章作者mid
    pub mid: i64,
    /// 文章作者昵称
    pub author_name: String,
    /// true 作用尚不明确
    pub is_author: bool,
    /// 动态封面
    pub image_urls: Vec<String>,
    /// 封面图片
    pub origin_image_urls: Vec<String>,
    /// true 作用尚不明确
    pub shareable: bool,
    /// true 作用尚不明确
    pub show_later_watch: bool,
    /// true 作用尚不明确
    pub show_small_window: bool,
    /// 是否收于文集 false：否 true：是
    pub in_list: bool,
    /// 上一篇文章cvid 无为0
    pub pre: i64,
    /// 下一篇文章cvid 无为0
    pub next: i64,
    /// 分享方式列表
    pub share_channels: Vec<ShareChannel>,
    /// 文章类别 0：文章 2：笔记
    pub r#type: i32,
    /// 视频URL
    #[serde(default)]
    pub video_url: String,
    /// 位置信息
    #[serde(default)]
    pub location: String,
    /// 是否禁用分享
    #[serde(default)]
    pub disable_share: bool,
}

/// 分享方式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareChannel {
    /// 分享名称
    pub name: String,
    /// 分享图片url
    pub picture: String,
    /// 分享代号
    pub share_channel: String,
}

impl BpiClient {
    /// 获取专栏文章基本信息
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明              |
    /// | ---- | ---- | ----------------- |
    /// | `id` | i64  | 专栏 cvid (必要)  |
    ///
    /// # 文档
    /// [获取专栏文章基本信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/article/info.md#获取专栏文章基本信息)
    pub async fn article_info(&self, id: i64) -> Result<BpiResponse<ArticleInfoData>, BpiError> {
        self.get("https://api.bilibili.com/x/article/viewinfo")
            .query(&[("id", id.to_string())])
            .send_bpi("获取专栏文章基本信息")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CVID: i64 = 2;

    #[tokio::test]
    async fn test_article_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let cvid = TEST_CVID;

        let result = bpi.article_info(cvid).await?;

        let data = result.data.unwrap();
        assert!(!data.title.is_empty());
        assert!(!data.author_name.is_empty());
        assert!(data.mid > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_article_stats() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let result = bpi.article_info(TEST_CVID).await?;

        let data = result.data.unwrap();
        let stats = &data.stats;
        assert!(stats.view >= 0);
        assert!(stats.favorite >= 0);
        assert!(stats.like >= 0);
        assert!(stats.reply >= 0);

        Ok(())
    }
}
