use serde::{ Deserialize, Serialize };

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

// ================= 数据结构 =================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LiveSubArea {
    /// 子分区id
    pub id: String,
    /// 父分区id
    pub parent_id: String,
    /// 旧分区id
    pub old_area_id: String,
    /// 子分区名
    pub name: String,
    /// 活动id
    pub act_id: String,
    /// pk状态
    pub pk_status: String,
    /// 是否为热门分区
    pub hot_status: i32,
    /// 锁定状态
    pub lock_status: String,
    /// 子分区标志图片url
    pub pic: String,
    /// 父分区名
    pub parent_name: String,
    /// 区域类型
    pub area_type: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct LiveParentArea {
    /// 父分区id
    pub id: i32,
    /// 父分区名
    pub name: String,
    /// 子分区列表
    pub list: Vec<LiveSubArea>,
}

impl BpiClient {
    /// 获取全部直播间分区列表
    ///

    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live)
    pub async fn live_area_list(&self) -> Result<BpiResponse<Vec<LiveParentArea>>, BpiError> {
        let resp = self
            .get("https://api.live.bilibili.com/room/v1/Area/getList")
            .send_bpi("获取全部直播间分区列表").await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_live_area_list() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let resp = bpi.live_area_list().await?;

        let data = resp.data.unwrap();
        assert!(data.len() > 0);
        Ok(())
    }
}
