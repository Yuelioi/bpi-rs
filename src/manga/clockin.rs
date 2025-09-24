//! 签到
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/manga/ClockIn.md)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// ================= 数据结构 =================

/// 补签请求参数
#[derive(Debug, Clone, Serialize)]
pub struct ClockInMakeupRequest {
    /// 补签类型
    pub r#type: i32,
    /// 补签日期，格式：YYYY-MM-DD
    pub date: String,
}

/// 签到状态信息中的积分信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PointInfo {
    /// 签到可获取积分
    pub point: i32,
    /// 原始积分
    pub origin_point: i32,
    /// 是否为活动
    pub is_activity: bool,
    /// 签到奖励描述
    pub title: String,
}

/// 签到状态信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ClockInInfoData {
    /// 连续签到天数
    pub day_count: i32,
    /// 今日是否已签到，0：未签到，1：已签到
    pub status: i32,
    /// 一次签到周期中每次签到可获得点数
    pub points: Vec<i32>,
    /// 积分图标
    pub credit_icon: String,
    /// 签到前图标
    pub sign_before_icon: String,
    /// 今日签到图标
    pub sign_today_icon: String,
    /// 呼吸图标
    pub breathe_icon: String,
    /// 新积分图标
    #[serde(default)]
    pub new_credit_x_icon: String,
    /// 优惠券图片
    #[serde(default)]
    pub coupon_pic: String,
    /// 积分信息
    pub point_infos: Vec<PointInfo>,
}

pub type ClockInInfoResponse = BpiResponse<ClockInInfoData>;

// ================= 实现 =================

impl BpiClient {
    /// 漫画签到
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/manga)
    pub async fn manga_clock_in(&self) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let params = [("platform", "android")];
        self
            .post("https://manga.bilibili.com/twirp/activity.v1.Activity/ClockIn")
            .form(&params)
            .send_bpi("漫画签到").await
    }

    /// 漫画补签
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/manga)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `date` | &str | 补签日期，YYYY-MM-DD |
    pub async fn manga_clock_in_makeup(
        &self,
        date: &str
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let params = ClockInMakeupRequest {
            r#type: 0,
            date: date.to_string(),
        };
        self
            .post("https://manga.bilibili.com/twirp/activity.v1.Activity/ClockIn?platform=android")
            .json(&params)
            .send_bpi("漫画补签").await
    }

    /// 获取漫画签到信息
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/manga)
    pub async fn manga_clock_in_info(&self) -> Result<ClockInInfoResponse, BpiError> {
        self
            .post("https://manga.bilibili.com/twirp/activity.v1.Activity/GetClockInInfo")
            .send_bpi("获取漫画签到信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manga_clock_in() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let result = bpi.manga_clock_in().await;
        match result {
            Ok(_) => tracing::info!("签到成功"),
            Err(error) => tracing::error!("{:#?}", error),
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_get_manga_clock_in_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let result = bpi.manga_clock_in_info().await?;

        let data = result.into_data()?;

        assert!(data.day_count >= 0);
        assert!(data.status == 0 || data.status == 1);
        assert_eq!(data.points.len(), 7); // 一周7天
        assert!(!data.point_infos.is_empty());

        Ok(())
    }
}
