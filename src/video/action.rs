//! B站视频交互接口(Web端)
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)

use crate::BpiResponse;
use serde::{Deserialize, Serialize};

/// 点赞视频 - 请求参数
#[derive(Debug, Clone, Serialize)]
pub struct LikeRequest {
    /// 稿件 avid （aid 与 bvid 任选一个）
    pub aid: Option<u64>,
    /// 稿件 bvid （aid 与 bvid 任选一个）
    pub bvid: Option<String>,
    /// 操作方式 (1: 点赞, 2: 取消赞)
    pub like: u8,
}

/// 投币视频 - 请求参数
#[derive(Debug, Clone, Serialize)]
pub struct CoinRequest {
    /// 稿件 avid
    pub aid: Option<u64>,
    /// 稿件 bvid
    pub bvid: Option<String>,
    /// 投币数量 (上限为 2)
    pub multiply: u8,
    /// 是否附加点赞 (0: 不点赞, 1: 点赞)，默认为 0
    pub select_like: Option<u8>,
}

/// 投币视频 - 响应结构体
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CoinData {
    /// 是否点赞成功
    pub like: bool,
}

/// 收藏视频 - 响应结构体
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FavoriteData {
    /// 是否为未关注用户收藏
    pub prompt: bool,
    /// 作用不明确
    pub ga_data: Option<serde_json::Value>,
    /// 提示消息
    pub toast_msg: Option<String>,
    /// 成功数
    pub success_num: u32,
}

pub type FavoriteResponse = BpiResponse<FavoriteData>;

#[cfg(test)]
mod tests {}
