//! 番剧基本信息
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/info.md)
use crate::models::VipLabel;
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 剧集地区
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BangumiArea {
    /// 中国大陆
    MainlandChina = 1,
    /// 日本
    Japan = 2,
    /// 美国
    UnitedStates = 3,
    /// 英国
    UnitedKingdom = 4,
    /// 加拿大
    Canada = 5,
    /// 中国香港
    HongKong = 6,
    /// 中国台湾
    Taiwan = 7,
    /// 韩国
    SouthKorea = 8,
    /// 法国
    France = 9,
    /// 泰国
    Thailand = 10,
    /// 马来西亚
    Malaysia = 11,
    /// 新加坡
    Singapore = 12,
    /// 西班牙
    Spain = 13,
    /// 俄罗斯
    Russia = 14,
    /// 德国
    Germany = 15,
    /// 其他
    Other = 16,
    /// 丹麦
    Denmark = 17,
    /// 乌克兰
    Ukraine = 18,
    /// 以色列
    Israel = 19,
    /// 伊朗
    Iran = 20,
    /// 保加利亚
    Bulgaria = 21,
    /// 克罗地亚
    Croatia = 22,
    /// 冰岛
    Iceland = 23,
    /// 匈牙利
    Hungary = 24,
    /// 南非
    SouthAfrica = 25,
    /// 印尼
    Indonesia = 26,
    /// 印度
    India = 27,
    /// 哥伦比亚
    Colombia = 28,
    /// 土耳其
    Turkey = 30,
    /// 墨西哥
    Mexico = 31,
    /// 委内瑞拉
    Venezuela = 32,
    /// 巴西
    Brazil = 33,
    /// 希腊
    Greece = 34,
    /// 意大利
    Italy = 35,
    /// 挪威
    Norway = 36,
    /// 捷克
    CzechRepublic = 37,
    /// 摩洛哥
    Morocco = 38,
    /// 新西兰
    NewZealand = 39,
    /// 智利
    Chile = 40,
    /// 比利时
    Belgium = 41,
    /// 波兰
    Poland = 42,
    /// 澳大利亚
    Australia = 43,
    /// 爱尔兰
    Ireland = 44,
    /// 瑞典
    Sweden = 45,
    /// 瑞士
    Switzerland = 46,
    /// 芬兰
    Finland = 47,
    /// 苏联
    SovietUnion = 48,
    /// 荷兰
    Netherlands = 49,
    /// 越南
    Vietnam = 50,
    /// 阿根廷
    Argentina = 51,
    /// 马耳他
    Malta = 52,
    /// 古巴
    Cuba = 53,
    /// 菲律宾
    Philippines = 54,
    /// 哈萨克斯坦
    Kazakhstan = 55,
    /// 黎巴嫩
    Lebanon = 56,
    /// 塞浦路斯
    Cyprus = 57,
    /// 卡塔尔
    Qatar = 58,
    /// 阿联酋
    UnitedArabEmirates = 59,
    /// 奥地利
    Austria = 60,
    /// 西德
    WestGermany = 61,
    /// 卢森堡
    Luxembourg = 62,
    /// 罗马尼亚
    Romania = 63,
    /// 印度尼西亚
    Indonesia2 = 64,
    /// 南斯拉夫
    Yugoslavia = 65,
    /// 蒙古
    Mongolia = 66,
    /// 葡萄牙
    Portugal = 70,
}

impl BangumiArea {
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

/// 剧集类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BangumiType {
    /// 番剧
    Anime = 1,
    /// 电影
    Movie = 2,
    /// 纪录片
    Documentary = 3,
    /// 国创
    ChineseAnimation = 4,
    /// 电视剧
    TVSeries = 5,
    /// 漫画
    Manga = 6,
    /// 综艺
    Variety = 7,
}

impl BangumiType {
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}

