//! 评论区相关操作 API
//!
//! [参考文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/comment/action)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 评论区类型枚举（部分示例，需按需求扩展）
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentType {
    Video = 1, // 视频
    Article = 12, // 专栏
    Dynamic = 17, // 动态
    Unknown = 0,
}

/// 举报原因枚举
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ReportReason {
    Other = 0,
    Ad = 1,
    Porn = 2,
    Spam = 3,
    Flame = 4,
    Spoiler = 5,
    Politics = 6,
    Abuse = 7,
    Irrelevant = 8,
    Illegal = 9,
    Vulgar = 10,
    Phishing = 11,
    Scam = 12,
    Rumor = 13,
    Incitement = 14,
    Privacy = 15,
    FloorSnatching = 16,
    HarmfulToYouth = 17,
}

/// 评论成功返回数据
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CommentData {
    pub rpid: u64,
    pub rpid_str: String,
    pub root: u64,
    pub root_str: String,
    pub parent: u64,
    pub parent_str: String,
    pub dialog: u64,
    pub dialog_str: String,
    pub success_toast: Option<String>,
}

/// 点赞评论
impl BpiClient {
    /// 发表评论
    ///
    /// 在指定评论区发表评论，支持回复根评论或父评论。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `type` | CommentType | 评论区类型 |
    /// | `oid` | u64 | 对象 ID |
    /// | `message` | &str | 评论内容 |
    /// | `root` | `Option<u64>` | 根评论 rpid，可选 |
    /// | `parent` | `Option<u64>` | 父评论 rpid，可选 |
    ///
    /// # 文档
    /// [发表评论](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/comment/action.md#发表评论)
    pub async fn comment_add(
        &self,
        r#type: CommentType,
        oid: u64,
        message: &str,
        root: Option<u64>,
        parent: Option<u64>
    ) -> Result<BpiResponse<CommentData>, BpiError> {
        let csrf = self.csrf()?;
        let mut params = vec![
            ("type", (r#type as u32).to_string()),
            ("oid", oid.to_string()),
            ("message", message.to_string()),
            ("plat", "1".to_string()), // 默认 web
            ("csrf", csrf.to_string())
        ];
        if let Some(r) = root {
            params.push(("root", r.to_string()));
        }
        if let Some(p) = parent {
            params.push(("parent", p.to_string()));
        }

        self
            .post("https://api.bilibili.com/x/v2/reply/add")
            .form(&params)
            .send_bpi("发表评论").await
    }
    /// 点赞评论
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `type` | CommentType | 评论区类型 |
    /// | `oid` | u64 | 对象 ID |
    /// | `rpid` | u64 | 评论 rpid |
    /// | `action` | u8 | 操作：0 取消，1 点赞 |
    ///
    /// # 文档
    /// [点赞评论](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/comment/action.md#点赞评论)
    pub async fn comment_like(
        &self,
        r#type: CommentType,
        oid: u64,
        rpid: u64,
        action: u8
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let params = [
            ("type", (r#type as u32).to_string()),
            ("oid", oid.to_string()),
            ("rpid", rpid.to_string()),
            ("action", action.to_string()), // 0 取消，1 点赞
            ("csrf", csrf.to_string()),
        ];

        self
            .post("https://api.bilibili.com/x/v2/reply/action")
            .form(&params)
            .send_bpi("点赞评论").await
    }

    /// 点踩评论
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `type` | CommentType | 评论区类型 |
    /// | `oid` | u64 | 对象 ID |
    /// | `rpid` | u64 | 评论 rpid |
    /// | `action` | u8 | 操作：0 取消，1 点踩 |
    ///
    /// # 文档
    /// [点踩评论](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/comment/action.md#点踩评论)
    pub async fn comment_dislike(
        &self,
        r#type: CommentType,
        oid: u64,
        rpid: u64,
        action: u8
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let params = [
            ("type", (r#type as u32).to_string()),
            ("oid", oid.to_string()),
            ("rpid", rpid.to_string()),
            ("action", action.to_string()), // 0 取消，1 点踩
            ("csrf", csrf.to_string()),
        ];

        self
            .post("https://api.bilibili.com/x/v2/reply/hate")
            .form(&params)
            .send_bpi("点踩评论").await
    }
    /// 删除评论
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `type` | CommentType | 评论区类型 |
    /// | `oid` | u64 | 对象 ID |
    /// | `rpid` | u64 | 评论 rpid |
    ///
    /// # 文档
    /// [删除评论](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/comment/action.md#删除评论)
    pub async fn comment_delete(
        &self,
        r#type: CommentType,
        oid: u64,
        rpid: u64
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let params = [
            ("type", (r#type as u32).to_string()),
            ("oid", oid.to_string()),
            ("rpid", rpid.to_string()),
            ("csrf", csrf.to_string()),
        ];

        self
            .post("https://api.bilibili.com/x/v2/reply/del")
            .form(&params)
            .send_bpi("删除评论").await
    }
    /// 置顶评论
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `type` | CommentType | 评论区类型 |
    /// | `oid` | u64 | 对象 ID |
    /// | `rpid` | u64 | 评论 rpid |
    /// | `action` | u8 | 操作：0 取消置顶，1 置顶 |
    ///
    /// # 文档
    /// [置顶评论](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/comment/action.md#置顶评论)
    pub async fn comment_top(
        &self,
        r#type: CommentType,
        oid: u64,
        rpid: u64,
        action: u8
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let params = [
            ("type", (r#type as u32).to_string()),
            ("oid", oid.to_string()),
            ("rpid", rpid.to_string()),
            ("action", action.to_string()), // 0 取消置顶，1 置顶
            ("csrf", csrf.to_string()),
        ];

        self
            .post("https://api.bilibili.com/x/v2/reply/top")
            .form(&params)
            .send_bpi("置顶评论").await
    }

    /// 举报评论
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `type` | CommentType | 评论区类型 |
    /// | `oid` | u64 | 对象 ID |
    /// | `rpid` | u64 | 评论 rpid |
    /// | `reason` | ReportReason | 举报原因 |
    /// | `content` | `Option<&str>` | 举报内容，可选 |
    ///
    /// # 文档
    /// [举报评论](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/comment/action.md#举报评论)
    pub async fn comment_report(
        &self,
        r#type: CommentType,
        oid: u64,
        rpid: u64,
        reason: ReportReason,
        content: Option<&str>
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let mut params = vec![
            ("type", (r#type as u32).to_string()),
            ("oid", oid.to_string()),
            ("rpid", rpid.to_string()),
            ("reason", (reason as u32).to_string()),
            ("csrf", csrf)
        ];
        if let Some(c) = content {
            params.push(("content", c.to_string()));
        }

        self
            .post("https://api.bilibili.com/x/v2/reply/report")
            .form(&params)
            .send_bpi("举报评论").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::time::{ SystemTime, UNIX_EPOCH };
    use tokio::time;

    const TEST_AID: u64 = 851944245;

    /// 测试辅助函数：添加评论并返回rpid
    async fn add_test_comment() -> Result<u64, BpiError> {
        let bpi = BpiClient::new();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // 简单伪随机：用当前秒对一个常数取模再加偏移
        let random_secs = (now % 1_000_000) + 1_600_000_000;
        let resp = bpi.comment_add(
            CommentType::Video,
            TEST_AID,
            &random_secs.to_string(),
            None,
            None
        ).await?;

        let data = resp.into_data()?;
        Ok(data.rpid)
    }

    /// 测试辅助函数：删除评论
    async fn delete_test_comment(rpid: u64) -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        bpi.comment_delete(CommentType::Video, TEST_AID, rpid).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_comment_like() -> Result<(), BpiError> {
        let rpid = add_test_comment().await?;
        time::sleep(Duration::from_secs(3)).await;

        let bpi = BpiClient::new();
        let resp = bpi.comment_like(CommentType::Video, TEST_AID, rpid, 1).await?;
        assert_eq!(resp.code, 0);

        time::sleep(Duration::from_secs(3)).await;
        delete_test_comment(rpid).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_comment_dislike() -> Result<(), BpiError> {
        let rpid = add_test_comment().await?;
        time::sleep(Duration::from_secs(3)).await;

        let bpi = BpiClient::new();
        let resp = bpi.comment_dislike(CommentType::Video, TEST_AID, rpid, 1).await?;

        assert_eq!(resp.code, 0);

        time::sleep(Duration::from_secs(3)).await;
        delete_test_comment(rpid).await?;

        Ok(())
    }
}
