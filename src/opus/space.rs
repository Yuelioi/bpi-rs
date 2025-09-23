//! 空间图文
//!
//! [空间图文](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/opus/space.md#空间图文)

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 空间图文封面信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpaceCover {
    /// 封面高度
    pub height: u32,
    /// 图片 URL
    pub url: String,
    /// 封面宽度
    pub width: u32,
}

/// 空间图文统计信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpaceStat {
    /// 点赞数（字符串）
    pub like: String,
    /// 浏览数（字符串，仅自己可见）
    pub view: Option<String>,
}

/// 空间图文单条信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpaceItem {
    /// 文本内容
    pub content: String,
    /// 封面信息，可选
    pub cover: Option<SpaceCover>,
    /// 跳转 URL
    pub jump_url: String,
    /// opus id
    pub opus_id: String,
    /// 统计信息
    pub stat: SpaceStat,
}

/// 空间图文响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpaceData {
    /// 是否还有更多
    pub has_more: bool,
    /// 图文列表
    pub items: Vec<SpaceItem>,
    /// 下一页 offset
    pub offset: String,
    /// 更新数
    pub update_num: u32,
}

impl BpiClient {
    /// 获取用户空间图文
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/opus
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `mid` | u64 | 用户 UID |
    /// | `page` | Option<u32> | 页码，默认 0 |
    /// | `offset` | Option<&str> | 下一页偏移量 |
    /// | `typ` | Option<&str> | 类型：`all`/`article`/`dynamic`，默认 `all` |
    pub async fn opus_space_feed(
        &self,
        mid: u64,
        page: Option<u32>,
        offset: Option<&str>,
        typ: Option<&str>, // all/article/dynamic
    ) -> Result<BpiResponse<SpaceData>, BpiError> {
        let query = vec![
            ("host_mid", mid.to_string()),
            ("page", page.unwrap_or(0).to_string()),
            ("offset", offset.unwrap_or("").to_string()),
            ("type", typ.unwrap_or("all").to_string()),
            ("web_location", "333.1387".to_string()),
        ];

        self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/opus/feed/space")
            .query(&query)
            .send_bpi("获取用户空间图文")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_opus_space_feed() {
        let bpi = BpiClient::new();
        let resp = bpi.opus_space_feed(4279370, Some(1), None, None).await;
        assert!(resp.is_ok());
        if let Ok(r) = resp {
            info!("空间图文返回: {:?}", r);
        }
    }
}
