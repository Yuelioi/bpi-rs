//! 创作中心统计数据 API
//!
//! 参考文档：https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/statistics&data.md

use serde::{ Deserialize, Serialize };

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

/// UP主视频状态数据
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UpStatData {
    /// 新增投币数
    #[serde(rename = "inc_coin")]
    pub inc_coin: i64,

    /// 新增充电数
    #[serde(rename = "inc_elec")]
    pub inc_elec: i64,

    /// 新增收藏数
    #[serde(rename = "inc_fav")]
    pub inc_fav: i64,

    /// 新增点赞数
    #[serde(rename = "inc_like")]
    pub inc_like: i64,

    /// 新增分享数
    #[serde(rename = "inc_share")]
    pub inc_share: i64,

    /// 新增播放数
    #[serde(rename = "incr_click")]
    pub incr_click: i64,

    /// 新增弹幕数
    #[serde(rename = "incr_dm")]
    pub incr_dm: i64,

    /// 新增粉丝数
    #[serde(rename = "incr_fans")]
    pub incr_fans: i64,

    /// 新增评论数
    #[serde(rename = "incr_reply")]
    pub incr_reply: i64,

    /// 总计播放数
    #[serde(rename = "total_click")]
    pub total_click: i64,

    /// 总计投币数
    #[serde(rename = "total_coin")]
    pub total_coin: i64,

    /// 总计弹幕数
    #[serde(rename = "total_dm")]
    pub total_dm: i64,

    /// 总计充电数
    #[serde(rename = "total_elec")]
    pub total_elec: i64,

    /// 总计粉丝数
    #[serde(rename = "total_fans")]
    pub total_fans: i64,

    /// 总计收藏数
    #[serde(rename = "total_fav")]
    pub total_fav: i64,

    /// 总计点赞数
    #[serde(rename = "total_like")]
    pub total_like: i64,

    /// 总计评论数
    #[serde(rename = "total_reply")]
    pub total_reply: i64,

    /// 总计分享数
    #[serde(rename = "total_share")]
    pub total_share: i64,
}

