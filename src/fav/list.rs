use crate::ids::MediaId;
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse, BpiResult};
use serde::{Deserialize, Serialize};

// --- 获取收藏夹内容明细列表 ---

/// 收藏夹内容明细列表中的 UP 主信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FavListUpper {
    pub mid: u64,
    pub name: String,
    pub face: String,
    pub followed: Option<bool>,
    pub vip_type: Option<u8>,
    pub vip_status: Option<u8>,
}

/// 收藏夹内容明细列表中的状态数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FavListCntInfo {
    /// 收藏
    pub collect: u64,
    /// 播放
    pub play: u64,

    /// 分享 (仅info)
    pub share: Option<u64>,
    /// 点赞 (仅info)
    pub thumb_up: Option<u64>,
    ///弹幕 (仅media)
    pub danmaku: Option<u64>,
    /// 播放文本 (仅media)
    pub view_text_1: Option<String>,
}

/// 收藏夹元数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FavListInfo {
    pub id: u64,
    pub fid: u64,
    pub mid: u64,
    pub attr: u32,
    pub title: String,
    pub cover: String,
    pub upper: FavListUpper,
    pub cover_type: u8,
    pub cnt_info: FavListCntInfo,
    #[serde(rename = "type")]
    pub type_name: u32,
    pub intro: String,
    pub ctime: u64,
    pub mtime: u64,
    pub state: u8,
    pub fav_state: u8,
    pub like_state: u8,
    pub media_count: u32,
}

/// 收藏夹中的单个内容
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FavListMedia {
    pub id: u64,
    #[serde(rename = "type")]
    pub type_name: u8,
    pub title: String,
    pub cover: String,
    pub intro: String,
    pub page: Option<u32>,
    pub duration: u32,
    pub upper: FavListUpper,
    pub attr: u8,
    pub cnt_info: FavListCntInfo,
    pub link: String,
    pub ctime: u64,
    pub pubtime: u64,
    pub fav_time: u64,
    pub bv_id: Option<String>,
    pub bvid: Option<String>,
    pub season: Option<serde_json::Value>,
}

/// 收藏夹内容明细列表数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FavListDetailData {
    pub info: FavListInfo,
    pub medias: Vec<FavListMedia>,
    pub has_more: bool,
    pub ttl: u64,
}

// --- 获取收藏夹全部内容id ---

/// 收藏夹全部内容ID列表中的单个ID
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FavResourceIdItem {
    pub id: u64,
    #[serde(rename = "type")]
    pub type_name: u8,
    pub bv_id: Option<String>,
    pub bvid: Option<String>,
}

/// Parameters for fetching a favorite folder's detailed resource list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FavListDetailParams {
    media_id: MediaId,
    tid: Option<u32>,
    keyword: Option<String>,
    order: Option<String>,
    typ: Option<u8>,
    ps: u32,
    pn: Option<u32>,
}

impl FavListDetailParams {
    /// Creates favorite-list detail parameters with the default page size.
    pub fn new(media_id: MediaId) -> Self {
        Self {
            media_id,
            tid: None,
            keyword: None,
            order: None,
            typ: None,
            ps: 20,
            pn: None,
        }
    }

    /// Sets the optional partition filter.
    pub fn tid(mut self, tid: u32) -> Self {
        self.tid = Some(tid);
        self
    }

    /// Sets the keyword filter.
    pub fn keyword(mut self, keyword: impl Into<String>) -> BpiResult<Self> {
        let keyword = keyword.into();
        validate_non_blank("keyword", &keyword)?;
        self.keyword = Some(keyword);
        Ok(self)
    }

    /// Sets the ordering key, such as `mtime`.
    pub fn order(mut self, order: impl Into<String>) -> BpiResult<Self> {
        let order = order.into();
        validate_non_blank("order", &order)?;
        self.order = Some(order);
        Ok(self)
    }

    /// Sets the content type filter.
    pub fn content_type(mut self, typ: u8) -> Self {
        self.typ = Some(typ);
        self
    }

    /// Sets the page size.
    pub fn page_size(mut self, ps: u32) -> BpiResult<Self> {
        if ps == 0 {
            return Err(BpiError::invalid_parameter(
                "ps",
                "page size must be non-zero",
            ));
        }
        self.ps = ps;
        Ok(self)
    }

