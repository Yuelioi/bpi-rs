// B站视频交互接口(Web端)
//
// [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)

use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResponse;
use crate::video::VideoClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

impl<'a> VideoClient<'a> {
    /// 点赞/取消点赞
    ///
    /// # 文档
    /// [查看API文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/video/action.md)
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
        like: u8,
    ) -> Result<BpiResponse<CoinData>, BpiError> {
        let csrf = self.client.csrf()?;

        let result = self
            .client
            .post("https://api.bilibili.com/x/web-interface/archive/like")
            .with_bilibili_headers()
            .form(&[
                ("aid", aid.unwrap_or(0).to_string()),
                ("bvid", bvid.unwrap_or("".to_string())),
                ("like", like.to_string()),
                ("csrf", csrf),
            ])
            .send_bpi("点赞")
            .await?;

        Ok(result)
    }

    /// 投币视频
    ///
    /// # 文档
    /// [查看API文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/video/action.md)
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
        select_like: Option<u8>,
    ) -> Result<BpiResponse<CoinData>, BpiError> {
        let csrf = self.client.csrf()?;

        self.client
            .post("https://api.bilibili.com/x/web-interface/coin/add")
            .with_bilibili_headers()
            .form(&[
                ("aid", aid.unwrap_or(0).to_string()),
                ("bvid", bvid.unwrap_or("".to_string())),
                ("multiply", multiply.to_string()),
                ("select_like", select_like.unwrap_or(0).to_string()),
                ("csrf", csrf),
            ])
            .send_bpi("投币")
            .await
    }

    /// 收藏视频
    ///
    /// # 文档
    /// [查看API文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/video/action.md)
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
        del_media_ids: Option<Vec<&str>>,
    ) -> Result<FavoriteResponse, BpiError> {
        if add_media_ids.is_none() && del_media_ids.is_none() {
            return Err(BpiError::InvalidParameter {
                field: "media_ids",
                message: "请至少指定一个操作",
            });
        }

        let csrf = self.client.csrf()?;

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

        self.client
            .post("https://api.bilibili.com/x/v3/fav/resource/deal")
            .with_bilibili_headers()
            .form(&params)
            .send_bpi("收藏视频")
            .await
    }
}

#[cfg(test)]
mod tests {}
