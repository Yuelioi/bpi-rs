//! 视频推荐相关接口
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// --- 视频推荐相关数据结构体 ---

/// 视频作者信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Owner {
    /// UP主mid
    pub mid: u64,
    /// UP昵称
    pub name: String,
    /// 头像URL
    pub face: String,
}

/// 视频统计数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stat {
    /// 播放量
    pub view: u64,
    /// 视频aid
    pub aid: u64,
    /// 弹幕数
    pub danmaku: u64,
    /// 评论数
    pub reply: u64,
    /// 收藏数
    pub favorite: u64,
    /// 硬币数
    pub coin: u64,
    /// 分享数
    pub share: u64,
    /// 当前排名
    pub now_rank: u64,
    /// 历史最高排名
    pub his_rank: u64,
    /// 点赞数
    pub like: u64,
    /// 点踩数
    pub dislike: u64,
}

/// 主页推荐视频/直播统计数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HomeRmdStat {
    /// 播放量
    pub view: u64,
    /// 弹幕数
    pub danmaku: u64,
    /// 点赞数
    pub like: u64,
}

/// 视频版权信息
#[derive(Debug, Clone, Deserialize, Serialize)]
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
}

/// 视频分辨率信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
    pub rotate: u8,
}

/// 单视频推荐列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelatedVideo {
    pub aid: u64,
    pub videos: u32,
    pub tid: u32,
    pub tname: String,
    pub copyright: u8,
    pub pic: String,
    pub title: String,
    pub pubdate: u64,
    pub ctime: u64,
    pub desc: String,
    pub state: i8,
    pub duration: u64,
    pub rights: Rights,
    pub owner: Owner,
    pub stat: Stat,
    pub dynamic: String,
    pub cid: u64,
    pub dimension: Dimension,
    pub bvid: String,
    #[serde(default)]
    pub short_link_v2: String,
}

/// 首页推荐视频列表项中的推荐理由
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RcmdReason {
    /// 原因类型, 0: 无, 1: 已关注, 3: 高点赞量
    #[serde(rename = "reason_type")]
    pub reason_type: u8,
    /// 原因描述
    pub content: Option<String>,
}

/// 首页推荐视频列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RcmdItem {
    pub av_feature: Option<serde_json::Value>,
    /// 商业推广信息，若无则为 null
    pub business_info: Option<serde_json::Value>,
    /// 视频bvid
    pub bvid: String,
    /// 稿件cid
    pub cid: u64,
    /// 视频时长
    pub duration: u64,
    /// 目标类型, "av": 视频, "ogv": 边栏, "live": 直播
    pub goto: String,
    /// 视频aid / 直播间id
    pub id: u64,
    /// 是否已关注, 0: 未关注, 1: 已关注
    pub is_followed: u8,
    pub is_stock: u8,
    /// UP主信息
    pub owner: Owner,
    /// 封面
    pub pic: String,
    pub pos: u8,
    /// 发布时间
    pub pubdate: u64,
    /// 推荐理由
    pub rcmd_reason: Option<RcmdReason>,
    /// 直播间信息
    pub room_info: Option<serde_json::Value>,
    pub show_info: u8,
    /// 视频状态信息
    pub stat: Option<HomeRmdStat>,
    /// 标题
    pub title: String,
    pub track_id: String,
    /// 目标页 URI
    pub uri: String,
}

/// 首页推荐列表响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RcmdFeedResponseData {
    /// 推荐列表
    pub item: Vec<RcmdItem>,
    /// 用户mid，未登录为0
    pub mid: u64,
    pub preload_expose_pct: f32,
    pub preload_floor_expose_pct: f32,
}

impl BpiClient {
    /// 获取单视频推荐列表
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/video/recommend.html#获取单视频推荐列表
    ///
    /// # 参数
    /// | 名称   | 类型         | 说明                 |
    /// | ------ | ------------| -------------------- |
    /// | `aid`  | Option<u64> | 稿件 avid，可选      |
    /// | `bvid` | Option<&str>| 稿件 bvid，可选      |
    ///
    /// `aid` 和 `bvid` 必须提供一个。
    pub async fn video_related_videos(
        &self,
        aid: Option<u64>,
        bvid: Option<&str>,
    ) -> Result<BpiResponse<Vec<RelatedVideo>>, BpiError> {
        if aid.is_none() && bvid.is_none() {
            return Err(BpiError::parse("必须提供 aid 或 bvid"));
        }

        let mut req = self.get("https://api.bilibili.com/x/web-interface/archive/related");

        if let Some(a) = aid {
            req = req.query(&[("aid", &a.to_string())]);
        }
        if let Some(b) = bvid {
            req = req.query(&[("bvid", b)]);
        }

        req.send_bpi("获取单视频推荐列表").await
    }

    /// 获取首页视频推荐列表
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/video/recommend.html#获取首页视频推荐列表
    ///
    /// # 参数
    /// | 名称        | 类型         | 说明                 |
    /// | ----------- | ------------| -------------------- |
    /// | `ps`        | Option<u8>  | 单页返回的记录条数，最多30，可选 |
    /// | `fresh_idx` | Option<u32> | 当前翻页号，可选，默认1 |
    /// | `fetch_row` | Option<u32> | 本次抓取的最后一行行号，可选 |
    pub async fn video_homepage_recommendations(
        &self,
        ps: Option<u8>,
        fresh_idx: Option<u32>,
        fetch_row: Option<u32>,
    ) -> Result<BpiResponse<RcmdFeedResponseData>, BpiError> {
        let ps_val = ps.unwrap_or(12);
        let fresh_idx_val = fresh_idx.unwrap_or(1);
        let fetch_row_val = fetch_row.unwrap_or(1);
        let params = vec![
            ("fresh_type", "4".to_string()),
            ("ps", ps_val.to_string()),
            ("fresh_idx", fresh_idx_val.to_string()),
            ("fresh_idx_1h", fresh_idx_val.to_string()),
            ("brush", fresh_idx_val.to_string()),
            ("fetch_row", fetch_row_val.to_string()),
        ];
        let params = self.get_wbi_sign2(params).await?;

        let req = self
            .get("https://api.bilibili.com/x/web-interface/wbi/index/top/feed/rcmd")
            .query(&params);

        req.send_bpi("获取首页视频推荐列表").await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    const TEST_AID: u64 = 10001;

    #[tokio::test]
    async fn test_video_related_videos_by_aid() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.video_related_videos(Some(TEST_AID), None).await?;
        let data = resp.into_data()?;

        info!("单视频推荐列表: {:?}", data);

        assert!(!data.is_empty());
        assert!(data.len() <= 40);

        Ok(())
    }

    #[tokio::test]

    async fn test_video_homepage_recommendations() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi
            .video_homepage_recommendations(Some(12), Some(1), Some(1))
            .await?;
        let data = resp.into_data()?;

        info!("首页推荐列表: {:?}", data);

        assert!(!data.item.is_empty());
        assert!(data.item.len() <= 30);

        Ok(())
    }
}
