//! B站 web 播放器相关接口
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/video)
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// --- 响应数据结构体 ---

/// web 播放器信息响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerInfoResponseData {
    /// 视频 aid
    pub aid: u64,
    /// 视频 bvid
    pub bvid: String,
    pub allow_bp: bool,
    pub no_share: bool,
    /// 视频 cid
    pub cid: u64,
    /// webmask 防挡字幕信息
    pub dm_mask: Option<DmMaskInfo>,
    /// 字幕信息
    pub subtitle: Option<SubtitleInfo>,
    /// 分段章节信息
    #[serde(default)]
    pub view_points: Vec<ViewPoint>,
    /// 请求 IP 信息
    pub ip_info: Option<serde_json::Value>,
    /// 登录用户 mid
    pub login_mid: u64,
    pub login_mid_hash: Option<String>,
    /// 是否为该视频 UP 主
    pub is_owner: bool,
    pub name: String,
    pub permission: String,
    /// 登录用户等级信息
    pub level_info: Option<serde_json::Value>,
    /// 登录用户 VIP 信息
    pub vip: Option<serde_json::Value>,
    /// 答题状态
    pub answer_status: u8,
    pub block_time: u64,
    pub role: String,
    /// 上次观看时间
    pub last_play_time: u64,
    /// 上次观看 cid
    pub last_play_cid: u64,
    /// 当前 UNIX 秒级时间戳
    pub now_time: u64,
    /// 在线人数
    pub online_count: Option<u64>,
    /// 是否必须登陆才能查看字幕
    pub need_login_subtitle: bool,
    /// 预告提示
    pub preview_toast: String,
    /// 互动视频资讯
    pub interaction: Option<InteractionInfo>,
    pub options: Option<PlayerOptions>,
    pub guide_attention: Option<serde_json::Value>,
    pub jump_card: Option<serde_json::Value>,
    pub operation_card: Option<serde_json::Value>,
    pub online_switch: Option<serde_json::Value>,
    pub fawkes: Option<serde_json::Value>,
    pub show_switch: Option<serde_json::Value>,
    /// 背景音乐信息
    pub bgm_info: Option<BgmInfo>,
    pub toast_block: bool,
    /// 是否为充电专属视频
    pub is_upower_exclusive: bool,
    pub is_upower_play: bool,
    pub is_ugc_pay_preview: bool,
    /// 充电专属视频信息
    pub elec_high_level: Option<ElecHighLevel>,
    pub disable_show_up_info: bool,
}

/// webmask 防挡字幕信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DmMaskInfo {
    /// 视频 cid
    pub cid: u64,
    pub plat: u8,
    /// webmask 取样 fps
    pub fps: u64,
    pub time: u64,
    /// webmask 资源 url
    pub mask_url: String,
}

/// 字幕信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubtitleInfo {
    pub allow_submit: bool,
    pub lan: String,
    pub lan_doc: String,
    #[serde(default)]
    pub subtitles: Vec<SubtitleItem>,
}

/// 单个字幕信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubtitleItem {
    pub ai_status: u8,
    pub ai_type: u8,
    pub id: u64,
    pub id_str: String,
    pub is_lock: bool,
    /// 语言类型英文字母缩写
    pub lan: String,
    /// 语言类型中文名称
    pub lan_doc: String,
    /// 资源 url 地址
    pub subtitle_url: String,
    #[serde(rename = "type")]
    pub subtitle_type: u8,
}

/// 分段章节信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewPoint {
    /// 分段章节名
    pub content: String,
    /// 分段章节起始秒数
    pub from: u64,
    /// 分段章节结束秒数
    pub to: u64,
    #[serde(rename = "type")]
    pub point_type: u8,
    /// 图片资源地址
    #[serde(rename = "imgUrl")]
    pub img_url: String,
    #[serde(rename = "logoUrl")]
    pub logo_url: String,
    pub team_type: String,
    pub team_name: String,
}

