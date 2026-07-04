use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResponse;
use crate::dynamic::DynamicClient;
use serde_json::json;

impl<'a> DynamicClient<'a> {
    /// 点赞动态
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `dyn_id_str` | &str | 动态 ID |
    /// | `up` | u8 | 点赞状态：0 切换，1 点赞，2 取消 |
    pub async fn dynamic_like(
        &self,
        dyn_id_str: &str,
        up: u8,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;

        let json_body = json!({
            "dyn_id_str": dyn_id_str,
            "up": up ,
            "spmid":"333.1369.0.0",
            "from_spmid":"333.999.0.0"

        });

        self.client
            .post("https://api.bilibili.com/x/dynamic/feed/dyn/thumb")
            .query(&[("csrf", csrf)])
            .json(&json_body)
            .send_bpi("点赞动态")
            .await
    }

    /// 删除定时发布动态
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `draft_id` | &str | 定时发布动态 ID |
    pub async fn dynamic_remove_draft(
        &self,
        draft_id: &str,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;

        self.client
            .post("https://api.vc.bilibili.com/dynamic_draft/v1/dynamic_draft/rm_draft")
            .form(&[("draft_id", draft_id), ("csrf", csrf.as_str())])
            .send_bpi("删除定时发布动态")
            .await
    }

    /// 设置置顶动态
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `dyn_str` | &str | 动态 ID |
    pub async fn dynamic_set_top(
        &self,
        dyn_str: &str,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;
        let json_body = json!({
            "dyn_str": dyn_str,
        });

        self.client
            .post("https://api.bilibili.com/x/dynamic/feed/space/set_top")
            .query(&[("csrf", csrf)])
            .json(&json_body)
            .send_bpi("设置置顶动态")
            .await
    }

    /// 取消置顶动态
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/dynamic)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `dyn_str` | &str | 动态 ID |
    pub async fn dynamic_remove_top(
        &self,
        dyn_str: &str,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;
        let json_body = json!({
            "dyn_str": dyn_str,
        });

        self.client
            .post("https://api.bilibili.com/x/dynamic/feed/space/rm_top")
            .query(&[("csrf", csrf)])
            .json(&json_body)
            .send_bpi("取消置顶动态")
            .await
    }
}

#[cfg(test)]
mod tests {}
