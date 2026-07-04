// 视频观看进度上报相关接口
//
// [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)

// --- 测试模块 ---

use crate::BilibiliRequest;
use crate::BpiError;
use crate::BpiResponse;
use crate::video::VideoClient;

impl<'a> VideoClient<'a> {
    /// 上报视频观看进度（双端）
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/video/report.html#上报视频观看进度)
    ///
    /// # 参数
    /// | 名称      | 类型         | 说明                 |
    /// | --------- | ------------| -------------------- |
    /// | `aid`     | u64         | 稿件 avid            |
    /// | `cid`     | u64         | 视频 cid             |
    /// | `progress`| `Option<u64>` | 观看进度，单位为秒，可选，默认0 |
    pub async fn video_report_watch_progress(
        &self,
        aid: u64,
        cid: u64,
        progress: Option<u64>,
    ) -> Result<BpiResponse<serde_json::Value>, BpiError> {
        let csrf = self.client.csrf()?;

        let mut form = reqwest::multipart::Form::new()
            .text("aid", aid.to_string())
            .text("cid", cid.to_string())
            .text("csrf", csrf.to_string());

        if let Some(p) = progress {
            form = form.text("progress", p.to_string());
        } else {
            form = form.text("progress", "0");
        }

        self.client
            .post("https://api.bilibili.com/x/v2/history/report")
            .multipart(form)
            .send_bpi("上报观看进度")
            .await
    }
}

#[cfg(test)]
mod tests {}
