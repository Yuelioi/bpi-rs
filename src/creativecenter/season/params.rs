use crate::ids::Aid;

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