    /// Sets the page number.
    pub fn page(mut self, pn: u32) -> BpiResult<Self> {
        if pn == 0 {
            return Err(BpiError::invalid_parameter(
                "pn",
                "page number must be non-zero",
            ));
        }
        self.pn = Some(pn);
        Ok(self)
    }

    fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut params = vec![
            ("media_id", self.media_id.to_string()),
            ("ps", self.ps.to_string()),
            ("platform", "web".to_string()),
        ];

        if let Some(tid) = self.tid {
            params.push(("tid", tid.to_string()));
        }
        if let Some(keyword) = self.keyword.as_ref() {
            params.push(("keyword", keyword.clone()));
        }
        if let Some(order) = self.order.as_ref() {
            params.push(("order", order.clone()));
        }
        if let Some(typ) = self.typ {
            params.push(("type", typ.to_string()));
        }
        if let Some(pn) = self.pn {
            params.push(("pn", pn.to_string()));
        }

        params
    }
}

fn validate_non_blank(field: &'static str, value: &str) -> BpiResult<()> {
    if value.trim().is_empty() {
        return Err(BpiError::invalid_parameter(field, "value cannot be blank"));
    }

    Ok(())
}

impl BpiClient {
    /// 获取收藏夹内容明细列表
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav)
    ///
    pub async fn fav_list_detail(
        &self,
        params: FavListDetailParams,
    ) -> Result<BpiResponse<FavListDetailData>, BpiError> {
        self.get("https://api.bilibili.com/x/v3/fav/resource/list")
            .query(&params.query_pairs())
            .send_bpi("获取收藏夹内容明细列表")
            .await
    }

    /// 获取收藏夹全部内容id
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `media_id` | u64 | 收藏夹 media_id |
    pub async fn fav_resource_ids(
        &self,
        media_id: u64,
    ) -> Result<BpiResponse<Vec<FavResourceIdItem>>, BpiError> {
        self.get("https://api.bilibili.com/x/v3/fav/resource/ids")
            .query(&[
                ("media_id", media_id.to_string()),
                ("platform", "web".to_string()),
            ])
            .send_bpi("获取收藏夹全部内容id")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::MediaId;
    use tracing::info;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_fav_list_detail() {
        let bpi = BpiClient::new().expect("client should build");
        let media_id = 1572769770;
        let params = FavListDetailParams::new(MediaId::new(media_id).expect("media id is valid"))
            .order("mtime")
            .expect("order is valid")
            .content_type(0)
            .page_size(5)
            .expect("page size is valid")
            .page(1)
            .expect("page is valid");
        let resp = bpi.fav_list_detail(params).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("has_more: {}", data.has_more);
            info!("total media count: {}", data.info.media_count);
            info!("retrieved media count: {}", data.medias.len());
            info!("first media item: {:?}", data.medias.first());
        }
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_fav_resource_ids() {
        let bpi = BpiClient::new().expect("client should build");
        let media_id = 1572769770;
        let resp = bpi.fav_resource_ids(media_id).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("total IDs retrieved: {}", data.len());
            info!("first ID item: {:?}", data.first());
        }
    }

    #[test]
    fn fav_list_detail_params_serializes_required_query() -> Result<(), BpiError> {
        let params = FavListDetailParams::new(MediaId::new(1572769770)?);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("media_id", "1572769770".to_string()),
                ("ps", "20".to_string()),
                ("platform", "web".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn fav_list_detail_params_serializes_optional_query() -> Result<(), BpiError> {
        let params = FavListDetailParams::new(MediaId::new(1572769770)?)
            .tid(3)
            .keyword("rust")?
            .order("mtime")?
            .content_type(0)
            .page_size(5)?
            .page(1)?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("media_id", "1572769770".to_string()),
                ("ps", "5".to_string()),
                ("platform", "web".to_string()),
                ("tid", "3".to_string()),
                ("keyword", "rust".to_string()),
                ("order", "mtime".to_string()),
                ("type", "0".to_string()),
                ("pn", "1".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn fav_list_detail_params_rejects_zero_page_size() {
        let err = FavListDetailParams::new(MediaId::new(1572769770).expect("media id is valid"))
            .page_size(0)
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "ps", .. }
        ));
    }
}
