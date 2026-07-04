// B站视频交互接口(Web端)
//
// [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)

use crate::BilibiliRequest;
use crate::BpiError;
use crate::response::BpiResult;
use crate::video::VideoClient;
use serde::{Deserialize, Serialize};

const LIKE_ENDPOINT: &str = "https://api.bilibili.com/x/web-interface/archive/like";
const COIN_ENDPOINT: &str = "https://api.bilibili.com/x/web-interface/coin/add";
const FAVORITE_ENDPOINT: &str = "https://api.bilibili.com/x/v3/fav/resource/deal";

/// 投币视频 - 响应结构体
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CoinData {
    /// 是否点赞成功
    pub like: bool,
}

/// 收藏视频 - 响应结构体
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FavoriteData {
    /// 是否为未关注用户收藏
    pub prompt: bool,
    /// 作用不明确
    pub ga_data: Option<serde_json::Value>,
    /// 提示消息
    pub toast_msg: Option<String>,
    /// 成功数
    pub success_num: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LegacyVideoIdForm {
    aid: String,
    bvid: String,
}

/// Parameters for video like/unlike operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoLikeParams {
    video_id: LegacyVideoIdForm,
    like: u8,
}

impl VideoLikeParams {
    pub fn from_aid(aid: u64, like: u8) -> BpiResult<Self> {
        Self::from_ids(Some(aid), None, like)
    }

    pub fn from_bvid(bvid: impl Into<String>, like: u8) -> BpiResult<Self> {
        Self::from_ids(None, Some(bvid.into()), like)
    }

    pub fn from_ids(aid: Option<u64>, bvid: Option<String>, like: u8) -> BpiResult<Self> {
        let video_id = legacy_video_id_form(aid, bvid)?;
        validate_like_action(like)?;

        Ok(Self { video_id, like })
    }

    fn form_pairs(&self, csrf: &str) -> Vec<(&'static str, String)> {
        vec![
            ("aid", self.video_id.aid.clone()),
            ("bvid", self.video_id.bvid.clone()),
            ("like", self.like.to_string()),
            ("csrf", csrf.to_string()),
        ]
    }
}

/// Parameters for video coin operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoCoinParams {
    video_id: LegacyVideoIdForm,
    multiply: u8,
    select_like: u8,
}

impl VideoCoinParams {
    pub fn from_aid(aid: u64, multiply: u8) -> BpiResult<Self> {
        Self::from_ids(Some(aid), None, multiply)
    }

    pub fn from_bvid(bvid: impl Into<String>, multiply: u8) -> BpiResult<Self> {
        Self::from_ids(None, Some(bvid.into()), multiply)
    }

    pub fn from_ids(aid: Option<u64>, bvid: Option<String>, multiply: u8) -> BpiResult<Self> {
        let video_id = legacy_video_id_form(aid, bvid)?;
        validate_coin_multiply(multiply)?;

        Ok(Self {
            video_id,
            multiply,
            select_like: 0,
        })
    }

    pub fn select_like(mut self, select_like: bool) -> Self {
        self.select_like = u8::from(select_like);
        self
    }

    fn form_pairs(&self, csrf: &str) -> Vec<(&'static str, String)> {
        vec![
            ("aid", self.video_id.aid.clone()),
            ("bvid", self.video_id.bvid.clone()),
            ("multiply", self.multiply.to_string()),
            ("select_like", self.select_like.to_string()),
            ("csrf", csrf.to_string()),
        ]
    }
}

/// Parameters for video favorite add/remove operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoFavoriteParams {
    rid: u64,
    add_media_ids: Vec<String>,
    del_media_ids: Vec<String>,
}

impl VideoFavoriteParams {
    pub fn new(
        rid: u64,
        add_media_ids: impl IntoIterator<Item = impl Into<String>>,
        del_media_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> BpiResult<Self> {
        if rid == 0 {
            return Err(BpiError::invalid_parameter("rid", "id must be non-zero"));
        }

        let add_media_ids = normalize_id_list("add_media_ids", add_media_ids)?;
        let del_media_ids = normalize_id_list("del_media_ids", del_media_ids)?;

        if add_media_ids.is_empty() && del_media_ids.is_empty() {
            return Err(BpiError::invalid_parameter(
                "media_ids",
                "at least one add or delete media id is required",
            ));
        }

        Ok(Self {
            rid,
            add_media_ids,
            del_media_ids,
        })
    }

    fn form_pairs(&self, csrf: &str) -> Vec<(&'static str, String)> {
        let mut pairs = vec![
            ("rid", self.rid.to_string()),
            ("type", "2".to_string()),
            ("csrf", csrf.to_string()),
        ];

        if !self.add_media_ids.is_empty() {
            pairs.push(("add_media_ids", self.add_media_ids.join(",")));
        }
        if !self.del_media_ids.is_empty() {
            pairs.push(("del_media_ids", self.del_media_ids.join(",")));
        }

        pairs
    }
}

impl<'a> VideoClient<'a> {
    /// Likes or unlikes a video and returns the canonical payload result.
    pub async fn like(&self, params: VideoLikeParams) -> BpiResult<CoinData> {
        let csrf = self.client.csrf()?;

        self.client
            .post(LIKE_ENDPOINT)
            .with_bilibili_headers()
            .form(&params.form_pairs(&csrf))
            .send_bpi_payload("video.like")
            .await
    }

