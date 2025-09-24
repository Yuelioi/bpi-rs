use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };

// --- Structs for `getChargeRecord` ---

/// 充电自动续费详情
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Renew {
    /// 自己的mid
    pub uid: u64,
    /// UP主的mid
    pub ruid: u64,
    /// 充电类型 172:一个月 173:连续包月 174:连续包年
    pub goods_id: u64,
    /// 充电状态 1
    pub status: u8,
    /// 下次续费时间秒级时间戳
    pub next_execute_time: u64,
    /// 签约时间秒级时间戳
    pub signed_time: u64,
    /// 下次续费金额单位为千分之一元人民币
    pub signed_price: u64,
    /// 签约平台 2:微信支付 4:支付宝
    pub pay_channel: u8,
    /// 下次充电天数
    pub period: u64,
    /// 充电渠道
    pub mobile_app: String,
}

/// 充电档位详情
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChargeItem {
    /// 充电档位代码
    pub privilege_type: u64,
    /// 充电图标
    pub icon: String,
    /// 充电档位名称
    pub name: String,
    /// 该档位过期时间秒级时间戳
    pub expire_time: u64,
    /// 充电自动续费详情
    pub renew: Option<Renew>,
    /// 该档位生效时间秒级时间戳
    pub start_time: u64,
    /// 充电自动续费列表
    pub renew_list: Option<Vec<Renew>>,
}

/// 包月充电UP主
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChargeUp {
    /// 充电UP主mid
    pub up_uid: u64,
    /// 充电UP主昵称
    pub user_name: String,
    /// 充电UP主头像url
    pub user_face: String,
    /// 充电详情
    pub item: Vec<ChargeItem>,
    /// 开始充电时间秒级时间戳
    pub start: u64,
    /// 是否可对UP主进行高档充电
    pub high_level_state: u8,
    /// 是否可对UP主进行专属问答 0:否 1:是 2:状态未知
    pub elec_reply_state: u8,
}

/// 包月充电列表数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChargeRecordData {
    /// 包月充电UP主列表
    pub list: Option<Vec<ChargeUp>>,
    /// 当前页数
    pub page: u64,
    /// 当前分页大小
    pub page_size: u64,
    /// 总页数
    pub total_page: u64,
    /// 用户总数
    pub total_num: u64,
    /// 是否有更多用户 0:否 1:是
    pub is_more: u8,
}

// --- Structs for `upower/item/detail` ---

/// 充电用户排名
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpowerRankUser {
    /// 充电用户索引
    pub rank: u64,
    /// 充电用户mid
    pub mid: u64,
    /// 充电用户昵称
    pub nickname: String,
    /// 充电用户头像url
    pub avatar: String,
}

/// 充电详情
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpowerRank {
    /// 充电用户总数
    pub total: u64,
    /// 充电总数文字说明
    pub total_desc: String,
    /// 充电用户列表
    pub list: Vec<UpowerRankUser>,
}

/// 充电介绍
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemDetailIntro {
    /// 充电介绍视频AV号
    pub intro_video_aid: String,
    /// 充电介绍语
    pub welcomes: String,
}

/// UP主信息卡片
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpUserCard {
    /// UP主头像url
    pub avatar: String,
    /// UP主昵称
    pub nickname: String,
}

/// 不同充电档位下的充电权益数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpowerRightCount {
    #[serde(flatten)]
    pub counts: HashMap<String, u64>,
}

/// 包月充电详情数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpowerItemDetail {
    /// 充电详情
    pub upower_rank: UpowerRank,
    /// 充电欢迎语信息
    pub item: ItemDetailIntro,
    /// UP主信息
    pub user_card: UpUserCard,
    /// UP主开通的充电等级 1:非高档充电 2:高档充电
    pub upower_level: u8,
    /// 是否可对UP主进行专属问答
    pub elec_reply_state: u8,
    /// 包月充电券信息
    pub voucher_state: serde_json::Value,
    /// 不同充电档位下的充电权益数
    pub upower_right_count: UpowerRightCount,
    /// 享有的权益仅为粉丝勋章
    pub only_contain_medal: bool,
    /// 当前给该UP主包月充电的档位
    pub privilege_type: u64,
}

// --- Structs for `charge/follow/info` ---

/// UP主信息卡片
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpCard {
    /// UP主mid
    pub mid: u64,
    /// UP主昵称
    pub nickname: String,
    /// UP主认证信息
    pub official_title: String,
    /// UP主头像url
    pub avatar: String,
}

