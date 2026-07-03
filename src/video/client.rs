use crate::request::BilibiliRequest;
use crate::{BpiClient, BpiResult};

use super::model::{VideoDetail, VideoPage, VideoView};
use super::params::{
    VideoDescParams, VideoDetailParams, VideoPageListParams, VideoPlayUrlParams, VideoViewParams,
};
use super::videostream_url::{PLAY_URL_ENDPOINT, PlayUrlResponseData};

const DESC_ENDPOINT: &str = "https://api.bilibili.com/x/web-interface/archive/desc";
const DETAIL_ENDPOINT: &str = "https://api.bilibili.com/x/web-interface/view/detail";
const PAGELIST_ENDPOINT: &str = "https://api.bilibili.com/x/player/pagelist";
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

    #[cfg(test)]
    pub(crate) fn detail_endpoint(&self) -> &'static str {
        DETAIL_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn page_list_endpoint(&self) -> &'static str {
        PAGELIST_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn desc_endpoint(&self) -> &'static str {
        DESC_ENDPOINT
    }

    /// Fetches web video detail by AV ID or BV ID.
    pub async fn view(&self, params: VideoViewParams) -> BpiResult<VideoView> {
        self.client
            .get(VIEW_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi_payload("video.view")
            .await
    }

    /// Fetches web video detail, including tags and related videos.
    pub async fn detail(&self, params: VideoDetailParams) -> BpiResult<VideoDetail> {
        self.client
            .get(DETAIL_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi_payload("video.detail")
            .await
    }

    /// Fetches the page/content IDs for a video.
    pub async fn page_list(&self, params: VideoPageListParams) -> BpiResult<Vec<VideoPage>> {
        self.client
            .get(PAGELIST_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi_payload("video.pagelist")
            .await
    }

    /// Fetches the plain text video description.
    pub async fn desc(&self, params: VideoDescParams) -> BpiResult<String> {
        self.client
            .get(DESC_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi_payload("video.desc")
            .await
    }

    /// Fetches signed web playback URLs by AV ID or BV ID plus page/content ID.
    pub async fn play_url(&self, params: VideoPlayUrlParams) -> BpiResult<PlayUrlResponseData> {
        let params = self.client.get_wbi_sign2(params.query_pairs()).await?;

        self.client
            .get(PLAY_URL_ENDPOINT)
            .with_bilibili_headers()
            .query(&params)
            .send_bpi_payload("video.play_url")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ApiEnvelope, BpiClient, BpiError, BpiResult,
        probe::{contract::HttpMethod, endpoint_contract::EndpointContract},
    };
    use serde::de::DeserializeOwned;

    fn contract(endpoint: &str) -> BpiResult<EndpointContract> {
        let bytes: &[u8] = match endpoint {
            "view" => include_bytes!("../../tests/contracts/video/info-read/view/contract.json"),
            "detail" => {
                include_bytes!("../../tests/contracts/video/info-read/detail/contract.json")
            }
            "pagelist" => {
                include_bytes!("../../tests/contracts/video/info-read/pagelist/contract.json")
            }
            "desc" => include_bytes!("../../tests/contracts/video/info-read/desc/contract.json"),
            _ => {
                return Err(BpiError::invalid_parameter(
                    "endpoint",
                    "unknown video contract",
                ));
            }
        };

        EndpointContract::from_slice(bytes)
    }

    fn local_probe_body(endpoint: &str, profile: &str) -> Option<serde_json::Value> {
        let path =
            format!("target/bpi-probe-runs/video/info-read/{endpoint}/{profile}.response.json");
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("response")
            .and_then(|response| response.get("body"))
            .cloned()
    }

    fn parse_local_probe_outputs<T>(endpoint: &str, profiles: &[&str]) -> BpiResult<()>
    where
        T: DeserializeOwned,
    {
        for profile in profiles {
            let Some(body) = local_probe_body(endpoint, profile) else {
                continue;
            };

            let _payload = serde_json::from_value::<ApiEnvelope<T>>(body)?.into_payload()?;
        }

        Ok(())
    }

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

    #[test]
    fn video_client_exposes_info_read_endpoints() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let video = client.video();

        assert_eq!(
            video.detail_endpoint(),
            "https://api.bilibili.com/x/web-interface/view/detail"
        );
        assert_eq!(
            video.page_list_endpoint(),
            "https://api.bilibili.com/x/player/pagelist"
        );
        assert_eq!(
            video.desc_endpoint(),
            "https://api.bilibili.com/x/web-interface/archive/desc"
        );
        Ok(())
    }

    #[test]
    fn video_client_methods_use_payload_request_helpers() {
        let source = include_str!("client.rs");
        let payload_helper = concat!(".send_", "bpi_payload");
        let legacy_envelope_helper = concat!(".send_", "bpi::<");
        let legacy_flat_playurl = concat!(".video_", "playurl(");

        assert!(
            source.matches(payload_helper).count() >= 5,
            "VideoClient read methods should return decoded payloads directly"
        );
        assert!(
            !source.contains(legacy_envelope_helper),
            "VideoClient should not use legacy envelope-returning request helpers"
        );
        assert!(
            !source.contains(legacy_flat_playurl),
            "VideoClient::play_url should be implemented as a payload-helper-backed domain method"
        );
    }

    #[test]
    fn video_info_read_contracts_match_endpoint_requests() -> BpiResult<()> {
        let expectations = [
            (
                "view",
                "video.view",
                VIEW_ENDPOINT,
                &[("bvid", "BV1xx411c7mD")][..],
                "VideoView",
            ),
            (
                "detail",
                "video.detail",
                DETAIL_ENDPOINT,
                &[("bvid", "BV1xx411c7mD"), ("need_elec", "0")][..],
                "VideoDetail",
            ),
            (
                "pagelist",
                "video.pagelist",
                PAGELIST_ENDPOINT,
                &[("bvid", "BV1xx411c7mD")][..],
                "Vec<VideoPage>",
            ),
            (
                "desc",
                "video.desc",
                DESC_ENDPOINT,
                &[("bvid", "BV1xx411c7mD")][..],
                "String",
            ),
        ];

        for (endpoint, name, url, query_pairs, rust_model) in expectations {
            let contract = contract(endpoint)?;

            assert_eq!(contract.name, name);
            assert_eq!(contract.request.method, HttpMethod::Get);
            assert_eq!(contract.request.url.as_str(), url);
            assert_eq!(contract.cases.len(), 3);
            assert!(
                contract
                    .cases
                    .iter()
                    .all(|case| case.response.api_code == Some(0)),
                "{endpoint} should have successful anonymous, normal, and vip cases"
            );
            assert!(
                contract
                    .cases
                    .iter()
                    .any(|case| case.response.rust_model.as_deref() == Some(rust_model)),
                "{endpoint} should declare {rust_model}"
            );

            for &(key, value) in query_pairs {
                assert_eq!(
                    contract.request.query.get(key).map(String::as_str),
                    Some(value)
                );
            }
        }

        Ok(())
    }

    #[test]
    fn video_info_read_response_fixtures_parse_declared_models() -> BpiResult<()> {
        let view = ApiEnvelope::<VideoView>::from_slice(include_bytes!(
            "../../tests/contracts/video/info-read/view/responses/success.json"
        ))?
        .into_payload()?;
        let detail = ApiEnvelope::<VideoDetail>::from_slice(include_bytes!(
            "../../tests/contracts/video/info-read/detail/responses/success.json"
        ))?
        .into_payload()?;
        let pagelist = ApiEnvelope::<Vec<VideoPage>>::from_slice(include_bytes!(
            "../../tests/contracts/video/info-read/pagelist/responses/success.json"
        ))?
        .into_payload()?;
        let desc = ApiEnvelope::<String>::from_slice(include_bytes!(
            "../../tests/contracts/video/info-read/desc/responses/success.json"
        ))?
        .into_payload()?;

        assert_eq!(view.bvid.as_str(), "BV1xx411c7mD");
        assert_eq!(detail.view.bvid.as_str(), "BV1xx411c7mD");
        assert_eq!(pagelist.len(), 1);
        assert_eq!(desc, "www");
        Ok(())
    }

    #[test]
    fn video_info_read_models_match_local_probe_outputs_when_available() -> BpiResult<()> {
        parse_local_probe_outputs::<VideoView>("view", &["anonymous", "normal", "vip"])?;
        parse_local_probe_outputs::<VideoDetail>("detail", &["anonymous", "normal", "vip"])?;
        parse_local_probe_outputs::<Vec<VideoPage>>("pagelist", &["anonymous", "normal", "vip"])?;
        parse_local_probe_outputs::<String>("desc", &["anonymous", "normal", "vip"])?;

        Ok(())
    }
}
