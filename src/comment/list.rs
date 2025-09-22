//! 评论查询 API
//!
//! 参考文档：https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/comment/list.md

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

use super::types::{
    Comment, // 评论条目对象，包含评论内容、发送者信息、回复等
    Config,
    Control,
    Cursor,
    PageInfo,
    Top,
    Upper,
};

/// 通用的评论列表响应
pub type CommentListResponse = BpiResponse<CommentListData>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentListData {
    pub page: Option<PageInfo>,
    pub cursor: Option<Cursor>,        // 评论列表游标
    pub replies: Option<Vec<Comment>>, // 评论列表，禁用时为 null
    pub top: Option<Top>,              // 评论列表顶部信息
    pub top_replies: Option<Vec<Comment>>,
    pub effects: Option<serde_json::Value>,
    pub assist: Option<u64>,    // 待确认
    pub blacklist: Option<u64>, // 待确认
    pub vote: Option<u64>,      // 投票评论？
    pub config: Option<Config>, // 评论区显示控制
    pub upper: Option<Upper>,   // 置顶评论

    pub control: Option<Control>, // 评论区输入属性
    pub note: Option<u32>,
    pub cm_info: Option<serde_json::Value>, // 评论区相关信息

                                            // pub page: Option<PageInfo>, // 页信息
                                            // pub hots: Option<Vec<Comment>>, // 热评列表，禁用时为 null
                                            // pub notice: Option<Notice>, // 评论区公告信息，无效时为 null
                                            // pub mode: Option<u64>, // 评论区类型 id
                                            // pub support_mode: Option<Vec<u64>>, // 评论区支持的类型 id
                                            // pub folder: Option<Folder>, // 折叠相关信息
                                            // pub lottery_card: Option<()>, // 待确认
                                            // pub show_bvid: Option<bool>, // 是否显示 bvid
}

/// 公告信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notice {
    pub content: Option<String>,
    pub id: Option<u64>,
    pub link: Option<String>,
    pub title: Option<String>,
}

