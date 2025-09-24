use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };
use serde_json::json;
// --- 保存视频笔记 ---

/// 保存视频笔记的响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoteAddResponseData {
    /// 笔记ID
    pub note_id: String,
}

// --- 删除视频笔记 ---

impl BpiClient {
    /// 保存视频笔记
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
    /// | `tags` | `Option<&str>` | 跳转标签列表 |
    /// | `publish` | `Option<bool>` | 是否公开：false 不公开，true 公开 |
    /// | `auto_comment` | `Option<bool>` | 是否添加到评论区 |
    pub async fn note_add(
        &self,
        oid: u64,
        title: &str,
        summary: &str,
        content: &str,
        note_id: Option<&str>,
        tags: Option<&str>,
        publish: Option<bool>,
        auto_comment: Option<bool>
    ) -> Result<BpiResponse<NoteAddResponseData>, BpiError> {
        let csrf = self.csrf()?;

        let content = json!([{"insert": content}]);

        let mut form = vec![
            ("oid", oid.to_string()),
            ("oid_type", "0".to_string()),
            ("title", title.to_string()),
            ("summary", summary.to_string()),
            ("content", content.to_string()),
            ("cls", "1".to_string()),
            ("from", "save".to_string()),
            ("platform", "web".to_string()),
            ("csrf", csrf)
        ];

        if let Some(tags) = tags {
            form.push(("tags", tags.to_string()));
        }

        if let Some(note_id) = note_id {
            form.push(("note_id", note_id.to_string()));
        }

        if let Some(publish) = publish {
            form.push(("publish", (if publish { "1" } else { "0" }).to_string()));
        }
        if let Some(auto_comment) = auto_comment {
            form.push(("auto_comment", (if auto_comment { "1" } else { "0" }).to_string()));
        }

        self.post("https://api.bilibili.com/x/note/add").form(&form).send_bpi("保存视频笔记").await
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
        note_id: Option<&str>
    ) -> Result<BpiResponse<NoteAddResponseData>, BpiError> {
        self.note_add(oid, title, summary, content, note_id, None, None, None).await
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
        note_id: Option<String>
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let mut form = vec![("oid", oid.to_string()), ("csrf", csrf)];

        if let Some(note_id) = note_id {
            form.push(("note_id", note_id.to_string()));
        }

        self.post("https://api.bilibili.com/x/note/del").form(&form).send_bpi("删除视频笔记").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_note_add_and_del() {
        let bpi = BpiClient::new();
        let oid = 464606672;
        let title = "测试笔记";
        let summary = "这是个测试摘要";
        let content =
            "Lorem Ipsum is simply dummy text \
        of the printing and typesetting industry. Lorem Ipsum\
         has been the industry's standard dummy text ever since \
         the 1500s, when an unknown printer took a galley of type \
         and scrambled it to make a type specimen book. \
         It has survived not only five centuries, but also the leap into electronic typesetting, \
         remaining essentially unchanged. It was popularised \
         in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, \
         and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum";

        // 1. 保存笔记
        let add_resp = bpi.note_add(
            oid,
            title,
            summary,
            content,
            None,
            None,
            Some(false),
            Some(false)
        ).await;

        info!("Add note result: {:?}", add_resp);
        assert!(add_resp.is_ok());

        let note_id = add_resp.unwrap().data.unwrap().note_id;
        info!("New note ID: {}", note_id);

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        // 2. 删除笔记
        let del_resp = bpi.note_del(oid, Some(note_id)).await;
        info!("Delete note result: {:?}", del_resp);
        assert!(del_resp.is_ok());
    }
}
