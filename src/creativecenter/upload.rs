//! 创作中心上传 API
//!
//! [参考文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/creativecenter/upload.md)

use serde::{Deserialize, Serialize};

/// 上传封面返回结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadCoverData {
    pub url: String,
}
