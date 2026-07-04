//! 漫画图片下载模型
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/manga/Comic.md)
//!
//! Status: not implemented.
//!
//! The current web reader requires browser-generated proof fields (`m2` for
//! `GetImageIndex` and `m1` for `ImageToken`). The legacy request shape returns
//! API `code = 99`, so this module intentionally exposes data models only until
//! the SDK has an explicit proof-provider boundary.

use serde::{Deserialize, Serialize};

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
    /// 图片host，通常为 `https://manga.hdslb.com`
    pub host: String,
    /// 视频信息
    pub video: MangaVideo,
}

/// 图片token信息
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ImageToken {
    /// 图片下载的地址
    pub url: String,
    /// 图片下载需要的token
    pub token: String,
}
