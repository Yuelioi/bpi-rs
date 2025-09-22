use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// --- 获取收藏夹元数据 ---

/// 收藏夹元数据的创建者信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FavFolderUpper {
    pub mid: u64,
    pub name: String,
    pub face: String,
    pub followed: bool,
    pub vip_type: u8,
    /// 阿b拼写错误
    #[serde(rename = "vip_statue")]
    pub vip_status: u8,
}

/// 收藏夹元数据的状态数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FavFolderCntInfo {
    pub collect: u64,
    pub play: u64,
    pub thumb_up: u64,
    pub share: u64,
}

/// 收藏夹元数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FavFolderInfo {
    pub id: u64,
    pub fid: u64,
    pub mid: u64,
    pub attr: u32,
    pub title: String,
    pub cover: String,
    pub upper: FavFolderUpper,
    pub cover_type: u8,
    pub cnt_info: FavFolderCntInfo,
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

// --- 获取指定用户创建的所有收藏夹信息 ---

/// 用户创建的收藏夹列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreatedFolderItem {
    pub id: u64,
    pub fid: u64,
    pub mid: u64,
    pub attr: u32,
    pub title: String,
    pub fav_state: u8,
    pub media_count: u32,
}

/// 用户创建的收藏夹信息数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreatedFolderListData {
    pub count: u32,
    pub list: Vec<CreatedFolderItem>,
}

// --- 查询用户收藏的视频收藏夹 ---

/// 用户收藏的视频收藏夹列表项的创建人信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollectedFolderUpper {
    pub mid: u64,
    pub name: String,
    pub face: String,
}

/// 用户收藏的视频收藏夹列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollectedFolderItem {
    pub id: u64,
    pub fid: u64,
    pub mid: u64,
    pub attr: u32,
    pub title: String,
    pub cover: String,
    pub upper: CollectedFolderUpper,
    pub cover_type: u8,
    pub intro: String,
    pub ctime: u64,
    pub mtime: u64,
    pub state: u8,
    pub fav_state: u8,
    pub media_count: u32,
}

/// 用户收藏的视频收藏夹列表数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollectedFolderListData {
    pub count: u32,
    pub list: Vec<CollectedFolderItem>,
}

// --- 批量获取指定收藏id的内容 ---

/// 内容信息列表中的UP主信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceInfoUpper {
    pub mid: u64,
    pub name: String,
    pub face: String,
}

/// 内容信息列表中的状态数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceInfoCntInfo {
    pub collect: u64,
    pub play: u64,
    pub danmaku: u64,
}

/// 批量获取的内容信息列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResourceInfoItem {
    pub id: u64,
    #[serde(rename = "type")]
    pub type_name: u8,
    pub title: String,
    pub cover: String,
    pub intro: String,
    pub page: Option<u32>,
    pub duration: u32,
    pub upper: ResourceInfoUpper,
    pub attr: u8,
    pub cnt_info: ResourceInfoCntInfo,
    pub link: String,
    pub ctime: u64,
    pub pubtime: u64,
    pub fav_time: u64,
    pub bv_id: Option<String>,
    pub bvid: Option<String>,
    pub season: Option<serde_json::Value>,
}

impl BpiClient {
    /// 获取收藏夹元数据
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav
    ///
    /// 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `media_id` | u64 | 收藏夹 media_id |
    pub async fn fav_folder_info(
        &self,
        media_id: u64,
    ) -> Result<BpiResponse<FavFolderInfo>, BpiError> {
        self.get("https://api.bilibili.com/x/v3/fav/folder/info")
            .query(&[("media_id", media_id)])
            .send_bpi("获取收藏夹元数据")
            .await
    }

    /// 获取指定用户创建的所有收藏夹信息
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav
    ///
    /// 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `up_mid` | u64 | 用户 mid |
    /// | `typ` | Option<u8> | 类型过滤 |
    /// | `rid` | Option<u64> | 关联资源 id |
    pub async fn fav_created_list(
        &self,
        up_mid: u64,
        typ: Option<u8>,
        rid: Option<u64>,
    ) -> Result<BpiResponse<CreatedFolderListData>, BpiError> {
        let mut request = self
            .get("https://api.bilibili.com/x/v3/fav/folder/created/list-all")
            .query(&[("up_mid", up_mid.to_string())]);

        if let Some(t) = typ {
            request = request.query(&[("type", t)]);
        }
        if let Some(r) = rid {
            request = request.query(&[("rid", r)]);
        }

        request
            .query(&[("web_location", "333.1387")])
            .send_bpi("获取指定用户创建的所有收藏夹信息")
            .await
    }

    /// 查询用户收藏的视频收藏夹
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav
    ///
    /// 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `up_mid` | u64 | 用户 mid |
    /// | `pn` | u32 | 页码 |
    /// | `ps` | u32 | 页大小 |
    pub async fn fav_collected_list(
        &self,
        up_mid: u64,
        pn: u32,
        ps: u32,
    ) -> Result<BpiResponse<CollectedFolderListData>, BpiError> {
        self.get("https://api.bilibili.com/x/v3/fav/folder/collected/list")
            .query(&[
                ("up_mid", up_mid.to_string()),
                ("pn", pn.to_string()),
                ("ps", ps.to_string()),
                ("platform", "web".to_string()),
            ])
            .send_bpi("查询用户收藏的视频收藏夹")
            .await
    }

    /// 批量获取指定收藏id的内容
    /// `resources`: "{内容id}:{内容类型},..."
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/fav
    ///
    /// 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `resources` | &str | 形如 "{内容id}:{内容类型},..." |
    pub async fn fav_resource_infos(
        &self,
        resources: &str,
    ) -> Result<BpiResponse<Vec<ResourceInfoItem>>, BpiError> {
        self.get("https://api.bilibili.com/x/v3/fav/resource/infos")
            .query(&[("resources", resources), ("platform", "web")])
            .send_bpi("批量获取指定收藏id的内容")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_get_fav_folder_info() {
        let bpi = BpiClient::new();
        // 替换为一个公开收藏夹的media_id
        let media_id = 3717139570;
        let resp = bpi.fav_folder_info(media_id).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("folder title: {}", data.title);
            info!("folder media_count: {}", data.media_count);
            info!("upper info: {:?}", data.upper);
        }
    }

    #[tokio::test]
    async fn test_get_fav_created_list() {
        let bpi = BpiClient::new();

        let up_mid = 4279370;
        let resp = bpi.fav_created_list(up_mid, None, None).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("created folders count: {}", data.count);
            info!("first folder info: {:?}", data.list.first());
        }
    }

    #[tokio::test]
    async fn test_get_fav_collected_list() {
        let bpi = BpiClient::new();

        let up_mid = 4279370;
        let resp = bpi.fav_collected_list(up_mid, 1, 20).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("collected folders count: {}", data.count);
            info!("first collected folder info: {:?}", data.list.first());
        }
    }

    #[tokio::test]
    async fn test_get_fav_resource_infos() {
        let bpi = BpiClient::new();
        let resources = "115087859779103:2";
        let resp = bpi.fav_resource_infos(resources).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("retrieved {} resources", data.len());
            info!("first resource info: {:?}", data.first());
        }
    }
}
