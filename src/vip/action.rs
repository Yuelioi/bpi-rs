use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResponse;
use crate::vip::VipClient;
use serde::{Deserialize, Serialize};

/// 大会员每日经验返回数据

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VipExperienceData {
    pub r#type: u32,
    /// 是否领取成功
    pub is_grant: bool,
}

impl<'a> VipClient<'a> {
    /// 兑换大会员卡券
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/vip/action.html#兑换大会员卡券)
    ///
    /// # 参数
    /// | 名称    | 类型 | 说明         |
    /// | ------- | ---- | ------------|
    /// | `type_` | u8   | 卡券类型     |
    pub async fn vip_receive_privilege(
        &self,
        type_: u8,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;

        let params = [("type", type_.to_string()), ("csrf", csrf)];
        self.client
            .post("https://api.bilibili.com/x/vip/privilege/receive")
            .form(&params)
            .send_bpi("兑换大会员卡券")
            .await
    }

    /// 领取大会员每日经验
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/vip/action.html#领取大会员每日经验)
    ///
    pub async fn vip_add_experience(&self) -> Result<BpiResponse<VipExperienceData>, BpiError> {
        let csrf = self.client.csrf()?;
        let params = [("csrf", csrf)];
        self.client
            .post("https://api.bilibili.com/x/vip/experience/add")
            .form(&params)
            .send_bpi("领取大会员每日经验")
            .await
    }
}

#[cfg(test)]
mod tests {}
