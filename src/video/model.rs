use serde::{Deserialize, Serialize};

use crate::ids::{Aid, Bvid, Cid, Mid};

/// Payload returned by `/x/web-interface/view`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoView {
    /// AV numeric video ID.
    pub aid: Aid,
    /// BV string video ID.
    pub bvid: Bvid,
    /// Number of video pages.
    pub videos: u32,
    /// Video title.
    pub title: String,
    /// Uploader information.
    pub owner: VideoOwner,
    /// Video statistics.
    pub stat: VideoStat,
    /// Default content/page ID.
    pub cid: Cid,
    /// Page list.
    #[serde(default)]
    pub pages: Vec<VideoPage>,
}

/// Payload returned by `/x/web-interface/view/detail`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDetail {
    /// Main video view payload.
    #[serde(rename = "View")]
    pub view: VideoView,
    /// Tags associated with the video.
    #[serde(default, rename = "Tags")]
    pub tags: Vec<VideoTag>,
    /// Related videos returned by Bilibili's recommendation surface.
    #[serde(default, rename = "Related")]
    pub related: Vec<VideoRelated>,
    /// Uploader card and space data. Kept raw because this is a display-oriented payload.
    #[serde(default, rename = "Card")]
    pub card: Option<serde_json::Value>,
    /// Reply preview data. Kept raw because it changes with comment surface experiments.
    #[serde(default, rename = "Reply")]
    pub reply: Option<serde_json::Value>,
}

/// Uploader information embedded in a video view payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoOwner {
    /// Uploader member ID.
    pub mid: Mid,
    /// Uploader display name.
    pub name: String,
    /// Uploader avatar URL.
    pub face: String,
}

/// Stable statistic fields returned by the video view endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStat {
    /// AV numeric video ID.
    pub aid: Aid,
    /// View count.
    pub view: u64,
    /// Danmaku count.
    pub danmaku: u64,
    /// Reply count.
    pub reply: u64,
    /// Favorite count on newer payloads.
    #[serde(default)]
    pub favorite: Option<u64>,
    /// Favorite count alias observed on some payloads.
    #[serde(default)]
    pub fav: Option<u64>,
    /// Coin count.
    pub coin: u64,
    /// Share count.
    pub share: u64,
    /// Like count.
    pub like: u64,
}

/// One page in a multi-part video.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoPage {
    /// Content/page ID.
    pub cid: Cid,
    /// 1-based page index.
    pub page: u32,
    /// Page title.
    pub part: String,
    /// Duration in seconds.
    pub duration: u64,
}

/// One video tag returned by the detail endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoTag {
    /// Tag ID.
    pub tag_id: u64,
    /// Tag display name.
    #[serde(default)]
    pub tag_name: String,
    /// Optional tag jump URL.
    #[serde(default)]
    pub jump_url: String,
}

/// Stable fields for a related video returned by the detail endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoRelated {
    /// AV numeric video ID.
    pub aid: Aid,
    /// BV string video ID.
    pub bvid: Bvid,
    /// Related video title.
    #[serde(default)]
    pub title: String,
    /// Default content/page ID when Bilibili returns it.
    #[serde(default)]
    pub cid: Option<Cid>,
    /// Uploader information when present.
    #[serde(default)]
    pub owner: Option<VideoOwner>,
    /// Video statistics when present.
    #[serde(default)]
    pub stat: Option<VideoStat>,
}
