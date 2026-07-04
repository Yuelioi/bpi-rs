use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct BcoinQuickPayForm<'a> {
    pub bp_num: i32,
    pub is_bp_remains_prior: bool,
    pub up_mid: i64,
    pub otype: &'a str, // "up" | "archive"
    pub oid: i64,
    pub csrf: &'a str,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BcoinQuickPayData {
    /// 本用户 mid
    pub mid: i64,
    /// 目标用户 mid
    pub up_mid: i64,
    /// 留言 token（用于添加充电留言）
    pub order_no: String,
    /// 充电贝壳数（字符串）
    pub bp_num: String,
    /// 获得经验数
    pub exp: u32,
    /// 返回结果
    /// - `4`：成功
    /// - `-2`：低于 20 电池下限
    /// - `-4`：B 币不足
    pub status: i32,
    /// 错误信息（默认为空）
    pub msg: String,
}

#[cfg(test)]
mod tests {}
