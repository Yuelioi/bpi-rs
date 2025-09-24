//! 查询视频分P列表 (avid/bvid 转 cid)
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)
//!
//! [文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/video/video.html#查询视频分p列表)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 分P分辨率信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageDimension {
    /// 宽度
    pub width: u32,
    /// 高度
    pub height: u32,
    /// 是否将宽高对换，0: 正常，1: 对换
    pub rotate: u8,
}

/// 分P信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageItem {
    /// 当前分P cid
    pub cid: u64,
    /// 当前分P页码
    pub page: u32,
    /// 视频来源：vupload/芒果TV/腾讯
    pub from: String,
    /// 当前分P标题
    pub part: String,
    /// 当前分P持续时间（秒）
    pub duration: u32,
    /// 站外视频 vid
    pub vid: String,
    /// 站外跳转 URL
    pub weblink: String,
    /// 分辨率信息，部分视频可能不存在
    pub dimension: Option<PageDimension>,
    /// 分P封面
    pub first_frame: Option<String>,

    pub ctime: u64,
}

/// 分P列表响应
type PageListResponse = BpiResponse<Vec<PageItem>>;
impl BpiClient {
    /// 查询视频分P列表 获取视频cid
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/video/video.html#查询视频分p列表)
    ///
    /// # 参数
    /// | 名称   | 类型         | 说明                 |
    /// | ------ | ------------| -------------------- |
    /// | `aid`  | `Option<u64>` | 稿件 avid，可选      |
    /// | `bvid` | `Option<&str>`| 稿件 bvid，可选      |
    ///
    /// 两者任选一个
    pub async fn video_pagelist(
        &self,
        aid: Option<u64>,
        bvid: Option<&str>
    ) -> Result<PageListResponse, BpiError> {
        let aid = aid.map(|v| v.to_string());
        let bvid = bvid.map(|v| v.to_string());
        self
            .get("https://api.bilibili.com/x/player/pagelist")
            .query(
                &[
                    ("aid", aid),
                    ("bvid", bvid),
                ]
            )
            .send_bpi("查询视频分P列表").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_video_pagelist() {
        let bpi = BpiClient::new();

        match bpi.video_pagelist(Some(10001), None).await {
            Ok(resp) => {
                if resp.code == 0 {
                    for item in resp.data.unwrap() {
                        tracing::info!(
                            "P{}: {}, cid={}, duration={}秒",
                            item.page,
                            item.part,
                            item.cid,
                            item.duration
                        );
                        if let Some(dim) = item.dimension {
                            tracing::info!(
                                "分辨率: {}x{}, rotate={}",
                                dim.width,
                                dim.height,
                                dim.rotate
                            );
                        }
                    }
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
