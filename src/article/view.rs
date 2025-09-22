//! 专栏内容
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/article/view.md

use super::models::{ArticleAuthor, ArticleCategory, ArticleMedia, ArticleStats};
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 专栏内容响应类型
pub type ArticleViewResponse = BpiResponse<ArticleViewData>;

/// 专栏内容数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleViewData {
    /// 操作ID?
    pub act_id: i64,
    /// 应用时间?
    pub apply_time: String,
    /// 属性位?
    pub attributes: Option<i32>,
    /// 授权码?
    #[serde(rename = "authenMark")]
    pub authen_mark: Option<serde_json::Value>,
    /// 作者信息
    pub author: ArticleAuthor,
    /// 文章头图URL 空则为无
    pub banner_url: String,
    /// 专栏分类信息 首项为主分区, 第二项为子分区
    pub categories: Vec<ArticleCategory>,
    /// 专栏分类信息 子分区
    pub category: ArticleCategory,
    /// 检查状态?
    pub check_state: i32,
    /// 检查时间?
    pub check_time: String,
    /// 文章内容 type字段为0为HTML, 3为JSON
    pub content: String,
    /// 内容图片列表?
    pub content_pic_list: Option<serde_json::Value>,
    /// 封面视频AV号 0为无视频
    pub cover_avid: i64,
    /// 创建时间 UNIX秒级时间戳
    pub ctime: i64,
    /// 争议信息?
    pub dispute: Option<serde_json::Value>,
    /// 动态opus id
    pub dyn_id_str: String,
    /// 动态信息? 可能不存在
    pub dynamic: Option<String>,
    /// 专栏文章ID
    pub id: i64,
    /// 图片URL
    pub image_urls: Vec<String>,
    /// 是否喜欢?
    pub is_like: bool,
    /// 关键词 以逗号分隔
    pub keywords: String,
    /// 文集信息
    pub list: Option<ArticleList>,
    /// 媒体信息?
    pub media: ArticleMedia,
    /// 修改时间 UNIX秒级时间戳
    pub mtime: i64,
    /// opus信息 当type字段为3时存在, 包含了更加详细的富文本信息
    pub opus: Option<ArticleOpus>,
    /// 原始图片URL
    pub origin_image_urls: Vec<String>,
    /// 原始模板ID?
    pub origin_template_id: i32,
    /// 是否原创 0: 非原创 1: 原创
    pub original: i32,
    /// 仅自己可见
    pub private_pub: i32,
    /// 发布时间 UNIX秒级时间戳
    pub publish_time: i64,
    /// 是否允许转载 0: 不允许 1: 允许规范转载
    pub reprint: i32,
    /// 专栏状态
    pub state: i32,
    /// 统计数据
    pub stats: ArticleStats,
    /// 专栏开头部分内容 纯文本
    pub summary: String,
    /// 专栏标签
    pub tags: Vec<ArticleTag>,
    /// 模板ID?
    pub template_id: i32,
    /// 专栏标题
    pub title: String,
    /// 封面食品信息?
    pub top_video_info: Option<serde_json::Value>,
    /// 作者总文章数
    pub total_art_num: i64,
    /// 类型?
    pub r#type: i32,
    /// 版本ID?
    pub version_id: i64,
    /// 文章总词数
    pub words: i64,
}

/// 作者VIP信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorVip {
    /// VIP类型
    pub r#type: i32,
    /// VIP状态
    pub status: i32,
    /// 到期时间
    pub due_date: i64,
    /// 支付类型
    pub vip_pay_type: i32,
    /// 主题类型
    pub theme_type: i32,
    /// 标签
    pub label: Option<serde_json::Value>,
}

/// 专栏文集信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleList {
    /// 文集ID
    pub id: i64,
    /// 文集名称
    pub name: String,
    /// 文集图片
    pub image_url: String,
    /// 文集更新时间
    pub update_time: i64,
    /// 文集创建时间
    pub ctime: i64,
    /// 文集发布时间
    pub publish_time: i64,
    /// 文集简介
    pub summary: String,
    /// 文集字数
    pub words: i64,
    /// 文集阅读量
    pub read: i64,
    /// 文集内文章数量
    pub articles_count: i32,
    /// 文集状态
    pub state: i32,
    /// 文集原因
    pub reason: String,
    /// 文集申请时间
    pub apply_time: String,
    /// 文集审核时间
    pub check_time: String,
}

/// 专栏标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleTag {
    /// 标签ID
    pub tid: i32,
    /// 标签名称
    pub name: String,
    // /// 标签类型
    // pub r#type: i32,
}

/// 专栏Opus信息（富文本内容）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleOpus {
    /// 以JSON呈现的文本内容
    pub ops: Vec<OpusOperation>,
}

