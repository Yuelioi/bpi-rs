//! 创作中心视频管理 API
//!
//! [参考文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/videos.md)

use serde::{ Deserialize, Serialize };

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

/// 稿件统计信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ArchiveStat {
    pub aid: i64,
    pub view: i64,
    pub danmaku: i64,
    pub reply: i64,
    pub favorite: i64,
    pub coin: i64,
    pub share: i64,
    pub now_rank: i64,
    pub his_rank: i64,
    pub like: i64,
    pub dislike: i64,
    pub vt: i64,
    pub vv: i64,
}

/// 稿件基本信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Archive {
    pub aid: i64,
    pub bvid: String,
    pub title: String,
    pub cover: String,
    pub duration: i64,
    pub desc: String,
}

/// 稿件列表项
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ArcAudit {
    #[serde(rename = "Archive")]
    pub archive: Option<Archive>,
    #[serde(rename = "Videos")]
    pub videos: Option<serde_json::Value>,
    pub stat: ArchiveStat,
    pub state_panel: i64,
    pub parent_tname: Option<String>,
    pub typename: Option<String>,
    pub open_appeal: i64,
    pub activity: Option<serde_json::Value>,
    pub season_add_state: i64,
}

/// 分页信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PageInfo {
    pub pn: i64,
    pub ps: i64,
    pub count: i64,
}

/// 稿件列表数据
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SpArchivesData {
    pub arc_audits: Vec<ArcAudit>,
    pub page: PageInfo,
    pub play_type: i64,
}

/// 分P 视频信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct VideoPart {
    /// 分P cid
    pub cid: i64,
    /// 分P 序号
    pub index: i64,
    /// 分P 标题
    pub title: String,
    /// 视频时长（秒）
    pub duration: i64,
}

/// 稿件信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ArchiveInfo {
    /// av号
    pub aid: i64,
    /// bvid
    pub bvid: String,
    /// 标题
    pub title: String,
}

/// 视频基础信息数据
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ArchiveVideosData {
    /// 稿件信息
    pub archive: ArchiveInfo,
    /// 分P 视频列表
    pub videos: Vec<VideoPart>,
}

impl BpiClient {
    /// 获取稿件列表
    ///
    /// 获取 UP 主的稿件列表，支持分页查询。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `pn` | i64 | 页码 |
    /// | `ps` | `Option<i64>` | 每页数量，可选 |
    ///
    /// # 文档
    /// [获取稿件列表](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/videos.md#获取稿件列表)
    pub async fn up_archives_list(
        &self,
        pn: i64,
        ps: Option<i64>
    ) -> Result<BpiResponse<SpArchivesData>, BpiError> {
        let mut req = self
            .get("https://member.bilibili.com/x2/creative/web/archives/sp")
            .query(&[("pn", pn)]);
        if let Some(ps) = ps {
            req = req.query(&[("ps", ps)]);
        }
        req.send_bpi("获取稿件列表").await
    }

    /// 获取视频基础信息
    ///
    /// 获取指定视频的基础信息，包括分P列表等。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `aid` | i64 | 视频 av 号 |
    ///
    /// # 文档
    /// [获取视频基础信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/videos.md#获取视频基础信息)
    pub async fn up_archive_videos(
        &self,
        aid: i64
    ) -> Result<BpiResponse<ArchiveVideosData>, BpiError> {
        self
            .get("https://member.bilibili.com/x/web/archive/videos")
            .query(&[("aid", aid)])
            .send_bpi("获取视频基础信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    const TEST_AID: i64 = 113602455409683;

    #[tokio::test]
    async fn test_archives_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.up_archives_list(1, Some(10)).await?.into_data()?;
        info!("稿件列表: {:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn test_archive_videos() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.up_archive_videos(TEST_AID).await?.into_data()?;
        info!("视频基础信息: {:?}", data);
        Ok(())
    }
}
