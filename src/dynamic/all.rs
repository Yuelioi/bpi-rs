use serde::{Deserialize, Serialize};

use crate::dynamic::params::{DynamicAllParams, DynamicCheckNewParams};
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
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`DynamicAllParams`] | 动态流筛选和翻页参数 |
    pub async fn dynamic_all(
        &self,
        params: DynamicAllParams,
    ) -> Result<BpiResponse<DynamicAllData>, BpiError> {
        self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/feed/all")
            .query(&params.query_pairs())
            .send_bpi("获取全部动态列表")
            .await
    }

    /// 检测是否有新动态
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`DynamicCheckNewParams`] | 更新基线和类型筛选参数 |
    pub async fn dynamic_check_new(
        &self,
        params: DynamicCheckNewParams,
    ) -> Result<BpiResponse<DynamicUpdateData>, BpiError> {
        self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/feed/all/update")
            .query(&params.query_pairs())
            .send_bpi("检测新动态")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_dynamic_get_all() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi.dynamic_all(DynamicAllParams::new()).await?;
        assert_eq!(resp.code, 0);

        let data = resp.into_data()?;

        info!("成功获取 {} 条动态", data.items.len());

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_dynamic_check_new() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let update_baseline = "0";
        let resp = bpi
            .dynamic_check_new(DynamicCheckNewParams::new(update_baseline)?)
            .await?;
        let data = resp.into_data().unwrap();

        info!("成功检测到 {} 条新动态", data.update_num);

        Ok(())
    }
}
