use crate::request::BilibiliRequest;
use crate::{BpiClient, BpiResult};

use super::model::{
    UserAlbumCount, UserBangumiFollowList, UserBatchCard, UserBatchInfo, UserCardProfile,
    UserFollowTag, UserFollowers, UserFollowings, UserMedalWall, UserNameToUid, UserNavStat,
    UserRelationStat, UserSpaceNotice, UserSpaceProfile, UserUpStat, UserUploadedVideos,
};
use super::params::{
    UserAlbumCountParams, UserBangumiFollowListParams, UserCardParams, UserCardsParams,
    UserFollowersParams, UserFollowingsParams, UserInfosParams, UserMedalWallParams,
    UserNameToUidParams, UserNavStatParams, UserRelationStatParams, UserSpaceNoticeParams,
    UserSpaceParams, UserUpStatParams, UserUploadedVideosParams,
};

const ALBUM_COUNT_ENDPOINT: &str = "https://api.vc.bilibili.com/link_draw/v1/doc/upload_count";
const BANGUMI_FOLLOW_LIST_ENDPOINT: &str = "https://api.bilibili.com/x/space/bangumi/follow/list";
const CARD_ENDPOINT: &str = "https://api.bilibili.com/x/web-interface/card";
const CARDS_ENDPOINT: &str = "https://api.vc.bilibili.com/account/v1/user/cards";
const FOLLOWERS_ENDPOINT: &str = "https://api.bilibili.com/x/relation/fans";
const FOLLOWINGS_ENDPOINT: &str = "https://api.bilibili.com/x/relation/followings";
const FOLLOW_TAGS_ENDPOINT: &str = "https://api.bilibili.com/x/relation/tags";
const INFOS_ENDPOINT: &str = "https://api.vc.bilibili.com/x/im/user_infos";
const MEDAL_WALL_ENDPOINT: &str = "https://api.live.bilibili.com/xlive/web-ucenter/user/MedalWall";
const NAME_TO_UID_ENDPOINT: &str = "https://api.bilibili.com/x/polymer/web-dynamic/v1/name-to-uid";
const NAV_STAT_ENDPOINT: &str = "https://api.bilibili.com/x/space/navnum";
const RELATION_STAT_ENDPOINT: &str = "https://api.bilibili.com/x/relation/stat";
const SPACE_INFO_ENDPOINT: &str = "https://api.bilibili.com/x/space/wbi/acc/info";
const SPACE_NOTICE_ENDPOINT: &str = "https://api.bilibili.com/x/space/notice";
const UP_STAT_ENDPOINT: &str = "https://api.bilibili.com/x/space/upstat";
const UPLOADED_VIDEOS_ENDPOINT: &str = "https://api.bilibili.com/x/space/wbi/arc/search";

/// User domain API client.
#[derive(Clone, Copy)]
pub struct UserClient<'a> {
    client: &'a BpiClient,
}

impl<'a> UserClient<'a> {
    pub(crate) fn new(client: &'a BpiClient) -> Self {
        Self { client }
    }

