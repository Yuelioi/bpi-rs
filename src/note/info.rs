use crate::{
    BilibiliRequest, BpiClient, BpiError, BpiResponse,
    note::{NoteIsForbidParams, NotePrivateInfoParams, NotePublicInfoParams},
};
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
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note)
    ///
    /// - params: 稿件 avid 参数
    pub async fn note_is_forbid(
        &self,
        params: NoteIsForbidParams,
    ) -> Result<BpiResponse<NoteIsForbidData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/is_forbid")
            .query(&params.query_pairs())
            .send_bpi("查询稿件是否禁止笔记")
            .await
    }

    /// 查询私有笔记内容
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | `NotePrivateInfoParams` | 私有笔记查询参数 |
    pub async fn note_get_private_info(
        &self,
        params: NotePrivateInfoParams,
    ) -> Result<BpiResponse<PrivateNoteInfoData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/info")
            .query(&params.query_pairs())
            .send_bpi("查询私有笔记内容")
            .await
    }

    /// 查询公开笔记内容
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note)
    ///
    /// - params: 公开笔记 cvid 参数
    pub async fn note_get_public_info(
        &self,
        params: NotePublicInfoParams,
    ) -> Result<BpiResponse<PublicNoteInfoData>, BpiError> {
        self.get("https://api.bilibili.com/x/note/publish/info")
            .query(&params.query_pairs())
            .send_bpi("查询公开笔记内容")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{Aid, Cvid, NoteId};
    use crate::note::{NoteIsForbidParams, NotePrivateInfoParams, NotePublicInfoParams};
    use tracing::info;

    const TEST_AID: u64 = 338_677_252;
    const TEST_PRIVATE_AID: u64 = 676_931_260;
    const TEST_NOTE_ID: u64 = 83_577_722_856_540_160;
    const TEST_CVID: u64 = 15_160_286;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_note_is_forbid() {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi
            .note_is_forbid(NoteIsForbidParams::new(
                Aid::new(TEST_AID).expect("test aid should be valid"),
            ))
            .await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("forbid_note_entrance: {}", data.forbid_note_entrance);
        }
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_note_get_private_info() {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi
            .note_get_private_info(NotePrivateInfoParams::new(
                Aid::new(TEST_PRIVATE_AID).expect("test aid should be valid"),
                NoteId::new(TEST_NOTE_ID).expect("test note id should be valid"),
            ))
            .await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        if let Some(data) = resp_data.data {
            info!("note title: {}", data.title);
            info!("note content: {}", data.content);
        }
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_note_get_public_info() {
        let bpi = BpiClient::new().expect("client should build");
        let resp = bpi
            .note_get_public_info(NotePublicInfoParams::new(
                Cvid::new(TEST_CVID).expect("test cvid should be valid"),
            ))
            .await;

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

    #[test]
    fn note_is_forbid_params_serializes_aid() -> Result<(), BpiError> {
        let params = NoteIsForbidParams::new(Aid::new(TEST_AID)?);

        assert_eq!(params.query_pairs(), vec![("aid", TEST_AID.to_string())]);
        Ok(())
    }

    #[test]
    fn note_private_info_params_serializes_required_query() -> Result<(), BpiError> {
        let params =
            NotePrivateInfoParams::new(Aid::new(TEST_PRIVATE_AID)?, NoteId::new(TEST_NOTE_ID)?);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("oid", TEST_PRIVATE_AID.to_string()),
                ("oid_type", "0".to_string()),
                ("note_id", TEST_NOTE_ID.to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn cvid_rejects_zero() {
        let err = Cvid::new(0).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "cvid", .. }
        ));
    }
}
