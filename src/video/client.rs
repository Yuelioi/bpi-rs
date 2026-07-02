use crate::request::BilibiliRequest;
use crate::{BpiClient, BpiResult};

use super::model::VideoView;
use super::params::{VideoPlayUrlParams, VideoViewParams};
use super::videostream_url::PlayUrlResponseData;

const VIEW_ENDPOINT: &str = "https://api.bilibili.com/x/web-interface/view";

/// Video domain API client.
#[derive(Clone, Copy)]
pub struct VideoClient<'a> {
    client: &'a BpiClient,
}

impl<'a> VideoClient<'a> {
    pub(crate) fn new(client: &'a BpiClient) -> Self {
        Self { client }
    }

    #[cfg(test)]
    pub(crate) fn endpoint(&self) -> &'static str {
        VIEW_ENDPOINT
    }

    /// Fetches web video detail by AV ID or BV ID.
    pub async fn view(&self, params: VideoViewParams) -> BpiResult<VideoView> {
        self.client
            .get(VIEW_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi::<VideoView>("video.view")
            .await?
            .into_data()
    }

    /// Fetches signed web playback URLs by AV ID or BV ID plus page/content ID.
    pub async fn play_url(&self, params: VideoPlayUrlParams) -> BpiResult<PlayUrlResponseData> {
        self.client.video_playurl(params).await?.into_data()
    }
}

#[cfg(test)]
mod tests {
    use crate::BpiClient;

    #[test]
    fn video_client_borrows_root_client() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let video = client.video();

        assert_eq!(
            video.endpoint(),
            "https://api.bilibili.com/x/web-interface/view"
        );
        Ok(())
    }
}
