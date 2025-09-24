use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use reqwest::Body;
use reqwest::multipart::{ Form, Part };
use serde::{ Deserialize, Serialize };
use serde_json::json;
use std::path::Path;
use tokio::fs::File;
use tokio_util::codec::{ BytesCodec, FramedRead };

// --- 图片上传 API 结构体 ---

/// 图片上传响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UploadPicData {
    /// 已上传图片 URL
    pub image_url: String,
    /// 已上传图片宽度
    pub image_width: u64,
    /// 已上传图片高度
    pub image_height: u64,
    /// 已上传图片大小k
    pub img_size: f64,
}

// --- 创建投票 API 结构体 ---

/// 创建投票响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateVoteData {
    /// 投票 ID
    pub vote_id: u64,
}

// --- 发布纯文本动态 API 结构体 ---

/// 纯文本动态响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateDynamicData {
    /// 动态 ID
    pub dynamic_id: u64,
    /// 动态 ID 字符串格式
    pub dynamic_id_str: String,
    // ... 其他字段
}

// 复杂动态

/// 动态内容组件
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicContentItem {
    /// 组件类型 ID，例如 @用户
    #[serde(rename = "type")]
    pub type_num: u8,
    /// 组件的内容 ID，例如用户的 mid
    pub biz_id: Option<String>,
    /// 文本内容
    pub raw_text: String,
}

/// 动态图片
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicPic {
    /// 图片 URL
    pub img_src: String,
    /// 图片高度
    pub img_height: u64,
    /// 图片宽度
    pub img_width: u64,
    /// 图片大小 (KB)
    pub img_size: f64,
}

/// 动态话题
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicTopic {
    /// 话题 ID
    pub id: u64,
    /// 话题名
    pub name: String,
    /// 来源，非必要
    pub from_source: Option<String>,
    /// 来源话题 ID，非必要
    pub from_topic_id: Option<u64>,
}

/// 动态互动设置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicOption {
    /// 开启精选评论
    pub up_choose_comment: Option<u8>,
    /// 关闭评论
    pub close_comment: Option<u8>,
}

/// 复杂动态请求体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicRequest {
    /// 特殊卡片，非必要
    pub attach_card: Option<serde_json::Value>,
    /// 动态内容
    pub content: DynamicContent,
    /// 元信息，非必要
    pub meta: Option<serde_json::Value>,
    /// 动态类型
    pub scene: u8,
    /// 携带的图片
    pub pics: Option<Vec<DynamicPic>>,
    /// 话题
    pub topic: Option<DynamicTopic>,
    /// 互动设置
    pub option: Option<DynamicOption>,
}

/// 动态内容
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DynamicContent {
    pub contents: Vec<DynamicContentItem>,
}

/// 复杂动态响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateComplexDynamicData {
    pub dyn_id: u64,
    pub dyn_id_str: String,
    pub dyn_type: u8,
}

impl BpiClient {
    /// 为图片动态上传图片
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `file_path` | &Path | 图片文件路径 |
    /// | `category` | `Option<&str>` | 图片类型，可选 `daily/draw/cos` |
    pub async fn dynamic_upload_pic(
        &self,
        file_path: &Path,
        category: Option<&str>
    ) -> Result<BpiResponse<UploadPicData>, BpiError> {
        let csrf = self.csrf()?;

        let file = File::open(file_path).await.map_err(|_| BpiError::parse("打开文件失败"))?;
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);

        let file_name = file_path
            .file_name()
            .ok_or_else(|| {
                BpiError::parse("Invalid file path, cannot get file name".to_string())
            })?;

        let file_part = Part::stream(body)
            .file_name(file_name.to_string_lossy().into_owned())
            .mime_str("image/jpeg")?;

        let mut form = Form::new().part("file_up", file_part).text("csrf", csrf.clone());

        if let Some(cat) = category {
            form = form.text("category", cat.to_string());
        } else {
            form = form.text("category", "daily".to_string());
        }

        form = form.text("biz", "new_dyn".to_string());

