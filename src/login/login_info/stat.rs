//! 登录用户状态数（双端）
//!
//! [查看 API 文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/login/login_info.html#登录用户状态数-双端)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 登录用户状态数 - 信息体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStat {
    /// 当前关注数
    pub following: u64,
    /// 当前粉丝数
    pub follower: u64,
    /// 发布的动态数
    pub dynamic_count: u64,
}

impl BpiClient {
    /// 获取登录用户状态数（关注/粉丝/动态）
    pub async fn login_info_user_stat(&self) -> Result<BpiResponse<UserStat>, BpiError> {
        let result = self
            .get("https://api.bilibili.com/x/web-interface/nav/stat")
            .send_bpi("获取登录用户状态").await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_user_stat() {
        let bpi = BpiClient::new();

        match bpi.login_info_user_stat().await {
            Ok(resp) => {
                if resp.code == 0 {
                    let data = resp.data.unwrap();

                    tracing::info!(
                        "关注数: {}, 粉丝数: {}, 动态数: {}",
                        data.following,
                        data.follower,
                        data.dynamic_count
                    );
                } else {
                    tracing::info!("请求失败: code={}, message={}", resp.code, resp.message);
                }
            }
            Err(err) => {
                panic!("请求出错: {}", err);
            }
        }
    }
}
