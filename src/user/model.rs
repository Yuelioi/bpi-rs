use serde::{Deserialize, Deserializer, Serialize, de};

use crate::ids::{Aid, Bvid, Mid};

/// Payload returned by `/x/web-interface/card`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCardProfile {
    /// User card summary.
    pub card: UserCardSummary,
    /// Whether the current session follows this user.
    pub following: bool,
    /// Number of uploaded archives.
    #[serde(default)]
    pub archive_count: u64,
    /// Number of articles.
    #[serde(default)]
    pub article_count: u64,
    /// Follower count.
    #[serde(default)]
    pub follower: u64,
    /// Total likes received.
    #[serde(default)]
    pub like_num: u64,
}

/// Stable fields from the nested `card` object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCardSummary {
    /// User member ID.
    #[serde(deserialize_with = "deserialize_mid_from_string_or_number")]
    pub mid: Mid,
    /// Display name.
    pub name: String,
    /// Profile gender text.
    #[serde(default)]
    pub sex: Option<String>,
    /// Avatar URL.
    pub face: String,
    /// Profile signature.
    #[serde(default)]
    pub sign: String,
    /// Fan count embedded in the card summary.
    #[serde(default)]
    pub fans: u64,
    /// Following count embedded in the card summary.
    #[serde(default)]
    pub attention: u64,
}

/// Compact user card returned by `/account/v1/user/cards`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserBatchCard {
    /// User member ID.
    pub mid: Mid,
    /// Display name.
    pub name: String,
    /// Avatar URL.
    pub face: String,
    /// Profile signature.
    #[serde(default)]
    pub sign: String,
    /// Display rank returned by Bilibili.
    #[serde(default)]
    pub rank: i32,
    /// Current account level.
    #[serde(default)]
    pub level: i32,
    /// User silence/ban state returned by Bilibili.
    #[serde(default)]
    pub silence: i32,
}

/// Detailed batch user information returned by `/x/im/user_infos`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBatchInfo {
    /// User member ID.
    pub mid: Mid,
    /// Display name.
    pub name: String,
    /// Profile signature.
    #[serde(default)]
    pub sign: String,
    /// Display rank returned by Bilibili.
    #[serde(default)]
    pub rank: i32,
    /// Current account level.
    #[serde(default)]
    pub level: i32,
    /// User silence/ban state returned by Bilibili.
    #[serde(default)]
    pub silence: i32,
    /// Profile gender text.
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub sex: Option<String>,
    /// Avatar URL.
    pub face: String,
    /// VIP status summary.
    #[serde(default)]
    pub vip: Option<UserBatchVip>,
    /// Official verification summary.
    #[serde(default)]
    pub official: Option<UserOfficialSummary>,
    /// Whether Bilibili marks this account as fake.
    #[serde(default)]
    pub is_fake_account: Option<u32>,
    /// Expert payload returned by Bilibili, kept raw because the shape is display-oriented.
    #[serde(default)]
    pub expert_info: Option<serde_json::Value>,
}

/// VIP status fields embedded in `/x/im/user_infos` batch responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserBatchVip {
    /// VIP type code.
    #[serde(default, rename = "type")]
    pub kind: i32,
    /// VIP status code.
    #[serde(default)]
    pub status: i32,
    /// VIP due timestamp in milliseconds.
    #[serde(default)]
    pub due_date: i64,
    /// VIP payment type code.
    #[serde(default)]
    pub vip_pay_type: i32,
    /// VIP theme type code.
    #[serde(default)]
    pub theme_type: i32,
}

