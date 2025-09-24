//! B站视频交互接口(Web端)
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)

use std::collections::HashMap;

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

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

impl BpiClient {
    /// 点赞/取消点赞
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/video/action.md)
    ///
    /// # 参数
    /// | 名称   | 类型           | 说明                 |
    /// | ------ | --------------| -------------------- |
    /// | `aid`  | `Option<u64>`   | 稿件 avid，可选      |
    /// | `bvid` | `Option<String>`| 稿件 bvid，可选      |
    /// | `like` | u8            | 操作方式 (1:点赞, 2:取消) |
    pub async fn video_like(
        &self,
        aid: Option<u64>,
        bvid: Option<String>,
        like: u8
    ) -> Result<BpiResponse<CoinData>, BpiError> {
        let csrf = self.csrf()?;

        let result = self
            .post("https://api.bilibili.com/x/web-interface/archive/like")
            .with_bilibili_headers()
            .form(
                &[
                    ("aid", aid.unwrap_or(0).to_string()),
                    ("bvid", bvid.unwrap_or("".to_string())),
                    ("like", like.to_string()),
                    ("csrf", csrf),
                ]
            )
            .send_bpi("点赞").await?;

        Ok(result)
    }

    /// 投币视频
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/video/action.md)
    ///
    /// # 参数
    /// | 名称         | 类型           | 说明                 |
    /// | ------------ | --------------| -------------------- |
    /// | `aid`        | `Option<u64>`   | 稿件 avid，可选      |
    /// | `bvid`       | `Option<String>`| 稿件 bvid，可选      |
    /// | `multiply`   | u8            | 投币数量（上限2）    |
    /// | `select_like`| `Option<u8>`    | 是否附加点赞，0:否，1:是，默认0 |
    pub async fn video_coin(
        &self,
        aid: Option<u64>,
        bvid: Option<String>,
        multiply: u8,
        select_like: Option<u8>
    ) -> Result<BpiResponse<CoinData>, BpiError> {
        let csrf = self.csrf()?;

        self
            .post("https://api.bilibili.com/x/web-interface/coin/add")
            .with_bilibili_headers()
            .form(
                &[
                    ("aid", aid.unwrap_or(0).to_string()),
                    ("bvid", bvid.unwrap_or("".to_string())),
                    ("multiply", multiply.to_string()),
                    ("select_like", select_like.unwrap_or(0).to_string()),
                    ("csrf", csrf),
                ]
            )
            .send_bpi("投币").await
    }

    /// 收藏视频
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/video/action.md)
    ///
    /// # 参数
    /// | 名称           | 类型                 | 说明                 |
    /// | -------------- | --------------------| -------------------- |
    /// | `rid`          | u64                 | 资源ID（视频avid）   |
    /// | `add_media_ids`| Option<Vec<&str>>   | 要添加的收藏夹ID列表，可选 |
    /// | `del_media_ids`| Option<Vec<&str>>   | 要删除的收藏夹ID列表，可选 |
    pub async fn video_favorite(
        &self,
        rid: u64,
        add_media_ids: Option<Vec<&str>>,
        del_media_ids: Option<Vec<&str>>
    ) -> Result<FavoriteResponse, BpiError> {
        if add_media_ids.is_none() && del_media_ids.is_none() {
            return Err(BpiError::InvalidParameter {
                field: "media_ids",
                message: "请至少指定一个操作",
            });
        }

        let csrf = self.csrf()?;

        let mut params = HashMap::new();

        params.extend([
            ("rid", rid.to_string()),
            ("type", "2".to_string()),
            ("csrf", csrf),
        ]);

        if let Some(ids) = add_media_ids {
            let s = ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.insert("add_media_ids", s);
        }
        if let Some(ids) = del_media_ids {
            let s = ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            params.insert("del_media_ids", s);
        }

        self
            .post("https://api.bilibili.com/x/v3/fav/resource/deal")
            .with_bilibili_headers()
            .form(&params)
            .send_bpi("收藏视频").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_like_video() {
        let bpi = BpiClient::new();

        match bpi.video_like(Some(10001), None, 1).await {
            Ok(resp) => tracing::info!("点赞响应: {:?}", resp),
            Err(e) =>
                match e.code() {
                    Some(65006) => { tracing::info!("已赞过") }
                    _ => { panic!("请求失败: {}", e) }
                }
        }
    }

    #[tokio::test]
    async fn test_coin_video() {
        let bpi = BpiClient::new();

        match bpi.video_coin(Some(10001), None, 1, Some(1)).await {
            Ok(resp) => tracing::info!("投币响应: {:?}", resp),
            Err(e) =>
                match e.code() {
                    Some(34005) => { tracing::info!("超过投币上限") }
                    _ => { panic!("请求失败: {}", e) }
                }
        }
    }

    #[tokio::test]
    async fn test_favorite_video() {
        let bpi = BpiClient::new();

        match bpi.video_favorite(10001, Some(vec!["44717370"]), None).await {
            Ok(resp) => tracing::info!("收藏响应: {:?}", resp),
            Err(e) => panic!("请求失败: {}", e),
        }
    }
}
