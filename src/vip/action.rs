use serde::{Deserialize, Serialize};

/// 大会员每日经验返回数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VipExperienceData {
    pub r#type: u32,
    /// 是否领取成功
    pub is_grant: bool,
}

#[cfg(test)]
mod tests {}
