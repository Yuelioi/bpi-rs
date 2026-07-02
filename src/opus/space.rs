//! 空间图文
//!
//! [空间图文](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/opus/space.md#空间图文)

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

use super::OpusSpaceFeedParams;

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
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/opus)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`OpusSpaceFeedParams`] | 用户、分页和类型参数 |
    pub async fn opus_space_feed(
        &self,
        params: OpusSpaceFeedParams,
    ) -> Result<BpiResponse<SpaceData>, BpiError> {
        let query = params.query_pairs();

        self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/opus/feed/space")
            .query(&query)
            .send_bpi("获取用户空间图文")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::Mid;
    use crate::opus::{OpusSpaceFeedKind, OpusSpaceFeedParams};
    use tracing::info;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_opus_space_feed() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let params = OpusSpaceFeedParams::new(Mid::new(4279370)?)
            .with_page(1)
            .with_kind(OpusSpaceFeedKind::All);
        let resp = bpi.opus_space_feed(params).await;
        assert!(resp.is_ok());
        if let Ok(r) = resp {
            info!("空间图文返回: {:?}", r);
        }
        Ok(())
    }

    #[test]
    fn opus_space_feed_params_serializes_default_query() -> Result<(), BpiError> {
        let params = OpusSpaceFeedParams::new(Mid::new(4279370)?);

        assert_eq!(
            params.query_pairs(),
            [
                ("host_mid", "4279370".to_string()),
                ("page", "0".to_string()),
                ("type", "all".to_string()),
                ("web_location", "333.1387".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn opus_space_feed_params_serializes_optional_query() -> Result<(), BpiError> {
        let params = OpusSpaceFeedParams::new(Mid::new(4279370)?)
            .with_page(2)
            .with_offset("offset-token")?
            .with_kind(OpusSpaceFeedKind::Article);

        assert_eq!(
            params.query_pairs(),
            [
                ("host_mid", "4279370".to_string()),
                ("page", "2".to_string()),
                ("offset", "offset-token".to_string()),
                ("type", "article".to_string()),
                ("web_location", "333.1387".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn opus_space_feed_params_rejects_blank_offset() -> Result<(), BpiError> {
        let err = OpusSpaceFeedParams::new(Mid::new(4279370)?)
            .with_offset("   ")
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "offset",
                ..
            }
        ));
        Ok(())
    }
}
