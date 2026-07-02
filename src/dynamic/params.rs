use crate::ids::DynamicId;
use crate::{BpiError, BpiResult};

const DEFAULT_DETAIL_FEATURES: &str = "htmlNewStyle,itemOpusStyle,decorationCard";

/// Parameters for `/x/polymer/web-dynamic/v1/detail`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicDetailParams {
    id: DynamicId,
    features: String,
}

impl DynamicDetailParams {
    pub fn new(id: DynamicId) -> Self {
        Self {
            id,
            features: DEFAULT_DETAIL_FEATURES.to_string(),
        }
    }

    pub fn with_features(mut self, features: impl Into<String>) -> BpiResult<Self> {
        self.features = normalize_non_blank("features", features.into())?;
        Ok(self)
    }

    pub fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("id", self.id.to_string()),
            ("features", self.features.clone()),
        ]
    }
}

/// Parameters for `/x/polymer/web-dynamic/v1/detail/reaction`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicReactionsParams {
    id: DynamicId,
    offset: Option<String>,
}

impl DynamicReactionsParams {
    pub fn new(id: DynamicId) -> Self {
        Self { id, offset: None }
    }

    pub fn with_offset(mut self, offset: impl Into<String>) -> BpiResult<Self> {
        self.offset = Some(normalize_non_blank("offset", offset.into())?);
        Ok(self)
    }

    pub fn query_pairs(&self) -> Vec<(&'static str, String)> {
        dynamic_offset_query(&self.id, self.offset.as_deref())
    }
}

/// Parameters for `/lottery_svr/v1/lottery_svr/lottery_notice`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicLotteryNoticeParams {
    business_id: DynamicId,
}

impl DynamicLotteryNoticeParams {
    pub fn new(business_id: DynamicId) -> Self {
        Self { business_id }
    }

    pub fn query_pairs(&self, csrf: &str) -> Vec<(&'static str, String)> {
        vec![
            ("business_id", self.business_id.to_string()),
            ("business_type", "1".to_string()),
            ("csrf", csrf.to_string()),
        ]
    }
}

/// Parameters for `/x/polymer/web-dynamic/v1/detail/forward`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicForwardsParams {
    id: DynamicId,
    offset: Option<String>,
}

impl DynamicForwardsParams {
    pub fn new(id: DynamicId) -> Self {
        Self { id, offset: None }
    }

    pub fn with_offset(mut self, offset: impl Into<String>) -> BpiResult<Self> {
        self.offset = Some(normalize_non_blank("offset", offset.into())?);
        Ok(self)
    }

    pub fn query_pairs(&self) -> Vec<(&'static str, String)> {
        dynamic_offset_query(&self.id, self.offset.as_deref())
    }
}

/// Parameters for `/x/polymer/web-dynamic/v1/detail/pic`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicPicsParams {
    id: DynamicId,
}

impl DynamicPicsParams {
    pub fn new(id: DynamicId) -> Self {
        Self { id }
    }

    pub fn query_pairs(&self) -> [(&'static str, String); 1] {
        [("id", self.id.to_string())]
    }
}

/// Parameters for `/x/polymer/web-dynamic/v1/detail/forward/item`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicForwardItemParams {
    id: DynamicId,
}

impl DynamicForwardItemParams {
    pub fn new(id: DynamicId) -> Self {
        Self { id }
    }

    pub fn query_pairs(&self) -> [(&'static str, String); 1] {
        [("id", self.id.to_string())]
    }
}

fn dynamic_offset_query(id: &DynamicId, offset: Option<&str>) -> Vec<(&'static str, String)> {
    let mut query = vec![("id", id.to_string())];
    if let Some(offset) = offset {
        query.push(("offset", offset.to_string()));
    }
    query
}

fn normalize_non_blank(field: &'static str, value: String) -> BpiResult<String> {
    let value = value.trim();
    if value.is_empty() {
        return Err(BpiError::invalid_parameter(field, "value cannot be blank"));
    }

    Ok(value.to_string())
}