/// 用户信息卡片
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserCard {
    /// 用户头像url
    pub avatar: String,
    /// 用户昵称
    pub nickname: String,
}

/// 与UP主的包月充电关系数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChargeFollowInfo {
    /// 已保持多少天包月充电状态
    pub days: u64,
    /// UP主信息
    pub up_card: UpCard,
    /// 自己的信息
    pub user_card: UserCard,
    /// 剩余天数 未处于包月充电状态为-1
    pub remain_days: i64,
    /// 剩余的天数是否小于1天 0:否 1:是 未处于包月充电状态为0
    pub remain_less_1day: u8,
    /// 充电详情
    pub upower_rank: UpowerRank,
    /// 充电图标url 仅在处于包月充电状态时有内容
    pub upower_icon: String,
    /// 当前自己享有该UP主的充电权益数
    pub upower_right_count: i64,
    /// 享有的权益仅为粉丝勋章
    pub only_contain_medal: bool,
    /// 当前给该UP主包月充电的档位代码
    pub privilege_type: u64,
    /// 充电挑战信息
    pub challenge_info: ChallengeInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChallengeInfo {
    pub challenge_id: String,
    pub description: String,
    pub challenge_type: i64,
    pub remaining_days: i64,
    pub end_time: String,
    pub progress: i64,
    pub targets: Vec<serde_json::Value>,
    pub state: i64,
    pub end_time_unix: i64,
    pub pub_dyn: i64,
    pub dyn_content: String,
}

/// UP主信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpInfo {
    /// UP主mid
    pub mid: u64,
    /// UP主昵称
    pub nickname: String,
    /// UP主头像url
    pub avatar: String,
    /// UP主认证类型
    pub r#type: i32,
    /// UP主认证文字
    pub title: String,
    /// UP主充电功能开启状态
    pub upower_state: u8,
}

/// 充电用户排名
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RankInfo {
    /// 充电用户mid
    pub mid: u64,
    /// 充电用户昵称
    pub nickname: String,
    /// 充电用户头像url
    pub avatar: String,
    /// 充电用户排名
    pub rank: u64,
    /// 包月充电天数
    pub day: u64,
    /// 包月充电过期时间恒为0
    pub expire_at: u64,
    /// 剩余天数恒为0
    pub remain_days: u64,
}

/// 自己的充电关系信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberUserInfo {
    /// 用户mid
    pub mid: u64,
    /// 用户昵称
    pub nickname: String,
    /// 用户头像url
    pub avatar: String,
    /// 包月充电排名
    pub rank: i64,
    /// 包月充电天数
    pub day: u64,
    /// 包月充电过期时间秒级时间戳
    pub expire_at: u64,
    /// 剩余天数
    pub remain_days: u64,
}

/// 充电档位信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LevelInfo {
    /// 充电档位代码
    pub privilege_type: u64,
    /// 档位名称
    pub name: String,
    /// 档位价格单位为百分之一元人民币
    pub price: u64,
    /// 当前档位的用户总数
    pub member_total: u64,
}

/// 包月充电用户排名数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemberRankData {
    /// UP主信息
    pub up_info: UpInfo,
    /// 当前档位的充电用户排名
    pub rank_info: Vec<RankInfo>,
    /// 自己在该档位下与UP主的充电关系
    pub user_info: MemberUserInfo,
    /// 当前档位充电用户总数
    pub member_total: u64,
    /// 当前充电档位代码
    pub privilege_type: u64,
    /// 自己是否给该UP主包月充电过
    pub is_charge: bool,
    /// 可显示排名的充电档位代码列表
    pub tabs: Vec<u64>,
    /// 可显示排名的充电档位信息
    pub level_info: Vec<LevelInfo>,
}

