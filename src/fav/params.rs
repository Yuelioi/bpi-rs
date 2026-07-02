use crate::ids::{MediaId, Mid};
use crate::{BpiError, BpiResult};

/// Parameters for `/x/v3/fav/folder/info`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FavFolderInfoParams {
    media_id: MediaId,
}

impl FavFolderInfoParams {
    pub fn new(media_id: MediaId) -> Self {
        Self { media_id }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("media_id", self.media_id.to_string())]
    }
}

/// Parameters for `/x/v3/fav/folder/created/list-all`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FavCreatedListParams {
    up_mid: Mid,
    typ: Option<u8>,
    rid: Option<u64>,
    web_location: String,
}

impl FavCreatedListParams {
    pub fn new(up_mid: Mid) -> Self {
        Self {
            up_mid,
            typ: None,
            rid: None,
            web_location: "333.1387".to_string(),
        }
    }

    pub fn with_type(mut self, typ: u8) -> Self {
        self.typ = Some(typ);
        self
    }

    pub fn with_resource_id(mut self, rid: u64) -> BpiResult<Self> {
        self.rid = Some(validate_positive_u64("rid", rid)?);
        Ok(self)
    }

    pub fn with_web_location(mut self, web_location: impl Into<String>) -> BpiResult<Self> {
        self.web_location = normalize_non_blank("web_location", web_location.into())?;
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![("up_mid", self.up_mid.to_string())];

        if let Some(typ) = self.typ {
            pairs.push(("type", typ.to_string()));
        }
        if let Some(rid) = self.rid {
            pairs.push(("rid", rid.to_string()));
        }
        pairs.push(("web_location", self.web_location.clone()));

        pairs
    }
}

/// Parameters for `/x/v3/fav/folder/collected/list`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FavCollectedListParams {
    up_mid: Mid,
    page: u32,
    page_size: u32,
    platform: String,
}

impl FavCollectedListParams {
    pub fn new(up_mid: Mid) -> Self {
        Self {
            up_mid,
            page: 1,
            page_size: 20,
            platform: "web".to_string(),
        }
    }

    pub fn with_page(mut self, page: u32) -> BpiResult<Self> {
        self.page = validate_positive_u32("pn", page)?;
        Ok(self)
    }

    pub fn with_page_size(mut self, page_size: u32) -> BpiResult<Self> {
        self.page_size = validate_positive_u32("ps", page_size)?;
        Ok(self)
    }

    pub fn with_platform(mut self, platform: impl Into<String>) -> BpiResult<Self> {
        self.platform = normalize_non_blank("platform", platform.into())?;
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("up_mid", self.up_mid.to_string()),
            ("pn", self.page.to_string()),
            ("ps", self.page_size.to_string()),
            ("platform", self.platform.clone()),
        ]
    }
}

/// Parameters for `/x/v3/fav/resource/infos`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FavResourceInfosParams {
    resources: String,
    platform: String,
}

impl FavResourceInfosParams {
    pub fn new(resources: impl Into<String>) -> BpiResult<Self> {
        Ok(Self {
            resources: normalize_non_blank("resources", resources.into())?,
            platform: "web".to_string(),
        })
    }

    pub fn with_platform(mut self, platform: impl Into<String>) -> BpiResult<Self> {
        self.platform = normalize_non_blank("platform", platform.into())?;
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("resources", self.resources.clone()),
            ("platform", self.platform.clone()),
        ]
    }
}

/// Parameters for `/x/v3/fav/resource/ids`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FavResourceIdsParams {
    media_id: MediaId,
    platform: String,
}

impl FavResourceIdsParams {
    pub fn new(media_id: MediaId) -> Self {
        Self {
            media_id,
            platform: "web".to_string(),
        }
    }

    pub fn with_platform(mut self, platform: impl Into<String>) -> BpiResult<Self> {
        self.platform = normalize_non_blank("platform", platform.into())?;
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("media_id", self.media_id.to_string()),
            ("platform", self.platform.clone()),
        ]
    }
}

fn validate_positive_u32(field: &'static str, value: u32) -> BpiResult<u32> {
    if value == 0 {
        return Err(BpiError::invalid_parameter(field, "value must be non-zero"));
    }

    Ok(value)
}

fn validate_positive_u64(field: &'static str, value: u64) -> BpiResult<u64> {
    if value == 0 {
        return Err(BpiError::invalid_parameter(field, "value must be non-zero"));
    }

    Ok(value)
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
    fn fav_folder_info_params_serializes_media_id() -> BpiResult<()> {
        let params = FavFolderInfoParams::new(MediaId::new(1572769770)?);

        assert_eq!(
            params.query_pairs(),
            vec![("media_id", "1572769770".to_string())]
        );
        Ok(())
    }

    #[test]
    fn fav_created_list_params_serializes_defaults() -> BpiResult<()> {
        let params = FavCreatedListParams::new(Mid::new(4279370)?);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("up_mid", "4279370".to_string()),
                ("web_location", "333.1387".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn fav_created_list_params_serializes_optional_filters() -> BpiResult<()> {
        let params = FavCreatedListParams::new(Mid::new(4279370)?)
            .with_type(2)
            .with_resource_id(170001)?
            .with_web_location("333.999")?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("up_mid", "4279370".to_string()),
                ("type", "2".to_string()),
                ("rid", "170001".to_string()),
                ("web_location", "333.999".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn fav_created_list_params_rejects_zero_resource_id() -> BpiResult<()> {
        let err = FavCreatedListParams::new(Mid::new(4279370)?)
            .with_resource_id(0)
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "rid", .. }
        ));
        Ok(())
    }

    #[test]
    fn fav_collected_list_params_serializes_defaults() -> BpiResult<()> {
        let params = FavCollectedListParams::new(Mid::new(4279370)?);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("up_mid", "4279370".to_string()),
                ("pn", "1".to_string()),
                ("ps", "20".to_string()),
                ("platform", "web".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn fav_collected_list_params_serializes_pagination() -> BpiResult<()> {
        let params = FavCollectedListParams::new(Mid::new(4279370)?)
            .with_page(2)?
            .with_page_size(30)?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("up_mid", "4279370".to_string()),
                ("pn", "2".to_string()),
                ("ps", "30".to_string()),
                ("platform", "web".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn fav_collected_list_params_rejects_zero_page() -> BpiResult<()> {
        let err = FavCollectedListParams::new(Mid::new(4279370)?)
            .with_page(0)
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "pn", .. }
        ));
        Ok(())
    }

    #[test]
    fn fav_resource_infos_params_serializes_defaults() -> BpiResult<()> {
        let params = FavResourceInfosParams::new("115087859779103:2")?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("resources", "115087859779103:2".to_string()),
                ("platform", "web".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn fav_resource_infos_params_rejects_blank_resources() {
        let err = FavResourceInfosParams::new("  ").unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "resources",
                ..
            }
        ));
    }

    #[test]
    fn fav_resource_ids_params_serializes_defaults() -> BpiResult<()> {
        let params = FavResourceIdsParams::new(MediaId::new(1572769770)?);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("media_id", "1572769770".to_string()),
                ("platform", "web".to_string())
            ]
        );
        Ok(())
    }
}