/// 剧集基本信息（mdid方式）响应
pub type BangumiInfoResponse = BpiResponse<BangumiInfoResult>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiInfoResult {
    pub media: BangumiMedia,
    pub review: Option<BangumiReview>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiMedia {
    pub areas: Vec<BangumiAreaInfo>,
    pub cover: String,
    pub horizontal_picture: String,
    pub media_id: u64,
    pub new_ep: BangumiMediaNewEp,
    pub rating: BangumiRating,
    pub season_id: u64,
    pub share_url: String,
    pub title: String,
    pub r#type: u32,
    pub type_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiAreaInfo {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiMediaNewEp {
    pub id: u64,
    pub index: String,
    pub index_show: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiDetailNewEp {
    pub id: u64,
    pub desc: String,
    pub is_new: u32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiRating {
    pub count: u64,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiReview {
    pub is_coin: u32,
    pub is_open: u32,
}

/// 获取剧集明细（web端）响应
pub type BangumiDetailResponse = BpiResponse<BangumiDetailResult>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiDetailResult {
    pub activity: Option<BangumiActivity>,
    pub actors: String,
    pub alias: String,
    pub areas: Vec<BangumiAreaInfo>,
    pub bkg_cover: String,
    pub cover: String,
    pub delivery_fragment_video: bool,
    pub enable_vt: bool,
    pub episodes: Vec<BangumiEpisode>,
    pub evaluate: String,
    pub freya: Option<BangumiFreya>,
    pub hide_ep_vv_vt_dm: u32,
    pub icon_font: Option<BangumiIconFont>,
    pub jp_title: String,
    pub link: String,
    pub media_id: u64,
    pub mode: u32,
    pub multi_view_info: Option<BangumiMultiViewInfo>,
    pub new_ep: BangumiDetailNewEp,
    pub payment: Option<BangumiPayment>,
    #[serde(rename = "payPack")]
    pub pay_pack: Option<BangumiPayPack>,
    pub play_strategy: Option<BangumiPlayStrategy>,
    pub positive: Option<BangumiPositive>,
    pub publish: BangumiPublish,
    pub rating: Option<BangumiRating>,
    pub record: String,
    pub rights: BangumiRights,
    pub season_id: u64,
    pub season_title: String,
    pub seasons: Vec<BangumiSeason>,
    pub section: Option<Vec<BangumiSection>>,
    pub section_bottom_desc: Option<String>,
    pub series: Option<BangumiSeries>,
    pub share_copy: String,
    pub share_sub_title: String,
    pub share_url: String,
    pub show: Option<BangumiShow>,
    pub show_season_type: u32,
    pub square_cover: String,
    pub staff: String,
    pub stat: BangumiStat,
    pub status: u32,
    pub styles: Vec<String>,
    pub subtitle: String,
    pub title: String,
    pub total: u32,
    pub r#type: u32,
    pub up_info: Option<BangumiUpInfo>,
    pub user_status: Option<BangumiUserStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiActivity {
    pub head_bg_url: String,
    pub id: u64,
    pub title: String,
    pub link: Option<String>,
    pub pendants: Option<Vec<BangumiPendant>>,
    pub cover: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiPendant {
    pub image: String,
    pub name: String,
    pub pid: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiEpisode {
    pub aid: u64,
    pub badge: String,
    pub badge_info: Option<BangumiBadgeInfo>,
    pub badge_type: u32,
    pub bvid: String,
    pub cid: u64,
    pub cover: String,
    pub dimension: Option<BangumiDimension>,
    pub duration: u64,
    pub enable_vt: bool,
    pub ep_id: u64,
    pub from: String,
    pub id: u64,
    pub interaction: Option<BangumiInteraction>,
    pub is_view_hide: bool,
    pub link: String,
    pub long_title: String,
    pub multi_view_eps: Option<Vec<BangumiMultiViewEp>>,
    pub pub_time: u64,
    pub pv: u64,
    pub release_date: String,
    pub rights: Option<BangumiEpisodeRights>,
    pub section_type: u32,
    pub share_copy: String,
    pub share_url: String,
    pub short_link: String,
    pub show_title: String,
    #[serde(rename = "showDrmLoginDialog")]
    pub show_drm_login_dialog: bool,
    pub skip: Option<BangumiSkip>,
    pub status: u32,
    pub subtitle: String,
    pub title: String,
    pub vid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiBadgeInfo {
    pub bg_color: String,
    pub bg_color_night: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiDimension {
    pub height: u32,
    pub rotate: u32,
    pub width: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiInteraction {
    pub graph_version: u32,
    pub interaction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiMultiViewEp {
    pub ep_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiEpisodeRights {
    pub allow_dm: u32,
    pub allow_download: u32,
    pub area_limit: u32,

    pub allow_demand: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiSkip {
    pub ed: Option<BangumiSkipTime>,
    pub op: Option<BangumiSkipTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiSkipTime {
    pub end: u32,
    pub start: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiFreya {
    pub bubble_desc: String,
    pub bubble_show_cnt: u32,
    pub icon_show: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiIconFont {
    pub name: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiMultiViewInfo {
    pub changing_dance: String,
    pub is_multi_view_season: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiPayment {
    pub discount: u32,
    pub pay_type: BangumiPayType,
    pub price: String,
    pub promotion: String,
    pub tip: String,
    pub view_start_time: u64,
    pub vip_discount: u32,
    pub vip_first_promotion: String,
    pub vip_price: String,
    pub vip_promotion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiPayType {
    pub allow_discount: u32,
    pub allow_pack: u32,
    pub allow_ticket: u32,
    pub allow_time_limit: u32,
    pub allow_vip_discount: u32,
    pub forbid_bb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiPayPack {
    pub id: u64,
    pub not_paid_text_for_app: String,
    pub paid_text_for_app: String,
    pub pay_pack_url: String,
    pub status: u32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiPlayStrategy {
    pub strategies: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiPositive {
    pub id: u64,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiPublish {
    pub is_finish: u32,
    pub is_started: u32,
    pub pub_time: String,
    pub pub_time_show: String,
    pub unknow_pub_date: u32,
    pub weekday: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiRights {
    pub allow_bp: u32,
    pub allow_bp_rank: u32,
    pub allow_download: u32,
    pub allow_review: u32,
    pub area_limit: u32,
    pub ban_area_show: u32,
    pub can_watch: u32,
    pub copyright: String,
    pub forbid_pre: u32,
    pub freya_white: u32,
    pub is_cover_show: u32,
    pub is_preview: u32,
    pub only_vip_download: u32,
    pub resource: String,
    pub watch_platform: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiSeason {
    pub badge: String,
    pub badge_info: Option<BangumiBadgeInfo>,
    pub badge_type: u32,
    pub cover: String,
    pub enable_vt: bool,
    pub horizontal_cover_1610: String,
    pub horizontal_cover_169: String,
    pub icon_font: Option<BangumiIconFont>,
    pub media_id: u64,
    pub new_ep: Option<BangumiSeasonNewEp>,
    pub season_id: u64,
    pub season_title: String,
    pub season_type: u32,
    pub stat: Option<BangumiSeasonStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiSeasonNewEp {
    pub cover: String,
    pub id: u64,
    pub index_show: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiSeasonStat {
    pub favorites: u64,
    pub series_follow: u64,
    pub views: u64,
    pub vt: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiSection {
    pub attr: u32,
    pub episode_id: u64,
    pub episode_ids: Vec<u64>,
    pub episodes: Vec<BangumiSectionEpisode>,
    pub id: u64,
    pub report: Option<BangumiReport>,
    pub title: String,
    pub r#type: u32,
    pub type2: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiSectionEpisode {
    pub aid: u64,
    pub archive_attr: Option<u32>,
    pub badge: String,
    pub badge_info: Option<BangumiBadgeInfo>,
    pub badge_type: u32,
    pub bvid: String,
    pub cid: u64,
    pub cover: String,
    pub dimension: Option<BangumiDimension>,
    pub duration: u64,
    pub enable_vt: bool,
    pub ep_id: u64,
    pub from: String,
    pub icon_font: Option<BangumiIconFont>,
    pub id: u64,
    pub interaction: Option<BangumiInteraction>,
    pub is_view_hide: bool,
    pub link: String,
    pub link_type: String,
    pub long_title: String,
    pub pub_time: u64,
    pub pv: u64,
    pub release_date: String,
    pub report: Option<BangumiReport>,
    pub rights: Option<BangumiEpisodeRights>,
    pub section_type: u32,
    pub share_copy: String,
    pub share_url: String,
    pub short_link: String,
    pub show_title: String,
    #[serde(rename = "showDrmLoginDialog")]
    pub show_drm_login_dialog: bool,
    pub skip: Option<BangumiSkip>,
    pub stat: Option<BangumiStat>,
    pub stat_for_unity: Option<BangumiStatForUnity>,
    pub status: u32,
    pub subtitle: String,
    pub title: String,
    pub toast_title: String,
    pub up_info: Option<BangumiUpInfo>,
    pub vid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiReport {
    pub aid: String,
    pub ep_title: String,
    pub position: String,
    pub season_id: String,
    pub season_type: String,
    pub section_id: String,
    pub section_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiStatForUnity {
    pub coin: u64,
    pub danmaku: Option<BangumiDanmaku>,
    pub likes: u64,
    pub reply: u64,
    pub vt: Option<BangumiVt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiDanmaku {
    pub icon: String,
    pub pure_text: String,
    pub text: String,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiVt {
    pub icon: String,
    pub pure_text: String,
    pub text: String,
    pub value: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiStat {
    pub coins: u64,
    pub danmakus: u64,
    pub favorite: u64,
    pub favorites: u64,
    pub follow_text: String,
    pub hot: Option<u64>,
    pub likes: u64,
    pub reply: u64,
    pub share: u64,
    pub views: u64,
    pub vt: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiSeries {
    pub display_type: u32,
    pub series_id: u64,
    pub series_title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiShow {
    pub wide_screen: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiUpInfo {
    pub avatar: String,
    pub avatar_subscript_url: String,
    pub follower: u64,
    pub is_follow: u32,
    pub mid: u64,
    pub nickname_color: String,
    pub pendant: Option<BangumiPendant>,
    pub theme_type: u32,
    pub uname: String,
    pub verify_type: u32,
    pub vip_label: Option<VipLabel>,
    pub vip_status: u32,
    pub vip_type: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiUserStatus {
    pub area_limit: u32,
    pub ban_area_show: u32,
    pub follow: u32,
    pub follow_status: u32,
    pub login: u32,
    pub pay: u32,
    pub pay_pack_paid: u32,
    pub sponsor: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiSectionResult {
    pub main_section: BangumiMainSection,
    pub section: Vec<BangumiMainSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiMainSection {
    pub episodes: Vec<BangumiSectionEpisodeInfo>,
    pub id: u64,
    pub r#type: u32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiSectionEpisodeInfo {
    pub aid: u64,
    pub badge: String,
    pub badge_info: BangumiBadgeInfo,
    pub badge_type: u32,
    pub cid: u64,
    pub cover: String,
    pub from: String,
    pub id: u64,
    pub is_premiere: u32,
    pub long_title: String,
    pub share_url: String,
    pub status: u32,
    pub title: String,
    pub vid: String,
}

impl BpiClient {
    /// 获取剧集基本信息（mdid方式）
    ///
    /// # 参数
    /// * `media_id` - 剧集mdid
    ///
    /// # 文档
    /// [剧集基本信息（mdid方式）](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/info.md#剧集基本信息mdid方式)
    pub async fn bangumi_info(&self, media_id: u64) -> Result<BangumiInfoResponse, BpiError> {
        let result: BangumiInfoResponse = self
            .get("https://api.bilibili.com/pgc/review/user")
            .query(&[("media_id", media_id.to_string())])
            .send_bpi("获取剧集基本信息").await?;
        Ok(result)
    }

    /// 获取剧集明细（web端）（ssid方式）
    ///
    /// # 参数
    /// * `season_id` - 番剧ssid
    ///
    /// # 文档
    /// [获取剧集明细（web端）（ssid/epid方式）](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/info.md#获取剧集明细web端ssidepid方式)
    pub async fn bangumi_detail_by_season_id(
        &self,
        season_id: u64
    ) -> Result<BangumiDetailResponse, BpiError> {
        let result: BangumiDetailResponse = self
            .get("https://api.bilibili.com/pgc/view/web/season")
            .query(&[("season_id", season_id.to_string())])
            .send_bpi("获取剧集明细").await?;
        Ok(result)
    }

    /// 获取剧集明细（web端）（epid方式）
    ///
    /// # 参数
    /// * `ep_id` - 剧集epid
    ///
    /// # 文档
    /// [获取剧集明细（web端）（ssid/epid方式）](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/info.md#获取剧集明细web端ssidepid方式)
    pub async fn bangumi_detail_by_epid(
        &self,
        ep_id: u64
    ) -> Result<BangumiDetailResponse, BpiError> {
        self
            .get("https://api.bilibili.com/pgc/view/web/season")
            .query(&[("ep_id", ep_id.to_string())])
            .send_bpi("获取剧集明细").await
    }

    /// 获取剧集分集信息
    ///
    /// # 参数
    /// * `season_id` - 剧集ssid
    ///
    /// # 文档
    /// [获取剧集明细（web端）（ssid/epid方式）](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/info.md#获取剧集分集信息)
    pub async fn bangumi_sections_by_season_id(
        &self,
        season_id: u64
    ) -> Result<BpiResponse<BangumiSectionResult>, BpiError> {
        self
            .get("https://api.bilibili.com/pgc/web/season/section")
            .query(&[("season_id", season_id.to_string())])
            .send_bpi("获取剧集分集信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SEASON_ID: u64 = 1172; // ssid
    const TEST_EP_ID: u64 = 21265; // epid
    const TEST_MEDIA_ID: u64 = 28220978; //  mdid

    #[tokio::test]
    async fn test_bangumi_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.bangumi_info(TEST_MEDIA_ID).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert_eq!(data.media.media_id, TEST_MEDIA_ID);
        assert!(!data.media.title.is_empty());
        assert!(!data.media.areas.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_bangumi_detail_by_season_id() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.bangumi_detail_by_season_id(TEST_SEASON_ID).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert_eq!(data.season_id, TEST_SEASON_ID);
        assert!(!data.title.is_empty());
        assert!(!data.episodes.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_bangumi_detail_by_epid() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.bangumi_detail_by_epid(TEST_EP_ID).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert!(!data.title.is_empty());
        assert!(!data.episodes.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_bangumi_section() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.bangumi_sections_by_season_id(TEST_SEASON_ID).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert!(!data.main_section.episodes.is_empty());

        Ok(())
    }
}
