use crate::{BpiError, BpiResult};

/// 搜索目标类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchType {
    Video,
    MediaBangumi,
    MediaFt,
    Live,
    LiveRoom,
    LiveUser,
    Article,
    BiliUser,
}

impl SearchType {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchType::Video => "video",
            SearchType::MediaBangumi => "media_bangumi",
            SearchType::MediaFt => "media_ft",
            SearchType::Live => "live",
            SearchType::LiveRoom => "live_room",
            SearchType::LiveUser => "live_user",
            SearchType::Article => "article",
            SearchType::BiliUser => "bili_user",
        }
    }
}

/// 搜索结果排序
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchOrder {
    // 视频、专栏、相簿
    TotalRank,
    Click,
    PubDate,
    Dm,
    Stow,
    Scores,
    Attention, // 专栏
    // 直播
    Online,
    LiveTime,
    // 用户
    Default,
    Fans,
    Level,
}

impl SearchOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchOrder::TotalRank => "totalrank",
            SearchOrder::Click => "click",
            SearchOrder::PubDate => "pubdate",
            SearchOrder::Dm => "dm",
            SearchOrder::Stow => "stow",
            SearchOrder::Scores => "scores",
            SearchOrder::Attention => "attention",
            SearchOrder::Online => "online",
            SearchOrder::LiveTime => "live_time",
            SearchOrder::Default => "0",
            SearchOrder::Fans => "fans",
            SearchOrder::Level => "level",
        }
    }
}

/// 用户粉丝数及等级排序顺序
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSort {
    Descending, // 由高到低
    Ascending,  // 由低到高
}

impl OrderSort {
    pub fn as_num(&self) -> u8 {
        match self {
            OrderSort::Descending => 0,
            OrderSort::Ascending => 1,
        }
    }
}

/// 用户分类筛选
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserType {
    All,
    Up,
    Normal,
    Verified,
}

impl UserType {
    pub fn as_num(&self) -> u8 {
        match self {
            UserType::All => 0,
            UserType::Up => 1,
            UserType::Normal => 2,
            UserType::Verified => 3,
        }
    }
}

/// 视频时长筛选
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Duration {
    All,
    Under10,
    From10To30,
    From30To60,
    Over60,
}

impl Duration {
    pub fn as_num(&self) -> u8 {
        match self {
            Duration::All => 0,
            Duration::Under10 => 1,
            Duration::From10To30 => 2,
            Duration::From30To60 => 3,
            Duration::Over60 => 4,
        }
    }
}

/// 专栏及相簿分区筛选
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CategoryId {
    All,
    Animation,
    Game,
    Movie,
    Life,
    Interest,
    LightNovel,
    Technology,
    Huayou,      // 相簿画友
    Photography, // 相簿摄影
}

impl CategoryId {
    pub fn as_num(&self) -> u8 {
        match self {
            CategoryId::All => 0,
            CategoryId::Animation => 2,
            CategoryId::Game => 1,
            CategoryId::Movie => 28,
            CategoryId::Life => 3,
            CategoryId::Interest => 29,
            CategoryId::LightNovel => 16,
            CategoryId::Technology => 17,
            CategoryId::Huayou => 1,
            CategoryId::Photography => 2,
        }
    }
}

/// Parameters for video search.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchVideoParams {
    keyword: String,
    order: SearchOrder,
    duration: Duration,
    tids: u32,
    page: u32,
}

impl SearchVideoParams {
    pub fn new(keyword: impl Into<String>) -> BpiResult<Self> {
        let keyword = keyword.into().trim().to_string();
        if keyword.is_empty() {
            return Err(BpiError::invalid_parameter(
                "keyword",
                "search keyword cannot be blank",
            ));
        }

        Ok(Self {
            keyword,
            order: SearchOrder::TotalRank,
            duration: Duration::All,
            tids: 0,
            page: 1,
        })
    }

    pub fn with_order(mut self, order: SearchOrder) -> Self {
        self.order = order;
        self
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_tid(mut self, tid: u32) -> Self {
        self.tids = tid;
        self
    }

    pub fn with_page(mut self, page: u32) -> BpiResult<Self> {
        if page == 0 {
            return Err(BpiError::invalid_parameter(
                "page",
                "page number must be at least 1",
            ));
        }

        self.page = page;
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("search_type", SearchType::Video.as_str().to_string()),
            ("keyword", self.keyword.clone()),
            ("order", self.order.as_str().to_string()),
            ("duration", self.duration.as_num().to_string()),
            ("tids", self.tids.to_string()),
            ("page", self.page.to_string()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_video_params_serializes_default_query() -> Result<(), BpiError> {
        let params = SearchVideoParams::new("rust")?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("search_type", "video".to_string()),
                ("keyword", "rust".to_string()),
                ("order", "totalrank".to_string()),
                ("duration", "0".to_string()),
                ("tids", "0".to_string()),
                ("page", "1".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn search_video_params_serializes_optional_filters() -> Result<(), BpiError> {
        let params = SearchVideoParams::new("  rust 教程  ")?
            .with_order(SearchOrder::Online)
            .with_duration(Duration::From10To30)
            .with_tid(171)
            .with_page(2)?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("search_type", "video".to_string()),
                ("keyword", "rust 教程".to_string()),
                ("order", "online".to_string()),
                ("duration", "2".to_string()),
                ("tids", "171".to_string()),
                ("page", "2".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn search_video_params_rejects_blank_keyword() {
        let err = SearchVideoParams::new(" \t ").unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "keyword",
                ..
            }
        ));
    }

    #[test]
    fn search_video_params_rejects_zero_page() -> Result<(), BpiError> {
        let err = SearchVideoParams::new("rust")?.with_page(0).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "page", .. }
        ));
        Ok(())
    }
}
