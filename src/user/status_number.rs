//! B站用户关系、UP主状态、导航栏等相关接口
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// --- 响应数据结构体 ---

/// 用户关系状态数响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelationStatResponseData {
    /// 目标用户 mid
    pub mid: u64,
    /// 关注数
    pub following: u64,
    /// 悄悄关注数
    pub whisper: u64,
    /// 黑名单数
    pub black: u64,
    /// 粉丝数
    pub follower: u64,
}

/// UP主状态数中的视频播放量
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpstatArchive {
    /// 视频播放量
    pub view: u64,
}

/// UP主状态数中的专栏阅读量
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpstatArticle {
    /// 专栏阅读量
    pub view: u64,
}

/// UP主状态数响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpstatResponseData {
    /// 视频播放量
    pub archive: UpstatArchive,
    /// 专栏阅读量
    pub article: UpstatArticle,
    /// 获赞次数
    pub likes: u64,
}

/// 用户导航栏状态数中的视频列表数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NavnumChannel {
    /// 视频列表数
    pub master: u64,
    /// 视频列表数
    pub guest: u64,
}

/// 用户导航栏状态数中的收藏夹数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NavnumFavourite {
    /// 全部收藏夹数
    pub master: u64,
    /// 公开收藏夹数
    pub guest: u64,
}

/// 用户导航栏状态数响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NavnumResponseData {
    /// 投稿视频数
    pub video: u64,
    /// 追番数
    pub bangumi: u64,
    /// 追剧数
    pub cinema: u64,
    /// 视频列表数
    pub channel: NavnumChannel,
    /// 收藏夹数
    pub favourite: NavnumFavourite,
    /// 关注 TAG 数
    pub tag: u64,
    /// 投稿专栏数
    pub article: u64,
    pub playlist: u64,
    /// 投稿图文数
    pub album: u64,
    /// 投稿音频数
    pub audio: u64,
    /// 投稿课程数
    pub pugv: u64,
    /// 动态数
    pub opus: u64,
    /// 视频合集数
    #[serde(rename = "season_num")]
    pub season_num: u64,
}

/// 相簿投稿数响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AlbumCountResponseData {
    /// 相簿总数
    pub all_count: u64,
    /// 发布绘画数
    pub draw_count: u64,
    /// 发布摄影数
    pub photo_count: u64,
    /// 发布日常（图片动态）数
    pub daily_count: u64,
}

// --- API 实现 ---

impl BpiClient {
    /// 获取用户关系状态数
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明           |
    /// | ------ | ------ | -------------- |
    /// | `vmid` | u64    | 目标用户 mid   |
    pub async fn user_relation_stat(
        &self,
        vmid: u64
    ) -> Result<BpiResponse<RelationStatResponseData>, BpiError> {
        self
            .get("https://api.bilibili.com/x/relation/stat")
            .query(&[("vmid", &vmid.to_string())])
            .send_bpi("获取用户关系状态数").await
    }

    /// 获取UP主状态数
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明           |
    /// | ------ | ------ | -------------- |
    /// | `mid`  | u64    | 目标用户 mid   |
    pub async fn user_up_stat(
        &self,
        mid: u64
    ) -> Result<BpiResponse<UpstatResponseData>, BpiError> {
        self
            .get("https://api.bilibili.com/x/space/upstat")
            .query(&[("mid", &mid.to_string())])
            .send_bpi("获取 UP 主状态数").await
    }

    /// 获取用户导航栏状态数
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明           |
    /// | ------ | ------ | -------------- |
    /// | `mid`  | u64    | 目标用户 mid   |
    pub async fn user_navnum(&self, mid: u64) -> Result<BpiResponse<NavnumResponseData>, BpiError> {
        self
            .get("https://api.bilibili.com/x/space/navnum")
            .query(&[("mid", &mid.to_string())])
            .send_bpi("获取用户导航栏状态数").await
    }

    /// 获取相簿投稿数
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
    ///
    /// # 参数
    /// | 名称   | 类型   | 说明           |
    /// | ------ | ------ | -------------- |
    /// | `uid`  | u64    | 目标用户 mid   |
    pub async fn user_album_count(
        &self,
        uid: u64
    ) -> Result<BpiResponse<AlbumCountResponseData>, BpiError> {
        self
            .get("https://api.vc.bilibili.com/link_draw/v1/doc/upload_count")
            .query(&[("uid", &uid.to_string())])
            .send_bpi("获取相簿投稿数").await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    // 请在运行测试前设置环境变量 `BPI_COOKIE`，以包含 SESSDATA 等登录信息
    // mid 根据实际情况修改
    const TEST_MID: u64 = 332704117;
    const TEST_UP_MID: u64 = 456664753;
    const TEST_NAV_MID: u64 = 645769214;

    #[tokio::test]
    async fn test_get_relation_stat() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.user_relation_stat(TEST_MID).await?;
        let data = resp.into_data()?;

        info!("关系状态数: {:?}", data);
        assert_eq!(data.mid, TEST_MID);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_up_stat() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.user_up_stat(TEST_UP_MID).await?;
        let data = resp.into_data()?;

        info!("UP主状态数: {:?}", data);
        assert!(data.likes > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_nav_num() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.user_navnum(TEST_NAV_MID).await?;
        let data = resp.into_data()?;

        info!("用户导航栏状态数: {:?}", data);
        assert!(data.video > 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_album_count() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.user_album_count(TEST_NAV_MID).await?;
        let data = resp.into_data()?;

        info!("相簿投稿数: {:?}", data);
        assert!(data.all_count > 0);

        Ok(())
    }
}
