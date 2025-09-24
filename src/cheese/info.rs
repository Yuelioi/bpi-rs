//! 课程（PUGV）相关 API
//!
//! [参考文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/cheese/info.md)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// ==========================
// 数据结构（/pugv/view/web/season）
// ==========================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseInfo {
    pub brief: CourseBrief,
    pub coupon: CourseCoupon,
    pub cover: String,
    pub episode_page: CourseEpisodePage,
    pub episode_sort: i32,
    pub episodes: Vec<CourseEpisode>,
    pub faq: CourseFaq,
    pub faq1: CourseFaq1,
    pub payment: CoursePayment,
    pub purchase_note: CoursePurchaseNote,
    pub purchase_protocol: CoursePurchaseProtocol,
    pub release_bottom_info: String,
    pub release_info: String,
    pub release_info2: String,
    pub release_status: String,
    pub season_id: u64,
    pub share_url: String,
    pub short_link: String,
    pub stat: CourseStat,
    pub status: i32,
    pub subtitle: String,
    pub title: String,
    pub up_info: CourseUpInfo,
    pub user_status: CourseUserStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseBrief {
    pub content: String,
    pub img: Vec<CourseBriefImg>,
    pub title: String,
    pub r#type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseBriefImg {
    pub aspect_ratio: f64,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCoupon {
    pub amount: f64,
    pub expire_time: String, // YYYY-MM-DD HH:MM:SS
    pub start_time: String, // YYYY-MM-DD HH:MM:SS
    pub status: i32,
    pub title: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseEpisodePage {
    pub next: bool,
    pub num: u32,
    pub size: u32,
    pub total: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseEpisode {
    pub aid: u64, // 课程分集 avid（与普通稿件部分不互通）
    pub cid: u64, // 课程分集 cid（与普通视频部分不互通）
    pub duration: u64, // 单位：秒
    pub from: String, // "pugv"
    pub id: u64, // 课程分集 epid（与番剧不互通）
    pub index: u32, // 课程分集数
    pub page: u32, // 一般为 1
    pub play: u64, // 分集播放量
    pub release_date: u64, // 发布时间（时间戳）
    pub status: i32, // 1 可看、2 不可看
    pub title: String, // 分集标题
    pub watched: bool, // 是否观看（需登录 + 正确 Referer）
    #[serde(rename = "watchedHistory")] // 文档里为驼峰
    pub watched_history: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseFaq {
    pub content: String,
    pub link: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseFaq1 {
    pub items: Vec<CourseFaqItem>,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseFaqItem {
    pub answer: String,
    pub question: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoursePayment {
    pub desc: String,
    pub discount_desc: String,
    pub discount_prefix: String,
    pub pay_shade: String,
    pub price: f64,
    pub price_format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoursePurchaseNote {
    pub content: String,
    pub link: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoursePurchaseProtocol {
    pub link: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseStat {
    pub play: u64,
    pub play_desc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseUpInfo {
    pub avatar: String,
    pub brief: String,
    pub follower: u64,
    pub is_follow: i32, // 0 未关注，1 已关注
    pub link: String,
    pub mid: u64,
    pub pendant: CoursePendant,
    pub uname: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoursePendant {
    pub image: String,
    pub name: String,
    pub pid: u64,
    // pub follower: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseUserStatus {
    pub favored: i32, // 0 未收藏，1 已收藏
    pub favored_count: u64,
    pub payed: i32, // 0 未购买，1 已购买
    pub progress: CourseProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseProgress {
    pub last_ep_id: u64,
    pub last_ep_index: String,
    pub last_time: u64, // 秒
}

// ==========================
// 数据结构（/pugv/view/web/ep/list）
// ==========================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseEpList {
    pub items: Vec<CourseEpisode>, // 结构与 CourseEpisode 一致
    pub page: CourseEpPage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseEpPage {
    pub next: bool, // 是否存在下一页
    pub num: u32, // 当前页码
    pub size: u32, // 每页项数
    pub total: u32, // 总计项数
}

// ==========================
// API 封装
// ==========================

impl BpiClient {
    /// 获取课程基本信息
    ///
    /// 通过课程 season_id 或分集 ep_id 获取课程的详细信息，包括课程简介、
    /// 分集列表、UP主信息、统计数据等。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `season_id` | `Option<u64>` | 课程 season_id，与 ep_id 二选一 |
    /// | `ep_id` | `Option<u64>` | 课程分集 ep_id，与 season_id 二选一 |
    ///
    /// # 错误
    /// 当 season_id 和 ep_id 都未提供时返回参数错误
    ///
    /// # 文档
    /// [获取课程基本信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/cheese/info.md#获取课程基本信息)
    pub async fn cheese_info(
        &self,
        season_id: Option<u64>,
        ep_id: Option<u64>
    ) -> Result<BpiResponse<CourseInfo>, BpiError> {
        if season_id.is_none() && ep_id.is_none() {
            return Err(
                BpiError::parse("cheese_info: season_id 与 ep_id 必须至少提供一个".to_string())
            );
        }
        // 构造查询参数
        let mut req = self
            .get("https://api.bilibili.com/pugv/view/web/season")
            .with_bilibili_headers();

        if let Some(sid) = season_id {
            req = req.query(&[("season_id", sid)]);
        }
        if let Some(eid) = ep_id {
            req = req.query(&[("ep_id", eid)]);
        }

        req.send_bpi("获取课程基本信息").await
    }

    /// 通过 season_id 获取课程基本信息
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `season_id` | u64 | 课程 season_id |
    pub async fn cheese_info_by_season_id(
        &self,
        season_id: u64
    ) -> Result<BpiResponse<CourseInfo>, BpiError> {
        self.cheese_info(Some(season_id), None).await
    }

    /// 通过 ep_id 获取课程基本信息
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `ep_id` | u64 | 课程分集 ep_id |
    pub async fn cheese_info_by_ep_id(
        &self,
        ep_id: u64
    ) -> Result<BpiResponse<CourseInfo>, BpiError> {
        self.cheese_info(None, Some(ep_id)).await
    }

    /// 获取课程分集列表
    ///
    /// 获取指定课程的所有分集信息，支持分页查询。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `season_id` | u64 | 课程 season_id |
    /// | `ps` | `Option<u32>` | 每页数量，可选，默认值由 API 决定 |
    /// | `pn` | `Option<u32>` | 页码，可选，默认为 1 |
    ///
    /// # 文档
    /// [获取课程分集列表](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/cheese/info.md#获取课程分集列表)
    pub async fn cheese_ep_list(
        &self,
        season_id: u64,
        ps: Option<u32>,
        pn: Option<u32>
    ) -> Result<BpiResponse<CourseEpList>, BpiError> {
        let mut req = self
            .get("https://api.bilibili.com/pugv/view/web/ep/list")
            .query(&[("season_id", season_id)]);

        if let Some(ps) = ps {
            req = req.query(&[("ps", ps)]);
        }
        if let Some(pn) = pn {
            req = req.query(&[("pn", pn)]);
        }

        req.send_bpi("获取课程分集列表").await
    }
}

// ==========================
// 测试
// ==========================

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SEASON_ID: u64 = 556;
    const TEST_EP_ID: u64 = 20767;

    #[tokio::test]
    async fn test_cheese_info_by_season_id() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.cheese_info_by_season_id(TEST_SEASON_ID).await?.into_data()?;

        assert_eq!(data.season_id, TEST_SEASON_ID);
        tracing::info!("{:#?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn test_cheese_info_by_ep_id() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.cheese_info_by_ep_id(TEST_EP_ID).await?.into_data()?;
        assert_eq!(data.season_id, TEST_SEASON_ID);

        tracing::info!("课程标题: {:?}", data.title);
        tracing::info!("课程 ssid: {:?}", data.season_id);
        Ok(())
    }

    #[tokio::test]
    async fn test_cheese_ep_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.cheese_ep_list(TEST_SEASON_ID, Some(50), Some(1)).await?.into_data()?;
        assert_eq!(data.items.first().unwrap().id, TEST_SEASON_ID);

        tracing::info!("课程标题: {:?}", data.items.first().unwrap().title);
        tracing::info!("课程 ssid: {:?}", data.items.first().unwrap());
        Ok(())
    }
}
