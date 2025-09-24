use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

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

impl BpiClient {
    /// 获取收藏夹内容明细列表
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `media_id` | u64 | 收藏夹 media_id |
    /// | `tid` | `Option<u32>` | 分区 tid |
    /// | `keyword` | `Option<&str>` | 关键词过滤 |
    /// | `order` | `Option<&str>` | 排序，如 `mtime` |
    /// | `typ` | `Option<u8>` | 内容类型 |
    /// | `ps` | u32 | 每页条数 |
    /// | `pn` | `Option<u32>` | 页码 |
    pub async fn fav_list_detail(
        &self,
        media_id: u64,
        tid: Option<u32>,
        keyword: Option<&str>,
        order: Option<&str>,
        typ: Option<u8>,
        ps: u32,
        pn: Option<u32>
    ) -> Result<BpiResponse<FavListDetailData>, BpiError> {
        let mut request = self.get("https://api.bilibili.com/x/v3/fav/resource/list").query(
            &[
                ("media_id", media_id.to_string()),
                ("ps", ps.to_string()),
                ("platform", "web".to_string()),
            ]
        );

        if let Some(tid) = tid {
            request = request.query(&[("tid", tid)]);
        }
        if let Some(keyword) = keyword {
            request = request.query(&[("keyword", keyword)]);
        }
        if let Some(order) = order {
            request = request.query(&[("order", order)]);
        }
        if let Some(typ) = typ {
            request = request.query(&[("type", typ)]);
        }
        if let Some(pn) = pn {
            request = request.query(&[("pn", pn)]);
        }

        request.send_bpi("获取收藏夹内容明细列表").await
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
        media_id: u64
    ) -> Result<BpiResponse<Vec<FavResourceIdItem>>, BpiError> {
        self
            .get("https://api.bilibili.com/x/v3/fav/resource/ids")
            .query(
                &[
                    ("media_id", media_id.to_string()),
                    ("platform", "web".to_string()),
                ]
            )
            .send_bpi("获取收藏夹全部内容id").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_get_fav_list_detail() {
        let bpi = BpiClient::new();
        let media_id = 1572769770;
        let resp = bpi.fav_list_detail(
            media_id,
            None,
            None,
            Some("mtime"),
            Some(0),
            5,
            Some(1)
        ).await;

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

    #[tokio::test]
    async fn test_get_fav_resource_ids() {
        let bpi = BpiClient::new();
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
}
