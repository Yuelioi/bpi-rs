//! B站视频合集信息相关接口
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 稿件信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArchiveStat {
    /// 稿件播放量
    pub view: u64,
    /// vt
    pub vt: Option<u64>,
}

/// 合集/系列中的视频信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Archive {
    /// 稿件 avid
    pub aid: u64,
    /// 稿件 bvid
    pub bvid: String,
    /// 创建时间Unix 时间戳
    pub ctime: u64,
    /// 视频时长，单位为秒
    pub duration: u64,
    /// 是否是互动视频
    pub interactive_video: bool,
    /// 封面 URL
    pub pic: String,
    /// 会随着播放时间增长，播放完成后为 -1。单位为 %
    pub playback_position: u64,
    /// 发布日期Unix 时间戳
    pub pubdate: u64,
    /// 稿件信息
    pub stat: ArchiveStat,
    /// state
    pub state: u64,
    /// 稿件标题
    pub title: String,
    /// UGC 付费? 0: 否
    pub ugc_pay: u64,
    /// vt_display
    pub vt_display: String,

    pub is_lesson_video: Option<u32>,
}

/// 分页信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageInfo {
    /// 分页页码
    #[serde(alias = "num")]
    pub page_num: u64,
    /// 单页个数
    #[serde(alias = "size")]
    pub page_size: u64,
    /// 总页数/总数量
    pub total: u64,
}

/// 合集元数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeasonsArchivesMeta {
    /// category
    pub category: u64,
    /// 合集封面 URL
    pub cover: String,
    /// 合集描述
    pub description: String,
    /// UP 主 ID
    pub mid: u64,
    /// 合集标题
    pub name: String,
    /// 发布时间Unix 时间戳
    pub ptime: u64,
    /// 合集 ID
    pub season_id: u64,
    /// 合集内视频数量
    pub total: u64,
}

/// 获取视频合集信息响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSeasonsArchivesData {
    /// 稿件 avid 列表
    pub aids: Vec<u64>,
    /// 合集中的视频
    pub archives: Vec<Archive>,
    /// 合集元数据
    pub meta: SeasonsArchivesMeta,
    /// 分页信息
    pub page: PageInfo,
}

/// 合集元数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeasonsMeta {
    /// category
    pub category: u64,
    /// 封面 URL
    pub cover: String,
    /// 描述
    pub description: String,
    /// UP 主 ID
    pub mid: u64,
    /// 标题
    pub name: String,
    /// 创建时间?
    pub ptime: u64,
    /// 合集 ID
    pub season_id: u64,
    /// 视频数量
    pub total: u64,
}

/// 系列元数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeriesMeta {
    pub category: u64,
    pub creator: String,
    pub ctime: u64,
    pub description: String,
    pub keywords: Vec<String>,
    pub last_update_ts: u64,
    pub mid: u64,
    pub mtime: u64,
    pub name: String,
    pub raw_keywords: String,
    pub series_id: u64,
    pub state: u64,
    pub total: u64,
    pub cover: Option<String>,
}
/// 合集列表中的单个合集信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeasonsItem {
    /// 系列视频列表
    pub archives: Vec<Archive>,
    /// 系列元数据
    pub meta: SeasonsMeta,
    /// 系列视频 aid 列表
    pub recent_aids: Vec<u64>,
}
/// 系列列表中的单个系列信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SeriesItem {
    /// 系列视频列表
    pub archives: Vec<Archive>,
    /// 系列元数据
    pub meta: SeriesMeta,
    /// 系列视频 aid 列表
    pub recent_aids: Vec<u64>,
}

/// 系列和合集列表信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemsList {
    /// 分页信息
    pub page: PageInfo,
    /// 合集列表
    pub seasons_list: Vec<SeasonsItem>,
    /// 系列列表
    pub series_list: Vec<SeriesItem>,
}

/// 获取系列视频列表响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSeasonsSeriesData {
    /// 内容列表
    pub items_lists: ItemsList,
}

/// 查询指定系列响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSeriesData {
    /// 系列信息
    pub meta: SeriesMeta,
    /// 系列 aid 列表
    pub recent_aids: Vec<u64>,
}

