//! 互动视频相关接口
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// --- 响应数据结构体 ---

/// 互动视频模块详细信息响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractiveVideoInfoResponseData {
    /// 视频模块（分P）标题
    pub title: String,
    /// 当前模块 ID
    pub edge_id: u64,
    /// 进度回溯信息
    #[serde(default)]
    pub story_list: Vec<InteractiveVideoStory>,
    /// 当前模块信息
    pub edges: Option<InteractiveVideoEdges>,
    /// 预加载的分P
    pub preload: Option<InteractiveVideoPreload>,
    /// 变量列表
    #[serde(default)]
    pub hidden_vars: Vec<InteractiveVideoHiddenVar>,
    /// 是否为结束模块, 0: 普通模块, 1: 结束模块
    pub is_leaf: u8,
    /// 禁止记录选择, 1: 禁止
    #[serde(default)]
    pub no_tutorial: u8,
    /// 禁止进度回溯, 1: 禁止
    #[serde(default)]
    pub no_backtracking: u8,
    /// 禁止结尾评分, 1: 禁止
    #[serde(default)]
    pub no_evaluation: u8,
}

/// 进度回溯信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractiveVideoStory {
    /// 模块编号
    pub node_id: u64,
    /// 同上
    pub edge_id: u64,
    /// 模块（分P）标题
    pub title: String,
    /// 模块（分P）cid
    pub cid: u64,
    /// 记录播放开始位置，单位为毫秒
    pub start_pos: u64,
    /// 分P封面 url
    pub cover: String,
    /// 是否为当前模块, 1: 是
    #[serde(default)]
    pub is_current: u8,
    /// 进度序号，从0开始向上增长
    pub cursor: u64,
}

/// 当前模块信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractiveVideoEdges {
    /// 当前分P分辨率
    pub dimension: Option<InteractiveVideoDimension>,
    /// 问题列表，问题结束模块无此项
    #[serde(default)]
    pub questions: Vec<InteractiveVideoQuestion>,
    /// 问题外观
    pub skin: Option<serde_json::Value>,
}

/// 分辨率信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractiveVideoDimension {
    /// 宽度
    pub width: u32,
    /// 高度
    pub height: u32,
    /// 是否将宽高对换, 0: 正常, 1: 对换
    pub rotate: u8,
    /// 作用尚不明确
    pub sar: String,
}

/// 问题信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractiveVideoQuestion {
    /// 作用尚不明确
    pub id: u64,
    /// 选项显示模式, 0: 不显示选项, 1: 底部选项模式, 2: 坐标定点模式
    #[serde(rename = "type")]
    pub question_type: u8,
    /// 作用尚不明确
    pub start_time_r: u32,
    /// 回答限时，单位为毫秒，不限时为-1
    pub duration: i64,
    /// 是否暂停播放视频, 0: 不暂停, 1: 暂停播放
    pub pause_video: u8,
    /// 作用尚不明确
    pub title: String,
    /// 选项列表
    pub choices: Vec<InteractiveVideoChoice>,
}

/// 选项信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractiveVideoChoice {
    /// 选项所跳转的模块 id
    pub id: u64,
    /// 跳转信息文字, 例如 `JUMP+{模块编号}+{cid}`
    pub platform_action: String,
    /// 点击后对变量运算语句
    pub native_action: String,
    /// 选项出现条件判断语句
    pub condition: String,
    /// 选项所跳转分P的cid
    pub cid: u64,
    /// 选项文字
    pub option: String,
    /// 是否为默认选项, 1: 是
    #[serde(default)]
    pub is_default: Option<u8>,
    /// 是否为隐藏选项, 1: 是
    #[serde(default)]
    pub is_hidden: Option<u8>,
}

/// 预加载的分P信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractiveVideoPreload {
    /// 预加载的分P列表
    #[serde(default)]
    pub video: Vec<InteractiveVideoPreloadVideo>,
}

/// 预加载的分P
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractiveVideoPreloadVideo {
    /// 稿件avid
    pub aid: u64,
    /// 分P cid
    pub cid: u64,
}

/// 变量信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractiveVideoHiddenVar {
    /// 变量值
    pub value: i64,
    /// 变量编号
    pub id: String,
    /// 变量编号，语句中一般使用此项
    pub id_v2: String,
    /// 变量类型, 1: 普通变量, 2: 随机值
    #[serde(rename = "type")]
    pub var_type: u8,
    /// 是否展示变量, 0: 否, 1: 是
    pub is_show: u8,
    /// 变量名
    pub name: String,
}

impl BpiClient {
    /// 获取互动视频模块详细信息
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/video/interact_video.html#获取互动视频信息
    ///
    /// # 参数
    /// | 名称           | 类型           | 说明                 |
    /// | -------------- | --------------| -------------------- |
    /// | `aid`          | Option<u64>   | 稿件 avid，可选      |
    /// | `bvid`         | Option<&str>  | 稿件 bvid，可选      |
    /// | `graph_version`| u64           | 剧情图 ID            |
    /// | `edge_id`      | Option<u64>   | 模块编号，0或留空为起始模块，可选 |
    ///
    /// `aid` 和 `bvid` 必须提供一个。
    pub async fn video_interactive_video_info(
        &self,
        aid: Option<u64>,
        bvid: Option<&str>,
        graph_version: u64,
        edge_id: Option<u64>,
    ) -> Result<BpiResponse<InteractiveVideoInfoResponseData>, BpiError> {
        if aid.is_none() && bvid.is_none() {
            return Err(BpiError::parse("必须提供 aid 或 bvid"));
        }

        let mut req = self
            .get("https://api.bilibili.com/x/stein/edgeinfo_v2")
            .query(&[("graph_version", &graph_version.to_string())]);

        if let Some(a) = aid {
            req = req.query(&[("aid", &a.to_string())]);
        }
        if let Some(b) = bvid {
            req = req.query(&[("bvid", b)]);
        }
        if let Some(e) = edge_id {
            req = req.query(&[("edge_id", &e.to_string())]);
        }

        req.send_bpi("获取互动视频模块详细信息").await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    const TEST_AID: u64 = 114347430905959;
    const TEST_GRAPH_VERSION: u64 = 1273647;

    #[tokio::test]

    async fn test_video_interactive_video_info_by_aid() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi
            .video_interactive_video_info(Some(TEST_AID), None, TEST_GRAPH_VERSION, None)
            .await?;
        let data = resp.into_data()?;

        info!("互动视频信息: {:?}", data);
        assert!(!data.title.is_empty());
        assert!(!data.story_list.is_empty());

        Ok(())
    }
}
