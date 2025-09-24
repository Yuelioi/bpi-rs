//! 获取视频详细信息 (Web端)
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)

use crate::models::Account;
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 视频分辨率信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
    pub rotate: u8,
}

/// 视频状态统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    pub aid: u64,
    pub view: u64,
    pub danmaku: u64,
    pub reply: u64,
    #[serde(rename = "favorite")]
    pub favorite: u64,
    #[serde(rename = "fav")]
    pub fav: Option<u64>,
    pub coin: u64,
    pub share: u64,
    pub now_rank: i64,
    pub his_rank: i64,
    pub like: u64,
    pub dislike: u64,
    pub evaluation: String,
    #[serde(rename = "argue_msg")]
    pub argue_msg: Option<String>,
    pub vt: u64,
    pub vv: Option<u64>,
}

/// 视频争议/警告信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgueInfo {
    pub argue_link: String,
    pub argue_msg: String,
    pub argue_type: i32,
}

/// UP主信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub mid: u64,
    pub name: String,
    pub face: String,
}

/// rights对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rights {
    pub bp: u8,
    pub elec: u8,
    pub download: u8,
    pub movie: u8,
    pub pay: u8,
    pub hd5: u8,
    pub no_reprint: u8,
    pub autoplay: u8,
    pub ugc_pay: u8,
    pub is_cooperation: u8,
    pub ugc_pay_preview: u8,
    pub no_background: u8,
    pub clean_mode: u8,
    pub is_stein_gate: u8,
    pub arc_pay: u8,
    pub free_watch: u8,
    // 新增字段
    pub is_360: u8,
    pub no_share: u8,
}

/// 视频每P信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub cid: u64,
    pub page: u32,
    pub from: String,
    pub part: String,
    pub duration: u64,
    pub vid: String,
    pub weblink: String,
    pub dimension: Dimension,
    pub first_frame: Option<String>,
    pub ctime: u64,
}

/// 字幕上传者信息

/// 字幕列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleListItem {
    pub id: u64,
    pub lan: String,
    pub lan_doc: String,
    pub is_lock: bool,
    pub subtitle_url: String,
    #[serde(rename = "type")]
    pub sub_type: u8,
    pub id_str: String,
    pub ai_type: u8,
    pub ai_status: u8,
    pub author: Account,
}

/// 字幕信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtitle {
    pub allow_submit: bool,
    pub list: Vec<SubtitleListItem>,
}

/// staff成员大会员状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffVip {
    #[serde(rename = "type")]
    pub type_: u8,
    pub status: u8,
    pub due_date: u64,
    pub vip_pay_type: u8,
    pub theme_type: u8,
    pub label: serde_json::Value,
}

/// staff成员认证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffOfficial {
    pub role: i32,
    pub title: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub type_: i32,
}

/// staff成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffItem {
    pub mid: u64,
    pub title: String,
    pub name: String,
    pub face: String,
    pub vip: StaffVip,
    pub official: StaffOfficial,
    pub follower: u64,
    pub label_style: u8,
}

/// honor_reply信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HonorItem {
    pub aid: u64,
    pub hover_type: u8,
    pub desc: String,
    pub weekly_recommend_num: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HonorReply {
    pub honor: Vec<HonorItem>,
}

/// 用户装扮信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGarb {
    pub url_image_ani_cut: String,
}

/// ugc_season中的episodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeArc {
    pub aid: u64,
    pub videos: u32,
    pub type_id: u32,
    pub type_name: String,
    pub copyright: u8,
    pub pic: String,
    pub title: String,
    pub pubdate: u64,
    pub ctime: u64,
    pub desc: String,
    pub state: u8,
    pub duration: u64,
    pub rights: Rights,
    pub author: Owner,
    pub stat: Stat,
    pub dynamic: String,
    pub dimension: Dimension,
    pub desc_v2: serde_json::Value,
    pub is_chargeable_season: bool,
    pub is_blooper: bool,
    pub enable_vt: u8,
    pub vt_display: String,
    pub type_id_v2: u32,
    pub type_name_v2: String,
    pub is_lesson_video: u8,
}

/// ugc_season中的section中的episodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionEpisode {
    pub season_id: u64,
    pub section_id: u64,
    pub id: u64,
    pub aid: u64,
    pub cid: u64,
    pub title: String,
    pub attribute: u32,
    pub arc: EpisodeArc,
    pub page: Page,
    pub bvid: String,
    pub pages: Vec<Page>,
}

/// ugc_season section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub season_id: u64,
    pub id: u64,
    pub title: String,
    #[serde(rename = "type")]
    pub type_: u8,
    pub episodes: Vec<SectionEpisode>,
}

