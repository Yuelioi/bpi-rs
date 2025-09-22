use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// --- 查询该稿件是否禁止笔记 ---

/// 稿件是否禁止笔记的响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteIsForbidData {
    /// 是否禁止笔记
    pub forbid_note_entrance: bool,
}

// --- 查询私有笔记内容 ---

/// 私有笔记的视频稿件信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateNoteArc {
    pub oid: u64,
    pub oid_type: u8,
    pub title: String,
    pub pic: String,
    pub status: u32,
    pub desc: String,
}

/// 私有笔记的标签
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateNoteTag {
    pub cid: u64,
    pub status: u8,
    pub index: u32,
    pub seconds: u32,
    pub pos: u32,
}

/// 私有笔记的响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrivateNoteInfoData {
    pub arc: PrivateNoteArc,
    pub audit_status: u8,
    pub cid_count: u32,
    pub content: String,
    pub forbid_note_entrance: bool,
    pub pub_reason: Option<String>,
    pub pub_status: u8,
    pub pub_version: u32,
    pub summary: String,
    pub tags: Vec<PrivateNoteTag>,
    pub title: String,
}

// --- 查询公开笔记内容 ---

/// 公开笔记的稿件信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicNoteArc {
    pub oid: u64,
    pub oid_type: u8,
    pub title: String,
    pub status: u32,
    pub pic: String,
    pub desc: String,
}

/// 公开笔记的作者信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicNoteAuthor {
    pub mid: u64,
    pub name: String,
    pub face: String,
    pub level: u8,
    pub vip_info: serde_json::Value,
    pub pendant: serde_json::Value,
}

/// 公开笔记的响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PublicNoteInfoData {
    pub cvid: u64,
    pub note_id: u64,
    pub title: String,
    pub summary: String,
    pub content: String,
    pub cid_count: u32,
    pub pub_status: u8,
    pub tags: Vec<PrivateNoteTag>,
    pub arc: PublicNoteArc,
    pub author: PublicNoteAuthor,
    pub forbid_note_entrance: bool,
}

impl BpiClient {
    /// 查询该稿件是否禁止笔记
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note
    ///
    /// - aid: 稿件 avid
    pub async fn note_is_forbid(
        &self,
        aid: u64,
    ) -> Result<BpiResponse<NoteIsForbidData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/is_forbid")
            .query(&[("aid", aid)])
            .send_bpi("查询稿件是否禁止笔记")
            .await
    }

    /// 查询私有笔记内容
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note
    ///
    /// 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `oid` | u64 | 稿件 avid |
    /// | `note_id` | u64 | 笔记 ID |
    pub async fn note_get_private_info(
        &self,
        oid: u64,
        note_id: u64,
    ) -> Result<BpiResponse<PrivateNoteInfoData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/info")
            .query(&[("oid", oid), ("oid_type", 0), ("note_id", note_id)])
            .send_bpi("查询私有笔记内容")
            .await
    }

    /// 查询公开笔记内容
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note
    ///
    /// - cvid: 公开笔记 cvid
    pub async fn note_get_public_info(
        &self,
        cvid: u64,
    ) -> Result<BpiResponse<PublicNoteInfoData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/publish/info")
            .query(&[("cvid", cvid)])
            .send_bpi("查询公开笔记内容")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_note_is_forbid() {
        let bpi = BpiClient::new();
        // 替换为一个有效的avid
        let aid = 338677252;
        let resp = bpi.note_is_forbid(aid).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("forbid_note_entrance: {}", data.forbid_note_entrance);
        }
    }

    #[tokio::test]
    async fn test_note_get_private_info() {
        let bpi = BpiClient::new();
        let oid = 676931260;
        let note_id = 83577722856540160;
        let resp = bpi.note_get_private_info(oid, note_id).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("note title: {}", data.title);
            info!("note content: {}", data.content);
        }
    }

    #[tokio::test]
    async fn test_note_get_public_info() {
        let bpi = BpiClient::new();
        let cvid = 15160286;
        let resp = bpi.note_get_public_info(cvid).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("note title: {}", data.title);
            info!("note content: {}", data.content);
            info!("author name: {}", data.author.name);
        }
    }
}
