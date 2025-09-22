use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// --- 获取稍后再看视频列表 ---

/// 稿件属性标志
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToViewRights {
    pub bp: u8,
    pub elec: u8,
    pub download: u8,
    pub movie: u8,
    pub pay: u8,
    pub hd5: u8,
    pub no_reprint: u8,
    pub autoplay: u8,
    pub ugc_pay: u8,
    pub is_cooperation: u8,
    pub ugc_pay_preview: u8,
    pub no_background: u8,
}

/// 稿件 UP 主信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToViewOwner {
    pub mid: u64,
    pub name: String,
    pub face: String,
}

/// 稿件状态数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToViewStat {
    pub aid: u64,
    pub view: u64,
    pub danmaku: u64,
    pub reply: u64,
    pub favorite: u64,
    pub coin: u64,
    pub share: u64,
    pub now_rank: u64,
    pub his_rank: u32,
    pub like: u64,
    pub dislike: u64,
    pub vt: i64,
    pub vv: i64,
}

/// 稿件1P分辨率
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToViewDimension {
    pub width: u32,
    pub height: u32,
    pub rotate: u8,
}

/// 稍后再看视频列表中的单个视频
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToViewVideoItem {
    pub aid: u64,
    pub videos: u32,
    pub tid: u32,
    pub tname: String,
    pub copyright: u8,
    pub pic: String,
    pub title: String,
    pub pubdate: u64,
    pub ctime: u64,
    pub desc: String,
    pub state: i32,
    pub attribute: Option<u32>, // 历史保留字段，可能为 null
    pub duration: u32,
    pub rights: ToViewRights,
    pub owner: ToViewOwner,
    pub stat: ToViewStat,
    pub dynamic: Option<String>,
    pub dimension: ToViewDimension,
    pub count: Option<u32>,
    pub cid: u64,
    pub progress: u32,
    pub add_at: u64,
    pub bvid: String,
}

/// 稍后再看视频列表的数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToViewListData {
    /// 稍后再看视频数
    pub count: u32,
    /// 稍后再看视频列表
    pub list: Vec<ToViewVideoItem>,
}

impl BpiClient {
    /// 视频添加稍后再看（最多100个）
    /// avid 与 bvid 任选一个
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/historytoview
    ///
    /// 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `aid` | Option<u64> | 稿件 avid |
    /// | `bvid` | Option<&str> | 稿件 bvid |
    pub async fn toview_add_video(
        &self,
        aid: Option<u64>,
        bvid: Option<&str>,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let mut form = vec![("csrf", csrf)];
        if let Some(avid) = aid {
            form.push(("aid", avid.to_string()));
        }
        if let Some(bvid_str) = bvid {
            form.push(("bvid", bvid_str.to_string()));
        }

        self.post("https://api.bilibili.com/x/v2/history/toview/add")
            .form(&form)
            .send_bpi("添加稍后再看视频")
            .await
    }

    /// 获取稍后再看视频列表
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/historytoview
    pub async fn toview_list(&self) -> Result<BpiResponse<ToViewListData>, BpiError> {
        self.get("https://api.bilibili.com/x/v2/history/toview")
            .send_bpi("获取稍后再看视频列表")
            .await
    }

    /// 删除稍后再看视频
    /// `aid` 和 `viewed` 参数任选一个
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/historytoview
    ///
    /// 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `aid` | Option<u64> | 稿件 avid |
    /// | `viewed` | Option<bool> | 是否删除已观看 |
    pub async fn toview_delete(
        &self,
        aid: Option<u64>,
        viewed: Option<bool>,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let mut form = vec![("csrf", csrf)];
        if let Some(avid) = aid {
            form.push(("aid", avid.to_string()));
        }
        if let Some(is_viewed) = viewed {
            form.push(("viewed", is_viewed.to_string()));
        }

        self.post("https://api.bilibili.com/x/v2/history/toview/del")
            .form(&form)
            .send_bpi("删除稍后再看视频")
            .await
    }

    /// 清空稍后再看视频列表
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/historytoview
    pub async fn toview_clear(&self) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let form = [("csrf", csrf)];

        self.post("https://api.bilibili.com/x/v2/history/toview/clear")
            .form(&form)
            .send_bpi("清空稍后再看视频列表")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_toview_add_and_get() {
        let bpi = BpiClient::new();
        let aid = 10001;

        // 1. 添加视频
        let add_resp = bpi.toview_add_video(Some(aid), None).await;
        info!("Add video result: {:?}", add_resp);
        assert!(add_resp.is_ok());

        // 2. 获取列表
        let get_resp = bpi.toview_list().await;
        info!("Get list result: {:?}", get_resp);
        assert!(get_resp.is_ok());

        let list_data = get_resp.unwrap().data.unwrap();
        info!("Total to view videos: {}", list_data.count);
        info!("First video in list: {:?}", list_data.list.first());

        // 3. 删除视频
        let del_resp = bpi.toview_delete(Some(aid), Some(true)).await; // 尝试删除所有已观看的
        info!("Delete viewed videos result: {:?}", del_resp);
        assert!(del_resp.is_ok());
    }
}
