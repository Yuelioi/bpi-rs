//! 创建合集 API
//!
//! 参考文档：https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season.md

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };
use serde_json::json;

/// 合集视频条目（添加用）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpisodeAdd {
    pub title: String,
    pub aid: u64,
    pub cid: u64,

    #[serde(default)]
    pub charging_pay: i64,
    #[serde(default)]
    pub member_first: i64,
    #[serde(default)]
    pub limited_free: bool,
}

impl BpiClient {
    /// 创建合集
    ///
    /// 创建一个新的视频合集，需要提供标题、封面等信息。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `title` | &str | 合集标题 |
    /// | `desc` | Option<&str> | 合集简介，可选 |
    /// | `cover` | &str | 封面图 URL（从上传接口获取） |
    /// | `season_price` | Option<u32> | 合集价格，可选，默认 0 |
    ///
    /// # 文档
    /// [创建合集](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season/create.md#创建合集)
    pub async fn season_create(
        &self,
        title: &str,
        desc: Option<&str>,
        cover: &str,
        season_price: Option<u32>
    ) -> Result<BpiResponse<u64>, BpiError> {
        // 校验 csrf
        let csrf = self.csrf()?;

        let mut form = vec![
            ("title", title.to_string()),
            ("cover", cover.to_string()),
            ("csrf", csrf)
        ];

        if let Some(d) = desc {
            form.push(("desc", d.to_string()));
        }
        if let Some(price) = season_price {
            form.push(("season_price", price.to_string()));
        }

        self
            .post("https://member.bilibili.com/x2/creative/web/season/add")
            .form(&form)
            .send_bpi("创建合集").await
    }

    /// 删除合集
    ///
    /// 删除指定的合集，需要提供合集 ID。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `season_id` | u64 | 合集 ID |
    ///
    /// # 文档
    /// [删除合集](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season/del.md#删除合集)
    pub async fn season_delete(
        &self,
        season_id: u64
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let form = vec![("id", season_id.to_string()), ("csrf", csrf)];

        self
            .post("https://member.bilibili.com/x2/creative/web/season/del")
            .form(&form)
            .send_bpi("删除合集").await
    }

    /// 添加视频到合集
    ///
    /// 将视频添加到指定的合集小节中。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `section_id` | u64 | 合集小节 ID |
    /// | `episodes` | Vec<EpisodeAdd> | 视频列表 |
    ///
    /// # 文档
    /// [添加视频到合集](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season/push.md#添加视频到合集)
    pub async fn season_episodes_add(
        &self,
        section_id: u64,
        episodes: Vec<EpisodeAdd>
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        // 校验 csrf
        let csrf = self.csrf()?;

        let payload =
            json!({
            "sectionId": section_id,
            "episodes": episodes
        });

        self
            .post("https://member.bilibili.com/x2/creative/web/season/section/episodes/add")
            .with_bilibili_headers()
            .query(&[("csrf", csrf)])
            .json(&payload)
            .send_bpi("添加视频到合集").await
    }
}

#[cfg(test)]
mod tests {
    use base64::{ Engine as _, engine::general_purpose };
    use std::fs;

    const TEST_AID: u64 = 772876546;
    const TEST_CID: u64 = 829554597;

    const TEST_SECTION_ID: u64 = 7032691;

    use super::*;

    #[tokio::test]
    async fn test_create_season() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let img_data = fs::read("./assets/test.jpg").map_err(|_| BpiError::parse("读取图片失败"))?;
        let img_base64 = general_purpose::STANDARD.encode(&img_data);
        let img_data_uri = format!("data:image/jpeg;base64,{}", img_base64);
        let resp = bpi.upload_cover("image/jpeg", &img_data_uri).await?;

        let result = bpi.season_create(
            "测试合集 - Powered by Rust",
            Some("这是一个通过 API 创建的测试合集"),
            resp.data.unwrap().url.as_str(),
            Some(0)
        ).await?;

        let season_id = result.data.unwrap();

        if let Some(season_id) = result.data {
            tracing::info!("创建成功，合集 ID = {}", season_id);
        } else {
            panic!("返回数据为空");
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        let season = bpi.season_info(season_id).await?;

        let section_id = season.into_data()?.sections.sections.first().unwrap().id;

        tracing::info!("获取成功，section ID = {}", section_id);

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        bpi.season_delete(season_id).await?;

        Ok(())
    }
    #[tokio::test]
    async fn test_add_season() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let episodes = vec![EpisodeAdd {
            aid: TEST_AID,
            cid: TEST_CID,

            title: "测试合集内单集".to_string(),
            ..Default::default()
        }];

        let result = bpi.season_episodes_add(TEST_SECTION_ID, episodes).await?;
        tracing::info!("{:?}", result);

        Ok(())
    }
}
