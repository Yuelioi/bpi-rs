//! 卡片信息
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/article/card.md)

use super::models::{ ArticleAuthor, ArticleCategory, ArticleMedia, ArticleStats };
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 卡片信息响应类型
pub type CardResponse = BpiResponse<std::collections::HashMap<String, CardItem>>;

/// 卡片项目（可以是视频、专栏或直播间）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CardItem {
    /// 视频卡片
    Video(VideoCard),
    /// 专栏卡片
    Article(ArticleCard),
    /// 直播间卡片
    Live(LiveCard),

    /// 未知卡片类型
    Unknown(serde_json::Value),
}

/// 视频卡片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoCard {
    /// 视频aid
    pub aid: i64,
    /// 视频bvid
    pub bvid: String,
    /// 视频cid
    pub cid: i64,
    /// 版权信息
    pub copyright: i32,
    /// 封面图片
    pub pic: String,
    /// 创建时间
    pub ctime: i64,
    /// 视频描述
    pub desc: String,
    /// 视频尺寸信息
    pub dimension: VideoDimension,
    /// 视频时长
    pub duration: i64,
    /// 动态内容
    pub dynamic: String,
    /// UP主信息
    pub owner: VideoOwner,
    /// 发布时间
    pub pubdate: i64,
    /// 视频权限
    pub rights: VideoRights,
    /// 短链接
    pub short_link_v2: String,
    /// 视频统计信息
    pub stat: VideoStat,
    /// 视频状态
    pub state: i32,
    /// 分区ID
    pub tid: i32,
    /// 视频标题
    pub title: String,
    /// 分区名称
    pub tname: String,
    /// 分P数量
    pub videos: i32,
    /// VT开关
    pub vt_switch: bool,
}

/// 视频尺寸信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDimension {
    /// 高度
    pub height: i32,
    /// 旋转角度
    pub rotate: i32,
    /// 宽度
    pub width: i32,
}

/// 视频UP主信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoOwner {
    /// UP主头像
    pub face: String,
    /// UP主mid
    pub mid: i64,
    /// UP主昵称
    pub name: String,
}

/// 视频权限信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoRights {
    /// 是否付费
    pub arc_pay: i32,
    /// 是否自动播放
    pub autoplay: i32,
    /// 是否可充电
    pub bp: i32,
    /// 是否可下载
    pub download: i32,
    /// 是否可充电
    pub elec: i32,
    /// 是否高清
    pub hd5: i32,
    /// 是否合作视频
    pub is_cooperation: i32,
    /// 是否电影
    pub movie: i32,
    /// 是否无背景
    pub no_background: i32,
    /// 是否禁止转载
    pub no_reprint: i32,
    /// 是否付费
    pub pay: i32,
    /// 是否付费观看
    pub pay_free_watch: i32,
    /// 是否UGC付费
    pub ugc_pay: i32,
    /// 是否UGC付费预览
    pub ugc_pay_preview: i32,
}

/// 视频统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoStat {
    /// 视频aid
    pub aid: i64,
    /// 投币数
    pub coin: i64,
    /// 弹幕数
    pub danmaku: i64,
    /// 点踩数
    pub dislike: i64,
    /// 收藏数
    pub favorite: i64,
    /// 历史排名
    pub his_rank: i32,
    /// 点赞数
    pub like: i64,
    /// 当前排名
    pub now_rank: i32,
    /// 评论数
    pub reply: i64,
    /// 分享数
    pub share: i64,
    /// 播放数
    pub view: i64,
    /// VT值
    pub vt: i32,
    /// VV值
    pub vv: i32,
}