/// 获取指定系列视频响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetSeriesArchivesData {
    /// 视频 aid 列表
    pub aids: Vec<u64>,
    /// 页码信息
    pub page: PageInfo,
    /// 视频信息列表
    pub archives: Vec<Archive>,
}

impl BpiClient {
    /// 获取视频合集信息
    ///
    /// 此接口用于获取特定UP主某个视频合集的详细信息，包括合集内的所有视频列表和元数据。
    ///
    /// # 参数
    /// * `mid` - 用户 mid，必填。
    /// * `season_id` - 视频合集 ID，必填。
    /// * `sort_reverse` - 排序方式，可选。`true`: 升序排序，`false`: 默认排序。
    /// * `page_num` - 页码索引，可选，默认为 1。
    /// * `page_size` - 单页内容数量，可选，默认为 30。
    pub async fn video_seasons_list(
        &self,
        mid: u64,
        season_id: u64,
        sort_reverse: Option<bool>,
        page_num: Option<u64>,
        page_size: Option<u64>
    ) -> Result<BpiResponse<GetSeasonsArchivesData>, BpiError> {
        let mut params = vec![
            ("mid", mid.to_string()),
            ("season_id", season_id.to_string()),
            // 默认分页参数
            ("page_num", "1".to_string()),
            ("page_size", "20".to_string())
        ];

        if let Some(sort) = sort_reverse {
            params.push(("sort_reverse", sort.to_string()));
        }
        if let Some(num) = page_num {
            params.push(("page_num", num.to_string()));
        }
        if let Some(size) = page_size {
            params.push(("page_size", size.to_string()));
        }

        // 签名
        let params = self.get_wbi_sign2(params).await?;

        self
            .get("https://api.bilibili.com/x/polymer/web-space/seasons_archives_list")
            .with_bilibili_headers()
            .query(&params)
            .send_bpi("获取视频合集信息").await
    }

    /// 只获取系列视频列表
    ///
    /// 此接口用于获取特定UP主创建的系列视频列表。
    ///
    /// # 参数
    /// * `mid` - 用户 mid，必填。
    /// * `page_num` - 页码索引，必填。
    /// * `page_size` - 单页内容数量，必填。
    pub async fn video_series_list(
        &self,
        mid: u64,
        page_num: u64,
        page_size: u64
    ) -> Result<BpiResponse<GetSeasonsSeriesData>, BpiError> {
        let params = vec![
            ("mid", mid.to_string()),
            ("page_num", page_num.to_string()),
            ("page_size", page_size.to_string())
        ];

        // 签名
        let params = self.get_wbi_sign2(params).await?;

        self
            .get("https://api.bilibili.com/x/polymer/web-space/home/seasons_series")
            .query(&params)
            .send_bpi("只获取系列视频列表").await
    }

    /// 获取系列和合集视频列表
    ///
    /// 此接口用于获取特定UP主创建的系列和合集视频列表，返回结果包含两种类型。
    ///
    /// # 参数
    /// * `mid` - 用户 mid，必填。
    /// * `page_num` - 页码索引，可选，默认为 1。
    /// * `page_size` - 每页数量，可选，默认为 20。
    pub async fn video_seasons_series_list(
        &self,
        mid: u64,
        page_num: Option<u64>,
        page_size: Option<u64>
    ) -> Result<BpiResponse<GetSeasonsSeriesData>, BpiError> {
        let mut params = vec![("mid", mid.to_string())];

        if let Some(num) = page_num {
            params.push(("page_num", num.to_string()));
        }
        if let Some(size) = page_size {
            params.push(("page_size", size.to_string()));
        }

        // 签名
        let params = self.get_wbi_sign2(params).await?;

        self
            .get("https://api.bilibili.com/x/polymer/web-space/seasons_series_list")
            .query(&params)
            .send_bpi("获取系列和合集视频列表").await
    }

