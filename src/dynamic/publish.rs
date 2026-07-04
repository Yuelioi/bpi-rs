use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {}
