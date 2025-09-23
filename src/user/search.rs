//! B站用户搜索相关接口
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// --- 响应数据结构体 ---

/// 合集或课堂统计数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollectionStat {
    pub coin: u64,
    pub danmaku: u64,
    pub favorite: u64,
    pub like: u64,
    pub mtime: u64,
    pub reply: u64,
    pub season_id: u64,
    pub share: u64,
    pub view: u64,
    pub vt: u64,
    pub vv: u64,
}

/// 所属合集或课堂元数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VideoMeta {
    pub attribute: u64,
    pub cover: String,
    pub ep_count: u64,
    pub ep_num: u64,
    pub first_aid: u64,
    pub id: u64,
    pub intro: String,
    pub mid: u64,
    pub ptime: u64,
    pub sign_state: u64,
    pub stat: Option<CollectionStat>,
    pub title: String,
}

/// 投稿视频列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContributedVideo {
    pub aid: u64,
    pub attribute: u64,
    pub author: String,
    pub bvid: String,
    pub comment: u64,
    pub copyright: String,
    pub created: u64,
    pub description: String,
    pub elec_arc_type: u8,
    pub enable_vt: u8,
    pub hide_click: bool,
    pub is_avoided: u8,
    pub is_charging_arc: bool,
    pub is_lesson_video: u8,
    pub is_lesson_finished: u8,
    pub is_live_playback: u8,
    pub is_pay: u8,
    pub is_self_view: bool,
    pub is_steins_gate: u8,
    pub is_union_video: u8,
    pub jump_url: Option<String>,
    pub length: String,
    pub mid: u64,
    pub meta: Option<VideoMeta>,
    pub pic: String,
    pub play: u64,
    pub playback_position: u64,
    pub review: u64,
    pub season_id: u64,
    pub subtitle: String,
    pub title: String,
    pub typeid: u64,
    pub video_review: u64,
    pub vt: u64,
    pub vt_display: String,
}

/// 投稿视频列表
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContributedVideoList {
    pub slist: Vec<serde_json::Value>,
    pub tlist: serde_json::Value,
    pub vlist: Vec<ContributedVideo>,
}

/// 页面信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PageInfo {
    pub count: u64,
    pub pn: u32,
    pub ps: u32,
}

/// 播放全部按钮
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EpisodicButton {
    pub text: String,
    pub uri: String,
}

/// 用户投稿视频明细响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContributedVideosResponseData {
    /// 列表信息
    pub list: ContributedVideoList,
    /// 页面信息
    pub page: PageInfo,
    /// “播放全部”按钮
    pub episodic_button: Option<EpisodicButton>,
    pub is_risk: bool,
    pub gaia_res_type: u8,
    pub gaia_data: Option<serde_json::Value>,
}

// --- API 实现 ---

impl BpiClient {
    /// 查询用户投稿视频明细
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `mid` | u64 | 目标用户 UID |
    /// | `order` | Option<&str> | 排序方式，默认 `pubdate` |
    /// | `tid` | Option<u64> | 分区筛选，默认 0 |
    /// | `keyword` | Option<&str> | 关键词筛选 |
    /// | `pn` | Option<u32> | 页码，默认 1 |
    /// | `ps` | Option<u32> | 每页项数，默认 30 |
    pub async fn user_contributed_videos(
        &self,
        mid: u64,
        order: Option<&str>,
        tid: Option<u64>,
        keyword: Option<&str>,
        pn: Option<u32>,
        ps: Option<u32>,
    ) -> Result<BpiResponse<ContributedVideosResponseData>, BpiError> {
        let pn_val = pn.unwrap_or(1);
        let ps_val = ps.unwrap_or(30);
        let order_val = order.unwrap_or("pubdate");
        let tid_val = tid.unwrap_or(0);

        let mut params = vec![
            ("mid", mid.to_string()),
            ("order", order_val.to_string()),
            ("tid", tid_val.to_string()),
            ("pn", pn_val.to_string()),
            ("ps", ps_val.to_string()),
        ];

        if let Some(k) = keyword {
            params.push(("keyword", k.to_string()));
        }

        let params = self.get_wbi_sign2(params).await?;

        let req = self
            .get("https://api.bilibili.com/x/space/wbi/arc/search")
            .query(&params);

        req.send_bpi("查询用户投稿视频明细").await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    // 假设这是一个已知用户
    const TEST_MID: u64 = 53456;
    const TEST_KEYWORD: &str = "科技";

    #[tokio::test]

    async fn test_user_contributed_videos_default() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi
            .user_contributed_videos(TEST_MID, None, None, None, Some(1), Some(2))
            .await?;
        let data = resp.into_data()?;

        info!("用户投稿视频明细: {:?}", data);
        assert_eq!(data.page.pn, 1);
        assert_eq!(data.page.ps, 2);
        assert_eq!(data.list.vlist.len(), 2);
        assert!(data.page.count > 0);

        Ok(())
    }

    #[tokio::test]

    async fn test_user_contributed_videos_with_keyword() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi
            .user_contributed_videos(TEST_MID, None, None, Some(TEST_KEYWORD), Some(1), Some(10))
            .await?;
        let data = resp.into_data()?;

        info!("用户投稿视频明细（关键词）: {:?}", data);
        assert!(data.page.count > 0);

        Ok(())
    }
}
