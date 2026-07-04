use crate::ids::Aid;
use crate::{BpiError, BpiResult};
use serde::{Deserialize, Serialize};
// --- 保存视频笔记 ---

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
}

fn validate_required(field: &'static str, value: &str) -> BpiResult<()> {
    if value.trim().is_empty() {
        return Err(BpiError::invalid_parameter(field, "value cannot be blank"));
    }

    Ok(())
}

// --- 删除视频笔记 ---

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
