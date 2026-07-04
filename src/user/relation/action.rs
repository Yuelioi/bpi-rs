//! B站用户关系操作相关接口
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
use serde::{Deserialize, Serialize};

// --- 响应数据结构体 ---

/// 操作用户关系响应数据
///
/// 该接口的响应 `data` 字段为 `null`，因此我们使用空元组 `()` 来表示。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModifyRelationResponseData;

// --- API 实现 ---

/// 操作代码
///
/// 用于 `act` 参数，定义了要执行的关系操作类型。
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum RelationAction {
    /// 关注
    Follow = 1,
    /// 取关
    Unfollow = 2,
    /// 悄悄关注（已下线）
    Whisper = 3,
    /// 取消悄悄关注
    Unwhisper = 4,
    /// 拉黑
    Blacklist = 5,
    /// 取消拉黑
    Unblacklist = 6,
    /// 踢出粉丝
    KickFan = 7,
}

/// 关注来源代码
///
/// 用于 `re_src` 参数，表示关注操作的来源。
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum RelationSource {
    /// 包月充电
    MonthlyCharge = 1,
    /// 个人空间
    Space = 11,
    /// 视频
    Video = 14,
    /// 评论区
    Comment = 15,
    /// 视频播放器结束页面
    VideoEndPage = 17,
    /// H5推荐关注
    H5Recommend = 58,
    /// H5关注列表
    H5FollowingList = 106,
    /// H5粉丝列表
    H5FanList = 107,
    /// 专栏
    Article = 115,
    /// 私信
    Message = 118,
    /// 搜索
    Search = 120,
    /// 视频播放器左上角关注按钮
    VideoPlayerButton = 164,
    /// H5共同关注
    H5CommonFollow = 167,
    /// 创作激励计划
    CreativeIncentive = 192,
    /// 活动页面
    ActivityPage = 222,
    /// 联合投稿视频
    JointVideo = 229,
    /// 消息中心点赞详情
    MessageCenterLike = 235,
    /// 视频播放器关注弹幕
    VideoPlayerDanmaku = 245,
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {}
