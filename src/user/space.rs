//! B站用户空间相关接口
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// --- 响应数据结构体 ---

/// 用户空间公告响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpaceNoticeResponseData(pub String);

/// 修改空间公告响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SetSpaceNoticeResponseData;

/// 追番/追剧列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BangumiFollowItem {
    pub season_id: i64,
    pub media_id: i64,
    pub season_type: i64,
    pub season_type_name: String,
    pub title: String,
    pub cover: String,
    pub total_count: i64,
    pub is_finish: i64,
    pub is_started: i64,
    pub is_play: i64,
    pub badge: String,
    pub badge_type: i64,
    pub rights: Rights,
    pub stat: Stat,
    pub new_ep: NewEp,
    pub rating: Option<Rating>,
    pub square_cover: String,
    pub season_status: i64,
    pub season_title: String,
    pub badge_ep: String,
    pub media_attr: i64,
    pub season_attr: i64,
    pub evaluate: String,
    pub areas: Vec<Area>,
    pub subtitle: String,
    pub first_ep: i64,
    pub can_watch: i64,
    pub series: Series,
    pub publish: Publish,
    pub mode: i64,
    pub section: Vec<Section>,
    pub url: String,
    pub badge_info: BadgeInfo,
    pub renewal_time: Option<String>,
    pub first_ep_info: FirstEpInfo,
    pub formal_ep_count: i64,
    pub short_url: String,
    pub badge_infos: BadgeInfos,
    pub season_version: String,
    pub subtitle_14: String,
    pub viewable_crowd_type: i64,
    pub summary: String,
    pub styles: Vec<String>,
    pub follow_status: i64,
    pub is_new: i64,
    pub progress: String,
    pub both_follow: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rights {
    pub allow_review: i64,
    pub is_selection: i64,
    pub selection_style: i64,
    pub is_rcmd: i64,

    pub demand_end_time: serde_json::Value,
    pub allow_preview: Option<i64>,
    pub allow_bp_rank: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stat {
    pub follow: i64,
    pub view: i64,
    pub danmaku: i64,
    pub reply: i64,
    pub coin: i64,
    pub series_follow: i64,
    pub series_view: i64,
    pub likes: i64,
    pub favorite: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewEp {
    pub id: i64,
    pub index_show: String,
    pub cover: String,
    pub title: String,
    pub long_title: Option<String>,
    pub pub_time: String,
    pub duration: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rating {
    pub score: f64,
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Area {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Series {
    pub series_id: i64,
    pub title: String,
    pub season_count: i64,
    pub new_season_id: i64,
    pub series_ord: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Publish {
    pub pub_time: String,
    pub pub_time_show: String,
    pub release_date: String,
    pub release_date_show: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
    pub section_id: i64,
    pub season_id: i64,
    pub limit_group: i64,
    pub watch_platform: i64,
    pub copyright: String,
    pub ban_area_show: i64,
    pub episode_ids: Vec<i64>,
    #[serde(rename = "type")]
    pub type_field: Option<i64>,
    pub title: Option<String>,
    pub attr: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BadgeInfo {
    pub text: String,
    pub bg_color: String,
    pub bg_color_night: String,
    pub img: String,
    pub multi_img: MultiImg,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiImg {
    pub color: String,
    pub medium_remind: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FirstEpInfo {
    pub id: i64,
    pub cover: String,
    pub title: String,
    pub long_title: Option<String>,
    pub pub_time: String,
    pub duration: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BadgeInfos {
    pub vip_or_pay: VipOrPay,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VipOrPay {
    pub text: String,
    pub bg_color: String,
    pub bg_color_night: String,
    pub img: String,
    pub multi_img: MultiImg2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiImg2 {
    pub color: String,
    pub medium_remind: String,
}

/// 用户追番/追剧明细响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BangumiFollowListResponseData {
    /// 追番列表
    pub list: Vec<BangumiFollowItem>,
    /// 当前页码
    pub pn: u32,
    /// 每页项数
    pub ps: u32,
    /// 总计追番数
    pub total: u64,
}

// --- API 实现 ---

impl BpiClient {
    /// 修改空间公告
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
    ///
    /// # 参数
    /// | 名称    | 类型           | 说明                 |
    /// | ------- | --------------| -------------------- |
    /// | `notice`| `Option<&str>`  | 公告内容，少于150字  |
    pub async fn user_space_notice_set(
        &self,
        notice: Option<&str>,
    ) -> Result<BpiResponse<()>, BpiError> {
        let csrf = self.csrf()?;
        let mut form = reqwest::multipart::Form::new().text("csrf", csrf.to_string());

        if let Some(n) = notice {
            if n.len() > 150 {
                return Err(BpiError::parse("公告内容超出150字符限制"));
            }
            form = form.text("notice", n.to_string());
        }

        self.post("https://api.bilibili.com/x/space/notice/set")
            .multipart(form)
            .send_bpi("修改空间公告")
            .await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::Mid;
    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;
    use crate::user::params::{UserBangumiFollowListParams, UserSpaceNoticeParams};
    use crate::{ApiEnvelope, BpiResult};
    use tracing::info;

    // 请在运行测试前设置环境变量 `BPI_COOKIE`，以包含 SESSDATA 等登录信息
    // mid 根据实际情况修改

    const TEST_MID: u64 = 4279370;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_user_space_notice() -> Result<(), BpiError> {
        if std::env::var_os("BPI_LIVE_TEST").is_none() {
            return Ok(());
        }

        let bpi = BpiClient::new().expect("client should build");
        let data = bpi
            .user()
            .space_notice(UserSpaceNoticeParams::new(Mid::new(TEST_MID)?))
            .await?;

        info!("空间公告: {:?}", data);

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_user_space_notice_set() -> Result<(), BpiError> {
        if std::env::var_os("BPI_LIVE_TEST").is_none()
            || std::env::var_os("BPI_MUTATING_TEST").is_none()
            || std::env::var_os("BPI_COOKIE").is_none()
        {
            return Ok(());
        }

        let bpi = BpiClient::new().expect("client should build");
        let notice = "这是一个通过 API 设置的测试公告。";
        let resp = bpi.user_space_notice_set(Some(notice)).await?;

        info!("设置空间公告结果: {:?}", resp);

        // 验证设置后内容
        let get_data = bpi
            .user()
            .space_notice(UserSpaceNoticeParams::new(Mid::new(TEST_MID)?))
            .await?;
        assert_eq!(get_data.content, notice);
        info!("验证公告内容成功");

        // 删除公告
        let delete_resp = bpi.user_space_notice_set(None).await?;
        info!("删除空间公告结果: {:?}", delete_resp);
        assert_eq!(delete_resp.code, 0);

        // 验证删除后内容
        let get_data_after_delete = bpi
            .user()
            .space_notice(UserSpaceNoticeParams::new(Mid::new(TEST_MID)?))
            .await?;
        assert!(get_data_after_delete.content.is_empty());
        info!("验证删除公告内容成功");

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_user_bangumi_follow_list() -> Result<(), BpiError> {
        if std::env::var_os("BPI_LIVE_TEST").is_none() {
            return Ok(());
        }

        let bpi = BpiClient::new().expect("client should build");
        let data = bpi
            .user()
            .bangumi_follow_list(
                UserBangumiFollowListParams::new(Mid::new(TEST_MID)?)
                    .with_page(1)
                    .with_page_size(15)?,
            )
            .await?;

        info!("追番列表: {:?}", data);
        assert_eq!(data.page, 1);
        assert_eq!(data.page_size, 15);
        assert!(!data.items.is_empty());

        Ok(())
    }

    fn public_read_contract(endpoint: &str) -> BpiResult<EndpointContract> {
        let bytes: &[u8] = match endpoint {
            "bangumi-follow-list" => include_bytes!(
                "../../tests/contracts/user/public-read/bangumi-follow-list/contract.json"
            ),
            "space-notice" => {
                include_bytes!("../../tests/contracts/user/public-read/space-notice/contract.json")
            }
            _ => {
                return Err(BpiError::invalid_parameter(
                    "endpoint",
                    "unknown user space contract",
                ));
            }
        };

        EndpointContract::from_slice(bytes)
    }

    #[test]
    fn legacy_user_space_contracts_match_endpoint_requests() -> BpiResult<()> {
        let notice = public_read_contract("space-notice")?;
        assert_eq!(notice.name, "user.space_notice");
        assert_eq!(notice.request.method, HttpMethod::Get);
        assert_eq!(
            notice.request.url.as_str(),
            "https://api.bilibili.com/x/space/notice"
        );
        assert_eq!(
            notice.request.query.get("mid").map(String::as_str),
            Some("2")
        );

        let bangumi = public_read_contract("bangumi-follow-list")?;
        assert_eq!(bangumi.name, "user.bangumi_follow_list");
        assert_eq!(
            bangumi.request.url.as_str(),
            "https://api.bilibili.com/x/space/bangumi/follow/list"
        );
        assert_eq!(
            bangumi.request.query.get("vmid").map(String::as_str),
            Some("4279370")
        );
        assert_eq!(
            bangumi.request.query.get("type").map(String::as_str),
            Some("1")
        );
        Ok(())
    }

    #[test]
    fn legacy_user_space_fixtures_parse_promoted_contract_models() -> BpiResult<()> {
        let notice = ApiEnvelope::<SpaceNoticeResponseData>::from_slice(include_bytes!(
            "../../tests/contracts/user/public-read/space-notice/responses/success.json"
        ))?
        .into_payload()?;
        let _notice_text = notice.0;

        let follow_list =
            ApiEnvelope::<BangumiFollowListResponseData>::from_slice(include_bytes!(
                "../../tests/contracts/user/public-read/bangumi-follow-list/responses/success.json"
            ))?
            .into_payload()?;
        assert_eq!(follow_list.pn, 1);
        Ok(())
    }
}
