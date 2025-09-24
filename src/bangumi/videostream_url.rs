//! 视频流URL
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/videostream_url.md)
use crate::models::{ Fnval, VideoQuality };
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 番剧视频流响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiVideoStreamData {
    #[serde(flatten)]
    pub base: crate::models::VideoStreamData,

    /// 响应码
    pub code: u32,
    /// fnver参数
    pub fnver: u32,
    /// 是否为视频项目
    pub video_project: bool,
    /// 数据类型
    pub r#type: String,
    /// bp参数
    pub bp: u32,
    /// VIP类型
    pub vip_type: Option<u32>,
    /// VIP状态
    pub vip_status: Option<u32>,
    /// 是否为DRM
    pub is_drm: bool,
    /// 是否重编码
    pub no_rexcode: u32,
    /// 记录信息
    pub record_info: Option<BangumiRecordInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangumiRecordInfo {
    pub record_icon: String,
    pub record: String,
}

impl BpiClient {
    /// 获取番剧视频流 URL
    ///
    /// # 参数
    /// | 名称    | 类型   | 说明                                |
    /// | ------- | ------ | ----------------------------------- |
    /// | `ep_id` | `Option<u64>`    | 稿件 epid                           |
    /// | `cid`   | `Option<u64>` | 视频 cid（可选，与 ep_id 二选一） |
    /// | `qn`    | u32    | 视频清晰度选择                       |
    /// | `fnval` | u32    | 视频获取方式选择                     |
    ///
    /// # 文档
    /// [获取番剧视频流URL](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/videostream_url.md#获取番剧视频流url)
    pub async fn bangumi_video_stream(
        &self,
        ep_id: Option<u64>,
        cid: Option<u64>,
        qn: Option<VideoQuality>,
        fnval: Option<Fnval>
    ) -> Result<BpiResponse<BangumiVideoStreamData>, BpiError> {
        // 验证参数
        if ep_id.is_none() && cid.is_none() {
            return Err(BpiError::InvalidParameter {
                field: "ep_id/cid",
                message: "ep_id和cid必须提供其中一个",
            });
        }

        let mut params = vec![("fnver", "0".to_string())];

        if fnval.is_some_and(|f| f.is_fourk()) {
            params.push(("fourk", "1".to_string()));
        }

        if let Some(ep) = ep_id {
            params.push(("ep_id", ep.to_string()));
        }
        if let Some(c) = cid {
            params.push(("cid", c.to_string()));
        }
        if let Some(q) = qn {
            params.push(("qn", q.as_u32().to_string()));
        }
        if let Some(fv) = fnval {
            params.push(("fnval", fv.bits().to_string()));
        }

        self
            .get("https://api.bilibili.com/pgc/player/web/playurl")
            .with_bilibili_headers()
            .query(&params)
            .send_bpi("获取番剧视频流URL").await
    }

    /// 获取番剧视频流 URL
    ///
    /// # 参数
    /// | 名称    | 类型   | 说明                                |
    /// | ------- | ------ | ----------------------------------- |
    /// | `ep_id` | u64    | 稿件 epid                           |
    /// | `qn`    | u32    | 视频清晰度选择                       |
    /// | `fnval` | u32    | 视频获取方式选择                     |
    ///
    /// # 文档
    /// [获取番剧视频流URL](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/videostream_url.md#获取番剧视频流url)
    pub async fn bangumi_video_stream_by_epid(
        &self,
        ep_id: u64,
        qn: Option<VideoQuality>,
        fnval: Option<Fnval>
    ) -> Result<BpiResponse<BangumiVideoStreamData>, BpiError> {
        self.bangumi_video_stream(Some(ep_id), None, qn, fnval).await
    }

    /// 获取番剧视频流 URL
    ///
    /// # 参数
    /// | 名称    | 类型   | 说明                                |
    /// | ------- | ------ | ----------------------------------- |
    /// | `cid`   | u64 | 视频 cid（|
    /// | `qn`    | u32    | 视频清晰度选择                       |
    /// | `fnval` | u32    | 视频获取方式选择                     |
    ///
    /// # 文档
    /// [获取番剧视频流URL](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/bangumi/videostream_url.md#获取番剧视频流url)
    pub async fn bangumi_video_stream_by_cid(
        &self,
        cid: u64,
        qn: Option<VideoQuality>,
        fnval: Option<Fnval>
    ) -> Result<BpiResponse<BangumiVideoStreamData>, BpiError> {
        self.bangumi_video_stream(None, Some(cid), qn, fnval).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EP_ID: u64 = 10001; // epid
    const TEST_CID: u64 = 772096113;

    #[tokio::test]
    async fn test_bangumi_video_stream_url_simple() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.bangumi_video_stream_by_epid(
            TEST_EP_ID,
            Some(VideoQuality::P8K),
            Some(
                Fnval::DASH |
                    Fnval::FOURK |
                    Fnval::EIGHTK |
                    Fnval::HDR |
                    Fnval::DOLBY_AUDIO |
                    Fnval::DOLBY_VISION |
                    Fnval::AV1
            )
        ).await?;

        let data = result.into_data()?;
        tracing::info!("==========最佳格式==========\n{:#?}", data.base.best_format());
        tracing::info!("==========最佳视频==========\n{:#?}", data.base.best_video());

        assert!(data.base.timelength.unwrap() > 0);
        assert!(!data.base.accept_format.is_empty());
        assert!(!data.base.accept_quality.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_bangumi_video_stream_url_by_cid() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let result = bpi.bangumi_video_stream_by_cid(
            TEST_CID,
            Some(VideoQuality::P480),
            Some(Fnval::DASH)
        ).await?;
        let data = result.into_data()?;
        tracing::info!("{:#?}", data);
        Ok(())
    }

    #[tokio::test]
    async fn test_bangumi_video_stream_url_no_params() {
        let bpi = BpiClient::new();
        let result = bpi.bangumi_video_stream(None, None, None, None).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        match error {
            BpiError::InvalidParameter { field, message } => {
                assert_eq!(field, "ep_id/cid");
                assert_eq!(message, "ep_id和cid必须提供其中一个");
            }
            _ => panic!("Expected InvalidParameter error"),
        }
    }
}