/// Public user space information returned by `/x/space/wbi/acc/info`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSpaceProfile {
    /// User member ID.
    pub mid: Mid,
    /// Display name.
    pub name: String,
    /// Profile gender text.
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub sex: Option<String>,
    /// Avatar URL.
    pub face: String,
    /// Profile signature.
    #[serde(default)]
    pub sign: String,
    /// Current account level.
    pub level: u8,
    /// User silence/ban state returned by Bilibili.
    pub silence: u8,
    /// Visible coin count. Bilibili returns `0` for other users.
    #[serde(default)]
    pub coins: f64,
    /// Whether this user has fan-medal information.
    #[serde(default)]
    pub fans_badge: bool,
    /// Whether the current session follows this user.
    #[serde(default)]
    pub is_followed: bool,
    /// Space top photo URL when Bilibili returns one.
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub top_photo: Option<String>,
    /// Official verification summary.
    #[serde(default)]
    pub official: Option<UserOfficialSummary>,
    /// VIP status summary.
    #[serde(default)]
    pub vip: Option<UserVipSummary>,
    /// Live-room summary.
    #[serde(default)]
    pub live_room: Option<UserSpaceLiveRoom>,
}

/// Official verification fields commonly returned in user space payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserOfficialSummary {
    /// Official role code.
    #[serde(default)]
    pub role: i32,
    /// Verification title.
    #[serde(default)]
    pub title: String,
    /// Verification description.
    #[serde(default)]
    pub desc: String,
    /// Verification type code.
    #[serde(default, rename = "type")]
    pub kind: i32,
}

/// VIP status fields commonly returned in user space payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserVipSummary {
    /// VIP type code.
    #[serde(default, rename = "type")]
    pub kind: i32,
    /// VIP status code.
    #[serde(default)]
    pub status: i32,
}

/// Live-room fields embedded in a user space payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSpaceLiveRoom {
    /// Whether a room exists for this user.
    #[serde(default, rename = "roomStatus")]
    pub room_status: u8,
    /// Whether the room is currently live.
    #[serde(default, rename = "liveStatus")]
    pub live_status: u8,
    /// Live room URL.
    #[serde(default)]
    pub url: String,
    /// Live room title.
    #[serde(default)]
    pub title: String,
    /// Live room ID.
    #[serde(default, rename = "roomid")]
    pub room_id: u64,
}

/// Public space notice returned by `/x/space/notice`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserSpaceNotice {
    /// Notice text. Empty when the user has no public notice.
    pub content: String,
}

/// Followed bangumi or cinema seasons returned by `/x/space/bangumi/follow/list`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBangumiFollowList {
    /// Followed seasons on the current page.
    #[serde(default, rename = "list")]
    pub items: Vec<UserBangumiFollow>,
    /// Current page number.
    #[serde(default, rename = "pn")]
    pub page: u32,
    /// Page size.
    #[serde(default, rename = "ps")]
    pub page_size: u32,
    /// Total followed seasons for this category.
    #[serde(default)]
    pub total: u64,
}

/// One followed bangumi or cinema season.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBangumiFollow {
    /// Season ID.
    pub season_id: i64,
    /// Media ID.
    pub media_id: i64,
    /// Season type code returned by Bilibili.
    #[serde(default)]
    pub season_type: i64,
    /// Season type display name.
    #[serde(default)]
    pub season_type_name: String,
    /// Season title.
    #[serde(default)]
    pub title: String,
    /// Cover image URL.
    #[serde(default)]
    pub cover: String,
    /// Total episode count.
    #[serde(default)]
    pub total_count: i64,
    /// Whether the season has finished.
    #[serde(default)]
    pub is_finish: i64,
    /// Whether the season has started.
    #[serde(default)]
    pub is_started: i64,
    /// Whether this season is playable.
    #[serde(default)]
    pub is_play: i64,
    /// Badge text returned by Bilibili.
    #[serde(default)]
    pub badge: String,
    /// Badge type code.
    #[serde(default)]
    pub badge_type: i64,
    /// Latest episode summary.
    #[serde(default, rename = "new_ep")]
    pub latest_episode: UserBangumiLatestEpisode,
    /// Season rating when Bilibili returns it.
    #[serde(default)]
    pub rating: Option<UserBangumiRating>,
    /// Season page URL.
    #[serde(default)]
    pub url: String,
    /// Short URL returned by Bilibili.
    #[serde(default)]
    pub short_url: String,
    /// Season description.
    #[serde(default)]
    pub summary: String,
    /// Style tags.
    #[serde(default)]
    pub styles: Vec<String>,
    /// Follow status code.
    #[serde(default)]
    pub follow_status: i64,
    /// User progress text.
    #[serde(default)]
    pub progress: String,
    /// Whether both users follow this season.
    #[serde(default)]
    pub both_follow: bool,
}

