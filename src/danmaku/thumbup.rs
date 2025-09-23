//! 弹幕点赞查询
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/danmaku/thumbup.md

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThumbupStatsItem {
    /// 对应弹幕所获得的点赞数
    pub likes: i64,
    /// 当前用户是否点赞
    pub user_like: i32,
    pub id_str: String,
}

pub type ThumbupStatsMap = HashMap<String, ThumbupStatsItem>;

pub type ThumbupStatsResponse = BpiResponse<ThumbupStatsMap>;

impl BpiClient {
    /// 查询指定 dmid 的点赞状态与统计
    pub async fn danmaku_thumbup_stats(
        &self,
        oid: i64,
        ids: &[i64]
    ) -> Result<ThumbupStatsResponse, BpiError> {
        let ids_join = ids
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let params = vec![("oid", oid.to_string()), ("ids", ids_join)];

        let resp: ThumbupStatsResponse = self
            .get("https://api.bilibili.com/x/v2/dm/thumbup/stats")
            .query(&params)
            .send_bpi("查询弹幕点赞状态").await?;

        Ok(resp)
    }
}
