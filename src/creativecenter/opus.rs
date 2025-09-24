//! 创作中心作品管理 API
//!
//! [参考文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/opus.md)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde_json::json;

impl BpiClient {
    /// 删除动态
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `dyn_id` | &str | 动态 ID |
    ///
    /// # 文档
    /// [删除动态](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/opus.md#删除动态)
    pub async fn dynamic_delete(
        &self,
        dyn_id: &str
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        self
            .post("https://api.bilibili.com/x/dynamic/feed/operate/remove")
            .query(&[("csrf", csrf)])
            .json(&json!({
              "dyn_id_str": dyn_id
            }))
            .send_bpi("删除动态").await
    }

    /// 删除专栏
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `aid` | u64 | 专栏文章 ID |
    ///
    /// # 文档
    /// [删除专栏](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/opus.md#删除专栏)
    pub async fn article_delete(
        &self,
        aid: u64
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        self
            .post("https://member.bilibili.com/x/web/article/delete")
            .form(
                &[
                    ("aid", aid.to_string()),
                    ("csrf", csrf),
                ]
            )
            .send_bpi("删除专栏").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DYN_ID: &str = "951560312615600129";
    const TEST_AID: u64 = 42997969;

    #[ignore]
    #[tokio::test]
    async fn test_dynamic_delete() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        bpi.dynamic_delete(TEST_DYN_ID).await?;

        Ok(())
    }
    #[ignore]
    #[tokio::test]
    async fn test_article_delete() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        bpi.article_delete(TEST_AID).await?;

        Ok(())
    }
}