/// Latest episode summary embedded in a bangumi follow-list item.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserBangumiLatestEpisode {
    /// Episode ID.
    #[serde(default)]
    pub id: i64,
    /// Display episode index.
    #[serde(default)]
    pub index_show: String,
    /// Episode cover image URL.
    #[serde(default)]
    pub cover: String,
    /// Episode title.
    #[serde(default)]
    pub title: String,
    /// Episode long title.
    #[serde(default)]
    pub long_title: Option<String>,
    /// Publish time text returned by Bilibili.
    #[serde(default)]
    pub pub_time: String,
    /// Episode duration in seconds.
    #[serde(default)]
    pub duration: i64,
}

/// Rating summary embedded in a bangumi follow-list item.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct UserBangumiRating {
    /// Rating score.
    #[serde(default)]
    pub score: f64,
    /// Rating count.
    #[serde(default)]
    pub count: i64,
}

/// Relation counts returned by `/x/relation/stat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRelationStat {
    /// User member ID.
    pub mid: Mid,
    /// Public following count.
    pub following: u64,
    /// Whisper-following count.
    #[serde(default)]
    pub whisper: u64,
    /// Blacklist count.
    #[serde(default)]
    pub black: u64,
    /// Follower count.
    pub follower: u64,
}

/// Following-list payload returned by `/x/relation/followings`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFollowings {
    /// Users followed by the requested member.
    #[serde(default)]
    pub list: Vec<UserFollowing>,
    /// Bilibili relation-list schema version.
    #[serde(default)]
    pub re_version: u32,
    /// Total number of followed users.
    #[serde(default)]
    pub total: u64,
}

/// One followed user in [`UserFollowings`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFollowing {
    /// Followed member ID.
    pub mid: Mid,
    /// Relation attribute returned by Bilibili.
    #[serde(default)]
    pub attribute: u8,
    /// Follow timestamp in seconds.
    #[serde(default)]
    pub mtime: u64,
    /// Relation tag IDs.
    #[serde(default)]
    pub tag: Option<Vec<u64>>,
    /// Whether this user is specially followed.
    #[serde(default)]
    pub special: u8,
    /// Display name.
    #[serde(default, rename = "uname")]
    pub name: String,
    /// Avatar URL.
    #[serde(default)]
    pub face: String,
    /// Profile signature.
    #[serde(default)]
    pub sign: String,
    /// Whether Bilibili marks the avatar as NFT.
    #[serde(default)]
    pub face_nft: u8,
    /// Official verification summary returned by the relation list.
    #[serde(default)]
    pub official_verify: Option<UserRelationOfficialVerify>,
    /// VIP summary. Kept raw because Bilibili changes this display payload often.
    #[serde(default)]
    pub vip: Option<serde_json::Value>,
}

/// Follower-list payload returned by `/x/relation/fans`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFollowers {
    /// Users following the requested member.
    #[serde(default)]
    pub list: Vec<UserFollower>,
    /// Pagination offset to pass to the next request.
    #[serde(default)]
    pub offset: String,
    /// Bilibili relation-list schema version.
    #[serde(default)]
    pub re_version: u32,
    /// Total number of followers.
    #[serde(default)]
    pub total: u64,
}

/// One follower in [`UserFollowers`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFollower {
    /// Follower member ID.
    pub mid: Mid,
    /// Relation attribute returned by Bilibili.
    #[serde(default)]
    pub attribute: u8,
    /// Follow timestamp in seconds when Bilibili returns it.
    #[serde(default)]
    pub mtime: Option<u64>,
    /// Relation tag IDs.
    #[serde(default)]
    pub tag: Option<Vec<u64>>,
    /// Whether this user is specially followed.
    #[serde(default)]
    pub special: u8,
    /// Contract display payload. Kept raw because this nested schema is unstable.
    #[serde(default)]
    pub contract_info: Option<serde_json::Value>,
    /// Display name.
    #[serde(default, rename = "uname")]
    pub name: String,
    /// Avatar URL.
    #[serde(default)]
    pub face: String,
    /// Profile signature.
    #[serde(default)]
    pub sign: String,
    /// Whether Bilibili marks the avatar as NFT.
    #[serde(default)]
    pub face_nft: u8,
    /// Official verification summary returned by the relation list.
    #[serde(default)]
    pub official_verify: Option<UserRelationOfficialVerify>,
    /// VIP summary. Kept raw because Bilibili changes this display payload often.
    #[serde(default)]
    pub vip: Option<serde_json::Value>,
}