impl BpiClient {
    /// 获取包月充电列表
    ///
    /// 注意: 此接口需要登录态 (Cookie: SESSDATA)
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/electric)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `page` | u64 | 页码 |
    /// | `charge_type` | u32 | 充电状态：1 使用中，2 已过期 |
    pub async fn electric_charge_record(
        &self,
        page: u64,
        charge_type: u32
    ) -> Result<BpiResponse<ChargeRecordData>, BpiError> {
        self
            .get("https://api.live.bilibili.com/xlive/revenue/v1/guard/getChargeRecord")
            .query(
                &[
                    ("page", page.to_string()),
                    ("type", charge_type.to_string()),
                ]
            )
            .send_bpi("获取包月充电列表").await
    }

    /// UP主包月充电详情
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/electric)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `up_mid` | u64 | 目标用户 mid |
    pub async fn electric_upower_item_detail(
        &self,
        up_mid: u64
    ) -> Result<BpiResponse<UpowerItemDetail>, BpiError> {
        self
            .get("https://api.bilibili.com/x/upower/item/detail")
            .query(&[("up_mid", up_mid)])
            .send_bpi("获取UP主包月充电详情").await
    }

    /// 与UP主的包月充电关系
    ///
    /// 注意: 此接口需要登录态 (Cookie: SESSDATA)
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/electric)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `up_mid` | u64 | 目标用户 mid |
    pub async fn electric_charge_follow_info(
        &self,
        up_mid: u64
    ) -> Result<BpiResponse<ChargeFollowInfo>, BpiError> {
        self
            .get("https://api.bilibili.com/x/upower/charge/follow/info")
            .query(&[("up_mid", up_mid)])
            .send_bpi("获取与UP主的包月充电关系").await
    }

    /// 包月充电用户排名
    ///
    /// 注意: 此接口需要登录态 (Cookie: SESSDATA)
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/electric)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `up_mid` | u64 | 目标用户 mid |
    /// | `pn` | u64 | 页码 |
    /// | `ps` | u64 | 每页项数，最大 101 |
    /// | `privilege_type` | `Option<u64>` | 充电档位代码 |
    pub async fn electric_upower_member_rank(
        &self,
        up_mid: u64,
        pn: u64,
        ps: u64,
        privilege_type: Option<u64>
    ) -> Result<BpiResponse<MemberRankData>, BpiError> {
        let mut req = self.get("https://api.bilibili.com/x/upower/up/member/rank/v2").query(
            &[
                ("up_mid", up_mid),
                ("pn", pn),
                ("ps", ps),
            ]
        );

        if let Some(ptype) = privilege_type {
            req = req.query(&[("privilege_type", ptype)]);
        }

        req.send_bpi("获取包月充电用户排名").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    async fn test_get_charge_record() {
        let bpi = BpiClient::new();
        // 获取自己使用中的包月充电列表
        let resp = bpi.electric_charge_record(1, 1).await;
        info!("响应: {:?}", resp);
        assert!(resp.is_ok());

        if let Ok(response) = resp {
            if let Some(list) = response.data.unwrap().list {
                info!("找到 {} 个正在充电的UP主", list.len());
            } else {
                info!("没有正在充电的UP主");
            }
        }
    }

    #[tokio::test]
    async fn test_get_upower_item_detail() {
        let bpi = BpiClient::new();
        // 替换为有效的UP主mid
        let up_mid = 1265680561;
        let resp = bpi.electric_upower_item_detail(up_mid).await;
        info!("响应: {:?}", resp);
        assert!(resp.is_ok());

        if let Ok(response) = resp {
            let data = response.data.unwrap();
            info!("UP主 {} 的充电总人数: {}", data.user_card.nickname, data.upower_rank.total);
        }
    }

    #[tokio::test]
    async fn test_get_charge_follow_info() {
        let bpi = BpiClient::new();
        let up_mid = 293793435;
        let resp = bpi.electric_charge_follow_info(up_mid).await;
        info!("响应: {:?}", resp);
        assert!(resp.is_ok());

        if let Ok(response) = resp {
            let data = response.data.unwrap();
            info!("与UP主 {} 的充电关系：已保持 {} 天", data.up_card.nickname, data.days);
        }
    }

    #[tokio::test]
    async fn test_get_upower_member_rank() {
        let bpi = BpiClient::new();
        // 替换为有效的UP主mid
        let up_mid = 1265680561;
        // 获取所有档位的用户排名
        let resp = bpi.electric_upower_member_rank(up_mid, 1, 10, None).await;
        info!("响应: {:?}", resp);
        assert!(resp.is_ok());

        if let Ok(response) = resp {
            let data = response.data.unwrap();

            info!("当前档位充电用户总数: {}", data.member_total);
            if let Some(first_rank) = data.rank_info.first() {
                info!("排名第一的用户: {}", first_rank.nickname);
            }
        }
    }
}
