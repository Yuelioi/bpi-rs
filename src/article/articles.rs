//! 文集基本信息
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/article/articles.md

use crate::article::models::{ArticleAuthor, ArticleCategory, ArticleStats};
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 文集基本信息数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticlesData {
    /// 文集概览
    pub list: ArticleList,
    /// 文集内的文章列表
    pub articles: Vec<ArticleItem>,
    /// 文集作者信息
    pub author: ArticleAuthor,
    /// 作用尚不明确 结构与data.articles[]中相似
    pub last: ArticleItem,
    /// 是否关注文集作者 false：未关注 true：已关注 需要登录(Cookie) 未登录为false
    pub attention: bool,
}

/// 文集概览
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleList {
    /// 文集rlid
    pub id: i64,
    /// 文集作者mid
    pub mid: i64,
    /// 文集名称
    pub name: String,
    /// 文集封面图片url
    pub image_url: String,
    /// 文集更新时间 时间戳
    pub update_time: i64,
    /// 文集创建时间 时间戳
    pub ctime: i64,
    /// 文集发布时间 时间戳
    pub publish_time: i64,
    /// 文集简介
    pub summary: String,
    /// 文集字数
    pub words: i64,
    /// 文集阅读量
    pub read: i64,
    /// 文集内文章数量
    pub articles_count: i32,
    /// 1或3 作用尚不明确
    pub state: i32,
    /// 空 作用尚不明确
    pub reason: String,
    /// 空 作用尚不明确
    pub apply_time: String,
    /// 空 作用尚不明确
    pub check_time: String,
}

/// 文章项目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleItem {
    /// 专栏cvid
    pub id: i64,
    /// 文章标题
    pub title: String,
    /// 0 作用尚不明确
    pub state: i32,
    /// 发布时间 秒时间戳
    pub publish_time: i64,
    /// 文章字数
    pub words: i64,
    /// 文章封面
    pub image_urls: Vec<String>,
    /// 文章标签
    pub category: ArticleCategory,
    /// 文章标签列表
    pub categories: Vec<ArticleCategory>,
    /// 文章摘要
    pub summary: String,
    // 文章状态数信息
    pub stats: Option<ArticleStats>,
    /// 是否点赞 0：未点赞 1：已点赞 需要登录(Cookie) 未登录为0
    pub like_state: Option<i32>,
}

/// 作者大会员状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorVip {
    /// 大会员类型
    pub r#type: i32,
    /// 大会员状态
    pub status: i32,
    /// 到期时间
    pub due_date: i64,
    /// 支付类型
    pub vip_pay_type: i32,
    /// 主题类型
    pub theme_type: i32,
    /// 标签
    pub label: Option<serde_json::Value>,
}

impl BpiClient {
    /// 获取文集基本信息
    ///
    /// # 参数
    /// | 名称   | 类型  | 说明              |
    /// | ------ | ----- | ----------------- |
    /// | `id`   | i64   | 文集 rlid (必要)  |
    ///
    /// # 文档
    /// [获取文集基本信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/article/articles.md#获取文集基本信息)
    pub async fn article_articles_info(
        &self,
        id: i64,
    ) -> Result<BpiResponse<ArticlesData>, BpiError> {
        self.get("https://api.bilibili.com/x/article/list/web/articles")
            .query(&[("id", id.to_string())])
            .send_bpi("获取文集基本信息")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_articles_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let rlid = 207146;

        let result = bpi.article_articles_info(rlid).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert!(!data.list.name.is_empty());
        assert!(!data.articles.is_empty());
        assert!(!data.author.name.is_empty());

        Ok(())
    }
}
