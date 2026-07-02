//! B站视频合集相关接口实现
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse, BpiResult};
use serde::{Deserialize, Serialize};

// --- 响应数据结构体 ---

/// 创建视频列表响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateSeriesResponseData {
    /// 视频列表 ID
    pub series_id: u64,
}

/// Parameters for editing an existing video series.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionUpdateSeriesParams {
    mid: u64,
    series_id: u64,
    name: String,
    keywords: Option<String>,
    description: Option<String>,
    add_aids: Option<String>,
    del_aids: Option<String>,
}

impl CollectionUpdateSeriesParams {
    /// Creates parameters with the required account, series and title fields.
    pub fn new(mid: u64, series_id: u64, name: impl Into<String>) -> BpiResult<Self> {
        if mid == 0 {
            return Err(BpiError::invalid_parameter("mid", "mid must be non-zero"));
        }
        if series_id == 0 {
            return Err(BpiError::invalid_parameter(
                "series_id",
                "series_id must be non-zero",
            ));
        }

        let name = name.into();
        if name.trim().is_empty() {
            return Err(BpiError::invalid_parameter(
                "name",
                "series name cannot be blank",
            ));
        }

        Ok(Self {
            mid,
            series_id,
            name,
            keywords: None,
            description: None,
            add_aids: None,
            del_aids: None,
        })
    }

    /// Sets comma-separated keywords.
    pub fn keywords(mut self, keywords: impl Into<String>) -> Self {
        self.keywords = Some(keywords.into());
        self
    }

    /// Sets the series description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets comma-separated AIDs to add to the series.
    pub fn add_aids(mut self, aids: impl Into<String>) -> Self {
        self.add_aids = Some(aids.into());
        self
    }

    /// Sets comma-separated AIDs to remove from the series.
    pub fn del_aids(mut self, aids: impl Into<String>) -> Self {
        self.del_aids = Some(aids.into());
        self
    }

    fn form_pairs(&self) -> Vec<(&'static str, String)> {
        let mut form = vec![
            ("mid", self.mid.to_string()),
            ("series_id", self.series_id.to_string()),
            ("name", self.name.clone()),
        ];

        if let Some(keywords) = self.keywords.as_ref() {
            form.push(("keywords", keywords.clone()));
        }
        if let Some(description) = self.description.as_ref() {
            form.push(("description", description.clone()));
        }
        if let Some(add_aids) = self.add_aids.as_ref() {
            form.push(("add_aids", add_aids.clone()));
        }
        if let Some(del_aids) = self.del_aids.as_ref() {
            form.push(("del_aids", del_aids.clone()));
        }

        form
    }

    fn into_multipart(self) -> reqwest::multipart::Form {
        self.form_pairs()
            .into_iter()
            .fold(reqwest::multipart::Form::new(), |form, (key, value)| {
                form.text(key, value)
            })
    }
}

impl BpiClient {
    /// 创建视频列表并添加视频
    ///
    /// # 文档
    /// [查看API文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/video/collection.md)
    ///
    /// # 参数
    /// | 名称         | 类型           | 说明                 |
    /// | ------------ | --------------| -------------------- |
    /// | `mid`        | u64           | 用户 mid             |
    /// | `name`       | &str          | 标题                 |
    /// | `keywords`   | `Option<&str>`  | 关键词，可选         |
    /// | `description`| `Option<&str>`  | 简介，可选           |
    /// | `aids`       | `Option<&str>`  | 视频 aid 列表，以`,`分隔，可选 |
    pub async fn collection_create_and_add_archives(
        &self,
        mid: u64,
        name: &str,
        keywords: Option<&str>,
        description: Option<&str>,
        aids: Option<&str>,
    ) -> Result<BpiResponse<CreateSeriesResponseData>, BpiError> {
        let csrf = self.csrf()?;
        let mut form = reqwest::multipart::Form::new()
            .text("mid", mid.to_string())
            .text("name", name.to_string());

        if let Some(k) = keywords {
            form = form.text("keywords", k.to_string());
        }
        if let Some(d) = description {
            form = form.text("description", d.to_string());
        }
        if let Some(a) = aids {
            form = form.text("aids", a.to_string());
        }

        self.post("https://api.bilibili.com/x/series/series/createAndAddArchives")
            .query(&[("csrf", csrf)])
            .multipart(form)
            .send_bpi("创建视频列表并添加视频")
            .await
    }

    /// 删除视频列表
    ///
    /// # 文档
    /// [查看API文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/video/collection.md)
    ///
    /// # 参数
    /// | 名称         | 类型           | 说明                 |
    /// | ------------ | --------------| -------------------- |
    /// | `mid`        | u64           | 用户 mid             |
    /// | `series_id`  | u64           | 视频列表 ID          |
    pub async fn collection_delete_series(
        &self,
        mid: u64,
        series_id: u64,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        self.post("https://api.bilibili.com/x/series/series/delete")
            .query(&[
                ("csrf", csrf),
                ("mid", mid.to_string()),
                ("series_id", series_id.to_string()),
                ("aids", "".to_string()),
            ])
            .send_bpi("删除视频列表")
            .await
    }

