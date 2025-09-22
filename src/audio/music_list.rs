//! 歌单&音频收藏夹详细信息
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/music_list.md
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioCollectionsListData {
    #[serde(rename = "curPage")]
    pub cur_page: i32,

    #[serde(rename = "pageCount")]
    pub page_count: i32,

    #[serde(rename = "totalSize")]
    pub total_size: i32,

    #[serde(rename = "pageSize")]
    pub page_size: i32,

    pub data: Vec<AudioCollection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioCollection {
    pub id: i64,
    pub uid: i64,
    pub uname: String,
    pub title: String,
    pub r#type: i32,
    pub published: i32,
    pub cover: String,
    pub ctime: i64,
    pub song: i32,
    pub desc: String,
    pub sids: Vec<i64>,
    #[serde(rename = "menuId")]
    pub menu_id: i64,
    pub statistic: AudioCollectionStatistic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioCollectionStatistic {
    pub sid: i64,
    pub play: i64,
    pub collect: i64,
    pub comment: Option<i64>,
    pub share: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioHotMenuData {
    #[serde(rename = "curPage")]
    pub cur_page: i32,

    #[serde(rename = "pageCount")]
    pub page_count: i32,

    #[serde(rename = "totalSize")]
    pub total_size: i32,

    #[serde(rename = "pageSize")]
    pub page_size: i32,

    pub data: Vec<AudioHotMenu>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioHotMenu {
    #[serde(rename = "menuId")]
    pub menu_id: i64,

    pub uid: i64,

    pub uname: String,

    pub title: String,

    pub cover: String,

    pub intro: String,

    pub r#type: i32,

    pub off: i32,

    pub ctime: i64,

    pub curtime: i64,

    pub statistic: AudioHotMenuStatistic,

    pub snum: i32,

    pub attr: i32,

    #[serde(rename = "isDefault")]
    pub is_default: i32,

    #[serde(rename = "collectionId")]
    pub collection_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioHotMenuStatistic {
    pub sid: i64,
    pub play: i64,
    pub collect: i64,
    pub comment: i64,
    pub share: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankMenuData {
    #[serde(rename = "curPage")]
    pub cur_page: i32,

    #[serde(rename = "pageCount")]
    pub page_count: i32,

    #[serde(rename = "totalSize")]
    pub total_size: i32,

    #[serde(rename = "pageSize")]
    pub page_size: i32,

    pub data: Vec<AudioRankMenu>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankMenu {
    #[serde(rename = "menuId")]
    pub menu_id: i64,

    pub uid: i64,

    pub uname: String,

    pub title: String,

    pub cover: String,

    pub intro: String,

    pub r#type: i32,

    pub off: i32,

    pub ctime: i64,

    pub curtime: i64,

    pub statistic: AudioRankMenuStatistic,

    pub snum: i32,

    pub attr: i32,

    #[serde(rename = "isDefault")]
    pub is_default: i32,

    #[serde(rename = "collectionId")]
    pub collection_id: i64,

    pub audios: Vec<AudioRankItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankMenuStatistic {
    pub sid: i64,
    pub play: i64,
    pub collect: i64,
    pub comment: i64,
    pub share: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRankItem {
    pub id: i64,
    pub title: String,
    pub duration: i64,
}

impl BpiClient {
    /// 查询自己创建的歌单
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明     |
    /// | ------ | ------ | -------- |
    /// | `pn`   | u32    | 页码     |
    /// | `ps`   | u32    | 每页项数 |
    ///
    /// # 文档
    /// [查询自己创建的歌单](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/music_list.md#查询自己创建的歌单)
    pub async fn audio_collections_list(
        &self,
        pn: u32,
        ps: u32,
    ) -> Result<BpiResponse<AudioCollectionsListData>, BpiError> {
        self.get("https://www.bilibili.com/audio/music-service-c/web/collections/list")
            .query(&[("pn", pn.to_string()), ("ps", ps.to_string())])
            .send_bpi("查询自己创建的歌单")
            .await
    }

    /// 查询音频收藏夹信息
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明                                     |
    /// | ------ | ------ | ---------------------------------------- |
    /// | `sid`  | u64    | 音频收藏夹 mlid（必须为默认收藏夹 mlid） |
    ///
    /// # 文档
    /// [查询音频收藏夹（默认歌单）信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/music_list.md#查询音频收藏夹默认歌单信息)
    pub async fn audio_collection_info(
        &self,
        sid: u64,
    ) -> Result<BpiResponse<AudioCollection>, BpiError> {
        self.get("https://www.bilibili.com/audio/music-service-c/web/collections/info")
            .query(&[("sid", sid.to_string())])
            .send_bpi("查询音频收藏夹信息")
            .await
    }

    /// 查询热门歌单
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明     |
    /// | ------ | ------ | -------- |
    /// | `pn`   | u32    | 页码     |
    /// | `ps`   | u32    | 每页项数 |
    ///
    /// # 文档
    /// [查询热门歌单](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/music_list.md#查询热门歌单)
    pub async fn audio_hot_menu(
        &self,
        pn: u32,
        ps: u32,
    ) -> Result<BpiResponse<AudioHotMenuData>, BpiError> {
        self.get("https://www.bilibili.com/audio/music-service-c/web/menu/hit")
            .query(&[("pn", pn.to_string()), ("ps", ps.to_string())])
            .send_bpi("查询热门歌单")
            .await
    }

    /// 查询热门榜单
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明     |
    /// | ------ | ------ | -------- |
    /// | `pn`   | u32    | 页码     |
    /// | `ps`   | u32    | 每页项数 |
    ///
    /// # 文档
    /// [查询热门榜单](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/audio/music_list.md#查询热门榜单)
    pub async fn audio_rank_menu(
        &self,
        pn: u32,
        ps: u32,
    ) -> Result<BpiResponse<AudioRankMenuData>, BpiError> {
        self.get("https://www.bilibili.com/audio/music-service-c/web/menu/rank")
            .query(&[("pn", pn.to_string()), ("ps", ps.to_string())])
            .send_bpi("查询热门榜单")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_audio_collections_list() {
        let bpi = BpiClient::new();
        let result = bpi.audio_collections_list(1, 2).await;
        if let Ok(response) = result {
            assert_eq!(response.code, 0);
            let data = response.data.unwrap();
            assert!(data.cur_page >= 1);
            assert!(data.page_size > 0);
        }
    }

    #[tokio::test]
    async fn test_audio_collection_info() {
        let bpi = BpiClient::new();
        let result = bpi.audio_collection_info(15967839).await;
        if let Ok(response) = result {
            assert_eq!(response.code, 0);
        }
    }

    #[tokio::test]
    async fn test_audio_hot_menu() {
        let bpi = BpiClient::new();
        let result = bpi.audio_hot_menu(1, 3).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.code, 0);
        let data = response.data.unwrap();

        assert!(data.cur_page >= 1);
        assert!(data.page_size > 0);
        assert!(!data.data.is_empty());
    }

    #[tokio::test]
    async fn test_audio_rank_menu() {
        let bpi = BpiClient::new();
        let result = bpi.audio_rank_menu(1, 6).await;
        assert!(result.is_ok());
        let response = result.unwrap();

        assert_eq!(response.code, 0);

        let data = response.data.unwrap();

        assert!(data.cur_page >= 1);
        assert!(data.page_size > 0);
        assert!(!data.data.is_empty());
        // 检查榜单中的音频信息
        for menu in &data.data {
            assert!(!menu.audios.is_empty());
            for audio in &menu.audios {
                assert!(audio.id > 0);
                assert!(!audio.title.is_empty());
                assert!(audio.duration > 0);
            }
        }
    }
}
