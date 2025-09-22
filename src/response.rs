use crate::err::error::BpiError;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BpiResponse<T> {
    /// 返回值 0：成功
    #[serde(default)]
    pub code: i32,

    #[serde(alias = "result")]
    pub data: Option<T>,

    /// 错误信息，默认为0
    #[serde(default)]
    pub message: String,

    /// 状态, 部分接口需要
    #[serde(default)]
    pub status: bool,
}

impl<T> BpiResponse<T> {
    pub fn into_data(self) -> Result<T, BpiError> {
        self.data.ok_or(BpiError::missing_data())
    }
}
