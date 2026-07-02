use crate::audio::musicstream_url::AudioQuality;
use crate::ids::AudioId;
use crate::{BpiError, BpiResult};

/// Parameters for audio endpoints that identify a single song by `sid`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioSongParams {
    sid: AudioId,
}

impl AudioSongParams {
    pub fn new(sid: AudioId) -> Self {
        Self { sid }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("sid", self.sid.to_string())]
    }
}

/// Pagination parameters for audio list endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioPageParams {
    page: u32,
    page_size: u32,
}

impl AudioPageParams {
    pub fn new(page: u32, page_size: u32) -> BpiResult<Self> {
        Ok(Self {
            page: validate_nonzero("pn", page)?,
            page_size: validate_nonzero("ps", page_size)?,
        })
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("pn", self.page.to_string()),
            ("ps", self.page_size.to_string()),
        ]
    }
}

/// Parameters for `/audio/music-service-c/web/collections/info`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioCollectionInfoParams {
    sid: u64,
}

impl AudioCollectionInfoParams {
    pub fn new(sid: u64) -> BpiResult<Self> {
        Ok(Self {
            sid: validate_nonzero("sid", sid)?,
        })
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("sid", self.sid.to_string())]
    }
}

/// Audio rank list categories accepted by Bilibili's rank period endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioRankListType {
    Hot,
    Original,
    Custom(u32),
}

impl AudioRankListType {
    fn query_value(self) -> String {
        match self {
            Self::Hot => "1".to_string(),
            Self::Original => "2".to_string(),
            Self::Custom(value) => value.to_string(),
        }
    }
}

/// Parameters for `/x/copyright-music-publicity/toplist/all_period`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioRankPeriodParams {
    list_type: AudioRankListType,
}

impl AudioRankPeriodParams {
    pub fn new(list_type: AudioRankListType) -> Self {
        Self { list_type }
    }

    pub fn custom(list_type: u32) -> BpiResult<Self> {
        Ok(Self {
            list_type: AudioRankListType::Custom(validate_nonzero("list_type", list_type)?),
        })
    }

    pub(crate) fn query_pairs(&self, csrf: &str) -> Vec<(&'static str, String)> {
        vec![
            ("list_type", self.list_type.query_value()),
            ("csrf", csrf.to_string()),
        ]
    }
}

/// Parameters for audio rank endpoints that identify a single rank list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioRankListParams {
    list_id: u64,
}

impl AudioRankListParams {
    pub fn new(list_id: u64) -> BpiResult<Self> {
        Ok(Self {
            list_id: validate_nonzero("list_id", list_id)?,
        })
    }

    pub(crate) fn query_pairs(&self, csrf: &str) -> Vec<(&'static str, String)> {
        vec![
            ("list_id", self.list_id.to_string()),
            ("csrf", csrf.to_string()),
        ]
    }
}

/// Parameters for `/audio/music-service-c/web/url`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioStreamUrlWebParams {
    sid: AudioId,
    quality: AudioQuality,
    privilege: u32,
}

impl AudioStreamUrlWebParams {
    pub fn new(sid: AudioId) -> Self {
        Self {
            sid,
            quality: AudioQuality::HighQuality,
            privilege: 2,
        }
    }

    pub fn with_quality(mut self, quality: AudioQuality) -> Self {
        self.quality = quality;
        self
    }

    pub fn with_privilege(mut self, privilege: u32) -> BpiResult<Self> {
        self.privilege = validate_nonzero("privilege", privilege)?;
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("sid", self.sid.to_string()),
            ("quality", self.quality.as_u32().to_string()),
            ("privilege", self.privilege.to_string()),
        ]
    }
}

/// Parameters for `/audio/music-service-c/url`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioStreamUrlParams {
    song_id: AudioId,
    quality: AudioQuality,
    privilege: u32,
    mid: u64,
    platform: String,
}

impl AudioStreamUrlParams {
    pub fn new(song_id: AudioId, quality: AudioQuality) -> Self {
        Self {
            song_id,
            quality,
            privilege: 2,
            mid: 2,
            platform: "android".to_string(),
        }
    }

    pub fn with_privilege(mut self, privilege: u32) -> BpiResult<Self> {
        self.privilege = validate_nonzero("privilege", privilege)?;
        Ok(self)
    }

    pub fn with_mid(mut self, mid: u64) -> BpiResult<Self> {
        self.mid = validate_nonzero("mid", mid)?;
        Ok(self)
    }