type HotCommentResponse = BpiResponse<HotCommentData>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HotCommentData {
    pub page: HotCommentPage,
    pub replies: Vec<Comment>, // 热评列表
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HotCommentPage {
    pub acount: i64, // 总评论数
    pub count: i64,  // 热评数
    pub num: i32,    // 当前页码
    pub size: i32,   // 每页项数
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CountData {
    count: u64,
}

impl BpiClient {
    /// 获取评论主列表
    ///
    /// 获取指定评论区的评论列表，支持分页和排序。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `type` | i32 | 评论区类型 |
    /// | `oid` | i64 | 对象 ID |
    /// | `pn` | Option<i32> | 页码，可选，默认为 1 |
    /// | `ps` | Option<i32> | 每页条数，可选，范围 1-20 |
    /// | `sort` | Option<i32> | 排序方式，可选：0 按时间，1 按点赞，2 按回复数 |
    /// | `nohot` | Option<i32> | 是否不显示热评，可选：0 显示，1 不显示 |
    ///
    /// # 文档
    /// [获取评论主列表](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/comment/list.md#获取评论主列表)
    pub async fn comment_list(
        &self,
        r#type: i32,
        oid: i64,
        pn: Option<i32>,
        ps: Option<i32>,
        sort: Option<i32>,
        nohot: Option<i32>,
    ) -> Result<CommentListResponse, BpiError> {
        let mut params = vec![("type", r#type.to_string()), ("oid", oid.to_string())];
        if let Some(pn) = pn {
            params.push(("pn", pn.to_string()));
        }
        if let Some(ps) = ps {
            params.push(("ps", ps.to_string()));
        }
        if let Some(sort) = sort {
            params.push(("sort", sort.to_string()));
        }
        if let Some(nohot) = nohot {
            params.push(("nohot", nohot.to_string()));
        }

        self.get("https://api.bilibili.com/x/v2/reply")
            .query(&params)
            .send_bpi("获取评论主列表")
            .await
    }

    /// 获取某条根评论下的子评论列表
    ///
    /// 获取指定根评论下的所有子评论，支持分页。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `type` | i32 | 评论区类型 |
    /// | `oid` | i64 | 对象 ID |
    /// | `root` | i64 | 根评论 rpid |
    /// | `pn` | Option<i32> | 页码，可选，默认为 1 |
    /// | `ps` | Option<i32> | 每页条数，可选 |
    ///
    /// # 文档
    /// [获取子评论列表](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/comment/list.md#获取子评论列表)
    pub async fn comment_replies(
        &self,
        r#type: i32,
        oid: i64,
        root: i64,
        pn: Option<i32>,
        ps: Option<i32>,
    ) -> Result<CommentListResponse, BpiError> {
        let mut params = vec![
            ("type", r#type.to_string()),
            ("oid", oid.to_string()),
            ("root", root.to_string()),
        ];
        if let Some(pn) = pn {
            params.push(("pn", pn.to_string()));
        }
        if let Some(ps) = ps {
            params.push(("ps", ps.to_string()));
        }

        self.get("https://api.bilibili.com/x/v2/reply/reply")
            .query(&params)
            .send_bpi("获取子评论列表")
            .await
    }

    /// 获取评论区热评列表
    ///
    /// 获取指定根评论下的热评列表，支持分页。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `type` | i32 | 评论区类型 |
    /// | `oid` | i64 | 对象 ID |
    /// | `root` | i64 | 根评论 rpid |
    /// | `pn` | Option<i32> | 页码，可选，默认为 1 |
    /// | `ps` | Option<i32> | 每页条数，可选 |
    ///
    /// # 文档
    /// [获取热评列表](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/comment/list.md#获取热评列表)
    pub async fn comment_hot(
        &self,
        r#type: i32,
        oid: i64,
        root: i64,
        pn: Option<i32>,
        ps: Option<i32>,
    ) -> Result<HotCommentResponse, BpiError> {
        let mut params = vec![
            ("type", r#type.to_string()),
            ("oid", oid.to_string()),
            ("root", root.to_string()),
        ];
        if let Some(pn) = pn {
            params.push(("pn", pn.to_string()));
        }
        if let Some(ps) = ps {
            params.push(("ps", ps.to_string()));
        }

        self.get("https://api.bilibili.com/x/v2/reply/hot")
            .query(&params)
            .send_bpi("获取评论区热评列表")
            .await
    }
    /// 获取评论区评论总数
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `type` | i32 | 评论区类型 |
    /// | `oid` | i64 | 对象 ID |
    ///
    /// # 文档
    /// [获取评论总数](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/comment/list.md#获取评论总数)
    pub async fn comment_count(
        &self,
        r#type: i32,
        oid: i64,
    ) -> Result<BpiResponse<CountData>, BpiError> {
        let params = [("type", r#type.to_string()), ("oid", oid.to_string())];
        self.get("https://api.bilibili.com/x/v2/reply/count")
            .query(&params)
            .send_bpi("获取评论区评论总数")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    const TEST_TYPE: i32 = 1;
    const TEST_OID: i64 = 23199;
    const TEST_ROOT_RPID: i64 = 2554491176;

    #[tokio::test]
    async fn test_comment_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let result = bpi
            .comment_list(TEST_TYPE, TEST_OID, Some(1), Some(5), Some(0), Some(0))
            .await?;
        let data = result.into_data()?;
        info!("总评论数: {}", data.replies.unwrap().len());

        Ok(())
    }

    #[tokio::test]
    async fn test_comment_replies() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let result = bpi
            .comment_replies(TEST_TYPE, TEST_OID, TEST_ROOT_RPID, Some(1), Some(5))
            .await?;
        let data = result.into_data()?;
        info!("总评论数: {}", data.replies.unwrap().len());

        Ok(())
    }

    #[tokio::test]
    async fn test_comment_hot() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let root_rpid = 654321;

        let result = bpi
            .comment_hot(TEST_TYPE, TEST_OID, root_rpid, Some(1), Some(5))
            .await?;
        let data = result.into_data()?;

        info!("热评数量: {}", data.replies.len());
        for comment in data.replies.iter() {
            info!("热评内容: {}", comment.content.message);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_comment_count() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let result = bpi.comment_count(TEST_TYPE, TEST_OID).await?;

        let data = result.into_data()?;
        info!("评论总数: {}", data.count);

        Ok(())
    }
}
