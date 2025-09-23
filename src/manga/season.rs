//! 漫画赛季
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/manga/Season.md

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// ================= 数据结构 =================

/// 赛季任务信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SeasonTask {
    // 任务相关字段
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    // 其他字段根据实际需要添加
}

/// 赛季奖励信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SeasonWelfare {
    // 奖励相关字段
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub title: String,
    // 其他字段根据实际需要添加
}

/// 赛季文案信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SeasonText {
    // 文案相关字段
    #[serde(default)]
    pub title: String,
    // 其他字段根据实际需要添加
}

/// 赛季排名信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SeasonRank {
    // 排名相关字段
    // 根据实际需要添加
}

/// 赛季信息数据
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SeasonInfoData {
    /// 当前时间字符串，ISO 8601格式
    pub current_time: String,
    /// 赛季开始时间，ISO 8601格式
    pub start_time: String,
    /// 赛季结束时间，ISO 8601格式
    pub end_time: String,
    /// 拥有积分，未登录为0
    pub remain_amount: i32,
    /// 第几个赛季
    pub season_id: String,
    /// 待领取奖励的任务，未登录/没有可领取时为空数组
    pub tasks: Vec<SeasonTask>,
    /// 赛季奖励
    pub welfare: Vec<SeasonWelfare>,
    /// 版头图片
    pub cover: String,
    /// 今日的任务完成情况
    pub today_tasks: Vec<SeasonTask>,
    /// 赛季相关文案，未登录为null
    #[serde(default)]
    pub text: Option<SeasonText>,
    /// 赛季标题
    pub season_title: String,
    /// 排名信息
    #[serde(default)]
    pub rank: Option<SeasonRank>,
    // 其他字段根据实际需要添加
}

pub type SeasonInfoResponse = BpiResponse<SeasonInfoData>;

// ================= 实现 =================

impl BpiClient {
    /// 获取漫画赛季信息
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/manga
    pub async fn manga_season_info(&self) -> Result<SeasonInfoResponse, BpiError> {
        self
            .post("https://manga.bilibili.com/twirp/user.v1.Season/GetSeasonInfo")
            .send_bpi("获取漫画赛季信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_manga_season_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let result = bpi.manga_season_info().await?;

        // 不需要登录也可以获取基本信息

        let data = result.into_data()?;

        tracing::info!("{:#?}", data);

        Ok(())
    }
}
