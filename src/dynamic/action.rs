use serde_json::json;

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

impl BpiClient {
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
        up: u8
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let json_body =
            json!({
            "dyn_id_str": dyn_id_str,
            "up": up ,
            "spmid":"333.1369.0.0",
            "from_spmid":"333.999.0.0"

        });

        self
            .post("https://api.bilibili.com/x/dynamic/feed/dyn/thumb")
            .query(&[("csrf", csrf)])
            .json(&json_body)
            .send_bpi("点赞动态").await
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
        draft_id: &str
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        self
            .post("https://api.vc.bilibili.com/dynamic_draft/v1/dynamic_draft/rm_draft")
            .form(
                &[
                    ("draft_id", draft_id),
                    ("csrf", csrf.as_str()),
                ]
            )
            .send_bpi("删除定时发布动态").await
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
        dyn_str: &str
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let json_body = json!({
            "dyn_str": dyn_str,
        });

        self
            .post("https://api.bilibili.com/x/dynamic/feed/space/set_top")
            .query(&[("csrf", csrf)])
            .json(&json_body)
            .send_bpi("设置置顶动态").await
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
        dyn_str: &str
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;
        let json_body = json!({
            "dyn_str": dyn_str,
        });

        self
            .post("https://api.bilibili.com/x/dynamic/feed/space/rm_top")
            .query(&[("csrf", csrf)])
            .json(&json_body)
            .send_bpi("取消置顶动态").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dynamic_like() {
        let bpi = BpiClient::new();
        let dynamic_id = "1099138163191840776";

        // 测试新版点赞 API
        let resp_new = bpi.dynamic_like(dynamic_id, 1).await;
        assert!(resp_new.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_dynamic_top() {
        let bpi = BpiClient::new();
        // 替换为你需要置顶或取消置顶的动态ID
        let dynamic_id = "1099138163191840776";

        // 测试置顶
        let resp_set_top = bpi.dynamic_set_top(dynamic_id).await;
        assert!(resp_set_top.is_ok());

        // 测试取消置顶
        let resp_rm_top = bpi.dynamic_remove_top(dynamic_id).await;
        if resp_rm_top.is_err() {
            tracing::info!("取消置顶失败: {}", resp_rm_top.err().unwrap());
        }
    }
}
