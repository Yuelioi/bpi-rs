//! 番剧或影视时间线
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/timeline.md
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 番剧类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BangumiTimelineType {
    /// 番剧
    Anime = 1,
    /// 电影
    Movie = 3,
    /// 国创
    ChineseAnimation = 4,
}

impl BangumiTimelineType {
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiTimelineDay {
    pub date: String,
    pub date_ts: i64,
    pub day_of_week: i32,
    pub episodes: Vec<BangumiTimelineEpisode>,
    pub is_today: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiTimelineEpisode {
    pub cover: String,
    pub delay: i32,
    pub delay_id: i64,
    pub delay_index: String,
    pub delay_reason: String,
    pub ep_cover: String,
    pub episode_id: i64,
    pub pub_index: String,
    pub pub_time: String,
    pub pub_ts: i64,
    pub published: i32,
    pub follows: String,
    pub plays: String,
    pub season_id: i64,
    pub square_cover: String,
    pub title: String,
}

impl BpiClient {
    /// 获取番剧或影视时间线
    ///
    /// # 参数
    /// * `types` - 类别（1：番剧，3：电影，4：国创）
    /// * `before` - 开始于前几日（0-7）
    /// * `after` - 结束于后几日（0-7）
    /// # 文档
    /// [获取番剧或影视时间线](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/timeline.md#获取番剧或影视时间线)
    pub async fn bangumi_timeline(
        &self,
        types: BangumiTimelineType,
        before: i32,
        after: i32,
    ) -> Result<BpiResponse<Vec<BangumiTimelineDay>>, BpiError> {
        // 验证参数
        if before < 0 || before > 7 {
            return Err(BpiError::InvalidParameter {
                field: "before",
                message: "before参数必须在0-7之间",
            });
        }
        if after < 0 || after > 7 {
            return Err(BpiError::InvalidParameter {
                field: "after",
                message: "after参数必须在0-7之间",
            });
        }

        self.get("https://api.bilibili.com/pgc/web/timeline")
            .query(&[
                ("types", types.as_i32().to_string()),
                ("before", before.to_string()),
                ("after", after.to_string()),
            ])
            .send_bpi("获取番剧或影视时间线")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bangumi_timeline() {
        let bpi = BpiClient::new();
        let result = bpi.bangumi_timeline(BangumiTimelineType::Anime, 3, 7).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.code, 0);
        let data = response.data.unwrap();

        assert!(!data.is_empty());
        for day in &data {
            assert!(!day.date.is_empty());
            assert!(day.day_of_week >= 1 && day.day_of_week <= 7);
            assert!(!day.episodes.is_empty());
            for episode in &day.episodes {
                assert!(!episode.title.is_empty());
                assert!(episode.season_id > 0);
            }
        }
    }

    #[tokio::test]
    async fn test_bangumi_timeline_invalid_before() {
        let bpi = BpiClient::new();
        let result = bpi.bangumi_timeline(BangumiTimelineType::Anime, 8, 7).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        match error {
            BpiError::InvalidParameter { field, message } => {
                assert_eq!(field, "before");
                assert_eq!(message, "before参数必须在0-7之间");
            }
            _ => panic!("Expected InvalidParameter error"),
        }
    }

    #[tokio::test]
    async fn test_bangumi_timeline_invalid_after() {
        let bpi = BpiClient::new();
        let result = bpi.bangumi_timeline(BangumiTimelineType::Anime, 3, 8).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        match error {
            BpiError::InvalidParameter { field, message } => {
                assert_eq!(field, "after");
                assert_eq!(message, "after参数必须在0-7之间");
            }
            _ => panic!("Expected InvalidParameter error"),
        }
    }

    #[test]
    fn test_bangumi_timeline_type() {
        assert_eq!(BangumiTimelineType::Anime.as_i32(), 1);
        assert_eq!(BangumiTimelineType::Movie.as_i32(), 3);
        assert_eq!(BangumiTimelineType::ChineseAnimation.as_i32(), 4);
    }
}
