//! 搜索
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/search/search_request.md

use super::result::{
    Article, Bangumi, BiliUser, LiveData, LiveRoom, LiveUser, Movie, SearchData, Video,
};
use super::search_params::{CategoryId, Duration, OrderSort, SearchOrder, SearchType, UserType};
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

impl BpiClient {
    /// 搜索专栏
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `keyword` | &str | 搜索关键词 |
    /// | `order` | Option<SearchOrder> | 排序方式 |
    /// | `category_id` | Option<CategoryId> | 专栏分区 |
    /// | `page` | Option<i32> | 页码（默认1） |
    pub async fn search_article(
        &self,
        keyword: &str,
        order: Option<SearchOrder>,
        category_id: Option<CategoryId>,
        page: Option<i32>,
    ) -> Result<BpiResponse<SearchData<Vec<Article>>>, BpiError> {
        let category_id_str = category_id.unwrap_or(CategoryId::All).as_num().to_string();
        let page_str = page.unwrap_or(1).to_string();

        let params = vec![
            ("search_type", SearchType::Article.as_str().to_string()),
            ("keyword", keyword.to_string()),
            (
                "order",
                order.unwrap_or(SearchOrder::TotalRank).as_str().to_string(),
            ),
            ("category_id", category_id_str),
            ("page", page_str),
        ];

        let signed_params = self.get_wbi_sign2(params).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .with_bilibili_headers()
            .query(&signed_params)
            .send_bpi("搜索专栏")
            .await
    }

    /// 搜索番剧
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `keyword` | &str | 搜索关键词 |
    /// | `page` | Option<i32> | 页码（默认1） |
    pub async fn search_bangumi(
        &self,
        keyword: &str,
        page: Option<i32>,
    ) -> Result<BpiResponse<SearchData<Vec<Bangumi>>>, BpiError> {
        let page_str = page.unwrap_or(1).to_string();

        let params = vec![
            ("search_type", SearchType::MediaBangumi.as_str().to_string()),
            ("keyword", keyword.to_string()),
            ("page", page_str),
        ];

        let signed_params = self.get_wbi_sign2(params).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索番剧")
            .await
    }

    /// 搜索用户
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `keyword` | &str | 搜索关键词 |
    /// | `order_sort` | Option<OrderSort> | 排序方向：降序/升序 |
    /// | `user_type` | Option<UserType> | 用户类型筛选 |
    /// | `page` | Option<i32> | 页码（默认1） |
    pub async fn search_bili_user(
        &self,
        keyword: &str,
        order_sort: Option<OrderSort>,
        user_type: Option<UserType>,
        page: Option<i32>,
    ) -> Result<BpiResponse<SearchData<Vec<BiliUser>>>, BpiError> {
        let page_str = page.unwrap_or(1).to_string();

        let params = vec![
            ("search_type", SearchType::BiliUser.as_str().to_string()),
            ("keyword", keyword.to_string()),
            (
                "order_sort",
                order_sort
                    .unwrap_or(OrderSort::Ascending)
                    .as_num()
                    .to_string(),
            ),
            (
                "user_type",
                user_type.unwrap_or(UserType::All).as_num().to_string(),
            ),
            ("page", page_str),
        ];

        let signed_params = self.get_wbi_sign2(params).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索用户")
            .await
    }

    /// 搜索直播间及主播
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `keyword` | &str | 搜索关键词 |
    /// | `page` | Option<i32> | 页码（默认1） |
    pub async fn search_live(
        &self,
        keyword: &str,
        page: Option<i32>,
    ) -> Result<BpiResponse<SearchData<LiveData>>, BpiError> {
        let page_str = page.unwrap_or(1).to_string();

        let params = vec![
            ("search_type", SearchType::Live.as_str().to_string()),
            ("keyword", keyword.to_string()),
            ("page", page_str),
        ];

        let signed_params = self.get_wbi_sign2(params).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索直播间及主播")
            .await
    }

