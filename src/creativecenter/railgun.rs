//! 电磁力等级 API
//!
//! [参考文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/railgun.md)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 电磁力等级信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectromagneticInfo {
    /// 当前用户 mid
    pub mid: u64,
    /// 电磁力等级
    pub level: u32,
    /// 电磁力分数
    pub score: u32,
    /// 信用分
    pub credit: u32,
    /// 状态 (文档不明，返回固定 2)
    pub state: i32,
}

impl BpiClient {
    /// 获取电磁力等级
    ///
    /// 获取当前用户的电磁力等级信息，包括等级、分数、信用分等。
    ///
    /// # 文档
    /// [获取电磁力等级](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/railgun.md#获取电磁力等级)
    pub async fn up_electromagnetic_info(
        &self
    ) -> Result<BpiResponse<ElectromagneticInfo>, BpiError> {
        self
            .get("https://api.bilibili.com/studio/up-rating/v3/rating/info")
            .send_bpi("获取电磁力等级").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_electromagnetic_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let data = bpi.up_electromagnetic_info().await?.into_data()?;

        tracing::info!(
            "mid={}, level={}, score={}, credit={}, state={}",
            data.mid,
            data.level,
            data.score,
            data.credit,
            data.state
        );

        Ok(())
    }
}
