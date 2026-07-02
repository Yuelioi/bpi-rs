use crate::{BpiError, BpiResult};

/// Parameters for `/x/article/viewinfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArticleInfoParams {
    id: i64,
}

impl ArticleInfoParams {
    pub fn new(id: i64) -> BpiResult<Self> {
        Ok(Self {
            id: validate_positive_i64("id", id)?,
        })
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("id", self.id.to_string())]
    }
}

/// Parameters for `/x/article/view`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleViewParams {
    id: i64,
    gaia_source: String,
}

impl ArticleViewParams {
    pub fn new(id: i64) -> BpiResult<Self> {
        Ok(Self {
            id: validate_positive_i64("id", id)?,
            gaia_source: "main_web".to_string(),
        })
    }

    pub fn with_gaia_source(mut self, gaia_source: impl Into<String>) -> BpiResult<Self> {
        let gaia_source = gaia_source.into();
        validate_non_blank("gaia_source", &gaia_source)?;
        self.gaia_source = gaia_source;
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("id", self.id.to_string()),
            ("gaia_source", self.gaia_source.clone()),
        ]
    }
}

/// Parameters for `/x/article/cards`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleCardsParams {
    ids: String,
    web_location: String,
}

impl ArticleCardsParams {
    pub fn new(ids: impl Into<String>) -> BpiResult<Self> {
        let ids = ids.into();
        validate_non_blank("ids", &ids)?;
        Ok(Self {
            ids,
            web_location: "333.1305".to_string(),
        })
    }

    pub fn with_web_location(mut self, web_location: impl Into<String>) -> BpiResult<Self> {
        let web_location = web_location.into();
        validate_non_blank("web_location", &web_location)?;
        self.web_location = web_location;
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("ids", self.ids.clone()),
            ("web_location", self.web_location.clone()),
        ]
    }
}

/// Parameters for `/x/article/list/web/articles`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArticleArticlesInfoParams {
    id: i64,
}

impl ArticleArticlesInfoParams {
    pub fn new(id: i64) -> BpiResult<Self> {
        Ok(Self {
            id: validate_positive_i64("id", id)?,
        })
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("id", self.id.to_string())]
    }
}

fn validate_positive_i64(field: &'static str, value: i64) -> BpiResult<i64> {
    if value <= 0 {
        return Err(BpiError::invalid_parameter(field, "value must be positive"));
    }

    Ok(value)
}

fn validate_non_blank(field: &'static str, value: &str) -> BpiResult<()> {
    if value.trim().is_empty() {
        return Err(BpiError::invalid_parameter(field, "value cannot be blank"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article_info_params_serializes_id() -> BpiResult<()> {
        let params = ArticleInfoParams::new(2)?;

        assert_eq!(params.query_pairs(), vec![("id", "2".to_string())]);
        Ok(())
    }

    #[test]
    fn article_info_params_rejects_zero_id() {
        let err = ArticleInfoParams::new(0).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "id", .. }
        ));
    }

    #[test]
    fn article_view_params_serializes_default_source() -> BpiResult<()> {
        let params = ArticleViewParams::new(2)?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("id", "2".to_string()),
                ("gaia_source", "main_web".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn article_view_params_serializes_custom_source() -> BpiResult<()> {
        let params = ArticleViewParams::new(2)?.with_gaia_source("article_test")?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("id", "2".to_string()),
                ("gaia_source", "article_test".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn article_cards_params_serializes_defaults() -> BpiResult<()> {
        let params = ArticleCardsParams::new("av2,cv1,cv2")?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("ids", "av2,cv1,cv2".to_string()),
                ("web_location", "333.1305".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn article_cards_params_rejects_blank_ids() {
        let err = ArticleCardsParams::new("  ").unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "ids", .. }
        ));
    }

    #[test]
    fn article_articles_info_params_serializes_id() -> BpiResult<()> {
        let params = ArticleArticlesInfoParams::new(207146)?;

        assert_eq!(params.query_pairs(), vec![("id", "207146".to_string())]);
        Ok(())
    }
}
