use crate::ids::Mid;
use crate::{BpiError, BpiResult};

/// Controls whether `/x/web-interface/card` should include the user's space header image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserCardPhoto {
    /// Include the user's space header image when Bilibili returns it.
    Include,
    /// Exclude the user's space header image.
    Exclude,
}

/// Parameters for `/x/web-interface/card`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserCardParams {
    mid: Mid,
    photo: Option<UserCardPhoto>,
}

impl UserCardParams {
    /// Creates card parameters for a validated user ID.
    pub fn new(mid: Mid) -> Self {
        Self { mid, photo: None }
    }

    /// Sets whether the response should include the user's space header image.
    pub fn with_photo(mut self, photo: UserCardPhoto) -> Self {
        self.photo = Some(photo);
        self
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![("mid", self.mid.to_string())];

        if let Some(photo) = self.photo {
            let value = match photo {
                UserCardPhoto::Include => "true",
                UserCardPhoto::Exclude => "false",
            };
            pairs.push(("photo", value.to_string()));
        }

        pairs
    }
}

/// Parameters for `/account/v1/user/cards`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserCardsParams {
    mids: Vec<Mid>,
}

impl UserCardsParams {
    /// Creates batch-card parameters for one or more validated user IDs.
    pub fn new<I>(mids: I) -> BpiResult<Self>
    where
        I: IntoIterator<Item = Mid>,
    {
        let mids = mids.into_iter().collect::<Vec<_>>();

        if mids.is_empty() {
            return Err(BpiError::invalid_parameter(
                "uids",
                "at least one user id is required",
            ));
        }

        Ok(Self { mids })
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![(
            "uids",
            self.mids
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","),
        )]
    }
}

/// Parameters for `/x/im/user_infos`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserInfosParams {
    mids: Vec<Mid>,
}

impl UserInfosParams {
    /// Creates batch-info parameters for one or more validated user IDs.
    pub fn new<I>(mids: I) -> BpiResult<Self>
    where
        I: IntoIterator<Item = Mid>,
    {
        let mids = mids.into_iter().collect::<Vec<_>>();

        if mids.is_empty() {
            return Err(BpiError::invalid_parameter(
                "uids",
                "at least one user id is required",
            ));
        }

        Ok(Self { mids })
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![(
            "uids",
            self.mids
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","),
        )]
    }
}

/// Parameters for `/x/space/wbi/acc/info`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserSpaceParams {
    mid: Mid,
}

impl UserSpaceParams {
    /// Creates space-info parameters for a validated user ID.
    pub fn new(mid: Mid) -> Self {
        Self { mid }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("mid", self.mid.to_string())]
    }
}

/// Parameters for `/x/space/notice`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserSpaceNoticeParams {
    mid: Mid,
}

impl UserSpaceNoticeParams {
    /// Creates space-notice parameters for a validated user ID.
    pub fn new(mid: Mid) -> Self {
        Self { mid }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("mid", self.mid.to_string())]
    }
}

/// Follow-list category for `/x/space/bangumi/follow/list`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserBangumiFollowKind {
    /// Followed bangumi/anime seasons.
    Bangumi,
    /// Followed cinema/drama seasons.
    Cinema,
}

impl UserBangumiFollowKind {
    fn as_query_value(self) -> &'static str {
        match self {
            Self::Bangumi => "1",
            Self::Cinema => "2",
        }
    }
}

/// Parameters for `/x/space/bangumi/follow/list`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserBangumiFollowListParams {
    mid: Mid,
    kind: UserBangumiFollowKind,
    page: u32,
    page_size: u32,
}

impl UserBangumiFollowListParams {
    /// Creates bangumi follow-list parameters for a validated user ID.
    pub fn new(mid: Mid) -> Self {
        Self {
            mid,
            kind: UserBangumiFollowKind::Bangumi,
            page: 1,
            page_size: 15,
        }
    }

    /// Sets whether to fetch followed bangumi or cinema seasons.
    pub fn with_kind(mut self, kind: UserBangumiFollowKind) -> Self {
        self.kind = kind;
        self
    }

