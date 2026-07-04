// --- 保存视频笔记 ---

use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResponse;
use crate::BpiResult;
use crate::ids::Aid;
use crate::note::NoteClient;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// 保存视频笔记的响应数据

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteAddResponseData {
    /// 笔记ID
    pub note_id: String,
}

/// Parameters for saving a video note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteAddParams {
    oid: Aid,
    title: String,
    summary: String,
    content: String,
    note_id: Option<String>,
    tags: Option<String>,
    publish: Option<bool>,
    auto_comment: Option<bool>,
}

impl NoteAddParams {
    /// Creates note-save parameters for a video AV ID.
    pub fn new(
        oid: Aid,
        title: impl Into<String>,
        summary: impl Into<String>,
        content: impl Into<String>,
    ) -> BpiResult<Self> {
        let params = Self {
            oid,
            title: title.into(),
            summary: summary.into(),
            content: content.into(),
            note_id: None,
            tags: None,
            publish: None,
            auto_comment: None,
        };
        params.validate()?;
        Ok(params)
    }

    /// Sets the note ID when updating an existing note.
    pub fn note_id(mut self, note_id: impl Into<String>) -> BpiResult<Self> {
        self.note_id = Some(note_id.into());
        self.validate()?;
        Ok(self)
    }

    /// Sets the note jump tags.
    pub fn tags(mut self, tags: impl Into<String>) -> BpiResult<Self> {
        self.tags = Some(tags.into());
        self.validate()?;
        Ok(self)
    }

    /// Controls whether the note should be public.
    pub fn publish(mut self, publish: bool) -> Self {
        self.publish = Some(publish);
        self
    }

    /// Controls whether the note should be added to comments.
    pub fn auto_comment(mut self, auto_comment: bool) -> Self {
        self.auto_comment = Some(auto_comment);
        self
    }

    fn validate(&self) -> BpiResult<()> {
        validate_required("title", &self.title)?;
        validate_required("summary", &self.summary)?;
        validate_required("content", &self.content)?;
        if let Some(note_id) = self.note_id.as_deref() {
            validate_required("note_id", note_id)?;
        }
        if let Some(tags) = self.tags.as_deref() {
            validate_required("tags", tags)?;
        }
        Ok(())
    }

    fn form_pairs(&self, csrf: impl Into<String>) -> Vec<(&'static str, String)> {
        let content = json!([{"insert": self.content}]);
        let mut form = vec![
            ("oid", self.oid.to_string()),
            ("oid_type", "0".to_string()),
            ("title", self.title.clone()),
            ("summary", self.summary.clone()),
            ("content", content.to_string()),
            ("cls", "1".to_string()),
            ("from", "save".to_string()),
            ("platform", "web".to_string()),
            ("csrf", csrf.into()),
        ];

        if let Some(tags) = self.tags.as_ref() {
            form.push(("tags", tags.clone()));
        }
        if let Some(note_id) = self.note_id.as_ref() {
            form.push(("note_id", note_id.clone()));
        }
        if let Some(publish) = self.publish {
            form.push(("publish", bool_flag(publish)));
        }
        if let Some(auto_comment) = self.auto_comment {
            form.push(("auto_comment", bool_flag(auto_comment)));
        }

        form
    }
}

fn validate_required(field: &'static str, value: &str) -> BpiResult<()> {
    if value.trim().is_empty() {
        return Err(BpiError::invalid_parameter(field, "value cannot be blank"));
    }

    Ok(())
}

fn bool_flag(value: bool) -> String {
    if value {
        "1".to_string()
    } else {
        "0".to_string()
    }
}

// --- 删除视频笔记 ---

impl<'a> NoteClient<'a> {
    /// 保存视频笔记
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note)
    ///
    pub async fn note_add(
        &self,
        params: NoteAddParams,
    ) -> Result<BpiResponse<NoteAddResponseData>, BpiError> {
        let csrf = self.client.csrf()?;
        let form = params.form_pairs(csrf);

        self.client
            .post("https://api.bilibili.com/x/note/add")
            .form(&form)
            .send_bpi("保存视频笔记")
            .await
    }

    /// 保存视频笔记（精简参数）
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `oid` | u64 | 目标 ID（视频 avid） |
    /// | `title` | &str | 笔记标题 |
    /// | `summary` | &str | 笔记预览文本 |
    /// | `content` | &str | 笔记正文 |
    /// | `note_id` | `Option<&str>` | 笔记 ID（创建时可省略） |
    pub async fn note_add_simple(
        &self,
        oid: u64,
        title: &str,
        summary: &str,
        content: &str,
        note_id: Option<&str>,
    ) -> Result<BpiResponse<NoteAddResponseData>, BpiError> {
        let mut params = NoteAddParams::new(Aid::new(oid)?, title, summary, content)?;
        if let Some(note_id) = note_id {
            params = params.note_id(note_id)?;
        }

        self.note_add(params).await
    }

    /// 删除视频笔记
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/note)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `oid` | u64 | 目标 ID（视频 avid） |
    /// | `note_id` | `Option<String>` | 笔记 ID |
    pub async fn note_del(
        &self,
        oid: u64,
        note_id: Option<String>,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;

        let mut form = vec![("oid", oid.to_string()), ("csrf", csrf)];

        if let Some(note_id) = note_id {
            form.push(("note_id", note_id.to_string()));
        }

        self.client
            .post("https://api.bilibili.com/x/note/del")
            .form(&form)
            .send_bpi("删除视频笔记")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn note_add_params_rejects_blank_content() {
        let err = NoteAddParams::new(
            Aid::new(170001).expect("test aid should be valid"),
            "title",
            "summary",
            "  ",
        )
        .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "content",
                ..
            }
        ));
    }
}
