//! 创作中心上传 API
//!
//! [参考文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/upload.md)

use std::collections::HashMap;

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

use base64::{ Engine as _, engine::general_purpose };
use serde::{ Deserialize, Serialize };
use std::fs;

/// 上传封面返回结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadCoverData {
    pub url: String,
}

impl BpiClient {
    /// 上传视频封面
    ///
    /// 上传视频封面图片，支持多种输入格式。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `mime_type` | &str | 图片 MIME 类型，如 image/jpeg |
    /// | `cover` | `AsRef<str>` | 封面数据，可以是：纯 base64、完整 data URI、文件路径 |
    ///
    /// # 注意
    /// 文件不得超过 20M
    ///
    /// # 文档
    /// [上传视频封面](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/creativecenter/upload.md#上传视频封面)
    pub async fn upload_cover(
        &self,
        mime_type: &str,
        cover: impl AsRef<str>
    ) -> Result<BpiResponse<UploadCoverData>, BpiError> {
        let csrf = self.csrf()?;
        let cover_str = cover.as_ref();

        let final_cover = if cover_str.starts_with("data:") {
            cover_str.to_string()
        } else if
            cover_str
                .chars()
                .all(|c| (c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '='))
        {
            format!("data:{};base64,{}", mime_type, cover_str)
        } else {
            let file_bytes = fs::read(cover_str).map_err(|e| BpiError::Parse {
                message: format!("读取文件失败: {}", e),
            })?;
            let file_base64 = general_purpose::STANDARD.encode(file_bytes);
            format!("data:{};base64,{}", mime_type, file_base64)
        };

        let mut form = HashMap::new();
        form.insert("csrf", csrf);
        form.insert("cover", final_cover);

        self
            .post("https://member.bilibili.com/x/vu/web/cover/up")
            .form(&form)
            .send_bpi("上传视频封面").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(unused_imports)]
    use base64::{ Engine as _, engine::general_purpose };
    use std::fs;

    async fn run_upload_test(
        bpi: &BpiClient,
        mime_type: &str,
        cover: &str
    ) -> Result<(), Box<BpiError>> {
        let data = bpi.upload_cover(mime_type, cover).await?.into_data()?;
        tracing::info!("上传成功，封面地址: {}", data.url);
        Ok(())
    }

    #[tokio::test]
    async fn test_cover_upload() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        // 1. 文件路径
        // run_upload_test(&bpi, "image/jpeg", "./assets/test.jpg").await?;

        // 2. 纯 Base64
        // let img_data = fs::read("./assets/test.jpg")?;
        // let img_base64 = general_purpose::STANDARD.encode(&img_data);
        // run_upload_test(&bpi, "image/jpeg", &img_base64).await?;

        // 3. 完整 Data URI
        let img_data = fs::read("./assets/test.jpg").map_err(|_| BpiError::parse("读取图片失败"))?;
        let img_base64 = general_purpose::STANDARD.encode(&img_data);
        let img_data_uri = format!("data:image/jpeg;base64,{}", img_base64);
        run_upload_test(&bpi, "image/jpeg", &img_data_uri).await?;

        Ok(())
    }
}
