//! 查询大会员状态
//!
//! [文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/login/member_center.html#查询大会员状态)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 大会员信息体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VipInfo {
    /// 我的mid
    pub mid: u64,
    /// 大会员类型
    /// - 0：无
    /// - 1：月度
    /// - 2：年度
    pub vip_type: u8,
    /// 大会员状态
    /// - 1：正常
    /// - 2：IP频繁更换，服务被冻结
    /// - 3：大会员账号风险过高，功能锁定
    pub vip_status: u8,
    /// 大会员到期时间（时间戳，毫秒）
    pub vip_due_date: u64,
    /// 是否已购买大会员
    /// - 0：未购买
    /// - 1：已购买
    pub vip_pay_type: u8,
    /// 作用尚不明确
    pub theme_type: u8,
}

impl BpiClient {
    /// 查询大会员状态
    pub async fn member_center_vip_info(&self) -> Result<BpiResponse<VipInfo>, BpiError> {
        let result = self
            .get("https://api.bilibili.com/x/vip/web/user/info")
            .send_bpi("查询大会员状态").await?;

        Ok(result)
    }

    pub async fn is_vip(&self) -> bool {
        self.member_center_vip_info().await
            .ok()
            .and_then(|resp| resp.data)
            .map(|data2| data2.vip_status == 1 && data2.vip_due_date > 0)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_get_vip_info() {
        let bpi = BpiClient::new();

        match bpi.member_center_vip_info().await {
            Ok(resp) => {
                if resp.code == 0 {
                    let data = resp.data.unwrap();
                    info!(
                        "mid: {}, vip_type: {}, vip_status: {}, vip_due_date: {}, vip_pay_type: {}, theme_type: {}",
                        data.mid,
                        data.vip_type,
                        data.vip_status,
                        data.vip_due_date,
                        data.vip_pay_type,
                        data.theme_type
                    );
                } else {
                    info!("请求失败: code={}, message={}", resp.code, resp.message);
                }
            }
            Err(err) => {
                panic!("请求出错: {}", err);
            }
        }
    }
    #[tokio::test]
    async fn test_is_vip() {
        let bpi = BpiClient::new();

        match bpi.is_vip().await {
            true => info!("是大会员"),
            false => info!("不是大会员"),
        }
    }
}
