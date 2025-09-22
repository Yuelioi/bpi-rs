use serde::{Deserialize, Serialize};

use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

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

impl BpiClient {
    /// 新版本B币充电
    /// # 参数
    /// - `bp_num`: 贝壳数量，必须在 2-9999 之间
    /// - `is_bp_remains_prior`: 是否优先扣除 B 币余额
    ///   - `true`: B 币充电时请选择 true
    ///   - `false`: 否则从贝壳余额中扣除
    /// - `up_mid`: 充电对象用户的 mid
    /// - `otype`: 充电来源
    ///   - `"up"`: 空间充电
    ///   - `"archive"`: 视频充电
    /// - `oid`: 充电来源代码
    ///   - 空间充电：传充电对象用户 mid
    ///   - 视频充电：传稿件 avid
    pub async fn electric_bcoin_quick_pay(
        &self,
        bp_num: i32,
        is_bp_remains_prior: bool,
        up_mid: i64,
        otype: &str,
        oid: i64,
    ) -> Result<BpiResponse<BcoinQuickPayData>, BpiError> {
        let csrf_owned = self.csrf()?;
        let form = BcoinQuickPayForm {
            bp_num,
            is_bp_remains_prior,
            up_mid,
            otype,
            oid,
            csrf: &csrf_owned,
        };

        self.post("https://api.bilibili.com/x/ugcpay/web/v2/trade/elec/pay/quick")
            .form(&form)
            .send_bpi("新版本B币充电")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_electric_bcoin_quick_pay_min() {
        let bpi = BpiClient::new();
        let resp = bpi
            .electric_bcoin_quick_pay(2, true, 107997089, "up", 107997089)
            .await;
        assert!(resp.is_ok());
        tracing::info!("{:?}", resp);
    }
}