/// Follow-group entry returned by `/x/relation/tags`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserFollowTag {
    /// Follow group ID. Bilibili uses negative IDs for built-in groups.
    #[serde(rename = "tagid")]
    pub id: i64,
    /// Follow group display name.
    pub name: String,
    /// Number of users in the group.
    #[serde(default)]
    pub count: i64,
    /// Optional UI hint returned by Bilibili.
    #[serde(default)]
    pub tip: Option<String>,
}

/// Fan-medal wall returned by `/xlive/web-ucenter/user/MedalWall`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMedalWall {
    /// Medal entries visible on the user's medal wall.
    #[serde(default)]
    pub list: Vec<UserMedalWallItem>,
    /// Number of visible medal entries.
    #[serde(default)]
    pub count: u32,
    /// Whether this user closes the space medal wall.
    #[serde(default)]
    pub close_space_medal: u32,
    /// Whether this user only shows the wearing medal.
    #[serde(default)]
    pub only_show_wearing: u32,
    /// Medal wall owner display name.
    #[serde(default)]
    pub name: String,
    /// Medal wall owner avatar URL.
    #[serde(default)]
    pub icon: String,
    /// Medal wall owner member ID.
    pub uid: Mid,
    /// Medal wall owner level.
    #[serde(default)]
    pub level: u32,
}

/// One fan-medal entry in [`UserMedalWall`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMedalWallItem {
    /// Medal metadata for the target creator.
    pub medal_info: UserMedalInfo,
    /// Target creator display name.
    #[serde(default)]
    pub target_name: String,
    /// Target creator avatar URL.
    #[serde(default)]
    pub target_icon: String,
    /// Target live-room link.
    #[serde(default)]
    pub link: String,
    /// Current target live status.
    #[serde(default)]
    pub live_status: u32,
    /// Official status field. Bilibili spells this key as `offical`.
    #[serde(default, rename = "offical")]
    pub official: Option<u32>,
    /// Medal display fields from the wall owner's point of view.
    #[serde(default)]
    pub uinfo_medal: Option<UserMedalOwnerInfo>,
}

impl UserMedalWallItem {
    /// Returns the target creator member ID for this medal.
    pub fn target_id(&self) -> Mid {
        self.medal_info.target_id
    }
}

/// Stable fan-medal metadata for one target creator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMedalInfo {
    /// Target creator member ID.
    pub target_id: Mid,
    /// Medal level.
    #[serde(default)]
    pub level: u32,
    /// Medal display name.
    #[serde(default, rename = "medal_name")]
    pub name: String,
    /// Medal gradient start color.
    #[serde(default)]
    pub medal_color_start: u32,
    /// Medal gradient end color.
    #[serde(default)]
    pub medal_color_end: u32,
    /// Medal border color.
    #[serde(default)]
    pub medal_color_border: u32,
    /// Guard level associated with this medal.
    #[serde(default)]
    pub guard_level: u32,
    /// Whether this medal is currently worn.
    #[serde(default)]
    pub wearing_status: u32,
    /// Medal ID.
    #[serde(default)]
    pub medal_id: u64,
    /// Current intimacy score.
    #[serde(default)]
    pub intimacy: u64,
    /// Required intimacy score for the next level.
    #[serde(default)]
    pub next_intimacy: u64,
    /// Intimacy fed today.
    #[serde(default)]
    pub today_feed: u64,
    /// Daily intimacy feed limit.
    #[serde(default)]
    pub day_limit: u64,
    /// Guard icon URL when Bilibili returns it.
    #[serde(default)]
    pub guard_icon: Option<String>,
    /// Honor icon URL when Bilibili returns it.
    #[serde(default)]
    pub honor_icon: Option<String>,
}

