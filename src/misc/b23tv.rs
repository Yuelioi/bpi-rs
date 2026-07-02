//! 用于生成 b23.tv 短链
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/misc/b23tv.md)

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

use super::MiscB23ShortLinkParams;

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
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/misc)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `params` | [`MiscB23ShortLinkParams`] | 稿件和客户端分享参数 |
    pub async fn misc_b23_short_link(
        &self,
        params: MiscB23ShortLinkParams,
    ) -> Result<BpiResponse<ShortLinkData>, BpiError> {
        let form = params.form_pairs();

        let mut result: BpiResponse<ShortLinkData> = self
            .post("https://api.biliapi.net/x/share/click")
            .form(&form)
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
    use crate::ids::Aid;
    use crate::misc::MiscB23ShortLinkParams;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_generate_short_link() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let params = MiscB23ShortLinkParams::new(Aid::new(10001)?);

        match bpi.misc_b23_short_link(params).await {
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
        Ok(())
    }

    #[test]
    fn misc_b23_short_link_params_serializes_default_form() -> Result<(), BpiError> {
        let params = MiscB23ShortLinkParams::new(Aid::new(10001)?);

        assert_eq!(
            params.form_pairs(),
            [
                ("platform", "unix".to_string()),
                ("share_channel", "COPY".to_string()),
                ("share_id", "main.ugc-video-detail.0.0.pv".to_string()),
                ("share_mode", "4".to_string()),
                ("oid", "10001".to_string()),
                ("buvid", "qwq".to_string()),
                ("build", "6114514".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn misc_b23_short_link_params_serializes_custom_form() -> Result<(), BpiError> {
        let params = MiscB23ShortLinkParams::new(Aid::new(10001)?)
            .with_platform("web")?
            .with_share_channel("WEIXIN")?
            .with_share_id("custom.share.id")?
            .with_share_mode(5)
            .with_buvid("custom-buvid")?
            .with_build(123456);

        assert_eq!(
            params.form_pairs(),
            [
                ("platform", "web".to_string()),
                ("share_channel", "WEIXIN".to_string()),
                ("share_id", "custom.share.id".to_string()),
                ("share_mode", "5".to_string()),
                ("oid", "10001".to_string()),
                ("buvid", "custom-buvid".to_string()),
                ("build", "123456".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn misc_b23_short_link_params_rejects_blank_buvid() -> Result<(), BpiError> {
        let err = MiscB23ShortLinkParams::new(Aid::new(10001)?)
            .with_buvid("   ")
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "buvid", .. }
        ));
        Ok(())
    }
}
