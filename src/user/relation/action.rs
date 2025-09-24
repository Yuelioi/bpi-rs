//! B站用户关系操作相关接口
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

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

impl BpiClient {
    /// 操作用户关系
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
    ///
    /// # 参数
    /// | 名称      | 类型                | 说明                       |
    /// | --------- | -------------------| -------------------------- |
    /// | `fid`     | u64                | 目标用户 mid               |
    /// | `action`  | RelationAction     | 操作代码，见 RelationAction 枚举 |
    /// | `source`  | `Option<RelationSource>` | 关注来源代码，可选，见 RelationSource 枚举 |
    pub async fn user_modify_relation(
        &self,
        fid: u64,
        action: RelationAction,
        source: Option<RelationSource>
    ) -> Result<BpiResponse<()>, BpiError> {
        let csrf = self.csrf()?;
        let mut form = reqwest::multipart::Form
            ::new()
            .text("fid", fid.to_string())
            .text("act", (action as u8).to_string())
            .text("csrf", csrf.to_string());

        if let Some(s) = source {
            form = form.text("re_src", (s as u32).to_string());
        }

        self
            .post("https://api.bilibili.com/x/relation/modify")
            .multipart(form)
            .send_bpi("操作用户关系").await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    const TEST_FID: u64 = 2;

    #[tokio::test]
    async fn test_modify_relation_follow() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.user_modify_relation(
            TEST_FID,
            RelationAction::Follow,
            Some(RelationSource::Space)
        ).await?;

        info!("关注用户结果: {:?}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_modify_relation_unfollow() -> Result<(), BpiError> {
        let bpi = BpiClient::new();

        let resp = bpi.user_modify_relation(TEST_FID, RelationAction::Unfollow, None).await?;

        info!("取关用户结果: {:?}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_modify_relation_blacklist() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.user_modify_relation(TEST_FID, RelationAction::Blacklist, None).await?;

        info!("拉黑用户结果: {:?}", resp);

        let _ = bpi.user_modify_relation(TEST_FID, RelationAction::Unblacklist, None).await;

        Ok(())
    }
}