    /// Sets the 1-based page number.
    pub fn with_page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    /// Sets the page size. Bilibili accepts values from 1 to 30.
    pub fn with_page_size(mut self, page_size: u32) -> BpiResult<Self> {
        if !(1..=30).contains(&page_size) {
            return Err(BpiError::invalid_parameter(
                "page_size",
                "page size must be between 1 and 30",
            ));
        }

        self.page_size = page_size;
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("vmid", self.mid.to_string()),
            ("type", self.kind.as_query_value().to_string()),
            ("pn", self.page.to_string()),
            ("ps", self.page_size.to_string()),
        ]
    }
}

/// Parameters for `/x/relation/stat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserRelationStatParams {
    mid: Mid,
}

impl UserRelationStatParams {
    /// Creates relation-stat parameters for a validated user ID.
    pub fn new(mid: Mid) -> Self {
        Self { mid }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("vmid", self.mid.to_string())]
    }
}

/// Parameters for `/x/relation/followings`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserFollowingsParams {
    mid: Mid,
    order_type: Option<String>,
    page_size: Option<u32>,
    page: Option<u32>,
}

impl UserFollowingsParams {
    /// Creates following-list parameters for a validated user ID.
    pub fn new(mid: Mid) -> Self {
        Self {
            mid,
            order_type: None,
            page_size: None,
            page: None,
        }
    }

    /// Sets the raw Bilibili order type, such as `attention`.
    pub fn with_order_type(mut self, order_type: impl Into<String>) -> Self {
        let order_type = order_type.into();
        if !order_type.trim().is_empty() {
            self.order_type = Some(order_type);
        }
        self
    }

    /// Sets the page size.
    pub fn with_page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Sets the 1-based page number.
    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![("vmid", self.mid.to_string())];

        if let Some(order_type) = &self.order_type {
            pairs.push(("order_type", order_type.to_string()));
        }
        if let Some(page_size) = self.page_size {
            pairs.push(("ps", page_size.to_string()));
        }
        if let Some(page) = self.page {
            pairs.push(("pn", page.to_string()));
        }

        pairs
    }
}

/// Parameters for `/x/relation/fans`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserFollowersParams {
    mid: Mid,
    page_size: Option<u32>,
    page: Option<u32>,
    offset: Option<String>,
    last_access_ts: Option<u64>,
    from: Option<String>,
}

impl UserFollowersParams {
    /// Creates follower-list parameters for a validated user ID.
    pub fn new(mid: Mid) -> Self {
        Self {
            mid,
            page_size: None,
            page: None,
            offset: None,
            last_access_ts: None,
            from: None,
        }
    }

    /// Sets the page size.
    pub fn with_page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Sets the 1-based page number.
    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the pagination offset returned by Bilibili.
    pub fn with_offset(mut self, offset: impl Into<String>) -> Self {
        let offset = offset.into();
        if !offset.trim().is_empty() {
            self.offset = Some(offset);
        }
        self
    }

    /// Sets the last-access timestamp in seconds.
    pub fn with_last_access_ts(mut self, last_access_ts: u64) -> Self {
        self.last_access_ts = Some(last_access_ts);
        self
    }

    /// Sets the raw Bilibili source marker, such as `main`.
    pub fn with_from(mut self, from: impl Into<String>) -> Self {
        let from = from.into();
        if !from.trim().is_empty() {
            self.from = Some(from);
        }
        self
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![("vmid", self.mid.to_string())];

        if let Some(page_size) = self.page_size {
            pairs.push(("ps", page_size.to_string()));
        }
        if let Some(page) = self.page {
            pairs.push(("pn", page.to_string()));
        }
        if let Some(offset) = &self.offset {
            pairs.push(("offset", offset.to_string()));
        }
        if let Some(last_access_ts) = self.last_access_ts {
            pairs.push(("last_access_ts", last_access_ts.to_string()));
        }
        if let Some(from) = &self.from {
            pairs.push(("from", from.to_string()));
        }

        pairs
    }
}