/// ugc_season
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UgcSeasonStat {
    pub season_id: u64,
    pub view: u64,
    pub danmaku: u64,
    pub reply: u64,
    pub fav: u64,
    pub coin: u64,
    pub share: u64,
    pub now_rank: i64,
    pub his_rank: i64,
    pub like: u64,
    pub vt: u64,
    pub vv: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UgcSeason {
    pub id: u64,
    pub title: String,
    pub cover: String,
    pub mid: u64,
    pub intro: String,
    pub sign_state: u8,
    pub attribute: u32,
    pub sections: Vec<Section>,
    pub stat: UgcSeasonStat,
    pub ep_count: u32,
    pub season_type: u8,
    pub is_pay_season: bool,
    pub enable_vt: u8,
}

/// video data完整结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoData {
    pub aid: u64,
    pub bvid: String,
    pub videos: u32,
    pub tid: u32,
    pub tid_v2: u32,
    pub tname: String,
    pub tname_v2: String,
    pub copyright: u8,
    pub pic: String,
    pub title: String,
    pub pubdate: u64,
    pub ctime: u64,
    pub desc: String,
    pub desc_v2: Vec<serde_json::Value>,
    pub state: u32,
    pub duration: u64, // 单位为秒
    #[serde(default = "default_forward")] // 仅撞车视频存在此字段
    pub forward: u64,
    pub mission_id: Option<u64>,
    #[serde(default = "default_redirect_url")] // 用于番剧&影视的av/bv->ep
    pub redirect_url: String,
    pub rights: Rights,
    pub owner: Owner,
    pub stat: Stat,
    pub argue_info: ArgueInfo,
    pub dynamic: String,
    pub cid: u64,
    pub dimension: Dimension,
    pub premiere: serde_json::Value,
    pub teenage_mode: u8,
    pub is_chargeable_season: bool,
    pub is_story: bool,
    pub is_upower_exclusive: bool,
    pub is_upower_play: bool,
    pub is_upower_preview: bool,
    pub no_cache: bool,
    pub pages: Vec<Page>,
    pub subtitle: Subtitle,
    pub ugc_season: Option<UgcSeason>, // 不在合集中的视频无此项
    #[serde(default = "default_staff")]
    pub staff: Vec<StaffItem>, // 非合作视频无此项
    pub is_season_display: bool,
    pub user_garb: UserGarb,
    pub honor_reply: serde_json::Value,
    pub like_icon: String,
    pub need_jump_bv: bool,
    pub disable_show_up_info: bool,
    pub is_story_play: u32,
    pub is_view_self: bool,
    pub is_upower_exclusive_with_qa: bool,
}

fn default_forward() -> u64 {
    0
}

fn default_redirect_url() -> String {
    String::new()
}

fn default_staff() -> Vec<StaffItem> {
    let v: Vec<StaffItem> = Vec::new();
    v
}

/// 获取视频详细信息响应类型
pub type VideoInfoResponse = BpiResponse<VideoData>;

impl BpiClient {
    /// 获取视频详细信息 (Web端)
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/video/video.html#获取视频详细信息)
    ///
    /// # 参数
    /// | 名称   | 类型         | 说明                 |
    /// | ------ | ------------| -------------------- |
    /// | `aid`  | `Option<u64>` | 稿件 avid，可选      |
    /// | `bvid` | `Option<&str>`| 稿件 bvid，可选      |
    ///
    /// 两者任选一个
    pub async fn video_info(
        &self,
        aid: Option<u64>,
        bvid: Option<&str>
    ) -> Result<VideoInfoResponse, BpiError> {
        let aid = aid.map(|v| v.to_string());
        let bvid = bvid.map(|v| v.to_string());

        self
            .get("https://api.bilibili.com/x/web-interface/view")
            .query(
                &[
                    ("aid", aid),
                    ("bvid", bvid),
                ]
            )
            .send_bpi("视频详细信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_video_info() {
        let bpi = BpiClient::new();

        let aid = Some(10001);
        let bvid = None;

        match bpi.video_info(aid, bvid).await {
            Ok(resp) => {
                if resp.code == 0 {
                    // tracing::info!("视频标题: {}", resp.data.title);
                    // tracing::info!("UP主: {} ({})", resp.data.owner.name, resp.data.owner.mid);
                    // tracing::info!("总播放数: {}", resp.data.stat.view);
                    // tracing::info!("分P数量: {}", resp.data.pages.len());
                } else {
                    tracing::info!("请求失败: code={}, message={}", resp.code, resp.message);
                }
            }
            Err(err) => {
                panic!("请求出错: {}", err);
            }
        }
    }
}
