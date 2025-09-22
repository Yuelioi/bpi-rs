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