    /// 搜索直播间
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `keyword` | &str | 搜索关键词 |
    /// | `order` | Option<SearchOrder> | 排序方式（默认 online） |
    /// | `page` | Option<i32> | 页码（默认1） |
    pub async fn search_live_room(
        &self,
        keyword: &str,
        order: Option<SearchOrder>,
        page: Option<i32>,
    ) -> Result<BpiResponse<SearchData<Vec<LiveRoom>>>, BpiError> {
        let page_str = page.unwrap_or(1).to_string();

        let params = vec![
            ("search_type", SearchType::LiveRoom.as_str().to_string()),
            ("keyword", keyword.to_string()),
            (
                "order",
                order.unwrap_or(SearchOrder::Online).as_str().to_string(),
            ),
            ("page", page_str),
        ];

        let signed_params = self.get_wbi_sign2(params).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索直播间")
            .await
    }

    /// 搜索主播
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `keyword` | &str | 搜索关键词 |
    /// | `order_sort` | Option<OrderSort> | 排序方向 |
    /// | `user_type` | Option<UserType> | 主播类型筛选 |
    /// | `page` | Option<i32> | 页码（默认1） |
    pub async fn search_live_user(
        &self,
        keyword: &str,
        order_sort: Option<OrderSort>,
        user_type: Option<UserType>,
        page: Option<i32>,
    ) -> Result<BpiResponse<SearchData<Vec<LiveUser>>>, BpiError> {
        let page_str = page.unwrap_or(1).to_string();

        let params = vec![
            ("search_type", SearchType::LiveUser.as_str().to_string()),
            ("keyword", keyword.to_string()),
            (
                "order_sort",
                order_sort
                    .unwrap_or(OrderSort::Ascending)
                    .as_num()
                    .to_string(),
            ),
            (
                "user_type",
                user_type.unwrap_or(UserType::All).as_num().to_string(),
            ),
            ("page", page_str),
        ];

        let signed_params = self.get_wbi_sign2(params).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索主播")
            .await
    }

    /// 搜索影视
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `keyword` | &str | 搜索关键词 |
    /// | `page` | Option<i32> | 页码（默认1） |
    pub async fn search_movie(
        &self,
        keyword: &str,
        page: Option<i32>,
    ) -> Result<BpiResponse<SearchData<Vec<Movie>>>, BpiError> {
        let page_str = page.unwrap_or(1).to_string();

        let params = vec![
            ("search_type", SearchType::MediaFt.as_str().to_string()),
            ("keyword", keyword.to_string()),
            ("page", page_str),
        ];

        let signed_params = self.get_wbi_sign2(params).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索影视")
            .await
    }