    pub fn with_platform(mut self, platform: impl Into<String>) -> BpiResult<Self> {
        let platform = platform.into();
        let platform = platform.trim();
        if platform.is_empty() {
            return Err(BpiError::invalid_parameter(
                "platform",
                "value cannot be blank",
            ));
        }

        self.platform = platform.to_string();
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("songid", self.song_id.to_string()),
            ("quality", self.quality.as_u32().to_string()),
            ("privilege", self.privilege.to_string()),
            ("mid", self.mid.to_string()),
            ("platform", self.platform.clone()),
        ]
    }
}

fn validate_nonzero<T>(field: &'static str, value: T) -> BpiResult<T>
where
    T: PartialEq + From<u8>,
{
    if value == T::from(0) {
        return Err(BpiError::invalid_parameter(field, "value must be non-zero"));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audio_song_params_serializes_sid_query() -> BpiResult<()> {
        let params = AudioSongParams::new(AudioId::new(13603)?);

        assert_eq!(params.query_pairs(), vec![("sid", "13603".to_string())]);
        Ok(())
    }

    #[test]
    fn audio_song_params_accepts_owned_audio_id() -> BpiResult<()> {
        let sid = AudioId::new(15664)?;
        let params = AudioSongParams::new(sid);

        assert_eq!(params.query_pairs(), vec![("sid", "15664".to_string())]);
        Ok(())
    }

    #[test]
    fn audio_page_params_serializes_page_query() -> BpiResult<()> {
        let params = AudioPageParams::new(1, 20)?;

        assert_eq!(
            params.query_pairs(),
            vec![("pn", "1".to_string()), ("ps", "20".to_string())]
        );
        Ok(())
    }

    #[test]
    fn audio_page_params_rejects_zero_page() {
        let err = AudioPageParams::new(0, 20).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "pn", .. }
        ));
    }

    #[test]
    fn audio_collection_info_params_serializes_collection_id_query() -> BpiResult<()> {
        let params = AudioCollectionInfoParams::new(15_967_839)?;

        assert_eq!(params.query_pairs(), vec![("sid", "15967839".to_string())]);
        Ok(())
    }

    #[test]
    fn audio_rank_period_params_serializes_builtin_list_type() {
        let params = AudioRankPeriodParams::new(AudioRankListType::Original);

        assert_eq!(
            params.query_pairs("csrf-token"),
            vec![
                ("list_type", "2".to_string()),
                ("csrf", "csrf-token".to_string())
            ]
        );
    }

    #[test]
    fn audio_rank_period_params_rejects_zero_custom_type() {
        let err = AudioRankPeriodParams::custom(0).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "list_type",
                ..
            }
        ));
    }

    #[test]
    fn audio_rank_list_params_serializes_list_id_query() -> BpiResult<()> {
        let params = AudioRankListParams::new(76)?;

        assert_eq!(
            params.query_pairs("csrf-token"),
            vec![
                ("list_id", "76".to_string()),
                ("csrf", "csrf-token".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn audio_stream_url_web_params_serializes_default_query() -> BpiResult<()> {
        let params = AudioStreamUrlWebParams::new(AudioId::new(13603)?);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("sid", "13603".to_string()),
                ("quality", "2".to_string()),
                ("privilege", "2".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn audio_stream_url_params_serializes_default_query() -> BpiResult<()> {
        let params = AudioStreamUrlParams::new(AudioId::new(15664)?, AudioQuality::HighQuality);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("songid", "15664".to_string()),
                ("quality", "2".to_string()),
                ("privilege", "2".to_string()),
                ("mid", "2".to_string()),
                ("platform", "android".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn audio_stream_url_params_serializes_custom_query() -> BpiResult<()> {
        let params = AudioStreamUrlParams::new(AudioId::new(15664)?, AudioQuality::Lossless)
            .with_privilege(3)?
            .with_mid(42)?
            .with_platform("ios")?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("songid", "15664".to_string()),
                ("quality", "3".to_string()),
                ("privilege", "3".to_string()),
                ("mid", "42".to_string()),
                ("platform", "ios".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn audio_stream_url_params_rejects_blank_platform() {
        let err = AudioStreamUrlParams::new(
            AudioId::new(15664).expect("valid audio id"),
            AudioQuality::HighQuality,
        )
        .with_platform("  ")
        .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "platform",
                ..
            }
        ));
    }
}
