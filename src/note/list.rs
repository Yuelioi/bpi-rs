use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
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
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note
    ///
    /// 参数
    /// - oid: 稿件 avid
    pub async fn note_list_archive(
        &self,
        oid: u64,
    ) -> Result<BpiResponse<NoteListArchiveData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/list/archive")
            .query(&[("oid", oid), ("oid_type", 0)])
            .send_bpi("查询稿件私有笔记")
            .await
    }

    /// 查询用户私有笔记
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `pn` | u32 | 页码 |
    /// | `ps` | u32 | 每页数量 |
    pub async fn note_list_user_private(
        &self,
        pn: u32,
        ps: u32,
    ) -> Result<BpiResponse<PrivateNoteListData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/list")
            .query(&[("pn", pn), ("ps", ps)])
            .send_bpi("查询用户私有笔记")
            .await
    }

    /// 查询稿件公开笔记
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `oid` | u64 | 稿件 avid |
    /// | `pn` | u32 | 页码 |
    /// | `ps` | u32 | 每页数量 |
    pub async fn note_list_public_archive(
        &self,
        oid: u64,
        pn: u32,
        ps: u32,
    ) -> Result<BpiResponse<PublicNoteListArchiveData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/publish/list/archive")
            .query(&[
                ("oid", oid.to_string()),
                ("oid_type", 0.to_string()),
                ("pn", pn.to_string()),
                ("ps", ps.to_string()),
            ])
            .send_bpi("查询稿件公开笔记")
            .await
    }

    /// 查询用户公开笔记
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `pn` | u32 | 页码 |
    /// | `ps` | u32 | 每页数量 |
    pub async fn note_list_public_user(
        &self,
        pn: u32,
        ps: u32,
    ) -> Result<BpiResponse<PublicNoteListUserData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/publish/list/user")
            .query(&[("pn", pn), ("ps", ps)])
            .send_bpi("查询用户公开笔记")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_note_list_archive() {
        let bpi = BpiClient::new();
        // 替换为一个有效的avid
        let oid = 676931260;
        let resp = bpi.note_list_archive(oid).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("note ids: {:?}", data.note_ids);
        }
    }

    #[tokio::test]
    async fn test_note_list_user_private() {
        let bpi = BpiClient::new();
        let resp = bpi.note_list_user_private(1, 10).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        resp_data
            .data
            .as_ref()
            .and_then(|data| data.list.as_ref())
            .and_then(|list| {
                info!("first note item: {:?}", list.first());
                Some(())
            });
    }

    #[tokio::test]
    async fn test_note_list_public_archive() {
        let bpi = BpiClient::new();
        // 替换为一个有效的avid
        let oid = 338677252;
        let resp = bpi.note_list_public_archive(oid, 1, 10).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("show_public_note: {}", data.show_public_note);
        }
    }

    #[tokio::test]
    async fn test_note_list_public_user() {
        let bpi = BpiClient::new();
        let resp = bpi.note_list_public_user(1, 10).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("total public notes: {}", data.page.as_ref().unwrap().total);
        }
    }
}
