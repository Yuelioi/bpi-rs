//! 获取我的信息
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/login/member_center.md

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// Bilibili 账号信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    /// 我的 mid（用户唯一 ID）
    pub mid: u64,

    /// 我的昵称
    pub uname: String,

    /// 我的用户名（登录用的 ID，不一定和昵称相同）
    pub userid: String,

    /// 我的个性签名
    pub sign: String,

    /// 我的生日（格式：YYYY-MM-DD）
    pub birthday: String,

    /// 我的性别
    /// 取值：
    /// - `"男"`
    /// - `"女"`
    /// - `"保密"`
    pub sex: String,

    /// 是否未设置昵称
    /// - `false`：已经设置过昵称
    /// - `true` ：未设置过昵称
    pub nick_free: bool,

    /// 我的会员等级
    /// 一般是字符串形式的数字，例如 `"0"`、`"6"`
    pub rank: String,
}

impl BpiClient {
    /// 获取我的账号信息
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/login
    pub async fn member_center_account_info(&self) -> Result<BpiResponse<AccountInfo>, BpiError> {
        let result = self
            .get("https://api.bilibili.com/x/member/web/account")
            .send_bpi("获取我的信息").await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_account_info() {
        let bpi = BpiClient::new();

        match bpi.member_center_account_info().await {
            Ok(resp) => {
                if resp.code == 0 {
                    let data = resp.data.unwrap();
                    tracing::info!("获取账号成功: mid={}, uname={}", data.mid, data.uname);
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
