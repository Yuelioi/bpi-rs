use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
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

/// Parameters for the search suggestion endpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchSuggestParams {
    term: String,
}

impl SearchSuggestParams {
    pub fn new(term: impl Into<String>) -> Result<Self, BpiError> {
        let term = term.into().trim().to_string();
        if term.is_empty() {
            return Err(BpiError::invalid_parameter(
                "term",
                "search suggestion term cannot be blank",
            ));
        }

        Ok(Self { term })
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![("term", self.term.clone())]
    }
}

impl BpiClient {
    /// 获取搜索建议关键词
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/search)
    ///
    /// # 参数
    /// - `params`: 搜索建议参数
    pub async fn search_suggest(
        &self,
        params: SearchSuggestParams,
    ) -> Result<BpiResponse<SearchSuggest>, BpiError> {
        self.get("https://s.search.bilibili.com/main/suggest")
            .query(&params.query_pairs())
            .send_bpi("获取搜索建议")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[test]
    fn search_suggest_params_serializes_term_query() -> Result<(), BpiError> {
        let params = SearchSuggestParams::new("rust lang")?;

        assert_eq!(
            params.query_pairs(),
            vec![("term", "rust lang".to_string())]
        );
        Ok(())
    }

    #[test]
    fn search_suggest_params_trims_term() -> Result<(), BpiError> {
        let params = SearchSuggestParams::new("  rust  ")?;

        assert_eq!(params.query_pairs(), vec![("term", "rust".to_string())]);
        Ok(())
    }

    #[test]
    fn search_suggest_params_rejects_blank_term() {
        let err = SearchSuggestParams::new(" \t ").unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "term", .. }
        ));
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_search_suggest() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let params = SearchSuggestParams::new("rust")?;
        let resp = bpi.search_suggest(params).await;

        assert!(resp.is_ok());

        if let Ok(r) = resp {
            info!("搜索建议返回: {:?}", r);

            if r.code == 0 {
                if let Some(suggests) = r.data {
                    if let Some(tags) = suggests.tag {
                        assert!(!tags.is_empty());
                        info!("获取到搜索建议列表，数量：{}", tags.len());

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

        Ok(())
    }
}