/// 互动视频资讯
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InteractionInfo {
    /// 剧情图 id
    pub graph_version: u64,
    /// 未登录有机会返回
    pub msg: Option<String>,
    /// 错误信息
    pub error_toast: Option<String>,
    pub mark: Option<u8>,
    pub need_reload: Option<u8>,
}

/// 播放器选项
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlayerOptions {
    /// 是否 360 全景视频
    pub is_360: bool,
    pub without_vip: bool,
}

/// 背景音乐信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BgmInfo {
    /// 音乐 id
    pub music_id: String,
    /// 音乐标题
    pub music_title: String,
    /// 跳转 URL
    pub jump_url: String,
}

/// 充电专属视频信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ElecHighLevel {
    /// 解锁视频所需最低定价档位的代码
    pub privilege_type: u64,
    /// 提示标题
    pub title: String,
    /// 提示子标题
    pub sub_title: String,
    /// 是否显示按钮
    pub show_button: bool,
    /// 按钮文本
    pub button_text: String,
    /// 跳转url信息
    pub jump_url: Option<serde_json::Value>,
    /// 充电介绍语
    pub intro: String,
    #[serde(default)]
    pub open: bool,
    #[serde(default)]
    pub new: bool,
    #[serde(default)]
    pub question_text: String,
    #[serde(default)]
    pub qa_detail_link: String,
}

impl BpiClient {
    /// 获取 web 播放器信息
    ///
    /// # 文档
    /// [查看API文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/video/player.html#获取web播放器信息)
    ///
    /// # 参数
    /// | 名称        | 类型           | 说明                 |
    /// | ----------- | --------------| -------------------- |
    /// | `aid`       | `Option<u64>`   | 稿件 avid，可选      |
    /// | `bvid`      | `Option<&str>`  | 稿件 bvid，可选      |
    /// | `cid`       | u64           | 稿件 cid             |
    /// | `season_id` | `Option<u64>`   | 番剧 season_id，可选 |
    /// | `ep_id`     | `Option<u64>`   | 剧集 ep_id，可选     |
    ///
    /// `aid` 和 `bvid` 必须提供一个。
    pub async fn video_player_info_v2(
        &self,
        aid: Option<u64>,
        bvid: Option<&str>,
        cid: u64,
        season_id: Option<u64>,
        ep_id: Option<u64>
    ) -> Result<BpiResponse<PlayerInfoResponseData>, BpiError> {
        if aid.is_none() && bvid.is_none() {
            return Err(BpiError::parse("必须提供 aid 或 bvid"));
        }

        let mut params = vec![("cid", cid.to_string())];
        if let Some(a) = aid {
            params.push(("aid", a.to_string()));
        }
        if let Some(b) = bvid {
            params.push(("bvid", b.to_string()));
        }
        if let Some(s) = season_id {
            params.push(("season_id", s.to_string()));
        }
        if let Some(e) = ep_id {
            params.push(("ep_id", e.to_string()));
        }
        let params = self.get_wbi_sign2(params).await?;

        self
            .get("https://api.bilibili.com/x/player/wbi/v2")
            .query(&params)
            .send_bpi("获取 web 播放器信息").await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    const TEST_AID: u64 = 1906473802;
    const TEST_CID: u64 = 636329244;

    #[tokio::test]
    async fn test_video_player_info_v2_by_aid() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.video_player_info_v2(Some(TEST_AID), None, TEST_CID, None, None).await?;
        let data = resp.into_data()?;

        info!("播放器信息: {:?}", data);

        assert_eq!(data.aid, TEST_AID);
        assert_eq!(data.cid, TEST_CID);

        Ok(())
    }

    #[tokio::test]
    async fn test_video_player_info_v2_by_bvid() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.video_player_info_v2(Some(TEST_AID), None, TEST_CID, None, None).await?;
        let data = resp.into_data()?;

        info!("播放器信息: {:?}", data);

        assert_eq!(data.aid, TEST_AID);
        assert_eq!(data.cid, TEST_CID);

        Ok(())
    }
}
