use serde::Serialize;
use thiserror::Error;

/// 错误类型分类
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ErrorCategory {
    /// 权限认证类错误
    Auth,
    /// 请求参数类错误
    Request,
    /// 服务器类错误
    Server,
    /// 业务逻辑类错误
    Business,
    /// 网络类错误
    Network,
    /// 未知错误
    Unknown,
}

#[derive(Debug, Error, Serialize)]
pub enum BpiError {
    /// 网络请求失败
    #[error("网络请求失败: {message}")]
    Network { message: String },

    /// HTTP状态码错误
    #[error("HTTP请求失败，状态码: {status}")]
    Http { status: u16 },

    /// JSON解析失败
    #[error("数据解析失败: {message}")]
    Parse { message: String },

    /// API返回的业务错误
    #[error("API错误 [{code}]: {message}")]
    Api {
        code: i32,
        message: String,
        category: ErrorCategory,
    },

    /// 验证错误
    #[error("验证失败: {message}")]
    Authentication { message: String },

    /// 参数错误
    #[error("参数错误 [{field}]: {message}")]
    InvalidParameter {
        field: &'static str,
        message: &'static str,
    },
}

impl BpiError {
    pub fn missing_csrf() -> Self {
        BpiError::InvalidParameter {
            field: "csrf",
            message: "缺少CSRF",
        }
    }

    pub fn missing_data() -> Self {
        BpiError::Parse {
            message: "数据解析失败, 缺少data字段".to_string(),
        }
    }

    pub fn auth_required() -> Self {
        BpiError::Authentication {
            message: "需要登录".to_string(),
        }
    }
}

/// 生成Error的From实现
impl BpiError {
    /// 根据API错误码创建BpiError
    pub fn from_code(code: i32) -> Self {
        let message = super::code::get_error_message(code);
        let category = super::code::categorize_error(code);

        BpiError::Api {
            code,
            message,
            category,
        }
    }

    // 不在错误码表中的API错误
    pub fn from_code_message(code: i32, message: String) -> Self {
        let category = super::code::categorize_error(code);
        BpiError::Api {
            code,
            message,
            category,
        }
    }

    /// 从API响应创建BpiError
    pub fn from_api_response<T>(resp: crate::response::BpiResponse<T>) -> Self {
        if resp.code == 0 {
            return BpiError::Api {
                code: 0,
                message: "API返回成功状态但被当作错误处理".to_string(),
                category: ErrorCategory::Unknown,
            };
        }
        Self::from_code(resp.code)
    }
}

/// 获取错误属性
impl BpiError {
    /// 获取错误码
    pub fn code(&self) -> Option<i32> {
        match self {
            BpiError::Api { code, .. } => Some(*code),
            _ => None,
        }
    }

    /// 获取错误分类
    pub fn category(&self) -> ErrorCategory {
        match self {
            BpiError::Api { category, .. } => category.clone(),
            BpiError::Network { .. } => ErrorCategory::Network,
            BpiError::Http { .. } => ErrorCategory::Network,
            BpiError::Parse { .. } => ErrorCategory::Request,
            BpiError::InvalidParameter { .. } => ErrorCategory::Request,
            BpiError::Authentication { .. } => ErrorCategory::Auth,
        }
    }
}

/// 错误创建函数
impl BpiError {
    /// 创建网络错误
    pub fn network(message: impl Into<String>) -> Self {
        BpiError::Network {
            message: message.into(),
        }
    }

    /// 创建HTTP错误
    pub fn http(status: u16) -> Self {
        BpiError::Http { status }
    }

    /// 创建解析错误
    pub fn parse(message: impl Into<String>) -> Self {
        BpiError::Parse {
            message: message.into(),
        }
    }

    /// 创建参数错误
    pub fn invalid_parameter(field: &'static str, message: &'static str) -> Self {
        BpiError::InvalidParameter { field, message }
    }

    pub fn auth(message: impl Into<String>) -> Self {
        BpiError::Api {
            code: 401,
            message: message.into(),
            category: ErrorCategory::Auth,
        }
    }
}

/// 错误判断
impl BpiError {
    /// 判断是否需要用户登录
    pub fn requires_login(&self) -> bool {
        matches!(self.code(), Some(-101) | Some(-401))
    }

    /// 判断是否为权限问题
    pub fn is_permission_error(&self) -> bool {
        matches!(self.category(), ErrorCategory::Auth)
            || matches!(self.code(), Some(-403) | Some(-4))
    }

    /// 判断是否需要VIP权限
    pub fn requires_vip(&self) -> bool {
        matches!(self.code(), Some(-106) | Some(-650))
    }

    /// 判断是否为业务逻辑错误
    pub fn is_business_error(&self) -> bool {
        matches!(self.category(), ErrorCategory::Business)
    }
}