/// 单个视频对比数据
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveCompareItem {
    /// av号
    pub aid: i64,
    /// bv号
    pub bvid: String,
    /// 封面 url
    pub cover: String,
    /// 标题
    pub title: String,
    /// 发布时间（秒级时间戳）
    pub pubtime: i64,
    /// 视频长度（秒）
    pub duration: i64,
    pub stat: Stat,
    #[serde(rename = "is_only_self")]
    pub is_only_self: bool,
    #[serde(rename = "hour_stat")]
    pub hour_stat: HourStat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    #[serde(rename = "not_ready_field")]
    pub not_ready_field: serde_json::Value,
    /// 播放数
    pub play: i64,
    pub vt: i64,
    // ===== 百分比类指标，B站返回一般是整数，100 表示 1% =====
    /// 完播比
    #[serde(rename = "full_play_ratio")]
    pub full_play_ratio: i64,
    /// 游客播放数占比
    #[serde(rename = "play_viewer_rate")]
    pub play_viewer_rate: i64,
    #[serde(rename = "play_viewer_rate_med")]
    pub play_viewer_rate_med: i64,
    /// 粉丝观看率
    #[serde(rename = "play_fan_rate")]
    pub play_fan_rate: i64,
    #[serde(rename = "play_fan_rate_med")]
    pub play_fan_rate_med: i64,
    #[serde(rename = "active_fans_rate")]
    pub active_fans_rate: i64,
    #[serde(rename = "active_fans_med")]
    pub active_fans_med: i64,
    /// 封标点击率
    #[serde(rename = "tm_rate")]
    pub tm_rate: i64,
    /// 自己平均封标点击率
    #[serde(rename = "tm_rate_med")]
    pub tm_rate_med: i64,
    /// 同类up粉丝封标点击率
    #[serde(rename = "tm_fan_simi_rate_med")]
    pub tm_fan_simi_rate_med: i64,
    /// 同类up游客封标点击率
    #[serde(rename = "tm_viewer_simi_rate_med")]
    pub tm_viewer_simi_rate_med: i64,
    /// 粉丝封标点击率
    #[serde(rename = "tm_fan_rate")]
    pub tm_fan_rate: i64,
    /// 游客封标点击率
    #[serde(rename = "tm_viewer_rate")]
    pub tm_viewer_rate: i64,
    /// 封标点击率超过n%同类稿件
    #[serde(rename = "tm_pass_rate")]
    pub tm_pass_rate: i64,
    /// 粉丝封标点击率超过n%同类稿件
    #[serde(rename = "tm_fan_pass_rate")]
    pub tm_fan_pass_rate: i64,
    /// 游客封标点击率超过n%同类稿件
    #[serde(rename = "tm_viewer_pass_rate")]
    pub tm_viewer_pass_rate: i64,
    /// 3秒退出率
    #[serde(rename = "crash_rate")]
    pub crash_rate: i64,
    #[serde(rename = "crash_rate_med")]
    pub crash_rate_med: i64,
    /// 同类up粉丝3秒退出率
    #[serde(rename = "crash_fan_simi_rate_med")]
    pub crash_fan_simi_rate_med: i64,
    /// 同类up游客3秒退出率
    #[serde(rename = "crash_viewer_simi_rate_med")]
    pub crash_viewer_simi_rate_med: i64,
    /// 粉丝3秒退出率
    #[serde(rename = "crash_fan_rate")]
    pub crash_fan_rate: i64,
    /// 游客3秒退出率
    #[serde(rename = "crash_viewer_rate")]
    pub crash_viewer_rate: i64,
    /// 互动率
    #[serde(rename = "interact_rate")]
    pub interact_rate: i64,
    #[serde(rename = "interact_rate_med")]
    pub interact_rate_med: i64,
    /// 同类up粉丝互动率
    #[serde(rename = "interact_fan_simi_rate_med")]
    pub interact_fan_simi_rate_med: i64,
    /// 同类up游客互动率
    #[serde(rename = "interact_viewer_simi_rate_med")]
    pub interact_viewer_simi_rate_med: i64,
    /// 粉丝互动率
    #[serde(rename = "interact_fan_rate")]
    pub interact_fan_rate: i64,
    /// 游客互动率
    #[serde(rename = "interact_viewer_rate")]
    pub interact_viewer_rate: i64,
    /// 平均播放时间（目前总是0）
    #[serde(rename = "avg_play_time")]
    pub avg_play_time: i64,
    #[serde(rename = "avg_play_time_int")]
    pub avg_play_time_int: i64,
    /// 涨粉
    #[serde(rename = "total_new_attention_cnt")]
    pub total_new_attention_cnt: i64,
    /// 播转粉率
    #[serde(rename = "play_trans_fan_rate")]
    pub play_trans_fan_rate: i64,
    /// 其他up平均播转粉率
    #[serde(rename = "play_trans_fan_rate_med")]
    pub play_trans_fan_rate_med: i64,
    /// 点赞数
    pub like: i64,
    /// 评论数
    pub comment: i64,
    /// 弹幕数
    pub dm: i64,
    /// 收藏数
    pub fav: i64,
    /// 投币数
    pub coin: i64,
    /// 分享数
    pub share: i64,
    #[serde(rename = "unfollow")]
    pub unfollow: i64,
    #[serde(rename = "tm_star")]
    pub tm_star: i64,
    #[serde(rename = "tm_viewer_star")]
    pub tm_viewer_star: i64,
    #[serde(rename = "tm_fan_star")]
    pub tm_fan_star: i64,
    #[serde(rename = "crash_p50")]
    pub crash_p50: i64,
    #[serde(rename = "crash_viewer_p50")]
    pub crash_viewer_p50: i64,
    #[serde(rename = "crash_fan_p50")]
    pub crash_fan_p50: i64,
    #[serde(rename = "interact_p50")]
    pub interact_p50: i64,
    #[serde(rename = "interact_viewer_p50")]
    pub interact_viewer_p50: i64,
    #[serde(rename = "interact_fan_p50")]
    pub interact_fan_p50: i64,
    #[serde(rename = "play_trans_fan_p50")]
    pub play_trans_fan_p50: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HourStat {
    #[serde(rename = "not_ready_field")]
    pub not_ready_field: serde_json::Value,
    /// 播放数
    pub play: i64,
    pub vt: i64,
    /// 点赞数
    pub like: i64,
    /// 评论数
    pub comment: i64,
    /// 弹幕数
    pub dm: i64,
    /// 收藏数
    pub fav: i64,
    /// 投币数
    pub coin: i64,
    /// 分享数
    pub share: i64,
    /// 封标点击率超过n%同类稿件
    #[serde(rename = "tm_pass_rate")]
    pub tm_pass_rate: i64,
    /// 互动率
    #[serde(rename = "interact_rate")]
    pub interact_rate: i64,
    #[serde(rename = "tm_star")]
    pub tm_star: i64,
}

/// UP主视频数据比较
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ArchiveCompareData {
    pub list: Vec<ArchiveCompareItem>,
}

/// UP主专栏状态数据
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UpArticleStatData {
    /// 总计阅读数
    pub view: i64,
    /// 总计评论数
    pub reply: i64,
    /// 总计点赞数
    pub like: i64,
    /// 总计投币数
    pub coin: i64,
    /// 总计收藏数
    pub fav: i64,
    /// 总计分享数
    pub share: i64,
    /// 新增阅读数
    #[serde(rename = "incr_view")]
    pub incr_view: i64,
    /// 新增评论数
    #[serde(rename = "incr_reply")]
    pub incr_reply: i64,
    /// 新增点赞数
    #[serde(rename = "incr_like")]
    pub incr_like: i64,
    /// 新增投币数
    #[serde(rename = "incr_coin")]
    pub incr_coin: i64,
    /// 新增收藏数
    #[serde(rename = "incr_fav")]
    pub incr_fav: i64,
    /// 新增分享数
    #[serde(rename = "incr_share")]
    pub incr_share: i64,
}

/// UP主视频数据增量趋势项
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct VideoTrendItem {
    /// 对应时间戳（前一天8:00）
    pub date_key: i64,
    /// 增加数量，数据类型决定
    pub total_inc: i64,
}

/// UP主专栏数据增量趋势项
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ArticleTrendItem {
    /// 对应时间戳（前一天8:00）
    pub date_key: i64,
    /// 增加数量，数据类型决定
    pub total_inc: i64,
}

/// 播放来源情况（播放方式）
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PageSource {
    /// 通过动态
    pub dynamic: i64,
    /// 其他方式
    pub other: i64,
    /// 通过推荐列表
    #[serde(rename = "related_video")]
    pub related_video: i64,
    /// 通过搜索
    pub search: i64,
    /// 空间列表播放
    pub space: i64,
    /// 天马来源（APP推荐信息流）
    pub tenma: i64,
}

/// 播放平台占比
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PlayProportion {
    /// 安卓端
    pub android: i64,
    /// 移动端H5
    pub h5: i64,
    /// iOS端
    pub ios: i64,
    /// 站外
    pub out: i64,
    /// PC网页版
    pub pc: i64,
}

/// 播放来源占比数据
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PlaySourceData {
    pub page_source: PageSource,
    pub play_proportion: PlayProportion,
}

/// 播放地区提示信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Period {
    pub module_one: Option<String>,
    pub module_two: Option<String>,
    pub module_three: Option<String>,
    pub module_four: Option<String>,
}

