use crate::ids::{Aid, SeasonId};

/// Parameters for `/x2/creative/web/season/aid`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeasonByAidParams {
    aid: Aid,
}

impl SeasonByAidParams {
    pub fn new(aid: Aid) -> Self {
        Self { aid }
    }

    pub fn query_pairs(self) -> [(&'static str, String); 1] {
        [("id", self.aid.to_string())]
    }
}

/// Parameters for `/x2/creative/web/season`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeasonInfoParams {
    season_id: SeasonId,
}

impl SeasonInfoParams {
    pub fn new(season_id: SeasonId) -> Self {
        Self { season_id }
    }

    pub fn query_pairs(self) -> [(&'static str, String); 1] {
        [("id", self.season_id.to_string())]
    }
}

/// Parameters for `/x2/creative/web/season/section`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeasonSectionEpisodesParams {
    season_id: SeasonId,
}

impl SeasonSectionEpisodesParams {
    pub fn new(season_id: SeasonId) -> Self {
        Self { season_id }
    }

    pub fn query_pairs(self) -> [(&'static str, String); 1] {
        [("id", self.season_id.to_string())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BpiError;

    #[test]
    fn season_info_params_serializes_query() -> Result<(), BpiError> {
        let params = SeasonInfoParams::new(SeasonId::new(4294056)?);

        assert_eq!(params.query_pairs(), [("id", "4294056".to_string())]);
        Ok(())
    }

    #[test]
    fn season_section_episodes_params_serializes_query() -> Result<(), BpiError> {
        let params = SeasonSectionEpisodesParams::new(SeasonId::new(176088)?);

        assert_eq!(params.query_pairs(), [("id", "176088".to_string())]);
        Ok(())
    }
}
