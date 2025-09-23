//! 用于生成 b23.tv 短链
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/misc/b23tv.md

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 生成 b23.tv 短链 - 响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortLinkData {
    /// 原始返回内容（标题 + 短链）
    pub content: String,

    /// 恒为 0
    pub count: i32,

    /// 纯短链 URL
    #[serde(skip_serializing, skip_deserializing)]
    pub link: String,
    /// 标题
    #[serde(skip_serializing, skip_deserializing)]
    pub title: String,
}

impl ShortLinkData {
    pub fn extract(&mut self) {
        if let Some(pos) = self.content.find("https://b23.tv/") {
            self.link = self.content[pos..].to_string().trim().to_string();
            self.title = self.content[..pos].trim().to_string();
        } else {
            self.link = String::new();
            self.title = self.content.clone();
        }
    }
}

impl BpiClient {
    /// 根据视频 aid 生成 b23.tv 短链
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/misc
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `aid` | u64 | 稿件 avid |
    pub async fn misc_b23_short_link(
        &self,
        aid: u64,
    ) -> Result<BpiResponse<ShortLinkData>, BpiError> {
        let params = [
            ("platform", "unix"),
            ("share_channel", "COPY"),
            ("share_id", "main.ugc-video-detail.0.0.pv"),
            ("share_mode", "4"),
            ("oid", &aid.to_string()),
            ("buvid", "qwq"),
            ("build", "6114514"),
        ];

        let mut result: BpiResponse<ShortLinkData> = self
            .post("https://api.biliapi.net/x/share/click")
            .form(&params)
            .send_bpi("生成短链")
            .await?;

        // 额外解析出纯短链

        if let Some(data) = &mut result.data {
            data.extract();
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_short_link() {
        let bpi = BpiClient::new();

        match bpi.misc_b23_short_link(10001).await {
            Ok(resp) => {
                if resp.code == 0 {
                    let data = resp.data.unwrap();
                    tracing::info!("原始内容: {}", data.content);
                    tracing::info!("提取短链: {}", data.link);
                    tracing::info!("提取标题: {}", data.title)
                } else {
                    tracing::info!("生成短链失败: code={}, message={}", resp.code, resp.message);
                }
            }
            Err(err) => {
                panic!("请求出错: {}", err);
            }
        }
    }
}
