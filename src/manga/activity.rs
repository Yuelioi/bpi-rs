//! 漫画任务操作
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/manga/Activity.md

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// ================= 数据结构 =================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ShareComicData {
    /// 获取积分
    pub point: i32,
}

pub type ShareComicResponse = BpiResponse<ShareComicData>;

// ================= 实现 =================

impl BpiClient {
    /// 分享漫画获取积分
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/manga
    pub async fn manga_share_comic(&self) -> Result<ShareComicResponse, BpiError> {
        let params = [("platform", "android")];
        self
            .post("https://manga.bilibili.com/twirp/activity.v1.Activity/ShareComic")
            .form(&params)
            .send_bpi("分享漫画").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_share_comic() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let result = bpi.manga_share_comic().await?;

        // 可能是成功获取积分，也可能是今日已分享

        // 如果是成功获取积分，则data存在且point为5
        let data = result.into_data()?;

        assert_eq!(data.point, 5);

        Ok(())
    }
}
