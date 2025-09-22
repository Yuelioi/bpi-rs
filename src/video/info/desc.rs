//! 查询稿件简介相关接口
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 稿件简介响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoDescResponse {
    /// 返回码
    /// - 0：成功
    /// - -400：请求错误
    /// - 62002：稿件不可见
    pub code: i32,
    /// 错误信息，默认为 "0"
    pub message: String,
    /// ttl，一般为1
    pub ttl: i32,
    /// 简介内容
    pub data: String,
}

impl BpiClient {
    /// 查询稿件简介
    ///
    /// 文档: https://socialsisteryi.github.io/bilibili-API-collect/docs/video/info.html#获取稿件简介
    ///
    /// # 参数
    /// | 名称   | 类型         | 说明                 |
    /// | ------ | ------------| -------------------- |
    /// | `aid`  | Option<u64> | 稿件 avid，可选      |
    /// | `bvid` | Option<&str>| 稿件 bvid，可选      |
    ///
    /// 两者任选一个
    pub async fn video_desc(
        &self,
        aid: Option<u64>,
        bvid: Option<&str>
    ) -> Result<BpiResponse<String>, BpiError> {
        let mut builder = self.get("http://api.bilibili.com/x/web-interface/archive/desc");

        if let Some(aid) = aid {
            builder = builder.query(&[("aid", aid.to_string())]);
        }
        if let Some(bvid) = bvid {
            builder = builder.query(&[("bvid", bvid)]);
        }

        builder.send_bpi("获取稿件简介").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_video_desc() {
        let bpi = BpiClient::new();

        match bpi.video_desc(Some(10001), None).await {
            Ok(resp) => {
                if resp.code == 0 {
                    tracing::info!("稿件简介: {}", resp.data.unwrap());
                } else {
                    tracing::info!("请求失败: code={}, message={}", resp.code, resp.message);
                }
            }
            Err(err) => {
                panic!("请求出错: {}", err);
            }
        }
    }
}