    /// Gives coins to a video and returns the canonical payload result.
    pub async fn coin(&self, params: VideoCoinParams) -> BpiResult<CoinData> {
        let csrf = self.client.csrf()?;

        self.client
            .post(COIN_ENDPOINT)
            .with_bilibili_headers()
            .form(&params.form_pairs(&csrf))
            .send_bpi_payload("video.coin")
            .await
    }

    /// Favorites a video to, or removes it from, favorite folders.
    pub async fn favorite(&self, params: VideoFavoriteParams) -> BpiResult<FavoriteData> {
        let csrf = self.client.csrf()?;

        self.client
            .post(FAVORITE_ENDPOINT)
            .with_bilibili_headers()
            .form(&params.form_pairs(&csrf))
            .send_bpi_payload("video.favorite")
            .await
    }
}

fn legacy_video_id_form(
    aid: Option<u64>,
    bvid: Option<String>,
) -> Result<LegacyVideoIdForm, BpiError> {
    let aid = match aid {
        Some(0) => {
            return Err(BpiError::invalid_parameter("aid", "id must be non-zero"));
        }
        Some(aid) => Some(aid.to_string()),
        None => None,
    };

    let bvid = match bvid {
        Some(bvid) if bvid.trim().is_empty() => {
            return Err(BpiError::invalid_parameter("bvid", "bvid cannot be blank"));
        }
        Some(bvid) => Some(bvid),
        None => None,
    };

    if aid.is_none() && bvid.is_none() {
        return Err(BpiError::invalid_parameter(
            "video_id",
            "aid or bvid is required",
        ));
    }

    Ok(LegacyVideoIdForm {
        aid: aid.unwrap_or_else(|| "0".to_string()),
        bvid: bvid.unwrap_or_default(),
    })
}

fn validate_like_action(like: u8) -> Result<(), BpiError> {
    if matches!(like, 1 | 2) {
        return Ok(());
    }

    Err(BpiError::invalid_parameter("like", "value must be 1 or 2"))
}

fn validate_coin_multiply(multiply: u8) -> Result<(), BpiError> {
    if matches!(multiply, 1 | 2) {
        return Ok(());
    }

    Err(BpiError::invalid_parameter(
        "multiply",
        "value must be 1 or 2",
    ))
}

fn normalize_id_list(
    field: &'static str,
    values: impl IntoIterator<Item = impl Into<String>>,
) -> BpiResult<Vec<String>> {
    values
        .into_iter()
        .map(|value| {
            let value = value.into();
            let value = value.trim();
            if value.is_empty() {
                return Err(BpiError::invalid_parameter(field, "value cannot be blank"));
            }

            Ok(value.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::BpiError;

    use super::{
        VideoCoinParams, VideoFavoriteParams, VideoLikeParams, legacy_video_id_form,
        validate_coin_multiply,
    };

    #[test]
    fn legacy_video_id_form_rejects_missing_video_id() {
        let err = legacy_video_id_form(None, None).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "video_id",
                ..
            }
        ));
    }

    #[test]
    fn validate_coin_multiply_rejects_oversized_value() {
        let err = validate_coin_multiply(3).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "multiply",
                ..
            }
        ));
    }

    #[test]
    fn video_like_params_serializes_aid() -> Result<(), BpiError> {
        let params = VideoLikeParams::from_aid(170001, 1)?;

        assert_eq!(
            params.form_pairs("csrf-token"),
            vec![
                ("aid", "170001".to_string()),
                ("bvid", String::new()),
                ("like", "1".to_string()),
                ("csrf", "csrf-token".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn video_coin_params_defaults_select_like_to_false() -> Result<(), BpiError> {
        let params = VideoCoinParams::from_bvid("BV1xx411c7mD", 2)?;

        assert_eq!(
            params.form_pairs("csrf-token"),
            vec![
                ("aid", "0".to_string()),
                ("bvid", "BV1xx411c7mD".to_string()),
                ("multiply", "2".to_string()),
                ("select_like", "0".to_string()),
                ("csrf", "csrf-token".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn video_favorite_params_rejects_empty_operation() {
        let err = VideoFavoriteParams::new(170001, Vec::<String>::new(), Vec::<String>::new())
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "media_ids",
                ..
            }
        ));
    }

    #[test]
    fn video_favorite_params_rejects_blank_media_id() {
        let err = VideoFavoriteParams::new(170001, [" "], Vec::<String>::new()).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "add_media_ids",
                ..
            }
        ));
    }
}
