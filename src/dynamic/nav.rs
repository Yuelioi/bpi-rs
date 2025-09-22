use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

// --- 导航栏动态 API 结构体 ---

/// 导航栏动态列表项的 UP 主信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicNavAuthor {
    /// UP 主头像 URL
    pub face: String,
    /// UP 主 mid (UID)
    pub mid: u64,
    /// UP 主昵称
    pub name: String,
}

/// 导航栏动态列表项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicNavItem {
    /// UP 主信息
    pub author: DynamicNavAuthor,
    /// 封面 URL
    pub cover: String,
    /// 动态 ID 字符串
    pub id_str: String,
    /// 发布时间（文字表述的相对时间）
    pub pub_time: String,
    /// 关联 ID，视频即 aid
    pub rid: u64,
    /// 标题
    pub title: String,
    /// 动态类型，8 表示视频
    #[serde(rename = "type")]
    pub type_num: u8,
    /// 是否可见
    pub visible: bool,
}

/// 导航栏动态列表响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicNavData {
    /// 是否有更多数据
    pub has_more: bool,
    /// 动态数据数组
    pub items: Vec<DynamicNavItem>,
    /// 偏移量，用于翻页
    pub offset: String,
    /// 更新基线，用于获取新动态
    pub update_baseline: String,
    /// 本次获取到的新动态条数
    pub update_num: u64,
}

impl BpiClient {
    /// 获取导航栏动态列表
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic
    ///
    /// 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `update_baseline` | Option<&str> | 更新基线，获取新动态时使用 |
    /// | `offset` | Option<&str> | 分页偏移量，翻页时使用 |
    pub async fn dynamic_nav_feed(
        &self,
        update_baseline: Option<&str>,
        offset: Option<&str>,
    ) -> Result<BpiResponse<DynamicNavData>, BpiError> {
        let mut req = self.get("https://api.bilibili.com/x/polymer/web-dynamic/v1/feed/nav");

        if let Some(baseline) = update_baseline {
            req = req.query(&[("update_baseline", baseline)]);
        }

        if let Some(off) = offset {
            req = req.query(&[("offset", off)]);
        }

        req.send_bpi("获取导航栏动态列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]

    async fn test_get_dynamic_nav_feed() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.dynamic_nav_feed(None, None).await?;
        let data = resp.into_data()?;

        info!("获取到 {} 条动态", data.items.len());
        info!("第一条动态ID: {}", data.items[0].id_str);

        assert!(!data.items.is_empty());

        Ok(())
    }
}
