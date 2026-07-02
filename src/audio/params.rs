use crate::ids::AudioId;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BpiResult;

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
}
