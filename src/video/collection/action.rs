//! B站视频合集相关接口实现
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)
use crate::{BpiError, BpiResult};
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
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;

    // 请在运行测试前设置环境变量 `BPI_COOKIE`，以包含 SESSDATA 等登录信息
    // mid 和 series_id 根据实际情况修改

    // 测试用的 mid
    // 测试用的合集 ID

    #[test]
    fn collection_update_series_params_rejects_blank_name() {
        let err = CollectionUpdateSeriesParams::new(42, 100, "  ").unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "name", .. }
        ));
    }
}
