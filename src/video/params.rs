use crate::ids::{Aid, Bvid, Cid};

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

/// Parameters for `/x/player/wbi/playurl`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoPlayUrlParams {
    id: VideoId,
    cid: Cid,
    qn: Option<u64>,
    fnval: Option<u64>,
    fnver: Option<u64>,
    fourk: Option<u8>,
    platform: String,
    high_quality: Option<u8>,
    try_look: Option<u8>,
}

impl VideoPlayUrlParams {
    /// Creates play URL parameters from a validated AV ID and page/content ID.
    pub fn from_aid(aid: Aid, cid: Cid) -> Self {
        Self::new(VideoId::Aid(aid), cid)
    }

    /// Creates play URL parameters from a validated BV ID and page/content ID.
    pub fn from_bvid(bvid: Bvid, cid: Cid) -> Self {
        Self::new(VideoId::Bvid(bvid), cid)
    }

    fn new(id: VideoId, cid: Cid) -> Self {
        Self {
            id,
            cid,
            qn: None,
            fnval: None,
            fnver: None,
            fourk: None,
            platform: "pc".to_string(),
            high_quality: None,
            try_look: None,
        }
    }

    /// Sets the requested quality code.
    pub fn quality(mut self, qn: u64) -> Self {
        self.qn = Some(qn);
        self
    }

    /// Sets the Bilibili stream format bitmask.
    pub fn format_flags(mut self, fnval: u64) -> Self {
        self.fnval = Some(fnval);
        self
    }

    /// Sets the stream format version.
    pub fn format_version(mut self, fnver: u64) -> Self {
        self.fnver = Some(fnver);
        self
    }

    /// Controls whether 4K streams are allowed.
    pub fn fourk(mut self, enabled: bool) -> Self {
        self.fourk = Some(u8::from(enabled));
        self
    }

    /// Sets the API platform marker. Defaults to `pc`.
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = platform.into();
        self
    }

    /// Sets the high-quality playback flag.
    pub fn high_quality(mut self, enabled: bool) -> Self {
        self.high_quality = Some(u8::from(enabled));
        self
    }

    /// Controls whether trial viewing should be requested.
    pub fn try_look(mut self, enabled: bool) -> Self {
        self.try_look = Some(u8::from(enabled));
        self
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut params = vec![("cid", self.cid.to_string())];

        match &self.id {
            VideoId::Aid(aid) => params.push(("avid", aid.to_string())),
            VideoId::Bvid(bvid) => params.push(("bvid", bvid.to_string())),
        }
        if let Some(qn) = self.qn {
            params.push(("qn", qn.to_string()));
        }
        if let Some(fnval) = self.fnval {
            params.push(("fnval", fnval.to_string()));
        }
        if let Some(fnver) = self.fnver {
            params.push(("fnver", fnver.to_string()));
        }
        if let Some(fourk) = self.fourk {
            params.push(("fourk", fourk.to_string()));
        }
        params.push(("platform", self.platform.clone()));
        if let Some(high_quality) = self.high_quality {
            params.push(("high_quality", high_quality.to_string()));
        }
        if let Some(try_look) = self.try_look {
            params.push(("try_look", try_look.to_string()));
        }

        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BpiError;
    use crate::ids::{Aid, Cid};

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

    #[test]
    fn video_play_url_params_serializes_aid_query_with_default_platform() -> Result<(), BpiError> {
        let params = VideoPlayUrlParams::from_aid(Aid::new(170001)?, Cid::new(180001)?);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("cid", "180001".to_string()),
                ("avid", "170001".to_string()),
                ("platform", "pc".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn video_play_url_params_serializes_optional_playback_flags() -> Result<(), BpiError> {
        let params = VideoPlayUrlParams::from_bvid("BV1xx411c7mD".parse()?, Cid::new(180001)?)
            .quality(120)
            .format_flags(16 | 128)
            .format_version(0)
            .fourk(true)
            .high_quality(true)
            .try_look(false);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("cid", "180001".to_string()),
                ("bvid", "BV1xx411c7mD".to_string()),
                ("qn", "120".to_string()),
                ("fnval", "144".to_string()),
                ("fnver", "0".to_string()),
                ("fourk", "1".to_string()),
                ("platform", "pc".to_string()),
                ("high_quality", "1".to_string()),
                ("try_look", "0".to_string())
            ]
        );
        Ok(())
    }
}
