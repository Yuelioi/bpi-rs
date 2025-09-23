use serde::{Deserialize, Serialize};

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

// ================= 数据结构 =================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct RedPocketAward {
    /// 礼物id
    pub gift_id: i64,
    /// 数量
    pub num: i32,
    /// 礼物名称
    pub gift_name: String,
    /// 礼物图片
    pub gift_pic: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PopularityRedPocket {
    /// 红包id
    pub lot_id: i64,
    /// 红包发送者uid
    pub sender_uid: i64,
    /// 红包发送者昵称
    pub sender_name: String,
    /// 红包发送者头像
    pub sender_face: String,
    /// 参与条件
    pub join_requirement: i32,
    /// 参与红包时自动发送的弹幕内容
    pub danmu: String,
    /// 红包内容
    pub awards: Vec<RedPocketAward>,
    /// 开始时间
    pub start_time: i64,
    /// 结束时间
    pub end_time: i64,
    /// 持续时间
    pub last_time: i64,
    /// 移除时间
    pub remove_time: i64,
    /// 替换时间
    pub replace_time: i64,
    /// 当前时间
    pub current_time: i64,
    /// 红包状态
    pub lot_status: i32,
    /// 红包界面
    pub h5_url: String,
    /// 用户是否已参与
    pub user_status: i32,
    /// 红包配置id
    pub lot_config_id: i64,
    /// 红包总计价格
    pub total_price: i64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ActivityBoxInfo {
    // 根据实际情况添加字段
    #[serde(flatten)]
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LotteryInfoData {
    /// 人气红包信息
    pub popularity_red_pocket: Option<Vec<PopularityRedPocket>>,
    /// 活动盒子信息
    pub activity_box_info: Option<ActivityBoxInfo>,
    // 其他可能的字段
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

pub type LotteryInfoResponse = BpiResponse<LotteryInfoData>;

// ================= 实现 =================

impl BpiClient {
    /// 获取指定直播间的红包信息
    ///

    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `room_id` | i64 | 直播间 ID |
    pub async fn live_lottery_info(&self, room_id: i64) -> Result<LotteryInfoResponse, BpiError> {
        let params = [("roomid", room_id.to_string())];

        let resp: LotteryInfoResponse = self
      .get("https://api.live.bilibili.com/xlive/lottery-interface/v1/lottery/getLotteryInfoWeb")
      .query(&params)
      .send_bpi("获取指定直播间的红包信息").await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_live_lottery_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        bpi.live_lottery_info(23174842).await?;

        // 注意：直播间可能没有红包，所以不做额外断言
        Ok(())
    }
}
