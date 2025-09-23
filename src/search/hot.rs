use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 默认搜索内容
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DefaultSearchData {
    /// 搜索 seid
    pub seid: String,
    /// 默认搜索 id
    pub id: u64,
    /// 类型，固定 0
    pub r#type: u32,
    /// 显示文字
    pub show_name: String,
    /// 空字段
    pub name: Option<String>,
    /// 跳转类型，1: 视频
    pub goto_type: u32,
    /// 搜索目标 id，视频为稿件 avid
    pub goto_value: String,
    /// 搜索目标跳转 url
    pub url: String,
}

/// 热搜条目
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HotWordItem {
    pub hot_id: u64,
    pub keyword: String,
    pub show_name: String,

    pub heat_score: u64,
    pub word_type: u32,

    // pub icon: Option<String>,
    // pub resource_id: Option<u64>,
    pub live_id: Option<Vec<serde_json::Value>>,
    // pub name_type: Option<String>,
}

/// 热搜返回数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HotWordDataResponse {
    pub code: u32,
    pub list: Vec<HotWordItem>,
}

impl BpiClient {
    /// 获取默认搜索内容（web端）
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search
    pub async fn search_default(&self) -> Result<BpiResponse<DefaultSearchData>, BpiError> {
        let signed_params = self.get_wbi_sign2(vec![("foo", "bar")]).await?;

        self
            .get("https://api.bilibili.com/x/web-interface/wbi/search/default")
            .query(&signed_params)
            .send_bpi("获取默认搜索内容").await
    }

    /// 获取热搜列表（web端）
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search
    ///
    /// - 无参数
    pub async fn search_hotwords(&self) -> Result<BpiResponse<HotWordDataResponse>, BpiError> {
        let response = self.get("https://s.search.bilibili.com/main/hotword").send().await?;

        let data: HotWordDataResponse = response.json().await?;

        let resp: BpiResponse<HotWordDataResponse> = BpiResponse {
            code: 0,
            data: Some(data),
            message: "".to_string(),
            status: false,
        };

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_default_search() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.search_default().await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_hotword_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.search_hotwords().await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);

        Ok(())
    }
}
