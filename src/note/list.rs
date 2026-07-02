use crate::{
    BilibiliRequest, BpiClient, BpiError, BpiResponse,
    note::{
        NoteArchiveListParams, NotePublicArchiveListParams, NoteUserPrivateListParams,
        NoteUserPublicListParams,
    },
};
use serde::{Deserialize, Serialize};

/// 稿件私有笔记列表数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteListArchiveData {
    /// 笔记ID列表
    #[serde(rename = "noteIds")]
    pub note_ids: Option<Vec<String>>,
}

// --- 查询用户私有笔记 ---

/// 用户私有笔记的视频信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateNoteArc {
    pub oid: u64,
    pub status: u8,
    pub oid_type: u8,
    pub aid: u64,

    // 老笔记没有以下内容
    pub bvid: Option<String>,
    pub pic: Option<String>,
    pub desc: Option<String>,
}

/// 用户私有笔记列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateNoteItem {
    pub title: String,
    pub summary: String,
    pub mtime: String,
    pub arc: PrivateNoteArc,
    pub note_id: u64,
    pub audit_status: u8,
    pub web_url: String,
    pub note_id_str: String,
    pub message: String,
    pub forbid_note_entrance: Option<bool>,
    pub likes: u64,
    pub has_like: bool,
}

/// 用户私有笔记列表数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateNoteListData {
    pub list: Option<Vec<PrivateNoteItem>>,
    pub page: Option<NotePage>,
}

// --- 查询稿件公开笔记 ---

/// 稿件公开笔记列表项的作者信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicNoteAuthor {
    pub mid: u64,
    pub name: String,
    pub face: String,
    pub level: u8,
    pub vip_info: serde_json::Value,
    pub pendant: serde_json::Value,
}

/// 稿件公开笔记列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicNoteItem {
    pub cvid: u64,
    pub title: String,
    pub summary: String,
    pub pubtime: String,
    pub web_url: String,
    pub message: String,
    pub author: PublicNoteAuthor,
    pub likes: u64,
    pub has_like: bool,
}

/// 稿件公开笔记分页信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotePage {
    pub total: u32,
    pub size: u32,
    pub num: u32,
}

/// 稿件公开笔记列表数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicNoteListArchiveData {
    pub list: Option<Vec<PublicNoteItem>>,
    pub page: Option<NotePage>,
    pub show_public_note: bool,
    pub message: String,
}

// --- 查询用户公开笔记 ---

/// 用户公开笔记列表数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicNoteListUserData {
    pub list: Option<Vec<PublicNoteItem>>,
    pub page: Option<NotePage>,
}

impl BpiClient {
    /// 查询稿件私有笔记
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note)
    ///
    /// # 参数
    /// - params: 稿件 avid 参数
    pub async fn note_list_archive(
        &self,
        params: NoteArchiveListParams,
    ) -> Result<BpiResponse<NoteListArchiveData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/list/archive")
            .query(&params.query_pairs())
            .send_bpi("查询稿件私有笔记")
            .await
    }

    /// 查询用户私有笔记
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `NoteUserPrivateListParams` | 用户私有笔记列表参数 |
    pub async fn note_list_user_private(
        &self,
        params: NoteUserPrivateListParams,
    ) -> Result<BpiResponse<PrivateNoteListData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/list")
            .query(&params.query_pairs())
            .send_bpi("查询用户私有笔记")
            .await
    }

    /// 查询稿件公开笔记
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `NotePublicArchiveListParams` | 稿件公开笔记列表参数 |
    pub async fn note_list_public_archive(
        &self,
        params: NotePublicArchiveListParams,
    ) -> Result<BpiResponse<PublicNoteListArchiveData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/publish/list/archive")
            .query(&params.query_pairs())
            .send_bpi("查询稿件公开笔记")
            .await
    }

    /// 查询用户公开笔记
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `NoteUserPublicListParams` | 用户公开笔记列表参数 |
    pub async fn note_list_public_user(
        &self,
        params: NoteUserPublicListParams,
    ) -> Result<BpiResponse<PublicNoteListUserData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/publish/list/user")
            .query(&params.query_pairs())
            .send_bpi("查询用户公开笔记")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::Aid;
    use crate::note::{
        NoteArchiveListParams, NotePublicArchiveListParams, NoteUserPrivateListParams,
        NoteUserPublicListParams,
    };
    use tracing::info;

    const TEST_PRIVATE_AID: u64 = 676_931_260;
    const TEST_PUBLIC_AID: u64 = 338_677_252;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_note_list_archive() {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi
            .note_list_archive(NoteArchiveListParams::new(
                Aid::new(TEST_PRIVATE_AID).expect("test aid should be valid"),
            ))
            .await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("note ids: {:?}", data.note_ids);
        }
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_note_list_user_private() {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi
            .note_list_user_private(
                NoteUserPrivateListParams::new()
                    .with_page(1)
                    .expect("test page should be valid")
                    .with_page_size(10)
                    .expect("test page size should be valid"),
            )
            .await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        if let Some(list) = resp_data.data.as_ref().and_then(|data| data.list.as_ref()) {
            info!("first note item: {:?}", list.first());
        }
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_note_list_public_archive() {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi
            .note_list_public_archive(
                NotePublicArchiveListParams::new(
                    Aid::new(TEST_PUBLIC_AID).expect("test aid should be valid"),
                )
                .with_page(1)
                .expect("test page should be valid")
                .with_page_size(10)
                .expect("test page size should be valid"),
            )
            .await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("show_public_note: {}", data.show_public_note);
        }
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_note_list_public_user() {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi
            .note_list_public_user(
                NoteUserPublicListParams::new()
                    .with_page(1)
                    .expect("test page should be valid")
                    .with_page_size(10)
                    .expect("test page size should be valid"),
            )
            .await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("total public notes: {}", data.page.as_ref().unwrap().total);
        }
    }

    #[test]
    fn note_archive_list_params_serializes_aid() -> Result<(), BpiError> {
        let params = NoteArchiveListParams::new(Aid::new(TEST_PRIVATE_AID)?);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("oid", TEST_PRIVATE_AID.to_string()),
                ("oid_type", "0".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn note_user_private_list_params_rejects_zero_page() {
        let err = NoteUserPrivateListParams::new().with_page(0).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "pn", .. }
        ));
    }

    #[test]
    fn note_public_archive_list_params_serializes_query() -> Result<(), BpiError> {
        let params = NotePublicArchiveListParams::new(Aid::new(TEST_PUBLIC_AID)?)
            .with_page(1)?
            .with_page_size(10)?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("oid", TEST_PUBLIC_AID.to_string()),
                ("oid_type", "0".to_string()),
                ("pn", "1".to_string()),
                ("ps", "10".to_string()),
            ]
        );
        Ok(())
    }
}