/// Parameters for `/xlive/web-ucenter/user/MedalWall`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserMedalWallParams {
    target_id: Mid,
}

impl UserMedalWallParams {
    /// Creates medal-wall parameters for a validated user ID.
    pub fn new(target_id: Mid) -> Self {
        Self { target_id }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("target_id", self.target_id.to_string())]
    }
}

/// Parameters for `/x/space/upstat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserUpStatParams {
    mid: Mid,
}

impl UserUpStatParams {
    /// Creates up-stat parameters for a validated user ID.
    pub fn new(mid: Mid) -> Self {
        Self { mid }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("mid", self.mid.to_string())]
    }
}

/// Parameters for `/x/space/navnum`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserNavStatParams {
    mid: Mid,
}

impl UserNavStatParams {
    /// Creates nav-stat parameters for a validated user ID.
    pub fn new(mid: Mid) -> Self {
        Self { mid }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("mid", self.mid.to_string())]
    }
}

/// Parameters for `/link_draw/v1/doc/upload_count`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserAlbumCountParams {
    mid: Mid,
}

impl UserAlbumCountParams {
    /// Creates album-count parameters for a validated user ID.
    pub fn new(mid: Mid) -> Self {
        Self { mid }
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("uid", self.mid.to_string())]
    }
}

/// Parameters for `/x/polymer/web-dynamic/v1/name-to-uid`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserNameToUidParams {
    names: Vec<String>,
}

impl UserNameToUidParams {
    /// Creates name-to-UID parameters from one or more non-blank display names.
    pub fn new<I, S>(names: I) -> BpiResult<Self>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let names = names
            .into_iter()
            .map(Into::into)
            .map(|name| name.trim().to_string())
            .collect::<Vec<_>>();

        if names.is_empty() {
            return Err(BpiError::invalid_parameter(
                "names",
                "at least one name is required",
            ));
        }

        if names.iter().any(|name| name.is_empty()) {
            return Err(BpiError::invalid_parameter(
                "names",
                "names cannot contain blank values",
            ));
        }

        Ok(Self { names })
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("names", self.names.join(","))]
    }
}

/// Sort order for uploaded videos in a user's space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserUploadedVideoOrder {
    /// Sort by publish time.
    Pubdate,
    /// Sort by view count.
    Click,
    /// Sort by favorite count.
    Stow,
}

impl UserUploadedVideoOrder {
    fn as_query_value(self) -> &'static str {
        match self {
            Self::Pubdate => "pubdate",
            Self::Click => "click",
            Self::Stow => "stow",
        }
    }
}

impl TryFrom<&str> for UserUploadedVideoOrder {
    type Error = BpiError;

    fn try_from(value: &str) -> BpiResult<Self> {
        match value.trim() {
            "pubdate" => Ok(Self::Pubdate),
            "click" => Ok(Self::Click),
            "stow" => Ok(Self::Stow),
            _ => Err(BpiError::invalid_parameter(
                "order",
                "uploaded video order must be pubdate, click, or stow",
            )),
        }
    }
}

/// Parameters for `/x/space/wbi/arc/search`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserUploadedVideosParams {
    mid: Mid,
    order: UserUploadedVideoOrder,
    tid: u64,
    keyword: Option<String>,
    page: u32,
    page_size: u32,
}

impl UserUploadedVideosParams {
    /// Creates uploaded-video parameters for a validated user ID.
    pub fn new(mid: Mid) -> Self {
        Self {
            mid,
            order: UserUploadedVideoOrder::Pubdate,
            tid: 0,
            keyword: None,
            page: 1,
            page_size: 30,
        }
    }

    /// Sets the video sort order.
    pub fn with_order(mut self, order: UserUploadedVideoOrder) -> Self {
        self.order = order;
        self
    }

    /// Sets the partition filter. `0` means all partitions.
    pub fn with_tid(mut self, tid: u64) -> Self {
        self.tid = tid;
        self
    }

