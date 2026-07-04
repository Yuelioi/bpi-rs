use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResult;
use crate::dynamic::DynamicClient;
use serde_json::json;

const LIKE_ENDPOINT: &str = "https://api.bilibili.com/x/dynamic/feed/dyn/thumb";
const REMOVE_DRAFT_ENDPOINT: &str =
    "https://api.vc.bilibili.com/dynamic_draft/v1/dynamic_draft/rm_draft";
const SET_TOP_ENDPOINT: &str = "https://api.bilibili.com/x/dynamic/feed/space/set_top";
const REMOVE_TOP_ENDPOINT: &str = "https://api.bilibili.com/x/dynamic/feed/space/rm_top";

/// Parameters for liking or unliking a dynamic item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicLikeParams {
    dyn_id_str: String,
    up: u8,
}

impl DynamicLikeParams {
    pub fn new(dyn_id_str: impl Into<String>, up: u8) -> BpiResult<Self> {
        if !(0..=2).contains(&up) {
            return Err(BpiError::invalid_parameter(
                "up",
                "value must be 0, 1, or 2",
            ));
        }

        Ok(Self {
            dyn_id_str: normalize_non_blank("dyn_id_str", dyn_id_str.into())?,
            up,
        })
    }

    fn json_body(&self) -> serde_json::Value {
        json!({
            "dyn_id_str": self.dyn_id_str,
            "up": self.up,
            "spmid": "333.1369.0.0",
            "from_spmid": "333.999.0.0",
        })
    }
}

/// Parameters for deleting a scheduled dynamic draft.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicDraftDeleteParams {
    draft_id: String,
}

impl DynamicDraftDeleteParams {
    pub fn new(draft_id: impl Into<String>) -> BpiResult<Self> {
        Ok(Self {
            draft_id: normalize_non_blank("draft_id", draft_id.into())?,
        })
    }

    fn form_pairs<'a>(&'a self, csrf: &'a str) -> [(&'static str, &'a str); 2] {
        [("draft_id", &self.draft_id), ("csrf", csrf)]
    }
}

/// Parameters for setting or removing a dynamic top item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DynamicTopParams {
    dyn_str: String,
}

impl DynamicTopParams {
    pub fn new(dyn_str: impl Into<String>) -> BpiResult<Self> {
        Ok(Self {
            dyn_str: normalize_non_blank("dyn_str", dyn_str.into())?,
        })
    }

    fn json_body(&self) -> serde_json::Value {
        json!({ "dyn_str": self.dyn_str })
    }
}

impl<'a> DynamicClient<'a> {
    /// Likes or unlikes a dynamic item and returns the canonical payload result.
    pub async fn like(&self, params: DynamicLikeParams) -> BpiResult<Option<serde_json::Value>> {
        let csrf = self.client.csrf()?;
        let json_body = params.json_body();

        self.client
            .post(LIKE_ENDPOINT)
            .query(&[("csrf", csrf)])
            .json(&json_body)
            .send_bpi_optional_payload("dynamic.like")
            .await
    }

    /// Deletes a scheduled dynamic draft and returns the canonical payload result.
    pub async fn delete_draft(
        &self,
        params: DynamicDraftDeleteParams,
    ) -> BpiResult<Option<serde_json::Value>> {
        let csrf = self.client.csrf()?;

        self.client
            .post(REMOVE_DRAFT_ENDPOINT)
            .form(&params.form_pairs(&csrf))
            .send_bpi_optional_payload("dynamic.draft.delete")
            .await
    }

    /// Sets a dynamic item as top and returns the canonical payload result.
    pub async fn set_top(&self, params: DynamicTopParams) -> BpiResult<Option<serde_json::Value>> {
        let csrf = self.client.csrf()?;
        let json_body = params.json_body();

        self.client
            .post(SET_TOP_ENDPOINT)
            .query(&[("csrf", csrf)])
            .json(&json_body)
            .send_bpi_optional_payload("dynamic.top.set")
            .await
    }

    /// Removes a dynamic item from top and returns the canonical payload result.
    pub async fn remove_top(
        &self,
        params: DynamicTopParams,
    ) -> BpiResult<Option<serde_json::Value>> {
        let csrf = self.client.csrf()?;
        let json_body = params.json_body();

        self.client
            .post(REMOVE_TOP_ENDPOINT)
            .query(&[("csrf", csrf)])
            .json(&json_body)
            .send_bpi_optional_payload("dynamic.top.remove")
            .await
    }
}

fn normalize_non_blank(field: &'static str, value: String) -> BpiResult<String> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(BpiError::invalid_parameter(field, "value cannot be blank"));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dynamic_like_params_rejects_invalid_action() {
        let err = DynamicLikeParams::new("123", 3).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "up", .. }
        ));
    }
}