    /// 查询指定系列信息
    ///
    /// 此接口用于获取指定系列的基本信息，如名称、描述、总视频数量等。
    ///
    /// # 参数
    /// * `series_id` - 系列ID，必填。
    pub async fn video_series_info(
        &self,
        series_id: u64
    ) -> Result<BpiResponse<GetSeriesData>, BpiError> {
        let req = self
            .get("https://api.bilibili.com/x/series/series")
            .query(&[("series_id", &series_id.to_string())]);

        req.send_bpi("查询指定系列信息").await
    }

    /// 获取指定系列视频列表
    ///
    /// 此接口用于获取指定系列内的所有视频列表，支持分页和排序。
    ///
    /// # 参数
    /// * `mid` - 用户 mid，必填。
    /// * `series_id` - 系列ID，必填。
    /// * `only_normal` - 作用尚不明确，可选，默认为 true。
    /// * `sort` - 排序方式，可选。`desc`: 默认排序，`asc`: 升序排序。
    /// * `page_num` - 页码索引，可选，默认为 1。
    /// * `page_size` - 每页数量，可选，默认为 20。
    pub async fn video_series_archives(
        &self,
        mid: u64,
        series_id: u64,
        only_normal: Option<bool>,
        sort: Option<&str>,
        page_num: Option<u64>,
        page_size: Option<u64>
    ) -> Result<BpiResponse<GetSeriesArchivesData>, BpiError> {
        let mut req = self.get("https://api.bilibili.com/x/series/archives").query(
            &[
                ("mid", &mid.to_string()),
                ("series_id", &series_id.to_string()),
            ]
        );

        if let Some(normal) = only_normal {
            req = req.query(&[("only_normal", &normal.to_string())]);
        }

        if let Some(s) = sort {
            req = req.query(&[("sort", s)]);
        }

        if let Some(num) = page_num {
            req = req.query(&[("pn", &num.to_string())]);
        }

        if let Some(size) = page_size {
            req = req.query(&[("ps", &size.to_string())]);
        }

        req.send_bpi("获取指定系列视频列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    // 测试用的 mid
    const TEST_MID: u64 = 4279370;
    // 测试用的合集 ID
    const TEST_SEASON_ID: u64 = 4294056;

    const TEST_SERIES_ID: u64 = 250285;

    #[tokio::test]
    async fn test_video_seasons_archives_list() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.video_seasons_list(TEST_MID, TEST_SEASON_ID, Some(false), None, None).await?;
        let data = resp.into_data()?;

        info!("测试结果: {:?}", data);
        assert!(!data.archives.is_empty(), "返回的合集视频列表不应为空");
        assert_eq!(data.meta.season_id, TEST_SEASON_ID, "合集ID应与请求ID一致");
        Ok(())
    }

    #[tokio::test]
    async fn test_video_seasons_series_only() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.video_series_list(TEST_MID, 1, 10).await?;
        let data = resp.into_data()?;

        info!("测试结果: {:?}", data);

        Ok(())
    }

    #[tokio::test]
    async fn test_video_seasons_series_list() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.video_seasons_series_list(TEST_MID, Some(1), Some(5)).await?;
        let data = resp.into_data()?;

        info!("测试结果: {:?}", data);
        assert!(!data.items_lists.series_list.is_empty(), "返回的系列列表不应为空");
        // 注意：合集列表可能为空，无法直接断言不为空
        assert_eq!(data.items_lists.page.page_size, 5, "返回的每页数量应为5");
        Ok(())
    }

    #[tokio::test]
    async fn test_video_series_info() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.video_series_info(TEST_SERIES_ID).await?;
        let data = resp.into_data()?;

        info!("测试结果: {:?}", data);
        assert_eq!(data.meta.series_id, TEST_SERIES_ID, "返回的系列ID应与请求ID一致");
        assert!(!data.recent_aids.is_empty(), "最近的aid列表不应为空");
        Ok(())
    }

    #[tokio::test]
    async fn test_video_series_archives() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.video_series_archives(
            TEST_MID,
            TEST_SERIES_ID,
            None,
            Some("asc"),
            Some(1),
            Some(10)
        ).await?;
        let data = resp.into_data()?;

        info!("测试结果: {:?}", data);
        assert!(!data.archives.is_empty(), "返回的系列视频列表不应为空");
        Ok(())
    }
}
