use crate::ids::{Aid, Bvid};

/// Identifies a Bilibili video by either AV numeric ID or BV string ID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoId {
    /// AV numeric video ID.
    Aid(Aid),
    /// BV string video ID.
    Bvid(Bvid),
}

/// Parameters for `/x/web-interface/view`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoViewParams {
    id: VideoId,
}

impl VideoViewParams {
    /// Creates view parameters from a validated AV ID.
    pub fn from_aid(aid: Aid) -> Self {
        Self {
            id: VideoId::Aid(aid),
        }
    }

    /// Creates view parameters from a validated BV ID.
    pub fn from_bvid(bvid: Bvid) -> Self {
        Self {
            id: VideoId::Bvid(bvid),
        }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        match &self.id {
            VideoId::Aid(aid) => vec![("aid", aid.to_string())],
            VideoId::Bvid(bvid) => vec![("bvid", bvid.to_string())],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BpiError;
    use crate::ids::Aid;

    #[test]
    fn video_view_params_serializes_bvid_query() -> Result<(), BpiError> {
        let params = VideoViewParams::from_bvid("BV1xx411c7mD".parse()?);

        assert_eq!(
            params.query_pairs(),
            vec![("bvid", "BV1xx411c7mD".to_string())]
        );
        Ok(())
    }

    #[test]
    fn video_view_params_serializes_aid_query() -> Result<(), BpiError> {
        let params = VideoViewParams::from_aid(Aid::new(170001)?);

        assert_eq!(params.query_pairs(), vec![("aid", "170001".to_string())]);
        Ok(())
    }
}
