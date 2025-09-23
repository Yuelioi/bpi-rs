use serde::{Deserialize, Serialize};

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DynamicAllData {
    pub has_more: bool,
    pub items: Vec<DynamicItem>,
    pub offset: String,
    pub update_baseline: String,
    pub update_num: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicItem {
    pub basic: Basic,
    pub id_str: String,
    pub modules: serde_json::Value,
    #[serde(rename = "type")]
    pub type_field: String,
    pub visible: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Basic {
    pub comment_id_str: String,
    pub comment_type: i64,
    pub like_icon: serde_json::Value,
    pub rid_str: String,
    pub is_only_fans: Option<bool>,
    pub jump_url: Option<String>,
}

/// 检测新动态响应数据
#[derive(Debug, Clone, Deserialize)]
pub struct DynamicUpdateData {
    /// 新动态的数量
    pub update_num: u64,
}

impl BpiClient {
    /// 获取全部动态列表
    /// GET https://api.bilibili.com/x/polymer/web-dynamic/v1/feed/all
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `host_mid` | Option<&str> | UP 主 UID |
    /// | `offset` | Option<&str> | 分页偏移量 |
    /// | `update_baseline` | Option<&str> | 更新基线，用于获取新动态 |
    pub async fn dynamic_all(
        &self,
        host_mid: Option<&str>,
        offset: Option<&str>,
        update_baseline: Option<&str>,
    ) -> Result<BpiResponse<DynamicAllData>, BpiError> {
        let mut req = self
          .get("https://api.bilibili.com/x/polymer/web-dynamic/v1/feed/all")
          .query(&[("features", "itemOpusStyle,listOnlyfans,opusBigCover,onlyfansVote,decorationCard,onlyfansAssetsV2,forwardListHidden,ugcDelete"), ("web_location", "333.1365")]);

        if let Some(mid) = host_mid {
            req = req.query(&[("host_mid", mid)]);
        }
        if let Some(off) = offset {
            req = req.query(&[("offset", off)]);
        }
        if let Some(baseline) = update_baseline {
            req = req.query(&[("update_baseline", baseline)]);
        }

        req.send_bpi("获取全部动态列表").await
    }

    /// 检测是否有新动态
    /// GET https://api.bilibili.com/x/polymer/web-dynamic/v1/feed/all/update
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `update_baseline` | &str | 上次列表返回的 update_baseline |
    /// | `type_str` | Option<&str> | 动态类型 |
    pub async fn dynamic_check_new(
        &self,
        update_baseline: &str,
        type_str: Option<&str>,
    ) -> Result<BpiResponse<DynamicUpdateData>, BpiError> {
        let mut req = self
            .get("https://api.bilibili.com/x/polymer/web-dynamic/v1/feed/all/update")
            .query(&[("update_baseline", update_baseline)]);

        if let Some(typ) = type_str {
            req = req.query(&[("type", typ)]);
        }

        req.send_bpi("检测新动态").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]

    async fn test_dynamic_get_all() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.dynamic_all(None, None, None).await?;
        assert_eq!(resp.code, 0);

        let data = resp.into_data()?;

        info!("成功获取 {} 条动态", data.items.len());

        Ok(())
    }

    #[tokio::test]

    async fn test_dynamic_check_new() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let update_baseline = "0";
        let resp = bpi.dynamic_check_new(update_baseline, None).await?;
        let data = resp.into_data().unwrap();

        info!("成功检测到 {} 条新动态", data.update_num);

        Ok(())
    }
}
