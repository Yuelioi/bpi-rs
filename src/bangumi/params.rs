use crate::ids::{EpisodeId, MediaId, SeasonId};
use crate::{BpiError, BpiResult};

use super::timeline::BangumiTimelineType;

/// Parameters for `/pgc/review/user`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BangumiInfoParams {
    media_id: MediaId,
}

impl BangumiInfoParams {
    pub fn new(media_id: MediaId) -> Self {
        Self { media_id }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("media_id", self.media_id.to_string())]
    }
}

/// Identifies a bangumi detail request by season ID or episode ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BangumiDetailId {
    Season(SeasonId),
    Episode(EpisodeId),
}

/// Parameters for `/pgc/view/web/season`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BangumiDetailParams {
    id: BangumiDetailId,
}

impl BangumiDetailParams {
    pub fn from_season_id(season_id: SeasonId) -> Self {
        Self {
            id: BangumiDetailId::Season(season_id),
        }
    }

    pub fn from_episode_id(episode_id: EpisodeId) -> Self {
        Self {
            id: BangumiDetailId::Episode(episode_id),
        }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        match self.id {
            BangumiDetailId::Season(season_id) => {
                vec![("season_id", season_id.to_string())]
            }
            BangumiDetailId::Episode(episode_id) => {
                vec![("ep_id", episode_id.to_string())]
            }
        }
    }
}

/// Parameters for `/pgc/web/season/section`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BangumiSectionsParams {
    season_id: SeasonId,
}

impl BangumiSectionsParams {
    pub fn new(season_id: SeasonId) -> Self {
        Self { season_id }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("season_id", self.season_id.to_string())]
    }
}

/// Parameters for `/pgc/web/timeline`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BangumiTimelineParams {
    timeline_type: BangumiTimelineType,
    before: i32,
    after: i32,
}

impl BangumiTimelineParams {
    pub fn new(timeline_type: BangumiTimelineType, before: i32, after: i32) -> BpiResult<Self> {
        Ok(Self {
            timeline_type,
            before: validate_timeline_day_offset("before", before)?,
            after: validate_timeline_day_offset("after", after)?,
        })
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("types", self.timeline_type.as_i32().to_string()),
            ("before", self.before.to_string()),
            ("after", self.after.to_string()),
        ]
    }
}

fn validate_timeline_day_offset(field: &'static str, value: i32) -> BpiResult<i32> {
    if !(0..=7).contains(&value) {
        return Err(BpiError::invalid_parameter(
            field,
            "value must be between 0 and 7",
        ));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bangumi_info_params_serializes_media_id() -> BpiResult<()> {
        let params = BangumiInfoParams::new(MediaId::new(28_220_978)?);

        assert_eq!(
            params.query_pairs(),
            vec![("media_id", "28220978".to_string())]
        );
        Ok(())
    }

    #[test]
    fn bangumi_detail_params_serializes_season_id() -> BpiResult<()> {
        let params = BangumiDetailParams::from_season_id(SeasonId::new(1172)?);

        assert_eq!(
            params.query_pairs(),
            vec![("season_id", "1172".to_string())]
        );
        Ok(())
    }

    #[test]
    fn bangumi_detail_params_serializes_episode_id() -> BpiResult<()> {
        let params = BangumiDetailParams::from_episode_id(EpisodeId::new(21265)?);

        assert_eq!(params.query_pairs(), vec![("ep_id", "21265".to_string())]);
        Ok(())
    }

    #[test]
    fn bangumi_sections_params_serializes_season_id() -> BpiResult<()> {
        let params = BangumiSectionsParams::new(SeasonId::new(1172)?);

        assert_eq!(
            params.query_pairs(),
            vec![("season_id", "1172".to_string())]
        );
        Ok(())
    }

    #[test]
    fn bangumi_timeline_params_serializes_query() -> BpiResult<()> {
        let params = BangumiTimelineParams::new(BangumiTimelineType::Anime, 3, 7)?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("types", "1".to_string()),
                ("before", "3".to_string()),
                ("after", "7".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn bangumi_timeline_params_rejects_large_before() {
        let err = BangumiTimelineParams::new(BangumiTimelineType::Anime, 8, 7).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "before",
                ..
            }
        ));
    }
}