    /// 从视频列表中删除稿件
    ///
    /// # 文档
    /// [查看API文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/video/collection.md)
    ///
    /// # 参数
    /// | 名称         | 类型           | 说明                 |
    /// | ------------ | --------------| -------------------- |
    /// | `mid`        | u64           | 用户 mid             |
    /// | `series_id`  | u64           | 视频列表 ID          |
    /// | `aids`       | &str          | 视频 aid 列表，以`,`分隔 |
    pub async fn collection_delete_archives_from_series(
        &self,
        mid: u64,
        series_id: u64,
        aids: &str,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let params = [
            ("mid", mid.to_string()),
            ("series_id", series_id.to_string()),
            ("aids", aids.to_string()),
        ];

        self.post("https://api.bilibili.com/x/series/series/delArchives")
            .query(&[("csrf", csrf)])
            .form(&params)
            .send_bpi("从视频列表中删除稿件")
            .await
    }

    /// 添加稿件至视频列表
    ///
    /// # 文档
    /// [查看API文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/video/collection.md)
    ///
    /// # 参数
    /// | 名称         | 类型           | 说明                 |
    /// | ------------ | --------------| -------------------- |
    /// | `mid`        | u64           | 用户 mid             |
    /// | `series_id`  | u64           | 视频列表 ID          |
    /// | `aids`       | &str          | 视频 aid 列表，以`,`分隔 |
    pub async fn collection_add_archives_to_series(
        &self,
        mid: u64,
        series_id: u64,
        aids: &str,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let params = [
            ("mid", mid.to_string()),
            ("series_id", series_id.to_string()),
            ("aids", aids.to_string()),
        ];

        self.post("https://api.bilibili.com/x/series/series/addArchives")
            .query(&[("csrf", csrf)])
            .form(&params)
            .send_bpi("添加稿件至视频列表")
            .await
    }

    /// 编辑视频列表信息
    ///
    /// # 文档
    /// [查看API文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/video/collection.md)
    ///
    pub async fn collection_update_series(
        &self,
        params: CollectionUpdateSeriesParams,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let form = params.into_multipart();

        self.post("https://api.bilibili.com/x/series/series/update")
            .query(&[("csrf", csrf)])
            .multipart(form)
            .send_bpi("编辑视频列表信息")
            .await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    // 请在运行测试前设置环境变量 `BPI_COOKIE`，以包含 SESSDATA 等登录信息
    // mid 和 series_id 根据实际情况修改

    // 测试用的 mid
    const TEST_MID: u64 = 4279370;
    // 测试用的合集 ID
    const TEST_SERIES_ID: u64 = 4954206;

    const TEST_AID: &str = "772876546";

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_create_and_add_archives() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi
            .collection_create_and_add_archives(
                TEST_MID,
                "Rust Bilibili API Test",
                Some("rust,api"),
                Some("这是一个用于 Rust Bilibili API 测试的视频列表"),
                Some(TEST_AID),
            )
            .await?;
        let data = resp.into_data()?;

        info!("创建的视频列表 ID: {:?}", data.series_id);
        assert!(data.series_id > 0);

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_add_archives_to_series() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi
            .collection_add_archives_to_series(TEST_MID, TEST_SERIES_ID, TEST_AID)
            .await?;

        info!("添加稿件至视频列表成功: {:?}", resp);

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_update_series() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let params = CollectionUpdateSeriesParams::new(
            TEST_MID,
            TEST_SERIES_ID,
            "Rust Bilibili API Test Updated",
        )?
        .keywords("rust,api,update")
        .description("更新后的简介")
        .add_aids(TEST_AID);
        let resp = bpi.collection_update_series(params).await?;

        info!("编辑视频列表成功: {:?}", resp);

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_delete_archives_from_series() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi
            .collection_delete_archives_from_series(TEST_MID, TEST_SERIES_ID, TEST_AID)
            .await?;

        info!("从视频列表中删除稿件成功: {:?}", resp);

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_delete_series() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        // 假设 TEST_SERIES_ID 是一个需要被删除的测试用列表
        let resp = bpi
            .collection_delete_series(TEST_MID, TEST_SERIES_ID)
            .await?;

        info!("删除视频列表成功: {:?}", resp);

        Ok(())
    }

    #[test]
    fn collection_update_series_params_serializes_required_fields() -> Result<(), BpiError> {
        let params = CollectionUpdateSeriesParams::new(42, 100, "Series")?;

        assert_eq!(
            params.form_pairs(),
            vec![
                ("mid", "42".to_string()),
                ("series_id", "100".to_string()),
                ("name", "Series".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn collection_update_series_params_serializes_optional_fields() -> Result<(), BpiError> {
        let params = CollectionUpdateSeriesParams::new(42, 100, "Series")?
            .keywords("rust,api")
            .description("desc")
            .add_aids("1,2")
            .del_aids("3");

        assert_eq!(
            params.form_pairs(),
            vec![
                ("mid", "42".to_string()),
                ("series_id", "100".to_string()),
                ("name", "Series".to_string()),
                ("keywords", "rust,api".to_string()),
                ("description", "desc".to_string()),
                ("add_aids", "1,2".to_string()),
                ("del_aids", "3".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn collection_update_series_params_rejects_blank_name() {
        let err = CollectionUpdateSeriesParams::new(42, 100, "  ").unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "name", .. }
        ));
    }
}
