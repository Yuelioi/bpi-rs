//! 搜索
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/search/search_request.md)

use super::result::{
    Article, Bangumi, BiliUser, LiveData, LiveRoom, LiveUser, Movie, SearchData, Video,
};
use super::search_params::{
    SearchArticleParams, SearchBangumiParams, SearchBiliUserParams, SearchLiveParams,
    SearchLiveRoomParams, SearchLiveUserParams, SearchMovieParams, SearchVideoParams,
};
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

impl BpiClient {
    /// 搜索专栏
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `SearchArticleParams` | 专栏搜索参数 |
    pub async fn search_article(
        &self,
        params: SearchArticleParams,
    ) -> Result<BpiResponse<SearchData<Vec<Article>>>, BpiError> {
        let signed_params = self.get_wbi_sign2(params.query_pairs()).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .with_bilibili_headers()
            .query(&signed_params)
            .send_bpi("搜索专栏")
            .await
    }

    /// 搜索番剧
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `SearchBangumiParams` | 番剧搜索参数 |
    pub async fn search_bangumi(
        &self,
        params: SearchBangumiParams,
    ) -> Result<BpiResponse<SearchData<Vec<Bangumi>>>, BpiError> {
        let signed_params = self.get_wbi_sign2(params.query_pairs()).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索番剧")
            .await
    }

    /// 搜索用户
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `SearchBiliUserParams` | 用户搜索参数 |
    pub async fn search_bili_user(
        &self,
        params: SearchBiliUserParams,
    ) -> Result<BpiResponse<SearchData<Vec<BiliUser>>>, BpiError> {
        let signed_params = self.get_wbi_sign2(params.query_pairs()).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索用户")
            .await
    }

    /// 搜索直播间及主播
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `SearchLiveParams` | 直播综合搜索参数 |
    pub async fn search_live(
        &self,
        params: SearchLiveParams,
    ) -> Result<BpiResponse<SearchData<LiveData>>, BpiError> {
        let signed_params = self.get_wbi_sign2(params.query_pairs()).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索直播间及主播")
            .await
    }

    /// 搜索直播间
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `SearchLiveRoomParams` | 直播间搜索参数 |
    pub async fn search_live_room(
        &self,
        params: SearchLiveRoomParams,
    ) -> Result<BpiResponse<SearchData<Vec<LiveRoom>>>, BpiError> {
        let signed_params = self.get_wbi_sign2(params.query_pairs()).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索直播间")
            .await
    }

    /// 搜索主播
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `SearchLiveUserParams` | 主播搜索参数 |
    pub async fn search_live_user(
        &self,
        params: SearchLiveUserParams,
    ) -> Result<BpiResponse<SearchData<Vec<LiveUser>>>, BpiError> {
        let signed_params = self.get_wbi_sign2(params.query_pairs()).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索主播")
            .await
    }

    /// 搜索影视
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `SearchMovieParams` | 影视搜索参数 |
    pub async fn search_movie(
        &self,
        params: SearchMovieParams,
    ) -> Result<BpiResponse<SearchData<Vec<Movie>>>, BpiError> {
        let signed_params = self.get_wbi_sign2(params.query_pairs()).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索影视")
            .await
    }

    /// 搜索视频
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `SearchVideoParams` | 视频搜索参数 |
    pub async fn search_video(
        &self,
        params: SearchVideoParams,
    ) -> Result<BpiResponse<SearchData<Vec<Video>>>, BpiError> {
        let signed_params = self.get_wbi_sign2(params.query_pairs()).await?;

        self.get("https://api.bilibili.com/x/web-interface/wbi/search/type")
            .query(&signed_params)
            .send_bpi("搜索视频")
            .await
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::{
        CategoryId, Duration, OrderSort, SearchArticleParams, SearchBangumiParams,
        SearchBiliUserParams, SearchLiveRoomParams, SearchLiveUserParams, SearchMovieParams,
        SearchOrder, SearchVideoParams, UserType,
    };
    use tracing::info;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_search_article() {
        let bpi = BpiClient::new().expect("client should build");
        let params = SearchArticleParams::new("Rust")
            .expect("keyword should be valid")
            .with_order(SearchOrder::PubDate)
            .with_category_id(CategoryId::Technology);
        let resp = bpi.search_article(params).await;
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

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_search_bangumi() {
        let bpi = BpiClient::new().expect("client should build");
        let params = SearchBangumiParams::new("天气之子").expect("keyword should be valid");
        let resp = bpi.search_bangumi(params).await;
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

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_search_bili_user() {
        let bpi = BpiClient::new().expect("client should build");
        let params = SearchBiliUserParams::new("老番茄")
            .expect("keyword should be valid")
            .with_order_sort(OrderSort::Descending)
            .with_user_type(UserType::All);
        let resp = bpi.search_bili_user(params).await;
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

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_search_live_room() {
        let bpi = BpiClient::new().expect("client should build");
        let params = SearchLiveRoomParams::new("游戏").expect("keyword should be valid");
        let resp = bpi.search_live_room(params).await;
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

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_search_live_user() {
        let bpi = BpiClient::new().expect("client should build");
        let params = SearchLiveUserParams::new("散人")
            .expect("keyword should be valid")
            .with_order_sort(OrderSort::Descending)
            .with_user_type(UserType::All);
        let resp = bpi.search_live_user(params).await;
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

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_search_movie() {
        let bpi = BpiClient::new().expect("client should build");
        let params = SearchMovieParams::new("哈利波特").expect("keyword should be valid");
        let resp = bpi.search_movie(params).await;
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

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_search_video() {
        let bpi = BpiClient::new().expect("client should build");
        let params = SearchVideoParams::new("Rust 教程")
            .expect("keyword should be valid")
            .with_order(SearchOrder::Online)
            .with_duration(Duration::From10To30)
            .with_tid(171);
        let resp = bpi.search_video(params).await;
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
