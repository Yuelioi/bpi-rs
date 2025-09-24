//! B站用户关注分组相关接口
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 关注分组
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FollowTag {
    pub tagid: i64, // 分组 id (-10: 特别关注, 0: 默认分组)
    pub name: String, // 分组名称
    pub count: i64, // 分组成员数
    pub tip: Option<String>, // 提示信息
}

impl BpiClient {
    /// 查询关注分组列表
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
    pub async fn user_follow_tags(&self) -> Result<BpiResponse<Vec<FollowTag>>, BpiError> {
        self.get("https://api.bilibili.com/x/relation/tags").send_bpi("查询关注分组列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_user_follow_tags_cookie() {
        let bpi = BpiClient::new();
        let resp = bpi.user_follow_tags().await;
        assert!(resp.is_ok());

        let data = resp.unwrap().data.unwrap();
        info!("关注分组列表: {:?}", data);
    }
}
