//! 获取合集列表 API
//!
//! [参考文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season.md)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

use super::models::{ Season, Section };

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SeasonInfoData {
    pub season: Season,
    pub course: serde_json::Value,
    pub checkin: serde_json::Value,
    #[serde(rename = "seasonStat")]
    pub season_stat: serde_json::Value,
    pub sections: Sections,
    pub part_episodes: serde_json::Value,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Sections {
    pub sections: Vec<Section>,
    pub total: i64,
}

impl BpiClient {
    pub async fn season_info(
        &self,
        season_id: u64
    ) -> Result<BpiResponse<SeasonInfoData>, BpiError> {
        self
            .get("https://member.bilibili.com/x2/creative/web/season")
            .query(&[("id", &season_id.to_string())])
            .send_bpi("获取合集信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SSID: u64 = 4294056;

    #[tokio::test]
    async fn test_season_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.season_info(TEST_SSID).await?.into_data()?;

        tracing::info!("共 {:?} 个合集", data);

        Ok(())
    }
}
