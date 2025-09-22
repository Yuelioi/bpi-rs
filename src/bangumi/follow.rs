//! 追番相关
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/follow.md
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiFollowResult {
    pub fmid: i64,
    pub relation: bool,
    pub status: i32,
    pub toast: String,
}

impl BpiClient {
    /// 追番
    ///
    /// # 参数
    /// * `season_id` - 剧集ssid
    ///
    /// # 文档
    /// [追番](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/follow.md#追番)
    pub async fn bangumi_follow(
        &self,
        season_id: u64,
    ) -> Result<BpiResponse<BangumiFollowResult>, BpiError> {
        let csrf = self.csrf()?;
        self.post("https://api.bilibili.com/pgc/web/follow/add")
            .with_bilibili_headers()
            .form(&[
                ("season_id", season_id.to_string()),
                ("csrf", csrf.to_string()),
            ])
            .send_bpi("追番")
            .await
    }

    /// 取消追番
    ///
    /// # 参数
    /// * `season_id` - 剧集ssid
    /// # 文档
    /// [取消追番](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/follow.md#取消追番)
    pub async fn bangumi_unfollow(
        &self,
        season_id: u64,
    ) -> Result<BpiResponse<BangumiFollowResult>, BpiError> {
        let csrf = self.csrf()?;
        self.post("https://api.bilibili.com/pgc/web/follow/del")
            .with_bilibili_headers()
            .form(&[
                ("season_id", season_id.to_string()),
                ("csrf", csrf.to_string()),
            ])
            .send_bpi("取消追番")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_BANGUMI_ID: u64 = 99644; // 小城日常
    #[tokio::test]
    async fn test_follow_bangumi() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.bangumi_follow(TEST_BANGUMI_ID).await?;

        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert_eq!(data.toast, "自己追的番就要好好看完哟^o^");

        Ok(())
    }

    #[tokio::test]
    async fn test_unfollow_bangumi() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.bangumi_unfollow(TEST_BANGUMI_ID).await?;

        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        assert_eq!(data.toast, "已取消追番");

        Ok(())
    }
}