    /// Sets a keyword filter.
    pub fn with_keyword(mut self, keyword: impl Into<String>) -> Self {
        let keyword = keyword.into().trim().to_string();
        if !keyword.is_empty() {
            self.keyword = Some(keyword);
        }
        self
    }

    /// Sets the 1-based page number.
    pub fn with_page(mut self, page: u32) -> BpiResult<Self> {
        if page == 0 {
            return Err(BpiError::invalid_parameter(
                "page",
                "page number must be at least 1",
            ));
        }

        self.page = page;
        Ok(self)
    }

    /// Sets the page size.
    pub fn with_page_size(mut self, page_size: u32) -> BpiResult<Self> {
        if page_size == 0 {
            return Err(BpiError::invalid_parameter(
                "page_size",
                "page size must be at least 1",
            ));
        }

        self.page_size = page_size;
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![
            ("mid", self.mid.to_string()),
            ("order", self.order.as_query_value().to_string()),
            ("tid", self.tid.to_string()),
            ("pn", self.page.to_string()),
            ("ps", self.page_size.to_string()),
        ];

        if let Some(keyword) = &self.keyword {
            pairs.push(("keyword", keyword.to_string()));
        }

        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_card_params_serializes_mid_query() -> Result<(), BpiError> {
        let params = UserCardParams::new(Mid::new(1001)?);

        assert_eq!(params.query_pairs(), vec![("mid", "1001".to_string())]);
        Ok(())
    }

    #[test]
    fn user_card_params_serializes_include_photo_query() -> Result<(), BpiError> {
        let params = UserCardParams::new(Mid::new(1001)?).with_photo(UserCardPhoto::Include);

        assert_eq!(
            params.query_pairs(),
            vec![("mid", "1001".to_string()), ("photo", "true".to_string())]
        );
        Ok(())
    }

    #[test]
    fn user_card_params_serializes_exclude_photo_query() -> Result<(), BpiError> {
        let params = UserCardParams::new(Mid::new(1001)?).with_photo(UserCardPhoto::Exclude);

        assert_eq!(
            params.query_pairs(),
            vec![("mid", "1001".to_string()), ("photo", "false".to_string())]
        );
        Ok(())
    }

    #[test]
    fn user_cards_params_serializes_uids_query() -> Result<(), BpiError> {
        let params = UserCardsParams::new([Mid::new(1001)?, Mid::new(1002)?])?;

        assert_eq!(
            params.query_pairs(),
            vec![("uids", "1001,1002".to_string())]
        );
        Ok(())
    }

    #[test]
    fn user_cards_params_rejects_empty_uid_list() {
        let err = UserCardsParams::new([]).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "uids", .. }
        ));
    }

    #[test]
    fn user_infos_params_serializes_uids_query() -> Result<(), BpiError> {
        let params = UserInfosParams::new([Mid::new(1001)?, Mid::new(1002)?])?;

        assert_eq!(
            params.query_pairs(),
            vec![("uids", "1001,1002".to_string())]
        );
        Ok(())
    }

    #[test]
    fn user_infos_params_rejects_empty_uid_list() {
        let err = UserInfosParams::new([]).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "uids", .. }
        ));
    }

    #[test]
    fn user_space_params_serializes_mid_query() -> Result<(), BpiError> {
        let params = UserSpaceParams::new(Mid::new(1001)?);

        assert_eq!(params.query_pairs(), vec![("mid", "1001".to_string())]);
        Ok(())
    }

    #[test]
    fn user_space_notice_params_serializes_mid_query() -> Result<(), BpiError> {
        let params = UserSpaceNoticeParams::new(Mid::new(1001)?);

        assert_eq!(params.query_pairs(), vec![("mid", "1001".to_string())]);
        Ok(())
    }

    #[test]
    fn user_bangumi_follow_list_params_serializes_default_query() -> Result<(), BpiError> {
        let params = UserBangumiFollowListParams::new(Mid::new(1001)?);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("vmid", "1001".to_string()),
                ("type", "1".to_string()),
                ("pn", "1".to_string()),
                ("ps", "15".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn user_bangumi_follow_list_params_serializes_optional_filters() -> Result<(), BpiError> {
        let params = UserBangumiFollowListParams::new(Mid::new(1001)?)
            .with_kind(UserBangumiFollowKind::Cinema)
            .with_page(2)
            .with_page_size(30)?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("vmid", "1001".to_string()),
                ("type", "2".to_string()),
                ("pn", "2".to_string()),
                ("ps", "30".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn user_bangumi_follow_list_params_rejects_zero_page_size() -> Result<(), BpiError> {
        let err = UserBangumiFollowListParams::new(Mid::new(1001)?)
            .with_page_size(0)
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "page_size",
                ..
            }
        ));
        Ok(())
    }

    #[test]
    fn user_bangumi_follow_list_params_rejects_large_page_size() -> Result<(), BpiError> {
        let err = UserBangumiFollowListParams::new(Mid::new(1001)?)
            .with_page_size(31)
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "page_size",
                ..
            }
        ));
        Ok(())
    }

    #[test]
    fn user_uploaded_videos_params_serializes_default_query() -> Result<(), BpiError> {
        let params = UserUploadedVideosParams::new(Mid::new(1001)?);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("mid", "1001".to_string()),
                ("order", "pubdate".to_string()),
                ("tid", "0".to_string()),
                ("pn", "1".to_string()),
                ("ps", "30".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn user_uploaded_video_order_parses_supported_values() -> Result<(), BpiError> {
        assert_eq!(
            UserUploadedVideoOrder::try_from("pubdate")?,
            UserUploadedVideoOrder::Pubdate
        );
        assert_eq!(
            UserUploadedVideoOrder::try_from("click")?,
            UserUploadedVideoOrder::Click
        );
        assert_eq!(
            UserUploadedVideoOrder::try_from("stow")?,
            UserUploadedVideoOrder::Stow
        );
        Ok(())
    }

    #[test]
    fn user_uploaded_video_order_rejects_unknown_value() {
        let err = UserUploadedVideoOrder::try_from("invalid").unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "order", .. }
        ));
    }

    #[test]
    fn user_uploaded_videos_params_serializes_optional_filters() -> Result<(), BpiError> {
        let params = UserUploadedVideosParams::new(Mid::new(1001)?)
            .with_order(UserUploadedVideoOrder::Click)
            .with_tid(33)
            .with_keyword("rust")
            .with_page(2)?
            .with_page_size(20)?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("mid", "1001".to_string()),
                ("order", "click".to_string()),
                ("tid", "33".to_string()),
                ("pn", "2".to_string()),
                ("ps", "20".to_string()),
                ("keyword", "rust".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn user_uploaded_videos_params_trims_keyword() -> Result<(), BpiError> {
        let params = UserUploadedVideosParams::new(Mid::new(1001)?).with_keyword("  rust  ");

        assert_eq!(
            params.query_pairs(),
            vec![
                ("mid", "1001".to_string()),
                ("order", "pubdate".to_string()),
                ("tid", "0".to_string()),
                ("pn", "1".to_string()),
                ("ps", "30".to_string()),
                ("keyword", "rust".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn user_uploaded_videos_params_ignores_blank_keyword() -> Result<(), BpiError> {
        let params = UserUploadedVideosParams::new(Mid::new(1001)?).with_keyword("   ");

        assert_eq!(
            params.query_pairs(),
            vec![
                ("mid", "1001".to_string()),
                ("order", "pubdate".to_string()),
                ("tid", "0".to_string()),
                ("pn", "1".to_string()),
                ("ps", "30".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn user_uploaded_videos_params_rejects_zero_page() -> Result<(), BpiError> {
        let err = UserUploadedVideosParams::new(Mid::new(1001)?)
            .with_page(0)
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "page", .. }
        ));
        Ok(())
    }

    #[test]
    fn user_uploaded_videos_params_rejects_zero_page_size() -> Result<(), BpiError> {
        let err = UserUploadedVideosParams::new(Mid::new(1001)?)
            .with_page_size(0)
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "page_size",
                ..
            }
        ));
        Ok(())
    }

    #[test]
    fn user_relation_stat_params_serializes_mid_query() -> Result<(), BpiError> {
        let params = UserRelationStatParams::new(Mid::new(1001)?);

        assert_eq!(params.query_pairs(), vec![("vmid", "1001".to_string())]);
        Ok(())
    }

    #[test]
    fn user_followings_params_serializes_default_query() -> Result<(), BpiError> {
        let params = UserFollowingsParams::new(Mid::new(1001)?);

        assert_eq!(params.query_pairs(), vec![("vmid", "1001".to_string())]);
        Ok(())
    }

    #[test]
    fn user_followings_params_serializes_optional_filters() -> Result<(), BpiError> {
        let params = UserFollowingsParams::new(Mid::new(1001)?)
            .with_order_type("attention")
            .with_page_size(20)
            .with_page(2);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("vmid", "1001".to_string()),
                ("order_type", "attention".to_string()),
                ("ps", "20".to_string()),
                ("pn", "2".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn user_followers_params_serializes_default_query() -> Result<(), BpiError> {
        let params = UserFollowersParams::new(Mid::new(1001)?);

        assert_eq!(params.query_pairs(), vec![("vmid", "1001".to_string())]);
        Ok(())
    }

    #[test]
    fn user_followers_params_serializes_optional_filters() -> Result<(), BpiError> {
        let params = UserFollowersParams::new(Mid::new(1001)?)
            .with_page_size(20)
            .with_page(2)
            .with_offset("next-offset")
            .with_last_access_ts(1_700_000_000)
            .with_from("main");

        assert_eq!(
            params.query_pairs(),
            vec![
                ("vmid", "1001".to_string()),
                ("ps", "20".to_string()),
                ("pn", "2".to_string()),
                ("offset", "next-offset".to_string()),
                ("last_access_ts", "1700000000".to_string()),
                ("from", "main".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn user_medal_wall_params_serializes_target_id_query() -> Result<(), BpiError> {
        let params = UserMedalWallParams::new(Mid::new(1001)?);

        assert_eq!(
            params.query_pairs(),
            vec![("target_id", "1001".to_string())]
        );
        Ok(())
    }

    #[test]
    fn user_up_stat_params_serializes_mid_query() -> Result<(), BpiError> {
        let params = UserUpStatParams::new(Mid::new(1001)?);

        assert_eq!(params.query_pairs(), vec![("mid", "1001".to_string())]);
        Ok(())
    }

    #[test]
    fn user_nav_stat_params_serializes_mid_query() -> Result<(), BpiError> {
        let params = UserNavStatParams::new(Mid::new(1001)?);

        assert_eq!(params.query_pairs(), vec![("mid", "1001".to_string())]);
        Ok(())
    }

    #[test]
    fn user_album_count_params_serializes_uid_query() -> Result<(), BpiError> {
        let params = UserAlbumCountParams::new(Mid::new(1001)?);

        assert_eq!(params.query_pairs(), vec![("uid", "1001".to_string())]);
        Ok(())
    }

    #[test]
    fn user_name_to_uid_params_serializes_joined_names_query() -> Result<(), BpiError> {
        let params = UserNameToUidParams::new(["fixture_user", "another_user"])?;

        assert_eq!(
            params.query_pairs(),
            vec![("names", "fixture_user,another_user".to_string())]
        );
        Ok(())
    }

    #[test]
    fn user_name_to_uid_params_rejects_empty_names() {
        let err = UserNameToUidParams::new(Vec::<&str>::new()).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "names", .. }
        ));
    }

    #[test]
    fn user_name_to_uid_params_rejects_blank_name() {
        let err = UserNameToUidParams::new(["fixture_user", "  "]).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "names", .. }
        ));
    }
}
