use crate::ids::Aid;
use crate::{BpiError, BpiResult};

/// Parameters for `/x2/creative/web/archives/sp`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpArchivesListParams {
    page: u32,
    page_size: Option<u32>,
}

impl UpArchivesListParams {
    pub fn new(page: u32) -> BpiResult<Self> {
        Ok(Self {
            page: validate_non_zero_u32("pn", page)?,
            page_size: None,
        })
    }

    pub fn with_page_size(mut self, page_size: u32) -> BpiResult<Self> {
        self.page_size = Some(validate_non_zero_u32("ps", page_size)?);
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut query = vec![("pn", self.page.to_string())];

        if let Some(page_size) = self.page_size {
            query.push(("ps", page_size.to_string()));
        }

        query
    }
}

/// Parameters for `/x/web/archive/videos`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpArchiveVideosParams {
    aid: Aid,
}

impl UpArchiveVideosParams {
    pub fn new(aid: Aid) -> Self {
        Self { aid }
    }

    pub(crate) fn query_pairs(&self) -> [(&'static str, String); 1] {
        [("aid", self.aid.to_string())]
    }
}

fn validate_non_zero_u32(field: &'static str, value: u32) -> BpiResult<u32> {
    if value == 0 {
        return Err(BpiError::invalid_parameter(field, "value must be non-zero"));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn up_archives_list_params_serializes_required_page() -> BpiResult<()> {
        let params = UpArchivesListParams::new(1)?;

        assert_eq!(params.query_pairs(), vec![("pn", "1".to_string())]);
        Ok(())
    }

    #[test]
    fn up_archives_list_params_serializes_optional_page_size() -> BpiResult<()> {
        let params = UpArchivesListParams::new(2)?.with_page_size(20)?;

        assert_eq!(
            params.query_pairs(),
            vec![("pn", "2".to_string()), ("ps", "20".to_string())]
        );
        Ok(())
    }

    #[test]
    fn up_archives_list_params_rejects_zero_page() {
        let err = UpArchivesListParams::new(0).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "pn", .. }
        ));
    }

    #[test]
    fn up_archives_list_params_rejects_zero_page_size() -> BpiResult<()> {
        let err = UpArchivesListParams::new(1)?.with_page_size(0).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "ps", .. }
        ));
        Ok(())
    }

    #[test]
    fn up_archive_videos_params_serializes_aid_query() -> BpiResult<()> {
        let params = UpArchiveVideosParams::new(Aid::new(113602455409683)?);

        assert_eq!(
            params.query_pairs(),
            [("aid", "113602455409683".to_string())]
        );
        Ok(())
    }
}
