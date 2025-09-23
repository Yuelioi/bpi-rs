//! 购买漫画章节
//!
//! https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/manga/Comic.md

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 漫画图片信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct MangaImage {
    /// 图片的路径，不包含host
    pub path: String,
    /// 图片宽度，单位：像素px
    pub x: i32,
    /// 图片高度，单位：像素px
    pub y: i32,
    /// 视频路径
    pub video_path: String,
    /// 视频大小
    pub video_size: String,
}

/// 漫画视频信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct MangaVideo {
    /// 视频ID
    pub svid: String,
    /// 文件名
    pub filename: String,
    /// 路由
    pub route: String,
    /// 资源
    pub resource: Vec<serde_json::Value>,
    /// 原始宽度
    pub raw_width: String,
    /// 原始高度
    pub raw_height: String,
    /// 原始旋转
    pub raw_rotate: String,
    /// 图片URL列表
    pub img_urls: Vec<String>,
    /// 二进制URL
    pub bin_url: String,
    /// 图片X长度
    pub img_x_len: i32,
    /// 图片X大小
    pub img_x_size: i32,
    /// 图片Y长度
    pub img_y_len: i32,
    /// 图片Y大小
    pub img_y_size: i32,
}

/// 漫画图片索引数据
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ImageIndexData {
    /// .index 文件路径
    pub path: String,
    /// 本话图片信息
    pub images: Vec<MangaImage>,
    /// 本话信息最后修改时间
    pub last_modified: String,
    /// 图片host，通常为 https://manga.hdslb.com
    pub host: String,
    /// 视频信息
    pub video: MangaVideo,
}

/// 图片token请求参数
#[derive(Debug, Clone, Serialize)]
pub struct ImageTokenRequest {
    /// 请求token的图片地址数组
    pub urls: String,
}

/// 图片token信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ImageToken {
    /// 图片下载的地址
    pub url: String,
    /// 图片下载需要的token
    pub token: String,
}

// ================= 实现 =================

impl BpiClient {
    /// 获取当前话全部图片地址
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/manga
    #[allow(dead_code)]
    async fn manga_image_index(&self, ep_id: i32) -> Result<BpiResponse<ImageIndexData>, BpiError> {
        let params = serde_json::json!({
            "ep_id": ep_id
        });

        self
            .post("https://manga.bilibili.com/twirp/comic.v1.Comic/GetImageIndex")
            .json(&params)
            .send_bpi("获取漫画图片索引").await
    }

    /// 获取某一图片的token
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/manga
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `image_path` | &str | 图片相对路径，如 /bfs/... |
    #[allow(dead_code)]
    async fn manga_image_token(
        &self,
        image_path: &str
    ) -> Result<BpiResponse<Vec<ImageToken>>, BpiError> {
        // 构建请求的图片URL
        let url = format!("[\"https://i0.hdslb.com{}\"]", image_path);

        let params = ImageTokenRequest { urls: url };

        self
            .post("https://manga.bilibili.com/twirp/comic.v1.Comic/ImageToken")
            .json(&params)
            .send_bpi("获取漫画图片token").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_manga_image_index() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let ep_id = 482133;

        let result = bpi.manga_image_index(ep_id).await?;
        let data = result.into_data()?;

        tracing::info!("{:#?}", data);

        Ok(())
    }
}