    #[cfg(test)]
    pub(crate) fn card_endpoint(&self) -> &'static str {
        CARD_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn cards_endpoint(&self) -> &'static str {
        CARDS_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn infos_endpoint(&self) -> &'static str {
        INFOS_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn bangumi_follow_list_endpoint(&self) -> &'static str {
        BANGUMI_FOLLOW_LIST_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn followings_endpoint(&self) -> &'static str {
        FOLLOWINGS_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn followers_endpoint(&self) -> &'static str {
        FOLLOWERS_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn follow_tags_endpoint(&self) -> &'static str {
        FOLLOW_TAGS_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn medal_wall_endpoint(&self) -> &'static str {
        MEDAL_WALL_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn album_count_endpoint(&self) -> &'static str {
        ALBUM_COUNT_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn name_to_uid_endpoint(&self) -> &'static str {
        NAME_TO_UID_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn relation_stat_endpoint(&self) -> &'static str {
        RELATION_STAT_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn nav_stat_endpoint(&self) -> &'static str {
        NAV_STAT_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn space_info_endpoint(&self) -> &'static str {
        SPACE_INFO_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn space_notice_endpoint(&self) -> &'static str {
        SPACE_NOTICE_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn uploaded_videos_endpoint(&self) -> &'static str {
        UPLOADED_VIDEOS_ENDPOINT
    }

    #[cfg(test)]
    pub(crate) fn up_stat_endpoint(&self) -> &'static str {
        UP_STAT_ENDPOINT
    }

    /// Fetches public user card information.
    pub async fn card(&self, params: UserCardParams) -> BpiResult<UserCardProfile> {
        self.client
            .get(CARD_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi::<UserCardProfile>("user.card")
            .await?
            .into_data()
    }

    /// Fetches compact public card information for one or more users.
    pub async fn cards(&self, params: UserCardsParams) -> BpiResult<Vec<UserBatchCard>> {
        self.client
            .get(CARDS_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi::<Vec<UserBatchCard>>("user.cards")
            .await?
            .into_data()
    }

    /// Fetches detailed public batch information for one or more users.
    pub async fn infos(&self, params: UserInfosParams) -> BpiResult<Vec<UserBatchInfo>> {
        self.client
            .get(INFOS_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi::<Vec<UserBatchInfo>>("user.infos")
            .await?
            .into_data()
    }

    /// Fetches public album submission counters for a user.
    pub async fn album_count(&self, params: UserAlbumCountParams) -> BpiResult<UserAlbumCount> {
        self.client
            .get(ALBUM_COUNT_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi::<UserAlbumCount>("user.album_count")
            .await?
            .into_data()
    }

    /// Fetches followed bangumi or cinema seasons for a public user.
    pub async fn bangumi_follow_list(
        &self,
        params: UserBangumiFollowListParams,
    ) -> BpiResult<UserBangumiFollowList> {
        self.client
            .get(BANGUMI_FOLLOW_LIST_ENDPOINT)
            .with_bilibili_headers()
            .query(&params.query_pairs())
            .send_bpi::<UserBangumiFollowList>("user.bangumi_follow_list")
            .await?
            .into_data()
    }

    /// Fetches users followed by a public member.
    pub async fn followings(&self, params: UserFollowingsParams) -> BpiResult<UserFollowings> {
        self.client
            .get(FOLLOWINGS_ENDPOINT)
            .with_bilibili_headers()
            .query(&params.query_pairs())
            .send_bpi::<UserFollowings>("user.followings")
            .await?
            .into_data()
    }

    /// Fetches users following a public member.
    pub async fn followers(&self, params: UserFollowersParams) -> BpiResult<UserFollowers> {
        self.client
            .get(FOLLOWERS_ENDPOINT)
            .with_bilibili_headers()
            .query(&params.query_pairs())
            .send_bpi::<UserFollowers>("user.followers")
            .await?
            .into_data()
    }

    /// Fetches follow groups for the current authenticated session.
    pub async fn follow_tags(&self) -> BpiResult<Vec<UserFollowTag>> {
        self.client
            .get(FOLLOW_TAGS_ENDPOINT)
            .with_bilibili_headers()
            .send_bpi::<Vec<UserFollowTag>>("user.follow_tags")
            .await?
            .into_data()
    }

    /// Fetches a public fan-medal wall for a user.
    pub async fn medal_wall(&self, params: UserMedalWallParams) -> BpiResult<UserMedalWall> {
        self.client
            .get(MEDAL_WALL_ENDPOINT)
            .with_bilibili_headers()
            .query(&params.query_pairs())
            .send_bpi::<UserMedalWall>("user.medal_wall")
            .await?
            .into_data()
    }

    /// Looks up member IDs by public display names.
    pub async fn name_to_uid(&self, params: UserNameToUidParams) -> BpiResult<UserNameToUid> {
        self.client
            .get(NAME_TO_UID_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi::<UserNameToUid>("user.name_to_uid")
            .await?
            .into_data()
    }

    /// Fetches public relation counts for a user.
    pub async fn relation_stat(
        &self,
        params: UserRelationStatParams,
    ) -> BpiResult<UserRelationStat> {
        self.client
            .get(RELATION_STAT_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi::<UserRelationStat>("user.relation_stat")
            .await?
            .into_data()
    }

    /// Fetches public space navigation counters for a user.
    pub async fn nav_stat(&self, params: UserNavStatParams) -> BpiResult<UserNavStat> {
        self.client
            .get(NAV_STAT_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi::<UserNavStat>("user.nav_stat")
            .await?
            .into_data()
    }

    /// Fetches public creator statistics for a user.
    pub async fn up_stat(&self, params: UserUpStatParams) -> BpiResult<UserUpStat> {
        self.client
            .get(UP_STAT_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi::<UserUpStat>("user.up_stat")
            .await?
            .into_data()
    }

    /// Fetches public user space information.
    pub async fn space_info(&self, params: UserSpaceParams) -> BpiResult<UserSpaceProfile> {
        let signed_params = self.client.sign_wbi_params(params.query_pairs()).await?;

        self.client
            .get(SPACE_INFO_ENDPOINT)
            .query(&signed_params)
            .send_bpi::<UserSpaceProfile>("user.space_info")
            .await?
            .into_data()
    }

    /// Fetches the public space notice for a user.
    pub async fn space_notice(&self, params: UserSpaceNoticeParams) -> BpiResult<UserSpaceNotice> {
        self.client
            .get(SPACE_NOTICE_ENDPOINT)
            .query(&params.query_pairs())
            .send_bpi::<UserSpaceNotice>("user.space_notice")
            .await?
            .into_data()
    }

    /// Fetches videos uploaded to a user's public space.
    pub async fn uploaded_videos(
        &self,
        params: UserUploadedVideosParams,
    ) -> BpiResult<UserUploadedVideos> {
        let signed_params = self.client.sign_wbi_params(params.query_pairs()).await?;

        self.client
            .get(UPLOADED_VIDEOS_ENDPOINT)
            .query(&signed_params)
            .send_bpi::<UserUploadedVideos>("user.uploaded_videos")
            .await?
            .into_data()
    }
}

#[cfg(test)]
mod tests {
    use crate::BpiClient;

    #[test]
    fn user_client_borrows_root_client() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.card_endpoint(),
            "https://api.bilibili.com/x/web-interface/card"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_cards_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.cards_endpoint(),
            "https://api.vc.bilibili.com/account/v1/user/cards"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_infos_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.infos_endpoint(),
            "https://api.vc.bilibili.com/x/im/user_infos"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_space_info_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.space_info_endpoint(),
            "https://api.bilibili.com/x/space/wbi/acc/info"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_space_notice_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.space_notice_endpoint(),
            "https://api.bilibili.com/x/space/notice"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_bangumi_follow_list_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.bangumi_follow_list_endpoint(),
            "https://api.bilibili.com/x/space/bangumi/follow/list"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_uploaded_videos_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.uploaded_videos_endpoint(),
            "https://api.bilibili.com/x/space/wbi/arc/search"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_relation_stat_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.relation_stat_endpoint(),
            "https://api.bilibili.com/x/relation/stat"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_followings_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.followings_endpoint(),
            "https://api.bilibili.com/x/relation/followings"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_followers_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.followers_endpoint(),
            "https://api.bilibili.com/x/relation/fans"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_follow_tags_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.follow_tags_endpoint(),
            "https://api.bilibili.com/x/relation/tags"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_medal_wall_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.medal_wall_endpoint(),
            "https://api.live.bilibili.com/xlive/web-ucenter/user/MedalWall"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_up_stat_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.up_stat_endpoint(),
            "https://api.bilibili.com/x/space/upstat"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_nav_stat_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.nav_stat_endpoint(),
            "https://api.bilibili.com/x/space/navnum"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_album_count_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.album_count_endpoint(),
            "https://api.vc.bilibili.com/link_draw/v1/doc/upload_count"
        );
        Ok(())
    }

    #[test]
    fn user_client_exposes_name_to_uid_endpoint() -> Result<(), crate::BpiError> {
        let client = BpiClient::new()?;
        let user = client.user();

        assert_eq!(
            user.name_to_uid_endpoint(),
            "https://api.bilibili.com/x/polymer/web-dynamic/v1/name-to-uid"
        );
        Ok(())
    }
}
