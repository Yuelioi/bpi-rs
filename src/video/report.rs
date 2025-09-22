//! 视频观看进度上报相关接口
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

impl BpiClient {
    /// 上报视频观看进度（双端）
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/video/report.html#上报视频观看进度
    ///
    /// # 参数
    /// | 名称      | 类型         | 说明                 |
    /// | --------- | ------------| -------------------- |
    /// | `aid`     | u64         | 稿件 avid            |
    /// | `cid`     | u64         | 视频 cid             |
    /// | `progress`| Option<u64> | 观看进度，单位为秒，可选，默认0 |
    pub async fn video_report_watch_progress(
        &self,
        aid: u64,
        cid: u64,
        progress: Option<u64>,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.csrf()?;

        let mut form = reqwest::multipart::Form::new()
            .text("aid", aid.to_string())
            .text("cid", cid.to_string())
            .text("csrf", csrf.to_string());

        if let Some(p) = progress {
            form = form.text("progress", p.to_string());
        } else {
            form = form.text("progress", "0");
        }

        self.post("https://api.bilibili.com/x/v2/history/report")
            .multipart(form)
            .send_bpi("上报观看进度")
            .await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    const TEST_AID: u64 = 10001;
    const TEST_CID: u64 = 16546;

    #[tokio::test]

    async fn test_report_watch_progress() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        // 上报观看进度为 1248 秒
        let resp = bpi
            .video_report_watch_progress(TEST_AID, TEST_CID, Some(120))
            .await?;

        info!("上报观看进度结果: {:?}", resp);

        Ok(())
    }
}