/// Opus操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpusOperation {
    /// 属性
    pub attribute: Option<OpusAttribute>,
    /// 插入内容
    pub insert: OpusInsert,
}

/// Opus属性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpusAttribute {
    /// 文字对齐
    pub align: Option<String>,
    /// 块级引用
    pub blockquote: Option<bool>,
    /// 加粗
    pub bold: Option<bool>,
    /// 类名
    pub class: Option<String>,
    /// 颜色
    pub color: Option<String>,
    /// 标题级别
    pub header: Option<i32>,
    /// 删除线
    pub strike: Option<bool>,
    /// 站内链接
    pub link: Option<String>,
    /// 斜体
    pub italic: Option<bool>,
    /// 列表
    pub list: Option<String>,
}

/// Opus插入内容
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpusInsert {
    /// 文本内容
    Text(String),
    /// 富文本内容
    Rich(OpusRichInsert),
}

/// Opus富文本插入内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpusRichInsert {
    /// 原生图片
    pub native_image: Option<OpusImage>,
    /// 分割线
    pub cut_off: Option<OpusCutOff>,
    /// 视频卡片
    pub video_card: Option<OpusVideoCard>,
    /// 专栏卡片
    pub article_card: Option<OpusArticleCard>,
    /// 投票卡片
    pub vote_card: Option<OpusVoteCard>,
    /// 直播卡片
    pub live_card: Option<OpusLiveCard>,
}

/// Opus图片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpusImage {
    /// 图像的备用文本描述
    pub alt: String,
    /// 图像的URL
    pub url: String,
    /// 图像的宽度
    pub width: i32,
    /// 图像的高度
    pub height: i32,
    /// 图像的文件大小
    pub size: i64,
    /// 图像状态
    pub status: String,
}

/// Opus分割线
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpusCutOff {
    /// 类型
    pub r#type: String,
    /// 分割线图片URL
    pub url: String,
}

/// Opus视频卡片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpusVideoCard {
    /// 备用文本
    pub alt: String,
    /// 卡片高度
    pub height: i32,
    /// 视频ID
    pub id: String,
    /// 大小
    pub size: Option<serde_json::Value>,
    /// 状态
    pub status: String,
    /// 类型ID
    pub tid: f64,
    /// 卡片图片URL
    pub url: String,
    /// 卡片宽度
    pub width: i32,
}

/// Opus专栏卡片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpusArticleCard {
    /// 备用文本
    pub alt: String,
    /// 卡片高度
    pub height: i32,
    /// 文章ID
    pub id: String,
    /// 大小
    pub size: Option<serde_json::Value>,
    /// 状态
    pub status: String,
    /// 类型ID
    pub tid: i32,
    /// 卡片图片URL
    pub url: String,
    /// 卡片宽度
    pub width: i32,
}

/// Opus投票卡片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpusVoteCard {
    /// 备用文本
    pub alt: String,
    /// 卡片高度
    pub height: i32,
    /// 投票ID
    pub id: String,
    /// 大小
    pub size: Option<serde_json::Value>,
    /// 状态
    pub status: String,
    /// 类型ID
    pub tid: i32,
    /// 卡片图片URL
    pub url: String,
    /// 卡片宽度
    pub width: i32,
}

/// Opus直播卡片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpusLiveCard {
    /// 备用文本
    pub alt: String,
    /// 卡片高度
    pub height: i32,
    /// 直播间ID
    pub id: String,
    /// 大小
    pub size: Option<serde_json::Value>,
    /// 状态
    pub status: String,
    /// 类型ID
    pub tid: i32,
    /// 卡片图片URL
    pub url: String,
    /// 卡片宽度
    pub width: i32,
}

impl BpiClient {
    /// 获取专栏正文内容
    ///
    /// # 参数
    /// * `id` - 专栏文章ID (必要)
    /// * `gaia_source` - 来源，默认为"main_web" (可选)
    pub async fn article_view(&self, id: i64) -> Result<ArticleViewResponse, BpiError> {
        let params = vec![
            ("id", id.to_string()),
            ("gaia_source", "main_web".to_string()),
        ];
        let params = self.get_wbi_sign2(params).await?;

        let result: ArticleViewResponse = self
            .get("https://api.bilibili.com/x/article/view")
            .query(&params)
            .send_bpi("获取专栏正文内容")
            .await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_article_view() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let cvid = 2;

        let result = bpi.article_view(cvid).await?;

        let data = result.data.unwrap();
        assert!(!data.title.is_empty());
        assert!(!data.content.is_empty());
        assert!(!data.author.name.is_empty());

        Ok(())
    }
}