/// 播放地区情况（粉丝或路人）
pub type ViewerAreaMap = std::collections::HashMap<String, i64>;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ViewerArea {
    pub fan: ViewerAreaMap,
    pub not_fan: ViewerAreaMap,
}

/// 播放数据情况（粉丝或路人）
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ViewerBaseDetail {
    pub male: i64,
    pub female: i64,
    #[serde(rename = "age_one")]
    pub age_one: i64,
    #[serde(rename = "age_two")]
    pub age_two: i64,
    #[serde(rename = "age_three")]
    pub age_three: i64,
    #[serde(rename = "age_four")]
    pub age_four: i64,
    #[serde(rename = "plat_pc")]
    pub plat_pc: i64,
    #[serde(rename = "plat_h5")]
    pub plat_h5: i64,
    #[serde(rename = "plat_out")]
    pub plat_out: i64,
    #[serde(rename = "plat_ios")]
    pub plat_ios: i64,
    #[serde(rename = "plat_android")]
    pub plat_android: i64,
    #[serde(rename = "plat_other_app")]
    pub plat_other_app: i64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ViewerBase {
    pub fan: ViewerBaseDetail,
    pub not_fan: ViewerBaseDetail,
}

/// 播放分布情况
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ViewerData {
    pub period: Period,
    pub viewer_area: ViewerArea,
    pub viewer_base: ViewerBase,
}

impl BpiClient {
    /// 获取 UP 主视频状态数据
    ///
    /// 获取 UP 主的视频统计数据，包括播放、点赞、投币、收藏等数据。
    ///
    /// # 文档
    /// [获取 UP 主视频状态数据](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/statistics&data.md#获取-up-主视频状态数据)
    pub async fn up_stat(&self) -> Result<BpiResponse<UpStatData>, BpiError> {
        self
            .get("https://member.bilibili.com/x/web/index/stat")
            .send_bpi("获取UP主视频状态数据").await
    }

    /// 获取 UP 主视频数据比较
    ///
    /// 获取 UP 主视频的数据对比分析，包括播放量、互动率等指标。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `t` | Option<i64> | 时间戳，可选 |
    /// | `size` | Option<i64> | 最近 N 条视频，可选，默认 5 |
    ///
    /// # 文档
    /// [获取 UP 主视频数据比较](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/statistics&data.md#获取-up-主视频数据比较)
    pub async fn up_archive_compare(
        &self,
        t: Option<i64>,
        size: Option<i64>
    ) -> Result<BpiResponse<ArchiveCompareData>, BpiError> {
        let mut req = self.get("https://member.bilibili.com/x/web/data/archive_diagnose/compare");

        if let Some(t) = t {
            req = req.query(&[("t", t)]);
        }
        if let Some(size) = size {
            req = req.query(&[("size", size)]);
        }

        req.send_bpi("获取UP主视频数据比较").await
    }

    /// 获取UP主专栏状态数据
    ///
    /// 获取 UP 主专栏的统计数据，包括阅读、评论、点赞等数据。
    ///
    /// # 文档
    /// [获取UP主专栏状态数据](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/statistics&data.md#获取up主专栏状态数据)
    pub async fn up_article_stat(&self) -> Result<BpiResponse<UpArticleStatData>, BpiError> {
        self
            .get("https://member.bilibili.com/x/web/data/article")
            .send_bpi("获取UP主专栏状态数据").await
    }

    /// 获取UP主视频数据增量趋势
    ///
    /// 获取 UP 主视频数据的增量趋势，支持多种数据类型。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `type_code` | i64 | 数据类型：1播放 2弹幕 3评论 4分享 5投币 6收藏 7充电 8点赞 |
    ///
    /// # 文档
    /// [获取UP主视频数据增量趋势](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/statistics&data.md#获取up主视频数据增量趋势)
    pub async fn up_video_trend(
        &self,
        type_code: i64
    ) -> Result<BpiResponse<Vec<VideoTrendItem>>, BpiError> {
        self
            .get("https://member.bilibili.com/x/web/data/pandect")
            .query(&[("type", type_code)])
            .send_bpi("获取UP主视频数据增量趋势").await
    }

    /// 获取UP主专栏数据增量趋势
    ///
    /// 获取 UP 主专栏数据的增量趋势，支持多种数据类型。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `type_code` | i64 | 数据类型：1阅读 2评论 3分享 4投币 5收藏 6点赞 |
    ///
    /// # 文档
    /// [获取UP主专栏数据增量趋势](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/statistics&data.md#获取up主专栏数据增量趋势)
    pub async fn up_article_trend(
        &self,
        type_code: i64
    ) -> Result<BpiResponse<Vec<ArticleTrendItem>>, BpiError> {
        self
            .get("https://member.bilibili.com/x/web/data/article/thirty")
            .query(&[("type", type_code)])
            .send_bpi("获取UP主专栏数据增量趋势").await
    }

    /// 获取播放来源占比
    ///
    /// 获取视频播放来源的占比情况，包括动态、搜索、推荐等来源。
    ///
    /// # 文档
    /// [获取播放来源占比](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/statistics&data.md#获取播放来源占比)
    #[allow(dead_code)]
    async fn up_play_source(&self) -> Result<BpiResponse<PlaySourceData>, BpiError> {
        self
            .get("https://member.bilibili.com/x/web/data/playsource")
            .with_bilibili_headers()
            .send_bpi("获取播放来源占比情况").await
    }

    /// 获取播放分布情况
    ///
    /// 获取视频播放的分布情况，包括粉丝与路人的观看数据。
    ///
    /// # 文档
    /// [获取播放分布情况](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/statistics&data.md#获取播放分布情况)
    pub async fn up_viewer_data(&self) -> Result<BpiResponse<ViewerData>, BpiError> {
        self.get("https://member.bilibili.com/x/web/data/base").send_bpi("获取播放分布情况").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_up_stat() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.up_stat().await?.into_data()?;
        info!("UP主视频状态数据: {:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn test_archive_compare() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.up_archive_compare(None, Some(3)).await?.into_data()?;
        info!("UP主视频数据比较: {:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn test_up_article_stat() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.up_article_stat().await?.into_data()?;
        info!("UP主专栏状态数据: {:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn test_video_trend() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.up_video_trend(1).await?.into_data()?; // 1 = 播放
        info!("UP主视频数据增量趋势: {:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn test_article_trend() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.up_article_trend(1).await?.into_data()?; // 1 = 阅读
        info!("UP主专栏数据增量趋势: {:?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn test_viewer_data() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let data = bpi.up_viewer_data().await?.into_data()?;
        info!("播放分布情况: {:?}", data);
        Ok(())
    }
}
