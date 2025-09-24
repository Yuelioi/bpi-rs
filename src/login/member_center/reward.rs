//! 查询每日奖励状态
//!
//! [文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/login/member_center.html#查询每日奖励状态)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 每日奖励状态信息体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReward {
    /// 每日登录奖励状态
    /// - false：未完成
    /// - true：已完成，完成奖励5经验
    pub login: bool,
    /// 每日观看奖励状态
    /// - false：未完成
    /// - true：已完成，完成奖励5经验
    pub watch: bool,
    /// 每日投币奖励经验（上限50）
    /// 注：该值更新存在延迟
    pub coins: u32,
    /// 每日分享奖励状态
    /// - false：未完成
    /// - true：已完成，完成奖励5经验
    pub share: bool,
    /// 绑定邮箱奖励状态
    /// - false：未完成
    /// - true：已完成，首次完成奖励20经验
    pub email: bool,
    /// 绑定手机号奖励状态
    /// - false：未完成
    /// - true：已完成，首次完成奖励100经验
    pub tel: bool,
    /// 设置密保问题奖励状态
    /// - false：未完成
    /// - true：已完成，首次完成奖励30经验
    pub safe_question: bool,
    /// 实名认证奖励状态
    /// - false：未完成
    /// - true：已完成，首次完成奖励50经验
    pub identify_card: bool,
}

impl BpiClient {
    /// 查询每日奖励状态
    pub async fn member_center_daily_reward(&self) -> Result<BpiResponse<DailyReward>, BpiError> {
        self
            .get("https://api.bilibili.com/x/member/web/exp/reward")
            .header("Referer", "")
            .send_bpi("查询每日奖励状态").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_daily_reward() -> Result<(), BpiError> {
        let bpi = BpiClient::new();

        let result = bpi.member_center_daily_reward().await?;
        println!("{:?}", result);
        Ok(())
    }
}
