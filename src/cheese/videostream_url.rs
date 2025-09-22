//! 课程视频流 URL API
//!
//! 参考文档：https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/cheese/videostream_url.md

use std::collections::HashMap;

use crate::models::{DashStreams, Fnval, SupportFormat, VideoQuality};
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};

/// 课程视频流数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseVideoStreamData {
    #[serde(flatten)]
    pub base: crate::models::VideoStreamData,

    /// 定位参数
    pub seek_param: String,
    /// 是否为视频项目
    pub video_project: bool,
    /// 数据类型
    #[serde(rename = "type")]
    pub data_type: String,
    /// 结果状态
    pub result: String,
    /// 定位类型
    pub seek_type: String,
    /// 来源
    pub from: String,
    /// 是否重编码
    pub no_rexcode: i32,
    /// 响应消息
    pub message: String,
    /// 分片视频信息
    pub fragment_videos: Option<Vec<FragmentVideo>>,
    /// 状态码
    pub status: i32,
}

/// 分片视频
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentVideo {
    pub fragment_info: FragmentInfo,
    pub playable_status: bool,
    pub video_info: VideoInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentInfo {
    pub fragment_type: String,
    pub index: i64,
    pub aid: i64,
    pub fragment_position: String,
    pub cid: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub no_rexcode: i64,
    pub fnval: i64,
    pub video_project: bool,
    pub expire_time: i64,
    pub backup_url: Vec<Option<serde_json::Value>>,
    pub fnver: i64,
    pub support_formats: Vec<String>,
    pub support_description: Vec<String>,
    #[serde(rename = "type")]
    pub video_info_type: String,
    pub url: String,
    pub quality: i64,
    pub timelength: i64,
    pub volume: CourseVolume,
    pub accept_formats: Vec<SupportFormat>,
    pub support_quality: Vec<i64>,
    pub file_info: HashMap<String, FileInfo>,
    pub dash: DashStreams,
    pub video_codecid: i64,
    pub cid: i64,
}

/// 音量信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseVolume {
    pub measured_i: f64,
    pub target_i: f64,
    pub target_offset: f64,
    pub measured_lra: f64,
    pub target_tp: f64,
    pub measured_tp: f64,
    pub measured_threshold: f64,
    pub multi_scene_args: MultiSceneArgs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSceneArgs {
    pub normal_target_i: String,
    pub undersized_target_i: String,
    pub high_dynamic_target_i: String,
}

/// 文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub infos: Vec<FileInfoEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfoEntry {
    pub ahead: String,
    pub vhead: String,
    pub filesize: i64,
    pub order: i64,
    pub timelength: i64,
}

impl BpiClient {
    /// 获取课程视频流 URL
    ///
    /// 获取课程视频的播放流地址，支持多种视频质量和格式。
    /// 仅课程视频可用，与普通视频 API 不互通。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `avid` | u64 | 课程视频 avid |
    /// | `ep_id` | u64 | 课程分集 ep_id |
    /// | `cid` | u64 | 视频 cid |
    /// | `qn` | Option<VideoQuality> | 视频质量，可选 |
    /// | `fnval` | Option<Fnval> | 视频格式标志，可选 |
    ///
    /// # 注意
    /// 需要 Cookie（SESSDATA）和 Referer: https://www.bilibili.com
    ///
    /// # 文档
    /// [获取课程视频流 URL](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/cheese/videostream_url.md)
    pub async fn cheese_video_stream(
        &self,
        avid: u64,
        ep_id: u64,
        cid: u64,
        qn: Option<VideoQuality>,
        fnval: Option<Fnval>,
    ) -> Result<BpiResponse<CourseVideoStreamData>, BpiError> {
        let mut params = vec![
            ("avid", avid.to_string()),
            ("ep_id", ep_id.to_string()),
            ("cid", cid.to_string()),
            ("fnver", "0".to_string()),
        ];

        if fnval.is_some_and(|f| f.is_fourk()) {
            params.push(("fourk", "1".to_string()));
        }

        if let Some(q) = qn {
            params.push(("qn", q.as_u32().to_string()));
        }
        if let Some(fv) = fnval {
            params.push(("fnval", fv.bits().to_string()));
        }

        self.get("https://api.bilibili.com/pugv/player/web/playurl")
            .with_bilibili_headers()
            .query(&params)
            .send_bpi("获取课程视频流 URL")
            .await
    }
}

// ==========================
// 测试
// ==========================

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_AVID: u64 = 997984154;
    const TEST_EP_ID: u64 = 163956;
    const TEST_CID: u64 = 1183682680;

    #[tokio::test]
    async fn test_cheese_playurl() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let data = bpi
            .cheese_video_stream(
                TEST_AVID,
                TEST_EP_ID,
                TEST_CID,
                Some(VideoQuality::P8K),
                Some(
                    Fnval::DASH
                        | Fnval::FOURK
                        | Fnval::EIGHTK
                        | Fnval::HDR
                        | Fnval::DOLBY_AUDIO
                        | Fnval::DOLBY_VISION
                        | Fnval::AV1,
                ),
            )
            .await?
            .into_data()?;

        tracing::info!("{:#?}", data);

        Ok(())
    }
}
