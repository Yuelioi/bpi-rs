//! 获取合集列表 API
//!
//! [参考文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season/list.md)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

use super::models::{ Season, Section };

/// 合集列表返回结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonListData {
    pub seasons: Vec<SeasonItem>,
    pub tip: serde_json::Value,
    pub total: u32,
    pub play_type: u32,
}

/// 单个合集条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonItem {
    pub season: Season,
    pub course: Option<serde_json::Value>,
    pub checkin: Option<CheckInInfo>,
    #[serde(rename = "seasonStat")]
    pub season_stat: Option<SeasonStat>,
    pub sections: Option<SectionsWrapper>,
    pub part_episodes: Vec<PartEpisode>,
}

/// 合集审核信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInInfo {
    pub status: i32,
    pub status_reason: Option<String>,
    pub season_status: i32,
}

/// 合集统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonStat {
    pub view: u64,
    pub danmaku: u64,
    pub reply: u64,
    pub fav: u64,
    pub coin: u64,
    pub share: u64,
    #[serde(rename = "nowRank")]
    pub now_rank: u32,
    #[serde(rename = "hisRank")]
    pub his_rank: u32,
    pub like: u64,
    pub subscription: u64,
    pub vt: u32,
}

/// 小节包装器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionsWrapper {
    pub sections: Vec<Section>,
}

/// 合集内视频条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartEpisode {
    pub id: u64,
    pub title: String,
    pub aid: u64,
    pub bvid: String,
    pub cid: u64,
    #[serde(rename = "seasonId")]
    pub season_id: u64,
    #[serde(rename = "sectionId")]
    pub section_id: u64,
    pub order: u32,
    #[serde(rename = "videoTitle")]
    pub video_title: Option<String>,
    #[serde(rename = "archiveTitle")]
    pub archive_title: Option<String>,
    #[serde(rename = "archiveState")]
    pub archive_state: i32,
    #[serde(rename = "rejectReason")]
    pub reject_reason: Option<String>,
    pub state: i32,
    pub cover: String,
    pub is_free: i32,
    pub aid_owner: bool,
    #[serde(rename = "charging_pay")]
    pub charging_pay: u32,
}

impl BpiClient {
    /// 获取合集列表
    ///
    /// 获取当前用户创建的合集列表，支持分页和排序。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `pn` | u32 | 页码，默认 1 |
    /// | `ps` | u32 | 每页数量，默认 30 |
    /// | `order` | `Option<&str>` | 排序方式：ctime 或 mtime |
    /// | `sort` | `Option<&str>` | 升降序：asc 或 desc |
    ///
    /// # 文档
    /// [获取合集列表](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season/list.md#获取合集列表)
    pub async fn season_list(
        &self,
        pn: u32,
        ps: u32,
        order: Option<&str>,
        sort: Option<&str>
    ) -> Result<BpiResponse<SeasonListData>, BpiError> {
        let mut query: Vec<(&str, String)> = vec![("pn", pn.to_string()), ("ps", ps.to_string())];

        if let Some(order_val) = order {
            query.push(("order", order_val.to_string()));
        }
        if let Some(sort_val) = sort {
            query.push(("sort", sort_val.to_string()));
        }

        self
            .get("https://member.bilibili.com/x2/creative/web/seasons")
            .query(&query)
            .send_bpi("获取合集列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_season_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.season_list(1, 10, Some("ctime"), Some("desc")).await?.into_data()?;

        tracing::info!("共 {} 个合集", data.total);
        for s in data.seasons {
            tracing::info!("合集: {} - {}", s.season.id, s.season.title);
        }

        Ok(())
    }
}
