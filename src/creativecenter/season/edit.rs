//! 编辑合集小节 API
//!
//! [参考文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season.md)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };
use serde_json::json;

/// 合集信息编辑
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SeasonEdit {
    pub id: u64, // 合集 ID
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
    pub id: u64, // 合集内视频 ID
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

#[derive(Serialize)]
struct EpisodeEditPayload {
    #[serde(flatten)]
    section: EpisodeEdit,
    sorts: Vec<EpisodeSort>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpisodeSort {
    pub id: u64,
    pub sort: u64,
}

/// 合集小节排序信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonSectionSort {
    pub id: u64, // 小节 ID
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

impl BpiClient {
    /// 编辑合集信息
    ///
    /// 编辑合集的基本信息，包括标题、封面、简介等。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `season` | SeasonEdit | 合集信息 |
    /// | `sorts` | `Vec<SeasonSectionSort>` | 小节排序列表 |
    ///
    /// # 文档
    /// [编辑合集信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season/edit.md#编辑合集信息)
    pub async fn season_edit(
        &self,
        season: SeasonEdit,
        sorts: Vec<SeasonSectionSort>
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let payload = json!({
            "season": season,
            "sorts": sorts
        });

        self
            .post("https://member.bilibili.com/x2/creative/web/season/edit")
            .query(&[("csrf", csrf)])
            .json(&payload)
            .send_bpi("编辑合集信息").await
    }
    /// 编辑合集小节(需要开启小节功能)
    ///
    /// 编辑合集中的小节信息，包括小节标题和视频排序。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `section` | SeasonSectionEdit | 小节信息 |
    /// | `sorts` | `Vec<SectionSort>` | 视频排序信息 |
    ///
    /// # 文档
    /// [编辑合集小节](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season/edit.md#编辑合集小节)
    pub async fn season_section_edit(
        &self,
        section: SeasonSectionEdit,
        sorts: Vec<SectionSort>
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let payload = json!({
            "section": section,
            "sorts": sorts
        });

        self
            .post("https://member.bilibili.com/x2/creative/web/season/section/edit")
            .query(&[("csrf", csrf)])
            .json(&payload)
            .send_bpi("编辑合集小节").await
    }

    /// 编辑小节中的章节
    ///
    /// 编辑合集中的小节信息，包括小节标题和视频排序。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `section` | SeasonSectionEdit | 小节信息 |
    /// | `sorts` | `Vec<SectionSort>` | 视频排序信息 |
    ///
    /// # 文档
    /// [编辑合集小节](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season/edit.md#编辑合集小节)
    pub async fn season_section_episode_edit(
        &self,
        section: EpisodeEdit,
        sorts: Vec<EpisodeSort>
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let payload = EpisodeEditPayload { section, sorts };

        self
            .post("https://member.bilibili.com/x2/creative/web/season/section/episode/edit")
            .query(&[("csrf", csrf)])
            .json(&payload)
            .send_bpi("编辑合集章节").await
    }

    /// 切换小节/正常显示
    ///
    /// # 参数
    /// * season_id 合集id

    pub async fn season_enable_section(
        &self,
        season_id: u64,
        enable: bool
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let params = vec![
            ("csrf", csrf),
            ("season_id", season_id.to_string()),
            ("no_section", (if enable { "0" } else { "1" }).to_string())
        ];

        self
            .post("https://member.bilibili.com/x2/creative/web/season/section/switch")
            .form(&params)
            .send_bpi("切换 小节/正常 模式").await
    }
    /// 添加视频到小节(需要开启小节功能)
    ///
    /// 将视频添加到指定的合集小节中。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `aid` | u64 | 视频 aid |
    /// | `season_id` | u64 | 合集 ID |
    /// | `section_id` | u64 | 小节 ID |
    /// | `title` | &str | 视频标题 |
    ///
    /// # 文档
    /// [编辑投稿视频合集/小节](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season/edit.md#编辑投稿视频合集小节)
    pub async fn season_section_add_episodes(
        &self,
        section_id: u64,
        episodes: Vec<Episode>
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let payload = SectionAddEpisodesRequest {
            section_id,
            episodes,
        };

        self
            .post("https://member.bilibili.com/x2/creative/web/season/section/episodes/add")
            .json(&payload)
            .query(&[("csrf", csrf)])
            .send_bpi("编辑投稿视频合集").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECTION_ID: u64 = 7032691;
    const TEST_SEASON_ID: u64 = 6363779;
    const TEST_PART_ID: u64 = 147443135; // 不知道从哪弄

    const TEST_AID: u64 = 772876546;
    const TEST_CID: u64 = 829554597;

    #[tokio::test]
    async fn test_edit_season_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let season = SeasonEdit {
            id: TEST_SEASON_ID,
            title: "修改后的合集标题".to_string(),
            cover: "https://archive.biliimg.com/bfs/archive/fb699e3f5ae17285cf5f6ebd42c156482f829215.jpg".to_string(),
            desc: Some("修改后的合集简介".to_string()),
            ..Default::default()
        };

        let sorts = vec![SeasonSectionSort {
            id: TEST_SECTION_ID,
            sort: 1,
        }];

        bpi.season_edit(season, sorts).await?;

        tracing::info!("编辑合集信息成功");
        Ok(())
    }

    #[tokio::test]
    async fn test_edit_season_section() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let section = SeasonSectionEdit {
            id: TEST_SECTION_ID,
            season_id: TEST_SEASON_ID,
            title: "测试修改小节标题".to_string(),
            ..Default::default()
        };

        let sorts = vec![SectionSort {
            id: TEST_PART_ID,
            order: 1,
        }];

        bpi.season_section_edit(section, sorts).await?;

        tracing::info!("编辑合集小节成功");
        Ok(())
    }

    #[tokio::test]
    async fn test_edit_season_episode() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let section = EpisodeEdit {
            id: TEST_PART_ID,
            season_id: TEST_SEASON_ID,
            section_id: TEST_SECTION_ID,
            sorts: vec![],
            title: "测试修改章节标题".to_string(),
            aid: TEST_AID,
            cid: TEST_CID,
            order: 1,
        };

        let sorts = vec![EpisodeSort {
            id: TEST_PART_ID,
            sort: 1,
        }];

        bpi.season_section_episode_edit(section, sorts).await?;

        tracing::info!("编辑合集小节成功");
        Ok(())
    }

    #[tokio::test]
    async fn test_add_episodes() -> Result<(), BpiError> {
        let bpi = BpiClient::new();

        bpi
            .season_section_add_episodes(
                TEST_SECTION_ID,
                vec![Episode {
                    title: "新增章节标题".to_string(),
                    aid: TEST_AID,
                    cid: TEST_CID,
                    ..Default::default()
                }]
            ).await
            .map(|_| ()) // 忽略 Ok 的内容
            .or_else(|e| {
                if e.code() == Some(20080) { Ok(()) } else { Err(e) }
            })?;

        Ok(())
    }
}
