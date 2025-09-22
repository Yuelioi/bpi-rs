//! 专栏点赞&投币&收藏
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/article/action.md

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

/// 投币响应数据
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CoinResponseData {
    /// 是否点赞成功 true：成功 false：失败 已赞过则附加点赞失败
    pub like: bool,
}

impl BpiClient {
    /// 点赞文章
    ///
    /// # 参数
    /// | 名称   | 类型  | 说明                     |
    /// | ------ | ----- | ------------------------ |
    /// | `id`   | u64   | 文章 cvid (必要)         |
    /// | `like` | bool  | 是否点赞，`true` 点赞，`false` 取消点赞 |
    ///
    /// # 文档
    /// [点赞文章](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/article/action.md#点赞文章)
    pub async fn article_like(
        &self,
        id: u64,
        like: bool,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let r#type = if like { 1 } else { 2 };

        let result = self
            .post("https://api.bilibili.com/x/article/like")
            .form(&[
                ("id", id.to_string()),
                ("type", r#type.to_string()),
                ("csrf", csrf),
            ])
            .send_bpi("点赞文章")
            .await?;

        Ok(result)
    }

    /// 投币文章
    ///
    /// # 参数
    /// | 名称       | 类型  | 说明                        |
    /// | ---------- | ----- | --------------------------- |
    /// | `aid`      | i64   | 文章 cvid (必要)            |
    /// | `upid`     | i64   | 文章作者 mid (必要)         |
    /// | `multiply` | u32   | 投币数量 (必要，上限为 2)   |
    ///
    /// # 文档
    /// [投币文章](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/article/action.md#投币文章)
    pub async fn article_coin(
        &self,
        aid: u64,
        upid: u64,
        multiply: u32,
    ) -> Result<BpiResponse<CoinResponseData>, BpiError> {
        let multiply = multiply.min(2);
        let csrf = self.csrf()?;

        let result = self
            .post("https://api.bilibili.com/x/web-interface/coin/add")
            .form(&[
                ("aid", aid.to_string()),
                ("upid", upid.to_string()),
                ("multiply", multiply.to_string()),
                ("avtype", "2".to_string()),
                ("csrf", csrf),
            ])
            .send_bpi("投币文章")
            .await?;

        Ok(result)
    }

    /// 收藏文章
    ///
    /// # 参数
    /// | 名称   | 类型  | 说明             |
    /// | ------ | ----- | ---------------- |
    /// | `id`   | u64   | 文章 cvid (必要) |
    ///
    /// # 文档
    /// [收藏文章](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/article/action.md#收藏文章)
    pub async fn article_favorite(
        &self,
        id: u64,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let result = self
            .post("https://api.bilibili.com/x/article/favorites/add")
            .form(&[("id", id.to_string()), ("csrf", csrf)])
            .send_bpi("收藏文章")
            .await?;

        Ok(result)
    }

    /// 收藏文章
    ///
    /// # 参数
    /// | 名称   | 类型  | 说明             |
    /// | ------ | ----- | ---------------- |
    /// | `id`   | i64   | 文章 cvid (必要) |
    ///
    /// # 文档
    /// [收藏文章](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/article/action.md#收藏文章)
    pub async fn article_unfavorite(
        &self,
        id: i64,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let result = self
            .post("https://api.bilibili.com/x/article/favorites/del")
            .form(&[("id", id.to_string()), ("csrf", csrf)])
            .send_bpi("收藏文章")
            .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CVID: u64 = 1;
    const TEST_UID: u64 = 91221505;
    #[tokio::test]
    async fn test_like_article() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        bpi.article_like(TEST_CVID, true)
            .await
            .map(|_| ())
            .or_else(|e| {
                if e.code() == Some(65006) {
                    Ok(())
                } else {
                    Err(Box::new(e))
                }
            })
    }

    #[tokio::test]
    async fn test_coin_article() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let multiply = 1;

        bpi.article_coin(TEST_CVID, TEST_UID, multiply)
            .await
            .map(|_| ())
            .or_else(|e| {
                if e.code() == Some(34005) {
                    Ok(())
                } else {
                    Err(Box::new(e))
                }
            })
    }

    #[tokio::test]
    async fn test_favorite_article() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        bpi.article_favorite(TEST_CVID).await?;
        Ok(())
    }
}