        self
            .post("https://api.bilibili.com/x/dynamic/feed/draw/upload_bfs")
            .multipart(form)
            .send_bpi("上传图片动态图片").await
    }

    /// 发布纯文本动态
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `content` | &str | 动态内容 |
    pub async fn dynamic_create_text(
        &self,
        content: &str
    ) -> Result<BpiResponse<CreateDynamicData>, BpiError> {
        let csrf = self.csrf()?;
        let form = Form::new()
            .text("dynamic_id", "0")
            .text("type", "4")
            .text("rid", "0")
            .text("content", content.to_string())
            .text("csrf", csrf.clone())
            .text("csrf_token", csrf);

        self
            .post("https://api.vc.bilibili.com/dynamic_svr/v1/dynamic_svr/create")
            .multipart(form)
            .send_bpi("发布纯文本动态").await
    }

    /// 发表复杂动态
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `scene` | u8 | 动态类型：1 纯文本，2 带图，4 转发 |
    /// | `contents` | `Vec<DynamicContentItem>` | 动态内容组件 |
    /// | `pics` | `Option<Vec<DynamicPic>>`| 动态图片，最多 9 个 |
    /// | `topic` | `Option<DynamicTopic>` | 话题 |
    pub async fn dynamic_create_complex(
        &self,
        scene: u8,
        contents: Vec<DynamicContentItem>,
        pics: Option<Vec<DynamicPic>>,
        topic: Option<DynamicTopic>
    ) -> Result<BpiResponse<CreateComplexDynamicData>, BpiError> {
        let csrf = self.csrf()?;

        let dyn_req = DynamicRequest {
            attach_card: None,
            content: DynamicContent { contents },
            meta: Some(
                json!({
                "app_meta": {
                    "from": "create.dynamic.web",
                    "mobi_app": "web"
                }
            })
            ),
            scene,
            pics,
            topic,
            option: None,
        };

        let request_body = json!({
            "dyn_req": dyn_req,
        });

        self
            .post("https://api.bilibili.com/x/dynamic/feed/create/dyn")
            .header("Content-Type", "application/json")
            .query(&[("csrf", csrf)])
            .body(request_body.to_string())
            .send_bpi("发表复杂动态").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_upload_dynamic_pic() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let test_file = Path::new("./assets/test.jpg");
        if !test_file.exists() {
            return Err(BpiError::parse("Test file 'test.jpg' not found.".to_string()));
        }

        let resp = bpi.dynamic_upload_pic(test_file, None).await?;
        let data = resp.into_data()?;

        info!("上传成功！图片 URL: {}", data.image_url);
        assert!(!data.image_url.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_create_text_dynamic() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let content = format!("Rust Bilibili API 指南测试动态：{}", chrono::Local::now());

        let resp = bpi.dynamic_create_text(&content).await?;
        let data = resp.into_data()?;

        info!("动态发布成功！动态ID: {}", data.dynamic_id_str);
        assert!(!data.dynamic_id_str.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_create_complex_dynamic_text() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let contents = vec![DynamicContentItem {
            type_num: 1,
            biz_id: None,
            raw_text: format!("Rust Bilibili API 复杂动态文本测试：{}", 123),
        }];

        let resp = bpi.dynamic_create_complex(1, contents, None, None).await?;
        let data = resp.into_data()?;

        info!("复杂动态发布成功！动态ID: {}", data.dyn_id_str);
        assert!(!data.dyn_id_str.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_create_complex_dynamic_with_pic() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let test_file = Path::new("./assets/test.jpg");
        if !test_file.exists() {
            return Err(BpiError::parse("Test file 'test.jpg' not found.".to_string()));
        }

        let resp = bpi.dynamic_upload_pic(test_file, None).await?;
        let data = resp.into_data()?;

        info!("上传成功！图片 URL: {}", data.image_url);
        let pics = vec![DynamicPic {
            img_src: data.image_url,
            img_height: data.image_height,
            img_width: data.image_width,
            img_size: data.img_size,
        }];

        let contents = vec![DynamicContentItem {
            type_num: 1,
            biz_id: None,
            raw_text: format!("Rust Bilibili API 复杂动态图片测试：{}", 234),
        }];

        let resp = bpi.dynamic_create_complex(2, contents, Some(pics), None).await?;
        let data = resp.into_data()?;

        info!("复杂动态（带图）发布成功！动态ID: {}", data.dyn_id_str);
        assert!(!data.dyn_id_str.is_empty());

        Ok(())
    }
}
