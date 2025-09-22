//! 获取视频超详细信息(Web端)
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video

use crate::models::{ LevelInfo, Nameplate, Official, OfficialVerify, Pendant, VipLabel };
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoDetailData {
    #[serde(rename = "View")]
    pub view: View,
    #[serde(rename = "Card")]
    pub card: Card,
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "Reply")]
    pub reply: Reply,

    #[serde(rename = "Related")]
    pub related: Vec<Related>,

    #[serde(rename = "Spec")]
    pub spec: Option<serde_json::Value>,
    pub hot_share: HotShare,
    pub elec: Option<serde_json::Value>,
    pub emergency: Emergency,

    pub view_addit: ViewAddit,
    pub guide: Option<serde_json::Value>,
    pub query_tags: Option<serde_json::Value>,
    pub participle: Option<serde_json::Value>,
    pub module_ctrl: Option<serde_json::Value>,
    pub replace_recommend: bool,
    pub is_hit_labour_day_activity: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct View {
    pub bvid: String,
    pub aid: i64,
    pub videos: i32,
    pub tid: i32,
    pub tid_v2: i32,
    pub tname: String,
    pub tname_v2: String,
    pub copyright: i32,
    pub pic: String,
    pub title: String,
    pub pubdate: i64,
    pub ctime: i64,
    pub desc: String,
    pub desc_v2: Vec<DescV2>,
    pub state: i32,
    pub duration: i32,
    pub rights: Rights,
    pub owner: Owner,
    pub stat: Stat,
    pub argue_info: ArgueInfo,
    pub dynamic: String,
    pub cid: i64,
    pub dimension: Dimension,
    pub premiere: Option<serde_json::Value>,

    pub teenage_mode: i32,
    pub is_chargeable_season: bool,
    pub is_story: bool,
    pub is_upower_exclusive: bool,
    pub is_upower_play: bool,
    pub is_upower_preview: bool,
    pub enable_vt: i32,
    pub vt_display: String,
    pub is_upower_exclusive_with_qa: bool,
    pub no_cache: bool,
    pub pages: Vec<Page>,

    pub subtitle: Subtitle,
    pub is_season_display: bool,
    pub user_garb: UserGarb,
    pub honor_reply: serde_json::Value,
    pub like_icon: String,
    pub need_jump_bv: bool,
    pub disable_show_up_info: bool,
    pub is_story_play: i32,
    pub is_view_self: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DescV2 {
    pub raw_text: String,
    #[serde(rename = "type")]
    pub desc_type: i32,
    pub biz_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rights {
    pub bp: i32,
    pub elec: i32,
    pub download: i32,
    pub movie: i32,
    pub pay: i32,
    pub hd5: i32,
    pub no_reprint: i32,
    pub autoplay: i32,
    pub ugc_pay: i32,
    pub is_cooperation: i32,
    pub ugc_pay_preview: i32,
    pub no_background: i32,
    pub clean_mode: i32,
    pub is_stein_gate: i32,
    pub is_360: i32,
    pub no_share: i32,
    pub arc_pay: i32,
    pub free_watch: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Owner {
    pub mid: i64,
    pub name: String,
    pub face: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stat {
    pub aid: i64,
    pub view: i32,
    pub danmaku: i32,
    pub reply: i32,
    pub favorite: i32,
    pub coin: i32,
    pub share: i32,
    pub now_rank: i32,
    pub his_rank: i32,
    pub like: i32,
    pub dislike: i32,
    pub evaluation: String,
    pub vt: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArgueInfo {
    pub argue_msg: String,
    pub argue_type: i32,
    pub argue_link: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dimension {
    pub width: i32,
    pub height: i32,
    pub rotate: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    pub cid: i64,
    pub page: i32,
    pub from: String,
    pub part: String,
    pub duration: i32,
    pub vid: String,
    pub weblink: String,
    pub dimension: Dimension,
    pub first_frame: Option<String>,
    pub ctime: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subtitle {
    pub allow_submit: bool,
    pub list: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserGarb {
    pub url_image_ani_cut: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub card: CardInfo,
    pub space: Space,
    pub following: bool,
    pub archive_count: i32,
    pub article_count: i32,
    pub follower: i32,
    pub like_num: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CardInfo {
    pub mid: String,
    pub name: String,
    pub approve: bool,
    pub sex: String,
    pub rank: String,
    pub face: String,
    pub face_nft: i32,
    pub face_nft_type: i32,
    #[serde(rename = "DisplayRank")]
    pub display_rank: String,
    pub regtime: i64,
    pub spacesta: i32,
    pub birthday: String,
    pub place: String,
    pub description: String,
    pub article: i32,
    pub attentions: Vec<serde_json::Value>,
    pub fans: i32,
    pub friend: i32,
    pub attention: i32,
    pub sign: String,
    pub level_info: LevelInfo,
    pub pendant: Pendant,
    pub nameplate: Nameplate,
    #[serde(rename = "Official")]
    pub official: Official,
    pub official_verify: OfficialVerify,
    pub vip: VIP,
    pub is_senior_member: i32,
    pub name_render: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VIP {
    #[serde(rename = "type")]
    pub vip_type: i32,
    pub status: i32,
    pub due_date: i64,
    pub vip_pay_type: i32,
    pub theme_type: i32,
    pub label: VipLabel,
    pub avatar_subscript: i32,
    pub nickname_color: String,
    pub role: i32,
    pub avatar_subscript_url: String,
    pub tv_vip_status: i32,
    pub tv_vip_pay_type: i32,
    pub tv_due_date: i64,
    pub avatar_icon: AvatarIcon,
    #[serde(rename = "vipType")]
    pub vip_type_alt: i32,
    #[serde(rename = "vipStatus")]
    pub vip_status_alt: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AvatarIcon {
    pub icon_resource: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Space {
    pub s_img: String,
    pub l_img: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub tag_id: i32,
    pub tag_name: String,
    pub music_id: String,
    pub tag_type: String,
    pub jump_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Reply {
    pub page: Option<serde_json::Value>,
    pub replies: Vec<ReplyItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReplyItem {
    pub rpid: i32,
    pub oid: i32,
    #[serde(rename = "type")]
    pub reply_type: i32,
    pub mid: i32,
    pub root: i32,
    pub parent: i32,
    pub dialog: i32,
    pub count: i32,
    pub rcount: i32,
    pub state: i32,
    pub fansgrade: i32,
    pub attr: i32,
    pub ctime: i64,
    pub like: i32,
    pub action: i32,
    pub content: Option<serde_json::Value>,
    pub replies: Option<Vec<ReplyItem>>,
    pub assist: i32,
    pub show_follow: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Related {
    pub aid: i64,
    pub videos: i32,
    pub tid: i32,
    pub tname: String,
    pub copyright: i32,
    pub pic: String,
    pub title: String,
    pub pubdate: i64,
    pub ctime: i64,
    pub desc: String,
    pub state: i32,
    pub duration: i32,
    pub mission_id: Option<i32>,
    pub rights: RelatedRights,
    pub owner: Owner,
    pub stat: RelatedStat,
    pub dynamic: String,

    pub cid: i64,
    pub dimension: Dimension,
    pub short_link_v2: String,
    pub up_from_v2: Option<i32>,
    pub first_frame: Option<String>,

    pub pub_location: Option<String>,
    pub cover43: String,
    pub tidv2: i32,
    pub tnamev2: String,
    pub pid_v2: i32,

    pub pid_name_v2: String,
    pub bvid: String,
    pub season_type: i32,
    pub season_id: Option<i32>,
    pub is_ogv: bool,
    pub ogv_info: Option<serde_json::Value>,
    pub rcmd_reason: String,
    pub enable_vt: i32,
    pub ai_rcmd: AIRcmd,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelatedRights {
    pub bp: i32,
    pub elec: i32,
    pub download: i32,
    pub movie: i32,
    pub pay: i32,
    pub hd5: i32,
    pub no_reprint: i32,
    pub autoplay: i32,
    pub ugc_pay: i32,
    pub is_cooperation: i32,
    pub ugc_pay_preview: i32,
    pub no_background: i32,
    pub arc_pay: i32,
    pub pay_free_watch: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RelatedStat {
    pub aid: i64,
    pub view: i32,
    pub danmaku: i32,
    pub reply: i32,
    pub favorite: i32,
    pub coin: i32,
    pub share: i32,
    pub now_rank: i32,
    pub his_rank: i32,
    pub like: i32,
    pub dislike: i32,
    pub vt: i32,
    pub vv: i32,
    pub fav_g: i32,
    pub like_g: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AIRcmd {
    pub id: i64,
    pub goto: String,
    pub trackid: String,
    pub uniq_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HotShare {
    pub show: bool,
    pub list: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Emergency {
    pub no_like: bool,
    pub no_coin: bool,
    pub no_fav: bool,
    pub no_share: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ViewAddit {
    #[serde(rename = "63")]
    pub field_63: bool,
    #[serde(rename = "64")]
    pub field_64: bool,
    #[serde(rename = "69")]
    pub field_69: bool,
    #[serde(rename = "71")]
    pub field_71: bool,
    #[serde(rename = "72")]
    pub field_72: bool,
}

/// 获取视频详细信息响应类型
pub type VideoDetailResponse = BpiResponse<VideoDetailData>;

impl BpiClient {
    /// 获取视频超详细信息 (Web端)
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/video/video.html#获取视频超详细信息-web端
    ///
    /// # 参数
    /// | 名称        | 类型         | 说明                 |
    /// | ----------- | ------------| -------------------- |
    /// | `aid`       | Option<u64> | 稿件 avid，可选      |
    /// | `bvid`      | Option<&str>| 稿件 bvid，可选      |
    /// | `need_elec` | Option<u8>  | 是否获取充电信息 0否 1是，可选 |
    ///
    /// `aid` 和 `bvid` 二选一
    pub async fn video_detail(
        &self,
        aid: Option<u64>,
        bvid: Option<&str>,
        need_elec: Option<u8>
    ) -> Result<VideoDetailResponse, BpiError> {
        let aid = aid.map(|aid| aid.to_string());
        let bvid = bvid.map(|bvid| bvid.to_string());
        let need_elec = need_elec.map(|need_elec| need_elec.to_string());

        self
            .get("https://api.bilibili.com/x/web-interface/view/detail")
            .query(
                &[
                    ("aid", aid),
                    ("bvid", bvid),
                    ("need_elec", need_elec),
                ]
            )
            .send_bpi("视频超详细信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_video_detail() {
        let bpi = BpiClient::new();

        let aid = Some(10001);
        // let aid = Some(114993303389765);
        let bvid = None;

        match bpi.video_detail(aid, bvid, Some(0)).await {
            Ok(resp) => {
                if resp.code == 0 {
                    // tracing::info!("视频标题: {}", resp.data.view.title);
                    // tracing::info!("回复: {:?} )", resp.data.reply.page);
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
