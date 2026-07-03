use crate::live::info::RoomInfoData;
use crate::live::live_area::LiveParentArea;
use crate::live::live_stream::LiveStreamData;
use crate::live::manage::PcLiveVersionData;
use crate::live::recommend::RecommendData;
use crate::{BilibiliRequest, BpiClient, BpiResult};

const AREA_LIST_ENDPOINT: &str = "https://api.live.bilibili.com/room/v1/Area/getList";
const ROOM_INFO_ENDPOINT: &str = "https://api.live.bilibili.com/room/v1/Room/get_info";
const STREAM_ENDPOINT: &str = "https://api.live.bilibili.com/room/v1/Room/playUrl";
const RECOMMEND_ENDPOINT: &str =
    "https://api.live.bilibili.com/xlive/web-interface/v1/webMain/getMoreRecList";
const VERSION_ENDPOINT: &str =
    "https://api.live.bilibili.com/xlive/app-blink/v1/liveVersionInfo/getHomePageLiveVersion";

/// Live API client.
#[derive(Clone, Copy)]
pub struct LiveClient<'a> {
    client: &'a BpiClient,
}

impl<'a> LiveClient<'a> {
    pub(crate) fn new(client: &'a BpiClient) -> Self {
        Self { client }
    }

    #[cfg(test)]
    pub(crate) fn area_list_endpoint(&self) -> &'static str {
        AREA_LIST_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn room_info_endpoint(&self) -> &'static str {
        ROOM_INFO_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn stream_endpoint(&self) -> &'static str {
        STREAM_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn recommend_endpoint(&self) -> &'static str {
        RECOMMEND_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn version_endpoint(&self) -> &'static str {
        VERSION_ENDPOINT
    }

    /// Fetches all live area categories.
    pub async fn area_list(&self) -> BpiResult<Vec<LiveParentArea>> {
        self.client
            .get(AREA_LIST_ENDPOINT)
            .with_bilibili_headers()
            .send_bpi_payload("live.area_list")
            .await
    }

    /// Fetches public room information by real room ID.
    pub async fn room_info(&self, room_id: i64) -> BpiResult<RoomInfoData> {
        self.client
            .get(ROOM_INFO_ENDPOINT)
            .with_bilibili_headers()
            .query(&[("room_id", room_id.to_string())])
            .send_bpi_payload("live.room_info")
            .await
    }

    /// Fetches live stream URLs by real room ID.
    pub async fn stream(
        &self,
        cid: i64,
        platform: Option<&str>,
        quality: Option<i32>,
        qn: Option<i32>,
    ) -> BpiResult<LiveStreamData> {
        let mut query = vec![("cid", cid.to_string())];

        if let Some(platform) = platform {
            query.push(("platform", platform.to_string()));
        }
        if let Some(quality) = quality {
            query.push(("quality", quality.to_string()));
        }
        if let Some(qn) = qn {
            query.push(("qn", qn.to_string()));
        }

        self.client
            .get(STREAM_ENDPOINT)
            .with_bilibili_headers()
            .query(&query)
            .send_bpi_payload("live.stream")
            .await
    }

    /// Fetches the web homepage live recommendation list.
    pub async fn recommend(&self) -> BpiResult<RecommendData> {
        self.client
            .get(RECOMMEND_ENDPOINT)
            .with_bilibili_headers()
            .query(&[("platform", "web"), ("web_location", "333.1007")])
            .send_bpi_payload("live.recommend")
            .await
    }

    /// Fetches the current PC live client version metadata.
    pub async fn version(&self) -> BpiResult<PcLiveVersionData> {
        self.client
            .get(VERSION_ENDPOINT)
            .with_bilibili_headers()
            .query(&[("system_version", "2")])
            .send_bpi_payload("live.version")
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;

    use crate::live::info::RoomInfoData;
    use crate::live::live_area::LiveParentArea;
    use crate::live::live_stream::LiveStreamData;
    use crate::live::manage::PcLiveVersionData;
    use crate::live::recommend::RecommendData;
    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;
    use crate::{BpiClient, BpiError, BpiResult};

    fn assert_area_list_future<F>(_future: F)
    where
        F: Future<Output = BpiResult<Vec<LiveParentArea>>>,
    {
    }

    fn assert_room_info_future<F>(_future: F)
    where
        F: Future<Output = BpiResult<RoomInfoData>>,
    {
    }

    fn assert_stream_future<F>(_future: F)
    where
        F: Future<Output = BpiResult<LiveStreamData>>,
    {
    }

    fn assert_recommend_future<F>(_future: F)
    where
        F: Future<Output = BpiResult<RecommendData>>,
    {
    }

    fn assert_version_future<F>(_future: F)
    where
        F: Future<Output = BpiResult<PcLiveVersionData>>,
    {
    }

    fn contract(endpoint: &str) -> BpiResult<EndpointContract> {
        let bytes: &[u8] = match endpoint {
            "area-list" => {
                include_bytes!("../../tests/contracts/live/public-core/area-list/contract.json")
            }
            "room-info" => {
                include_bytes!("../../tests/contracts/live/public-core/room-info/contract.json")
            }
            "stream" => {
                include_bytes!("../../tests/contracts/live/public-core/stream/contract.json")
            }
            "recommend" => {
                include_bytes!("../../tests/contracts/live/public-core/recommend/contract.json")
            }
            "version" => {
                include_bytes!("../../tests/contracts/live/public-core/version/contract.json")
            }
            _ => {
                return Err(BpiError::invalid_parameter(
                    "endpoint",
                    "unknown live contract",
                ));
            }
        };

        EndpointContract::from_slice(bytes)
    }

    #[test]
    fn live_public_core_methods_return_payload_futures() -> Result<(), BpiError> {
        let client = BpiClient::new()?;
        let live = client.live();

        assert_area_list_future(live.area_list());
        assert_room_info_future(live.room_info(23_174_842));
        assert_stream_future(live.stream(14_073_662, Some("web"), None, Some(10_000)));
        assert_recommend_future(live.recommend());
        assert_version_future(live.version());
        Ok(())
    }

    #[test]
    fn live_public_core_contracts_match_module_client_endpoints() -> BpiResult<()> {
        let client = BpiClient::new()?;
        let live = client.live();
        let cases = [
            ("area-list", "live.area_list", live.area_list_endpoint()),
            ("room-info", "live.room_info", live.room_info_endpoint()),
            ("stream", "live.stream", live.stream_endpoint()),
            ("recommend", "live.recommend", live.recommend_endpoint()),
            ("version", "live.version", live.version_endpoint()),
        ];

        for (endpoint, name, url) in cases {
            let contract = contract(endpoint)?;

            assert_eq!(contract.name, name);
            assert_eq!(contract.request.method, HttpMethod::Get);
            assert_eq!(contract.request.url.as_str(), url);
        }

        Ok(())
    }
}
