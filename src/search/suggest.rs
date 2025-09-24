use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::Deserialize;

/// 搜索建议结果
#[derive(Debug, Deserialize)]
pub struct SearchSuggest {
    pub tag: Option<Vec<SearchSuggestItem>>,
}

/// 搜索建议项
#[derive(Debug, Deserialize)]
pub struct SearchSuggestItem {
    pub value: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub item_type: Option<String>,
}

impl BpiClient {
    /// 获取搜索建议关键词
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search)
    ///
    /// # 参数
    /// - `term`: 搜索关键词
    pub async fn search_suggest(&self, term: &str) -> Result<BpiResponse<SearchSuggest>, BpiError> {
        let params = [("term", term)];

        self
            .get("https://s.search.bilibili.com/main/suggest")
            .query(&params)
            .send_bpi("获取搜索建议").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_search_suggest() {
        // 创建一个 BilibiliRequest 实例
        let bpi = BpiClient::new();
        // 传入一个搜索关键词
        let term = "rust";
        let resp = bpi.search_suggest(term).await;

        // 验证请求是否成功
        assert!(resp.is_ok());

        if let Ok(r) = resp {
            info!("搜索建议返回: {:?}", r);

            // 检查返回码是否为0
            if r.code == 0 {
                // 检查 result 是否存在

                // 检查搜索建议列表是否存在
                if let Some(suggests) = r.data {
                    // 检查 tag 数组是否不为空
                    if let Some(tags) = suggests.tag {
                        assert!(!tags.is_empty());
                        info!("获取到搜索建议列表，数量：{}", tags.len());

                        // 打印第一个搜索建议
                        if let Some(first_suggest) = tags.first() {
                            info!("第一个建议关键词: {:?}", first_suggest.value);
                            info!("第一个建议显示内容: {:?}", first_suggest.name);
                        }
                    } else {
                        info!("搜索建议列表为空。");
                    }
                } else {
                    info!("返回数据中没有 'result' 字段。");
                }
            } else {
                info!("API 返回码不为0，可能存在错误: {}", r.code);
            }
        }
    }
}