    /// 搜索视频
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `keyword` | &str | 搜索关键词 |
    /// | `order` | Option<SearchOrder> | 排序方式 |
    /// | `duration` | Option<Duration> | 时长筛选 |
    /// | `tids` | Option<u32> | 分区 ID |
    /// | `page` | Option<i32> | 页码（默认1） |
    pub async fn search_video(
        &self,
        keyword: &str,
        order: Option<SearchOrder>,
        duration: Option<Duration>,
        tids: Option<u32>,
        page: Option<i32>,
    ) -> Result<BpiResponse<SearchData<Vec<Video>>>, BpiError> {
        let page_str = page.unwrap_or(1).to_string();

        let params = vec![
            ("search_type", SearchType::Video.as_str().to_string()),
            ("keyword", keyword.to_string()),
            (
                "order",
                order.unwrap_or(SearchOrder::TotalRank).as_str().to_string(),
            ),
            (
                "duration",
                duration.unwrap_or(Duration::All).as_num().to_string(),
            ),
            ("tids", tids.unwrap_or(0).to_string()),
            ("page", page_str),
        ];

        let signed_params = self.get_wbi_sign2(params).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索视频")
            .await
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::search_params::{CategoryId, Duration, OrderSort, SearchOrder, UserType};
    use tracing::info;

    #[tokio::test]
    async fn test_search_article() {
        let bpi = BpiClient::new();
        let resp = bpi
            .search_article(
                "Rust",
                Some(SearchOrder::PubDate),
                Some(CategoryId::Technology),
                None,
            )
            .await;
        assert!(resp.is_ok());
        if let Ok(r) = resp {
            info!("搜索文章返回: {:?}", r);
            if let Some(data) = r.data {
                if let Some(results) = data.result {
                    assert!(!results.is_empty());
                    for article in results {
                        info!("文章标题: {}", article.title);
                    }
                } else {
                    info!("未找到任何文章结果。");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_search_bangumi() {
        let bpi = BpiClient::new();
        let resp = bpi.search_bangumi("天气之子", None).await;
        assert!(resp.is_ok());
        if let Ok(r) = resp {
            info!("搜索番剧返回: {:?}", r);
            if let Some(data) = r.data {
                if let Some(results) = data.result {
                    assert!(!results.is_empty());
                    for bangumi in results {
                        info!("番剧标题: {}", bangumi.title);
                    }
                } else {
                    info!("未找到任何番剧结果。");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_search_bili_user() {
        let bpi = BpiClient::new();
        let resp = bpi
            .search_bili_user(
                "老番茄",
                Some(OrderSort::Descending),
                Some(UserType::All),
                None,
            )
            .await;
        assert!(resp.is_ok());
        if let Ok(r) = resp {
            info!("搜索用户返回: {:?}", r);
            if let Some(data) = r.data {
                if let Some(results) = data.result {
                    assert!(!results.is_empty());
                    for user in results {
                        info!("用户昵称: {}", user.uname);
                    }
                } else {
                    info!("未找到任何用户结果。");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_search_live_room() {
        let bpi = BpiClient::new();
        let resp = bpi.search_live_room("游戏", None, None).await;
        assert!(resp.is_ok());
        if let Ok(r) = resp {
            info!("搜索直播间返回: {:?}", r);
            if let Some(data) = r.data {
                if let Some(results) = data.result {
                    assert!(!results.is_empty());
                    for room in results {
                        info!("直播间标题: {}", room.title);
                    }
                } else {
                    info!("未找到任何直播间结果。");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_search_live_user() {
        let bpi = BpiClient::new();
        let resp = bpi
            .search_live_user(
                "散人",
                Some(OrderSort::Descending),
                Some(UserType::All),
                None,
            )
            .await;
        assert!(resp.is_ok());
        if let Ok(r) = resp {
            info!("搜索主播返回: {:?}", r);
            if let Some(data) = r.data {
                if let Some(results) = data.result {
                    assert!(!results.is_empty());
                    for user in results {
                        info!("主播昵称: {}", user.uname);
                    }
                } else {
                    info!("未找到任何主播结果。");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_search_movie() {
        let bpi = BpiClient::new();
        let resp = bpi.search_movie("哈利波特", None).await;
        assert!(resp.is_ok());
        if let Ok(r) = resp {
            info!("搜索影视返回: {:?}", r);
            if let Some(data) = r.data {
                if let Some(results) = data.result {
                    assert!(!results.is_empty());
                    for movie in results {
                        info!("影视标题: {}", movie.title);
                    }
                } else {
                    info!("未找到任何影视结果。");
                }
            }
        }
    }

    #[tokio::test]
    async fn test_search_video() {
        let bpi = BpiClient::new();
        let resp = bpi
            .search_video(
                "Rust 教程",
                Some(SearchOrder::Online),
                Some(Duration::From10To30),
                Some(171),
                None,
            )
            .await;
        assert!(resp.is_ok());
        if let Ok(r) = resp {
            info!("搜索视频返回: {:?}", r);
            if let Some(data) = r.data {
                if let Some(results) = data.result {
                    assert!(!results.is_empty());
                    for video in results {
                        info!("视频标题: {}", video.title);
                    }
                } else {
                    info!("未找到任何视频结果。");
                }
            }
        }
    }
}
