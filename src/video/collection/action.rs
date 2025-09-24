//! B站视频合集相关接口实现
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// --- 响应数据结构体 ---

/// 创建视频列表响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateSeriesResponseData {
    /// 视频列表 ID
    pub series_id: u64,
}

impl BpiClient {
    /// 创建视频列表并添加视频
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/video/collection.md)
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
        aids: Option<&str>
    ) -> Result<BpiResponse<CreateSeriesResponseData>, BpiError> {
        let csrf = self.csrf()?;
        let mut form = reqwest::multipart::Form
            ::new()
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

        self
            .post("https://api.bilibili.com/x/series/series/createAndAddArchives")
            .query(&[("csrf", csrf)])
            .multipart(form)
            .send_bpi("创建视频列表并添加视频").await
    }

    /// 删除视频列表
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/video/collection.md)
    ///
    /// # 参数
    /// | 名称         | 类型           | 说明                 |
    /// | ------------ | --------------| -------------------- |
    /// | `mid`        | u64           | 用户 mid             |
    /// | `series_id`  | u64           | 视频列表 ID          |
    pub async fn collection_delete_series(
        &self,
        mid: u64,
        series_id: u64
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        self
            .post("https://api.bilibili.com/x/series/series/delete")
            .query(
                &[
                    ("csrf", csrf),
                    ("mid", mid.to_string()),
                    ("series_id", series_id.to_string()),
                    ("aids", "".to_string()),
                ]
            )
            .send_bpi("删除视频列表").await
    }

    /// 从视频列表中删除稿件
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/video/collection.md)
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
        aids: &str
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let params = [
            ("mid", mid.to_string()),
            ("series_id", series_id.to_string()),
            ("aids", aids.to_string()),
        ];

        self
            .post("https://api.bilibili.com/x/series/series/delArchives")
            .query(&[("csrf", csrf)])
            .form(&params)
            .send_bpi("从视频列表中删除稿件").await
    }

    /// 添加稿件至视频列表
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/video/collection.md)
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
        aids: &str
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let params = [
            ("mid", mid.to_string()),
            ("series_id", series_id.to_string()),
            ("aids", aids.to_string()),
        ];

        self
            .post("https://api.bilibili.com/x/series/series/addArchives")
            .query(&[("csrf", csrf)])
            .form(&params)
            .send_bpi("添加稿件至视频列表").await
    }

    /// 编辑视频列表信息
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/video/collection.md)
    ///
    /// # 参数
    /// | 名称         | 类型           | 说明                 |
    /// | ------------ | --------------| -------------------- |
    /// | `mid`        | u64           | 用户 mid             |
    /// | `series_id`  | u64           | 视频列表 ID          |
    /// | `name`       | &str          | 标题                 |
    /// | `keywords`   | `Option<&str>`  | 关键词，可选         |
    /// | `description`| `Option<&str>`  | 简介，可选           |
    /// | `add_aids`   | `Option<&str>`  | 要添加的视频 aid 列表，以`,`分隔，可选 |
    /// | `del_aids`   | `Option<&str>`  | 要删除的视频 aid 列表，以`,`分隔，可选 |
    pub async fn collection_update_series(
        &self,
        mid: u64,
        series_id: u64,
        name: &str,
        keywords: Option<&str>,
        description: Option<&str>,
        add_aids: Option<&str>,
        del_aids: Option<&str>
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let mut form = reqwest::multipart::Form
            ::new()
            .text("mid", mid.to_string())
            .text("series_id", series_id.to_string())
            .text("name", name.to_string());

        if let Some(k) = keywords {
            form = form.text("keywords", k.to_string());
        }
        if let Some(d) = description {
            form = form.text("description", d.to_string());
        }
        if let Some(a) = add_aids {
            form = form.text("add_aids", a.to_string());
        }
        if let Some(d) = del_aids {
            form = form.text("del_aids", d.to_string());
        }

        self
            .post("https://api.bilibili.com/x/series/series/update")
            .query(&[("csrf", csrf)])
            .multipart(form)
            .send_bpi("编辑视频列表信息").await
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

    #[tokio::test]
    async fn test_create_and_add_archives() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.collection_create_and_add_archives(
            TEST_MID,
            "Rust Bilibili API Test",
            Some("rust,api"),
            Some("这是一个用于 Rust Bilibili API 测试的视频列表"),
            Some(TEST_AID)
        ).await?;
        let data = resp.into_data()?;

        info!("创建的视频列表 ID: {:?}", data.series_id);
        assert!(data.series_id > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_add_archives_to_series() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.collection_add_archives_to_series(TEST_MID, TEST_SERIES_ID, TEST_AID).await?;

        info!("添加稿件至视频列表成功: {:?}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_update_series() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.collection_update_series(
            TEST_MID,
            TEST_SERIES_ID,
            "Rust Bilibili API Test Updated",
            Some("rust,api,update"),
            Some("更新后的简介"),
            Some(TEST_AID),
            None
        ).await?;

        info!("编辑视频列表成功: {:?}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_archives_from_series() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.collection_delete_archives_from_series(
            TEST_MID,
            TEST_SERIES_ID,
            TEST_AID
        ).await?;

        info!("从视频列表中删除稿件成功: {:?}", resp);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_series() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        // 假设 TEST_SERIES_ID 是一个需要被删除的测试用列表
        let resp = bpi.collection_delete_series(TEST_MID, TEST_SERIES_ID).await?;

        info!("删除视频列表成功: {:?}", resp);

        Ok(())
    }
}
