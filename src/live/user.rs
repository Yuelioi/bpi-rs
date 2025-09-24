use serde::{ Deserialize, Serialize };

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

// ================= 数据结构 =================

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PageInfo {
    /// 页码总长度
    pub total_page: i32,
    /// 当前返回的页码
    pub cur_page: i32,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct FansMedalItem {
    /// 可否删除
    #[serde(rename = "can_deleted")]
    pub can_delete: bool,
    /// 日经验上限（原力值）
    pub day_limit: i32,
    /// 大航海等级
    pub guard_level: i32,
    /// 加成状态
    pub guard_medal_title: String,
    /// 当前已得亲密度
    pub intimacy: i32,
    /// 是否点亮
    pub is_lighted: i32,
    /// 勋章等级
    pub level: i32,
    /// 勋章名
    pub medal_name: String,
    /// 勋章边框颜色信息
    pub medal_color_border: i32,
    /// 勋章起始颜色
    pub medal_color_start: i32,
    /// 勋章结束颜色
    pub medal_color_end: i32,
    /// 粉丝勋章id
    pub medal_id: i64,
    /// 升级所需经验
    pub next_intimacy: i32,
    /// 本日亲密度
    pub today_feed: i32,
    /// 直播间房间号
    pub roomid: i64,
    /// 状态
    pub status: i32,
    /// up主mid
    pub target_id: i64,
    /// up主用户名
    pub target_name: String,
    /// up主用户名
    pub uname: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct MyMedalsData {
    /// 勋章数量
    pub count: i32,
    /// 粉丝勋章信息本体
    pub items: Vec<FansMedalItem>,
    /// 页码信息
    pub page_info: PageInfo,
}

pub type MyMedalsResponse = BpiResponse<MyMedalsData>;

// ================= 实现 =================

impl BpiClient {
    /// 获取自己持有的粉丝勋章信息
    ///

    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/live)
    pub async fn live_my_medals(
        &self,
        page: i32,
        page_size: i32
    ) -> Result<MyMedalsResponse, BpiError> {
        let params = [
            ("page", page.to_string()),
            ("page_size", page_size.to_string()),
        ];

        let resp: MyMedalsResponse = self
            .get("https://api.live.bilibili.com/xlive/app-ucenter/v1/user/GetMyMedals")
            .query(&params)
            .send_bpi("获取自己持有的粉丝勋章信息").await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_my_medals() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let resp = bpi.live_my_medals(1, 10).await?;

        tracing::info!("{:?}", resp.data);
        Ok(())
    }
}
