use serde::{Deserialize, Serialize};

use crate::dynamic::params::{DynamicLiveUsersParams, DynamicUpUsersParams};
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

/// 直播的已关注者列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiveUser {
    /// 直播者头像 URL
    pub face: String,
    /// 直播链接
    pub link: String,
    /// 直播标题
    pub title: String,
    /// 直播者 ID
    pub uid: u64,
    /// 直播者昵称
    pub uname: String,
}

/// 正在直播的已关注者响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiveUsersData {
    /// 直播者数量
    pub count: u64,
    /// 作用尚不明确
    pub group: String,
    /// 直播者列表
    pub items: Vec<LiveUser>,
}

/// 发布新动态的已关注者列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynUpUser {
    pub user_profile: UserProfile,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserProfile {
    pub info: UserInfo,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
    pub uid: u64,
    pub uname: String,
    pub face: String,
}

/// 发布新动态的已关注者响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynUpUsersData {
    /// 作用尚不明确
    pub button_statement: String,
    /// 更新者列表
    pub items: Vec<DynUpUser>,
}

impl BpiClient {
    /// 获取正在直播的已关注者
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`DynamicLiveUsersParams`] | 直播关注者列表参数 |
    pub async fn dynamic_live_users(
        &self,
        params: DynamicLiveUsersParams,
    ) -> Result<BpiResponse<LiveUsersData>, BpiError> {
        self.get("https://api.vc.bilibili.com/dynamic_svr/v1/dynamic_svr/w_live_users")
            .query(&params.query_pairs())
            .send_bpi("获取正在直播的已关注者")
            .await
    }

    /// 获取发布新动态的已关注者
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`DynamicUpUsersParams`] | 新动态关注者列表参数 |
    pub async fn dynamic_up_users(
        &self,
        params: DynamicUpUsersParams,
    ) -> Result<BpiResponse<DynUpUsersData>, BpiError> {
        self.get("https://api.vc.bilibili.com/dynamic_svr/v1/dynamic_svr/w_dyn_uplist")
            .query(&params.query_pairs())
            .send_bpi("获取发布新动态的已关注者")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    // 您需要在 `Cargo.toml` 中添加 `dotenvy` 和 `tracing` 依赖，并在 `main.rs` 或测试入口处初始化日志
    // 例如: tracing_subscriber::fmt::init();

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_live_users() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi
            .dynamic_live_users(DynamicLiveUsersParams::new().with_size(1)?)
            .await?;
        let data = resp.into_data()?;

        info!("直播中的关注者数量: {}", data.count);
        info!("第一位直播中的关注者: {:?}", data.items.first());

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_dyn_up_users() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi.dynamic_up_users(DynamicUpUsersParams::new()).await?;
        let data = resp.into_data()?;

        info!("发布新动态的关注者列表: {:?}", data.items);
        assert!(!data.items.is_empty());

        Ok(())
    }
}