/// Medal display fields from the wall owner's point of view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMedalOwnerInfo {
    /// Medal display name.
    #[serde(default)]
    pub name: String,
    /// Medal level.
    #[serde(default)]
    pub level: u32,
    /// Medal gradient start color.
    #[serde(default)]
    pub color_start: u32,
    /// Medal gradient end color.
    #[serde(default)]
    pub color_end: u32,
    /// Medal border color.
    #[serde(default)]
    pub color_border: u32,
    /// Medal text color.
    #[serde(default)]
    pub color: u32,
    /// Medal ID.
    #[serde(default)]
    pub id: u64,
    /// Medal type returned by Bilibili.
    #[serde(default)]
    pub typ: u32,
    /// Whether the medal is lit.
    #[serde(default)]
    pub is_light: u32,
    /// Target creator member ID.
    pub ruid: Mid,
    /// Guard level associated with this medal.
    #[serde(default)]
    pub guard_level: u32,
    /// Current intimacy score.
    #[serde(default)]
    pub score: u64,
    /// Guard icon URL when Bilibili returns it.
    #[serde(default)]
    pub guard_icon: Option<String>,
    /// Honor icon URL when Bilibili returns it.
    #[serde(default)]
    pub honor_icon: Option<String>,
    /// V2 medal start color token.
    #[serde(default)]
    pub v2_medal_color_start: Option<String>,
    /// V2 medal end color token.
    #[serde(default)]
    pub v2_medal_color_end: Option<String>,
    /// V2 medal border color token.
    #[serde(default)]
    pub v2_medal_color_border: Option<String>,
    /// V2 medal text color token.
    #[serde(default)]
    pub v2_medal_color_text: Option<String>,
    /// V2 medal level color token.
    #[serde(default)]
    pub v2_medal_color_level: Option<String>,
    /// Number of users who received this medal when Bilibili returns it.
    #[serde(default)]
    pub user_receive_count: Option<u32>,
}

/// Official verification fields embedded in relation-list payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRelationOfficialVerify {
    /// Verification type code.
    #[serde(default, rename = "type")]
    pub kind: i8,
    /// Verification description.
    #[serde(default)]
    pub desc: String,
}

/// Creator content statistics returned by `/x/space/upstat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserUpStat {
    /// Uploaded-video view summary.
    pub archive: UserUpStatArchive,
    /// Article view summary.
    pub article: UserUpStatArticle,
    /// Total likes received by this user.
    pub likes: u64,
}

/// Uploaded-video view summary in [`UserUpStat`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserUpStatArchive {
    /// Total video views.
    #[serde(default)]
    pub view: u64,
}

/// Article view summary in [`UserUpStat`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserUpStatArticle {
    /// Total article views.
    #[serde(default)]
    pub view: u64,
}

/// User space navigation counters returned by `/x/space/navnum`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserNavStat {
    /// Uploaded video count.
    #[serde(default)]
    pub video: u64,
    /// Followed bangumi count.
    #[serde(default)]
    pub bangumi: u64,
    /// Followed cinema count.
    #[serde(default)]
    pub cinema: u64,
    /// Video-list counters.
    #[serde(default)]
    pub channel: UserNavStatPair,
    /// Favorite-folder counters.
    #[serde(default)]
    pub favourite: UserNavStatPair,
    /// Follow tag count.
    #[serde(default)]
    pub tag: u64,
    /// Article count.
    #[serde(default)]
    pub article: u64,
    /// Playlist count.
    #[serde(default)]
    pub playlist: u64,
    /// Album count.
    #[serde(default)]
    pub album: u64,
    /// Audio count.
    #[serde(default)]
    pub audio: u64,
    /// Course count.
    #[serde(default)]
    pub pugv: u64,
    /// Opus count.
    #[serde(default)]
    pub opus: u64,
    /// Video season/collection count.
    #[serde(default, rename = "season_num")]
    pub season_count: u64,
}

/// Master/guest split counters used by [`UserNavStat`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserNavStatPair {
    /// Total count visible to the owner.
    #[serde(default)]
    pub master: u64,
    /// Public count visible to visitors.
    #[serde(default)]
    pub guest: u64,
}

