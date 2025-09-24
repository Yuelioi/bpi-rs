//! 根据 aid 反查合集信息 API
//!
//! [参考文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season.md)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 合集信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonInfoData {
    /// 合集 ID
    pub id: u64,
    /// 合集标题
    pub title: String,
    /// 合集描述
    pub desc: Option<String>,
    /// 合集封面 URL
    pub cover: String,

    /// 是否已完结
    /// 0: 未完结 | 1: 已完结
    #[serde(rename = "isEnd")]
    pub is_end: u32,

    /// 合集作者 ID
    pub mid: u64,

    /// 是否为活动合集
    /// 0: 否 | 1: 是
    #[serde(rename = "isAct")]
    pub is_act: u32,

    /// 是否付费
    /// 0: 否 | 1: 是
    pub is_pay: u32,

    /// 合集状态
    /// 0: 正常显示 | -6: 正在审核
    pub state: i32,

    /// 合集分段状态
    /// 0: 正常
    #[serde(rename = "partState")]
    pub part_state: u32,

    /// 合集签名状态
    /// 0: 正常
    #[serde(rename = "signState")]
    pub sign_state: u32,

    /// 合集拒绝原因
    #[serde(rename = "rejectReason")]
    pub reject_reason: Option<String>,

    /// 创建时间 (UNIX 时间戳)
    pub ctime: u64,
    /// 修改时间 (UNIX 时间戳)
    pub mtime: u64,

    /// 是否设小节
    /// 1: 不设小节
    pub no_section: u32,

    /// 合集是否禁止
    /// 0: 否 | 1: 是
    pub forbid: u32,

    /// 协议 ID
    pub protocol_id: Option<String>,

    /// 视频数量
    pub ep_num: u32,

    /// 合集价格
    /// 0: 免费
    pub season_price: u32,

    /// 是否公开
    /// 1: 公开 | 0: 不公开
    pub is_opened: u32,

    /// 是否充电付费
    /// 0: 否 | 1: 是
    pub has_charging_pay: u32,

    /// 是否 PUGV 付费
    /// 0: 否 | 1: 是
    pub has_pugv_pay: u32,
}

impl BpiClient {
    /// 根据视频 aid 查询所属合集信息
    ///
    /// 通过视频 aid 查询该视频所属的合集信息。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `aid` | u64 | 视频 aid |
    ///
    /// # 文档
    /// [根据 aid 反查合集信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/season/aid.md#根据-aid-反查合集信息)
    pub async fn season_by_aid(&self, aid: u64) -> Result<BpiResponse<SeasonInfoData>, BpiError> {
        self
            .get("https://member.bilibili.com/x2/creative/web/season/aid")
            .query(&[("id", aid.to_string())])
            .send_bpi("根据 aid 查询合集").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_AID: u64 = 113602455409683;

    #[tokio::test]
    async fn test_season_by_aid() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let data = bpi.season_by_aid(TEST_AID).await?.into_data()?;
        tracing::info!("视频 {} 所属合集 {} - {}", TEST_AID, data.id, data.title);

        Ok(())
    }
}
