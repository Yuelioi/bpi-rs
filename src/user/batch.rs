//! B站用户批量信息相关接口
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// UID 查询返回的单个条目
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NameUidItem {
    /// 用户名
    pub name: String,
    /// 用户 mid
    pub uid: String,
}

/// 批量用户名查 UID 的数据本体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NameToUidData {
    pub uid_list: Vec<NameUidItem>,
}

impl BpiClient {
    /// 批量查询用户名对应的 UID
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user
    ///
    /// 参数
    /// - `names`: 用户名列表，多个用户名以逗号分隔
    pub async fn user_name_to_uid(
        &self,
        names: &[&str],
    ) -> Result<BpiResponse<NameToUidData>, BpiError> {
        let names_str = names.join(",");

        self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/name-to-uid")
            .query(&[("names", names_str)])
            .send_bpi("用户名查 UID")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_user_name_to_uid() {
        let bpi = BpiClient::new();
        let resp = bpi.user_name_to_uid(&["LexBurner", "某科学"]).await;
        assert!(resp.is_ok());
        if let Ok(r) = resp {
            info!("用户名查 UID 返回: {:?}", r);
        }
    }
}
