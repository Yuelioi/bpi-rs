use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 大会员每日经验返回数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VipExperienceData {
    pub r#type: u32,
    /// 是否领取成功
    pub is_grant: bool,
}

impl BpiClient {
    /// 兑换大会员卡券
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/vip/action.html#兑换大会员卡券
    ///
    /// # 参数
    /// | 名称    | 类型 | 说明         |
    /// | ------- | ---- | ------------|
    /// | `type_` | u8   | 卡券类型     |
    pub async fn vip_receive_privilege(
        &self,
        type_: u8
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let params = [
            ("type", type_.to_string()),
            ("csrf", csrf),
        ];
        self
            .post("https://api.bilibili.com/x/vip/privilege/receive")
            .form(&params)
            .send_bpi("兑换大会员卡券").await
    }

    /// 领取大会员每日经验
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/vip/action.html#领取大会员每日经验
    ///
    pub async fn vip_add_experience(&self) -> Result<BpiResponse<VipExperienceData>, BpiError> {
        let csrf = self.csrf()?;
        let params = [("csrf", csrf)];
        self
            .post("https://api.bilibili.com/x/vip/experience/add")
            .form(&params)
            .send_bpi("领取大会员每日经验").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_receive_vip_privilege() {
        let bpi = BpiClient::new();
        // 1: B币券，2: 会员购优惠券等
        let resp = bpi.vip_receive_privilege(1).await;
        match resp {
            Ok(resp) => { assert_eq!(resp.code, 0) }
            Err(e) => {
                assert_eq!(e.code().unwrap(), 69801);
            }
        }
    }

    #[tokio::test]
    async fn test_add_vip_experience() {
        let bpi = BpiClient::new();
        let resp = bpi.vip_add_experience().await;
        match resp {
            Ok(resp) => { assert_eq!(resp.code, 0) }
            Err(e) => {
                // 领过了 请求频繁
                assert!([69198, 6034007].contains(&e.code().unwrap()));
            }
        }
    }
}
