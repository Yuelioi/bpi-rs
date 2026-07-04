//! 专栏点赞&投币&收藏
//!
//! [查看 API 文档](https://github.com/Yuelioi/bilibili-API-collect/tree/cfc5fddcc8a94b74d91970bb5b4eaeb349addc47/docs/article/action.md)

/// 投币响应数据
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CoinResponseData {
    /// 是否点赞成功 true：成功 false：失败 已赞过则附加点赞失败
    pub like: bool,
}

#[cfg(test)]
mod tests {}