/// 专栏卡片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleCard {
    /// 活动ID
    pub act_id: i64,
    /// 申请时间
    pub apply_time: String,
    /// 属性
    pub attributes: i32,
    /// 认证标记
    #[serde(rename = "authenMark")]
    pub authen_mark: Option<serde_json::Value>,
    /// 作者信息
    pub author: ArticleAuthor,
    /// 横幅URL
    pub banner_url: String,
    /// 分类列表
    pub categories: Vec<ArticleCategory>,
    /// 主分类
    pub category: ArticleCategory,
    /// 审核状态
    pub check_state: i32,
    /// 审核时间
    pub check_time: String,
    /// 内容图片列表
    pub content_pic_list: Option<serde_json::Value>,
    /// 封面视频ID
    pub cover_avid: i64,
    /// 创建时间
    pub ctime: i64,
    /// 争议信息
    pub dispute: Option<serde_json::Value>,
    /// 动态内容
    pub dynamic: String,
    /// 专栏ID
    pub id: i64,
    /// 图片URL列表
    pub image_urls: Vec<String>,
    /// 是否点赞
    pub is_like: bool,
    /// 文集信息
    pub list: Option<ArticleList>,
    /// 媒体信息
    pub media: ArticleMedia,
    /// 修改时间
    pub mtime: i64,
    /// 原始图片URL列表
    pub origin_image_urls: Vec<String>,
    /// 原始模板ID
    pub origin_template_id: i32,
    /// 是否原创
    pub original: i32,
    /// 是否私密发布
    pub private_pub: i32,
    /// 发布时间
    pub publish_time: i64,
    /// 是否转载
    pub reprint: i32,
    /// 状态
    pub state: i32,
    /// 统计信息
    pub stats: ArticleStats,
    /// 摘要
    pub summary: String,
    /// 模板ID
    pub template_id: i32,
    /// 标题
    pub title: String,
    /// 顶部视频信息
    pub top_video_info: Option<serde_json::Value>,
    /// 类型
    pub r#type: i32,
    /// 字数
    pub words: i64,
}

/// 作者VIP信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorVip {
    /// 头像订阅
    pub avatar_subscript: i32,
    /// 到期时间
    pub due_date: i64,
    /// 标签信息
    pub label: VipLabel,
    /// 昵称颜色
    pub nickname_color: String,
    /// VIP状态
    pub status: i32,
    /// 主题类型
    pub theme_type: i32,
    /// VIP类型
    pub r#type: i32,
    /// 支付类型
    pub vip_pay_type: i32,
}

/// VIP标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VipLabel {
    /// 标签主题
    pub label_theme: String,
    /// 标签路径
    pub path: String,
    /// 标签文本
    pub text: String,
}

/// 专栏文集信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleList {
    /// 申请时间
    pub apply_time: String,
    /// 文章数量
    pub articles_count: i32,
    /// 审核时间
    pub check_time: String,
    /// 创建时间
    pub ctime: i64,
    /// 文集ID
    pub id: i64,
    /// 文集图片
    pub image_url: String,
    /// 作者ID
    pub mid: i64,
    /// 文集名称
    pub name: String,
    /// 发布时间
    pub publish_time: i64,
    /// 阅读量
    pub read: i64,
    /// 原因
    pub reason: String,
    /// 状态
    pub state: i32,
    /// 摘要
    pub summary: String,
    /// 更新时间
    pub update_time: i64,
    /// 字数
    pub words: i64,
}

/// 直播间卡片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveCard {
    /// 分区完整名称
    pub area_v2_name: String,
    /// 直播封面
    pub cover: String,
    /// 主播头像
    pub face: String,
    /// 直播状态
    pub live_status: i32,
    /// 在线人数
    pub online: i64,
    /// 挂件RU
    pub pendent_ru: String,
    /// 挂件RU颜色
    pub pendent_ru_color: String,
    /// 挂件RU图片
    pub pendent_ru_pic: String,
    /// 角色
    pub role: i32,
    /// 直播间长ID
    pub room_id: i64,
    /// 直播间标题
    pub title: String,
    /// 主播UID
    pub uid: i64,
    /// 主播用户名
    pub uname: String,
}

impl BpiClient {
    /// 获取专栏显示卡片信息
    ///
    /// # 参数
    /// | 名称   | 类型    | 说明                                                                 |
    /// | ------ | ------- | -------------------------------------------------------------------- |
    /// | `ids`  | String  | 被查询的 id 列表，以逗号分隔；可填视频完整 AV/BV 号、专栏 CV 号、直播间长/短 lv 号 |
    ///
    /// # 文档
    /// [获取专栏显示卡片信息](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/article/card.md#获取专栏显示卡片信息)
    pub async fn article_cards(&self, ids: &str) -> Result<CardResponse, BpiError> {
        let params = vec![("ids", ids.to_string()), ("web_location", "333.1305".to_string())];

        let params = self.get_wbi_sign2(params).await?;

        let result: CardResponse = self
            .get("https://api.bilibili.com/x/article/cards")
            .with_bilibili_headers()
            .query(&params)
            .send_bpi("获取专栏显示卡片信息").await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_article_cards() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let ids = "av2,cv1,cv2";

        let result = bpi.article_cards(ids).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        Ok(())
    }
}
