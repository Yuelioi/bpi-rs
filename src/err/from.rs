use super::error::BpiError;

// 从reqwest错误转换
impl From<reqwest::Error> for BpiError {
    fn from(err: reqwest::Error) -> Self {
        let message = if err.is_timeout() {
            "请求超时".to_string()
        } else if err.is_connect() {
            "连接失败".to_string()
        } else if err.is_request() {
            format!("请求错误: {}", err)
        } else if err.is_decode() {
            format!("响应解析错误: {}", err)
        } else {
            err.to_string()
        };

        BpiError::Network { message }
    }
}

// 从API响应转换
impl<T> From<crate::response::BpiResponse<T>> for BpiError {
    fn from(resp: crate::response::BpiResponse<T>) -> Self {
        BpiError::from_api_response(resp)
    }
}

// 从JSON序列化错误转换
impl From<serde_json::Error> for BpiError {
    fn from(err: serde_json::Error) -> Self {
        BpiError::Parse {
            message: err.to_string(),
        }
    }
}
