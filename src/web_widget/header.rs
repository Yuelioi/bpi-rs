//! B站首页头图相关接口
//!
//! 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/web_widget/header.html
use serde::{ Deserialize, Serialize };

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

/// B站首页头图数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HeaderData {
    /// 空
    pub name: String,
    /// 静态头图 URL
    pub pic: String,
    /// Bilibili logo URL
    pub litpic: String,
    /// 空
    pub url: String,
    /// 是否分层, 1: 是
    pub is_split_layer: u32,
    /// 分层信息，一个套在字符串里的 JSON 对象
    pub split_layer: String,

    pub split_layer_obj: Option<SplitLayer>,
}

impl HeaderData {
    pub fn parse_split_layer(&mut self) -> Result<(), BpiError> {
        let result = serde_json::from_str(&self.split_layer);
        match result {
            Ok(r) => {
                self.split_layer_obj = Some(r);
                Ok(())
            }
            Err(e) => Err(BpiError::parse(format!("解析split_layer失败: {:?}", e))),
        }
    }
}

/// 分层信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SplitLayer {
    /// 版本号
    pub version: String,
    /// 层信息
    pub layers: Vec<Layer>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Layer {
    pub resources: Vec<Resource>,
    pub scale: Scale,
    pub rotate: Rotate,
    pub translate: Translate,
    pub blur: Blur,
    pub opacity: Opacity,
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub src: String,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scale {
    pub initial: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotate {
    pub offset: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translate {
    pub offset: Option<Vec<i64>>,
    pub initial: Option<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blur {
    pub initial: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Opacity {
    pub wrap: String,
    pub initial: Option<f64>,
}

impl BpiClient {
    /// 获取首页头图
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/web_widget/header.html#获取首页头图
    ///
    pub async fn web_widget_header_page(&self) -> Result<BpiResponse<HeaderData>, BpiError> {
        let mut result = self
            .get("https://api.bilibili.com/x/web-show/page/header")
            .query(&[("resource_id", 142)])
            .send_bpi("获取首页头图").await?;
        let mut header: HeaderData = result.data.take().ok_or_else(|| BpiError::missing_data())?;

        header.parse_split_layer()?;

        // 将解析后的数据放回 response
        result.data = Some(header);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_get_header_page() {
        let bpi = BpiClient::new();
        let resp = bpi.web_widget_header_page().await;
        info!("响应: {:?}", resp);
        assert!(resp.is_ok());
    }
}
