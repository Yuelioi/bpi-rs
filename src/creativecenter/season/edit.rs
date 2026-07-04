//! 编辑合集小节 API
//!
//! [参考文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/creativecenter/season.md)

use serde::{Deserialize, Serialize};

/// 合集信息编辑
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SeasonEdit {
    pub id: u64,       // 合集 ID
    pub title: String, // 合集标题
    pub cover: String, // 封面图 URL
    #[serde(default)]
    pub desc: Option<String>, // 合集简介
    #[serde(default)]
    pub season_price: Option<u32>, // 合集价格（默认 0）
    #[serde(default, rename = "isEnd")]
    pub is_end: Option<u32>, // 是否完结 0:未完结 1:完结
}

/// 合集小节信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SeasonSectionEdit {
    pub id: u64,
    #[serde(rename = "type")]
    pub type_field: u64,
    #[serde(rename = "seasonId")]
    pub season_id: u64,
    pub title: String,
}

/// 合集内视频排序信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionSort {
    pub id: u64,    // 合集内视频 ID
    pub order: u32, // 排序位置
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpisodeEdit {
    pub id: u64,
    pub title: String,
    pub aid: u64,
    pub cid: u64,
    #[serde(rename = "seasonId")]
    pub season_id: u64,
    #[serde(rename = "sectionId")]
    pub section_id: u64,
    pub sorts: Vec<EpisodeSort>,
    pub order: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpisodeSort {
    pub id: u64,
    pub sort: u64,
}

/// 合集小节排序信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonSectionSort {
    pub id: u64,   // 小节 ID
    pub sort: u32, // 排序位置
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SectionAddEpisodesRequest {
    #[serde(rename = "sectionId")]
    pub section_id: u64,
    pub episodes: Vec<Episode>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Episode {
    pub title: String,
    pub aid: u64,
    pub cid: u64,
    pub charging_pay: i64,
    pub member_first: i64,
    pub limited_free: bool,
}

#[cfg(test)]
mod tests {}
