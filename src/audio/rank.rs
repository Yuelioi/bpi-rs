// 音频榜单
//
// [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/audio/rank.md)

use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResponse;
use crate::audio::AudioClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankPeriodData {
    pub list: std::collections::HashMap<String, Vec<AudioRankPeriod>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankPeriod {
    #[serde(rename = "ID")]
    pub id: u64,
    pub priod: u64,
    pub publish_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankDetailData {
    pub listen_fid: u64,
    pub all_fid: u64,
    pub fav_mid: u64,
    pub cover_url: String,
    pub is_subscribe: bool,
    pub listen_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankMusicListData {
    pub list: Vec<AudioRankMusicItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankMusicItem {
    pub music_id: String,
    pub music_title: String,
    pub singer: String,
    pub album: String,
    pub mv_aid: u64,
    pub mv_bvid: String,
    pub mv_cover: String,
    pub heat: u64,
    pub rank: u64,
    pub can_listen: bool,
    pub recommendation: String,
    pub creation_aid: u64,
    pub creation_bvid: String,
    pub creation_cover: String,
    pub creation_title: String,
    pub creation_up: u64,
    pub creation_nickname: String,
    pub creation_duration: u64,
    pub creation_play: u64,
    pub creation_reason: String,
    pub achievements: Vec<String>,
    pub material_id: u64,
    pub material_use_num: u64,
    pub material_duration: u64,
    pub material_show: u64,
    pub song_type: u64,
}

impl<'a> AudioClient<'a> {
    /// 订阅或退订榜单
    ///
    /// # 参数
    /// | 名称      | 类型           | 说明                       |
    /// | --------- | -------------- | -------------------------- |
    /// | `state`   | u32            | 操作代码（1：订阅，2：退订）|
    /// | `list_id` | `Option<u64>`    | 榜单 id（可选）            |
    ///
    /// # 文档
    /// [订阅或退订榜单](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/audio/rank.md#订阅或退订榜单)
    pub async fn audio_rank_subscribe(
        &self,
        state: u32,
        list_id: Option<u64>,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;
        let mut params = vec![("state", state.to_string()), ("csrf", csrf.to_string())];
        if let Some(id) = list_id {
            params.push(("list_id", id.to_string()));
        }

        self.client
            .post("https://api.bilibili.com/x/copyright-music-publicity/toplist/subscribe/update")
            .form(&params)
            .send_bpi("订阅或退订榜单")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audio::params::{AudioRankListParams, AudioRankListType, AudioRankPeriodParams};
    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;
    use crate::{ApiEnvelope, BpiResult};

    const TEST_LIST_ID: u64 = 76;

    fn contract(endpoint: &str) -> BpiResult<EndpointContract> {
        let bytes = match endpoint {
            "rank-period" => {
                include_bytes!("../../tests/contracts/audio/rank-period/contract.json").as_slice()
            }
            "rank-detail" => {
                include_bytes!("../../tests/contracts/audio/rank-detail/contract.json").as_slice()
            }
            "rank-music-list" => {
                include_bytes!("../../tests/contracts/audio/rank-music-list/contract.json")
                    .as_slice()
            }
            _ => unreachable!("unknown audio rank contract"),
        };

        EndpointContract::from_slice(bytes)
    }

    #[test]
    fn audio_rank_period_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = contract("rank-period")?;
        let params = AudioRankPeriodParams::new(AudioRankListType::Original);

        assert_eq!(contract.name, "audio.rank_period");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.bilibili.com/x/copyright-music-publicity/toplist/all_period"
        );
        assert_eq!(
            contract.request.query.get("list_type").map(String::as_str),
            Some("2")
        );
        assert_eq!(
            contract.request.query.get("csrf").map(String::as_str),
            Some("${csrf}")
        );
        assert_eq!(
            params.query_pairs(""),
            vec![("list_type", "2".to_string()), ("csrf", String::new())]
        );
        assert_eq!(contract.cases.len(), 3);
        assert_eq!(
            contract.cases[0].response.rust_model.as_deref(),
            Some("AudioRankPeriodData")
        );
        Ok(())
    }

    #[test]
    fn audio_rank_period_response_fixture_parses_declared_model() -> BpiResult<()> {
        let payload = ApiEnvelope::<AudioRankPeriodData>::from_slice(include_bytes!(
            "../../tests/contracts/audio/rank-period/responses/success.json"
        ))?
        .into_payload()?;

        assert!(!payload.list.is_empty());
        Ok(())
    }

    #[test]
    fn audio_rank_detail_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = contract("rank-detail")?;
        let params = AudioRankListParams::new(TEST_LIST_ID)?;

        assert_eq!(contract.name, "audio.rank_detail");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.bilibili.com/x/copyright-music-publicity/toplist/detail"
        );
        assert_eq!(
            contract.request.query.get("list_id").map(String::as_str),
            Some("76")
        );
        assert_eq!(
            params.query_pairs(""),
            vec![("list_id", "76".to_string()), ("csrf", String::new())]
        );
        assert_eq!(
            contract.cases[2].response.fixture.as_deref(),
            Some("responses/vip.success.json")
        );
        Ok(())
    }

    #[test]
    fn audio_rank_detail_response_fixtures_parse_declared_model() -> BpiResult<()> {
        let public_payload = ApiEnvelope::<AudioRankDetailData>::from_slice(include_bytes!(
            "../../tests/contracts/audio/rank-detail/responses/public.success.json"
        ))?
        .into_payload()?;
        let vip_payload = ApiEnvelope::<AudioRankDetailData>::from_slice(include_bytes!(
            "../../tests/contracts/audio/rank-detail/responses/vip.success.json"
        ))?
        .into_payload()?;

        assert_eq!(public_payload.listen_fid, vip_payload.listen_fid);
        assert!(!public_payload.is_subscribe);
        assert!(vip_payload.is_subscribe);
        Ok(())
    }

    #[test]
    fn audio_rank_music_list_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = contract("rank-music-list")?;

        assert_eq!(contract.name, "audio.rank_music_list");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.bilibili.com/x/copyright-music-publicity/toplist/music_list"
        );
        assert_eq!(
            contract.cases[0].response.rust_model.as_deref(),
            Some("AudioRankMusicListData")
        );
        Ok(())
    }

    #[test]
    fn audio_rank_music_list_response_fixture_parses_declared_model() -> BpiResult<()> {
        let payload = ApiEnvelope::<AudioRankMusicListData>::from_slice(include_bytes!(
            "../../tests/contracts/audio/rank-music-list/responses/success.json"
        ))?
        .into_payload()?;

        assert!(!payload.list.is_empty());
        assert_eq!(payload.list[0].rank, 1);
        Ok(())
    }

    fn local_probe_body(endpoint: &str, profile: &str) -> Option<serde_json::Value> {
        let path =
            format!("target/bpi-probe-runs/audio/extra-read/{endpoint}/{profile}.response.json");
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("response")
            .and_then(|response| response.get("body"))
            .cloned()
    }

    #[test]
    fn audio_rank_models_match_local_probe_outputs_when_available() -> BpiResult<()> {
        for profile in ["anonymous", "normal", "vip"] {
            if let Some(body) = local_probe_body("rank-period", profile) {
                let payload = serde_json::from_value::<ApiEnvelope<AudioRankPeriodData>>(body)?
                    .into_payload()?;

                assert!(!payload.list.is_empty());
            }

            if let Some(body) = local_probe_body("rank-detail", profile) {
                let payload = serde_json::from_value::<ApiEnvelope<AudioRankDetailData>>(body)?
                    .into_payload()?;

                assert!(payload.listen_fid > 0);
            }

            if let Some(body) = local_probe_body("rank-music-list", profile) {
                let payload = serde_json::from_value::<ApiEnvelope<AudioRankMusicListData>>(body)?
                    .into_payload()?;

                assert!(!payload.list.is_empty());
            }
        }
        Ok(())
    }
}