/// Album submission counters returned by `/link_draw/v1/doc/upload_count`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserAlbumCount {
    /// Total album submissions.
    #[serde(default)]
    pub all_count: u64,
    /// Drawing submissions.
    #[serde(default)]
    pub draw_count: u64,
    /// Photo submissions.
    #[serde(default)]
    pub photo_count: u64,
    /// Daily image-dynamic submissions.
    #[serde(default)]
    pub daily_count: u64,
}

/// Username lookup payload returned by `/x/polymer/web-dynamic/v1/name-to-uid`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserNameToUid {
    /// Matched username and member-ID entries.
    #[serde(default)]
    pub uid_list: Vec<UserNameToUidItem>,
}

/// One username lookup result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserNameToUidItem {
    /// Display name that matched the lookup.
    pub name: String,
    /// Matched member ID.
    #[serde(
        rename = "uid",
        deserialize_with = "deserialize_mid_from_string_or_number"
    )]
    pub mid: Mid,
}

/// Uploaded videos returned by `/x/space/wbi/arc/search`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUploadedVideos {
    /// Uploaded video list and partition summary.
    pub list: UserUploadedVideoList,
    /// Pagination summary.
    pub page: UserUploadedVideosPage,
    /// Optional play-all button returned by Bilibili.
    #[serde(default)]
    pub episodic_button: Option<UserUploadedVideosButton>,
    /// Whether the response was risk-controlled by Bilibili.
    #[serde(default)]
    pub is_risk: bool,
}

/// Uploaded video list payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUploadedVideoList {
    /// Partition summary keyed by partition ID.
    #[serde(default)]
    pub tlist: serde_json::Value,
    /// Uploaded videos on the current page.
    #[serde(default, rename = "vlist")]
    pub videos: Vec<UserUploadedVideo>,
}

/// One uploaded video in a user's space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUploadedVideo {
    /// AV numeric video ID.
    pub aid: Aid,
    /// BV string video ID.
    pub bvid: Bvid,
    /// Uploader member ID.
    pub mid: Mid,
    /// Video title.
    pub title: String,
    /// Uploader display name.
    #[serde(default)]
    pub author: String,
    /// Cover image URL.
    #[serde(default)]
    pub pic: String,
    /// Video duration text returned by Bilibili.
    #[serde(default)]
    pub length: String,
    /// Video description.
    #[serde(default)]
    pub description: String,
    /// Publish timestamp in seconds.
    #[serde(default)]
    pub created: u64,
    /// View count.
    #[serde(default)]
    pub play: u64,
    /// Reply count.
    #[serde(default)]
    pub comment: u64,
    /// Partition ID.
    #[serde(default)]
    pub typeid: u64,
    /// Danmaku count.
    #[serde(default)]
    pub video_review: u64,
    /// Whether Bilibili hides click details for this video.
    #[serde(default)]
    pub hide_click: bool,
}

/// Pagination summary for uploaded videos.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserUploadedVideosPage {
    /// Total matched videos.
    #[serde(default)]
    pub count: u64,
    /// Current page number.
    #[serde(default)]
    pub pn: u32,
    /// Page size.
    #[serde(default)]
    pub ps: u32,
}

/// Optional play-all button returned by uploaded-video searches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserUploadedVideosButton {
    /// Button text.
    #[serde(default)]
    pub text: String,
    /// Button target URI.
    #[serde(default)]
    pub uri: String,
}

fn deserialize_mid_from_string_or_number<'de, D>(deserializer: D) -> Result<Mid, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        serde_json::Value::Number(number) => {
            let mid = number
                .as_u64()
                .ok_or_else(|| de::Error::custom("mid must be a non-negative integer"))?;
            Mid::new(mid).map_err(de::Error::custom)
        }
        serde_json::Value::String(text) => text.parse::<Mid>().map_err(de::Error::custom),
        _ => Err(de::Error::custom("mid must be a string or number")),
    }
}

fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?
        .and_then(|value| (!value.trim().is_empty()).then_some(value)))
}
