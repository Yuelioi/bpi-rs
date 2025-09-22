use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 历史记录列表的页面信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryCursor {
    /// 最后一项目标 ID
    pub max: u64,
    /// 最后一项时间节点 (时间戳)
    pub view_at: u64,
    /// 最后一项业务类型
    pub business: String,
    /// 每页项数
    pub ps: u32,
}

/// 历史记录筛选类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryTab {
    /// 类型
    #[serde(rename = "type")]
    pub type_name: String,
    /// 类型名
    pub name: String,
}

/// 历史记录封面图组
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HistoryCovers {
    /// 专栏的封面图数组
    Array(Vec<String>),
    /// 其他条目的单封面图
    String(String),
}

/// 历史记录中的详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryDetail {
    /// 目标 ID，如稿件 avid、直播间 ID 等
    pub oid: u64,
    /// 剧集 epid，仅用于剧集
    pub epid: Option<u64>,
    /// 稿件 bvid，仅用于稿件视频
    pub bvid: Option<String>,
    /// 观看到的视频分P数，仅用于稿件视频
    pub page: Option<u32>,
    /// 观看到的对象 ID，如视频 cid、文章 cvid
    pub cid: Option<u64>,
    /// 观看到的视频分 P 标题，仅用于稿件视频
    pub part: Option<String>,
    /// 业务类型
    pub business: String,
    /// 记录查看的平台代码
    pub dt: u32,
}

/// 单个历史记录条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryListItem {
    /// 条目标题
    pub title: String,
    /// 条目副标题
    pub long_title: Option<String>,
    /// 条目封面图 URL，用于专栏以外的条目
    pub cover: Option<String>,
    /// 条目封面图组，仅用于专栏
    pub covers: Option<Vec<String>>,
    /// 重定向 URL，仅用于剧集和直播
    pub uri: Option<String>,
    /// 条目详细信息
    pub history: HistoryDetail,
    /// 视频分 P 数目，仅用于稿件视频
    pub videos: Option<u32>,
    /// UP 主昵称
    pub author_name: Option<String>,
    /// UP 主头像 URL
    pub author_face: Option<String>,
    /// UP 主 mid
    pub author_mid: Option<u64>,
    /// 查看时间 (时间戳)
    pub view_at: u64,
    /// 视频观看进度 (秒)
    pub progress: i32,
    /// 分 P 标题，用于稿件视频或剧集
    pub show_title: Option<String>,
    /// 视频总时长，用于稿件视频或剧集
    pub duration: Option<u32>,
    /// 备注
    pub current: Option<String>,
    /// 总计分集数，仅用于剧集
    pub total: Option<u32>,
    /// 最新一话 / 最新一 P 标识
    pub new_desc: Option<String>,
    /// 是否已完结，仅用于剧集
    pub is_finish: Option<u8>,
    /// 是否收藏
    pub is_fav: u8,
    /// 条目目标 id
    pub kid: u64,
    /// 子分区名，用于稿件视频和直播
    pub tag_name: Option<String>,
    /// 直播状态，仅用于直播
    pub live_status: Option<u8>,
}

/// 历史记录列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryListData {
    /// 历史记录页面信息
    pub cursor: HistoryCursor,
    /// 历史记录筛选类型
    pub tab: Vec<HistoryTab>,
    /// 分段历史记录列表
    pub list: Vec<HistoryListItem>,
}

impl BpiClient {
    /// 获取历史记录列表（web端）
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/historytoview
    ///
    /// 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `max` | Option<u64> | 截止目标 id（0/avid/ssid/直播间 id/rlid/cvid） |
    /// | `business` | Option<&str> | 业务类型：archive/pgc/live/article-list/article |
    /// | `view_at` | Option<u64> | 时间戳 |
    /// | `typ` | Option<&str> | 分类筛选：all/archive/live/article 等 |
    /// | `ps` | Option<u32> | 每页项数 |
    pub async fn history_list(
        &self,
        max: Option<u64>,
        business: Option<&str>,
        view_at: Option<u64>,
        typ: Option<&str>,
        ps: Option<u32>,
    ) -> Result<BpiResponse<HistoryListData>, BpiError> {
        let mut request = self.get("https://api.bilibili.com/x/web-interface/history/cursor");

        if let Some(m) = max {
            request = request.query(&[("max", m)]);
        }
        if let Some(b) = business {
            request = request.query(&[("business", b)]);
        }
        if let Some(v) = view_at {
            request = request.query(&[("view_at", v)]);
        }
        if let Some(t) = typ {
            request = request.query(&[("type", t)]);
        }
        if let Some(p) = ps {
            request = request.query(&[("ps", p)]);
        }

        request.send_bpi("获取历史记录列表").await
    }

    /// 删除历史记录
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/historytoview
    ///
    /// 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `kid` | &str | 记录目标 id |
    pub async fn history_delete(
        &self,
        kid: &str,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let payload = [("kid", kid), ("csrf", &csrf)];

        self.post("https://api.bilibili.com/x/v2/history/delete")
            .form(&payload)
            .send_bpi("删除历史记录")
            .await
    }

    /// 清空历史记录
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/historytoview
    pub async fn history_clear(&self) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let payload = [("csrf", &csrf)];

        self.post("https://api.bilibili.com/x/v2/history/clear")
            .form(&payload)
            .send_bpi("清空历史记录")
            .await
    }

    /// 停用历史记录
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/historytoview
    ///
    /// 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `switch` | bool | 是否停用 |
    pub async fn history_shadow_set(
        &self,
        switch: bool,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let payload = [("switch", switch.to_string()), ("csrf", csrf)];

        self.post("https://api.bilibili.com/x/v2/history/shadow/set")
            .form(&payload)
            .send_bpi("停用历史记录")
            .await
    }

    /// 查询历史记录停用状态
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/historytoview
    pub async fn history_shadow_get(&self) -> Result<BpiResponse<bool>, BpiError> {
        self.get("https://api.bilibili.com/x/v2/history/shadow")
            .send_bpi("查询历史记录停用状态")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_history_get_list() {
        let bpi = BpiClient::new();
        let resp = bpi.history_list(None, None, None, None, Some(10)).await;

        info!("{:?}", resp);
        assert!(resp.is_ok());

        let resp_data = resp.unwrap();
        info!("code: {}", resp_data.code);
        info!("message: {}", resp_data.message);
        if let Some(data) = resp_data.data {
            info!("cursor: {:?}", data.cursor);
            info!("tabs: {:?}", data.tab);
            info!("first item: {:?}", data.list.first());
        }
    }

    #[tokio::test]
    async fn test_history_shadow_set_and_get() {
        let bpi = BpiClient::new();

        // 获取当前状态
        let current_status_resp = bpi.history_shadow_get().await.unwrap();
        let current_status = current_status_resp.data.unwrap();

        // 切换状态
        let new_status = !current_status;
        let set_resp = bpi.history_shadow_set(new_status).await;
        info!("Set status to {}: {:?}", new_status, set_resp);
        assert!(set_resp.is_ok());

        // 再次获取状态，验证是否已切换
        let new_status_resp = bpi.history_shadow_get().await;
        info!("New status: {:?}", new_status_resp);
        assert!(new_status_resp.is_ok());
        assert_eq!(new_status_resp.unwrap().data.unwrap(), new_status);

        // 恢复原始状态
        let set_back_resp = bpi.history_shadow_set(current_status).await;
        info!(
            "Set back to original status {}: {:?}",
            current_status, set_back_resp
        );
        assert!(set_back_resp.is_ok());
    }
}
