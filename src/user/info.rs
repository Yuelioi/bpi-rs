//! B站用户信息相关接口
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
use crate::models::{ LevelInfo, Nameplate, Official, OfficialVerify, Pendant, Vip, VipLabel };
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

/// 用户空间详细信息响应结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserSpaceInfo {
    /// 用户mid
    pub mid: u64,
    /// 昵称
    pub name: String,
    /// 性别 男/女/保密
    pub sex: String,
    /// 头像链接
    pub face: String,
    /// 是否为NFT头像 0：不是NFT头像 1：是NFT头像
    pub face_nft: u8,
    /// NFT头像类型
    pub face_nft_type: Option<u8>,
    /// 签名
    pub sign: String,
    /// 用户权限等级
    pub rank: u32,
    /// 当前等级 0-6级
    pub level: u8,
    /// 注册时间 此接口返回恒为0
    pub jointime: u64,
    /// 节操值 此接口返回恒为0
    pub moral: u64,
    /// 封禁状态 0：正常 1：被封
    pub silence: u8,
    /// 硬币数 需要登录(Cookie) 只能查看自己的 默认为0
    pub coins: f64,
    /// 是否具有粉丝勋章
    pub fans_badge: bool,
    /// 粉丝勋章信息
    pub fans_medal: Option<FansMedal>,
    /// 认证信息
    pub official: Official,
    /// 会员信息
    pub vip: Vip,
    /// 头像框信息
    pub pendant: Pendant,
    /// 勋章信息
    pub nameplate: Nameplate,
    /// 用户荣誉信息
    pub user_honour_info: UserHonourInfo,
    /// 是否关注此用户 需要登录(Cookie) 未登录恒为false
    pub is_followed: bool,
    /// 主页头图链接
    pub top_photo: String,
    /// 主题信息
    pub theme: serde_json::Value,
    /// 系统通知
    pub sys_notice: SysNotice,
    /// 直播间信息
    pub live_room: LiveRoom,
    /// 生日 MM-DD 如设置隐私为空
    pub birthday: String,
    /// 学校
    pub school: School,
    /// 专业资质信息
    pub profession: Option<Profession>,
    /// 个人标签
    pub tags: Option<Vec<String>>,
    /// 系列信息
    pub series: Series,
    /// 是否为硬核会员 0：否 1：是
    pub is_senior_member: u8,
    /// MCN信息
    pub mcn_info: Option<serde_json::Value>,
    /// Gaia资源类型
    pub gaia_res_type: Option<u8>,
    /// Gaia数据
    pub gaia_data: Option<serde_json::Value>,
    /// 是否存在风险
    pub is_risk: bool,
    /// 充电信息
    pub elec: Elec,
    /// 是否显示老粉计划
    pub contract: Contract,
    /// 证书显示
    pub certificate_show: Option<bool>,
    /// 昵称渲染信息
    pub name_render: Option<serde_json::Value>,
}

/// 粉丝勋章信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FansMedal {
    /// 是否展示
    pub show: bool,
    /// 是否佩戴了粉丝勋章
    pub wear: bool,
    /// 粉丝勋章详细信息
    pub medal: Option<Medal>,
}

/// 粉丝勋章详细信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Medal {
    /// 粉丝勋章等级
    pub level: u8,
    /// 粉丝勋章等级
    pub guard_level: u8,
    /// 粉丝勋章名称
    pub medal_name: String,
}

/// 用户荣誉信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserHonourInfo {
    /// 用户mid
    pub mid: u64,
    /// 颜色
    pub colour: Option<String>,
    /// 标签
    pub tags: Option<Vec<serde_json::Value>>,
}

/// 系统通知
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SysNotice {
    /// 通知id
    pub id: Option<u32>,
    /// 显示文案
    pub content: Option<String>,
    /// 跳转地址
    pub url: Option<String>,
    /// 提示类型
    pub notice_type: Option<u8>,
    /// 前缀图标
    pub icon: Option<String>,
    /// 文字颜色
    pub text_color: Option<String>,
    /// 背景颜色
    pub bg_color: Option<String>,
}

/// 直播间信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LiveRoom {
    /// 直播间状态 0：无房间 1：有房间
    #[serde(rename = "roomStatus")]
    pub room_status: u8,
    /// 直播状态 0：未开播 1：直播中
    #[serde(rename = "liveStatus")]
    pub live_status: u8,
    /// 直播间网页url
    pub url: String,
    /// 直播间标题
    pub title: String,
    /// 直播间封面url
    pub cover: String,
    /// 观看显示信息
    pub watched_show: WatchedShow,
    /// 直播间id
    pub roomid: u64,
    /// 轮播状态 0：未轮播 1：轮播
    #[serde(rename = "roundStatus")]
    pub round_status: u8,
    /// 广播类型
    pub broadcast_type: u8,
}

/// 观看显示信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WatchedShow {
    /// 开关
    pub switch: bool,
    /// 观看人数
    pub num: u64,
    /// 小文本
    pub text_small: String,
    /// 大文本
    pub text_large: String,
    /// 图标
    pub icon: String,
    /// 图标位置
    pub icon_location: String,
    /// 网页图标
    pub icon_web: String,
}

/// 学校信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct School {
    /// 就读大学名称 没有则为空
    pub name: String,
}

/// 专业资质信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Profession {
    /// 资质名称
    pub name: String,
    /// 职位
    pub department: String,
    /// 所属机构
    pub title: String,
    /// 是否显示 0：不显示 1：显示
    pub is_show: u8,
}

/// 系列信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Series {
    /// 用户升级状态
    pub user_upgrade_status: u8,
    /// 是否显示升级窗口
    pub show_upgrade_window: bool,
}

/// 充电信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Elec {
    /// 显示的充电信息
    pub show_info: ShowInfo,
}

/// 显示的充电信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShowInfo {
    /// 是否显示充电按钮
    pub show: bool,
    /// 充电功能开启状态 -1：未开通充电功能 1：已开通自定义充电 2：已开通包月、自定义充电 3：已开通包月高档、自定义充电
    pub state: i8,
    /// 充电按钮显示文字
    pub title: String,
    /// 充电图标
    pub icon: String,
    /// 跳转url
    pub jump_url: String,
}

/// 老粉计划信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Contract {
    /// 是否显示
    pub is_display: bool,
    /// 是否在显示老粉计划 true：显示 false：不显示
    pub is_follow_display: bool,
}

/// 用户名片信息响应结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserCardInfo {
    /// 卡片信息
    pub card: Card,
    /// 是否关注此用户 true：已关注 false：未关注 需要登录(Cookie) 未登录为false
    pub following: bool,
    /// 用户稿件数
    pub archive_count: u32,
    /// 作用尚不明确
    pub article_count: u32,
    /// 粉丝数
    pub follower: u32,
    /// 点赞数
    pub like_num: u32,
}

/// 用户卡片详细信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Card {
    /// 用户mid
    pub mid: String,
    /// 用户昵称
    pub name: String,
    /// 用户性别 男/女/保密
    pub sex: String,
    /// 用户头像链接
    pub face: String,
    /// 显示排名 作用尚不明确
    #[serde(rename = "DisplayRank")]
    pub display_rank: String,
    /// 注册时间 作用尚不明确
    pub regtime: u64,
    /// 用户状态 0：正常 -2：被封禁
    pub spacesta: i32,
    /// 生日 作用尚不明确
    pub birthday: String,
    /// 地点 作用尚不明确
    pub place: String,
    /// 描述 作用尚不明确
    pub description: String,
    /// 文章数 作用尚不明确
    pub article: u32,
    /// 关注列表 作用尚不明确
    pub attentions: Vec<serde_json::Value>,
    /// 粉丝数
    pub fans: u32,
    /// 好友数
    pub friend: u32,
    /// 关注数
    pub attention: u32,
    /// 签名
    pub sign: String,
    /// 等级信息
    pub level_info: LevelInfo,
    /// 挂件信息
    pub pendant: Pendant,
    /// 勋章信息
    pub nameplate: Nameplate,
    /// 认证信息
    #[serde(rename = "Official")]
    pub official: Official,
    /// 认证信息2
    pub official_verify: OfficialVerify,
    /// 大会员状态
    pub vip: Vip,
    /// 主页头图
    pub space: Option<Space>,
}

/// 主页头图信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Space {
    /// 主页头图url 小图
    pub s_img: String,
    /// 主页头图url 正常
    pub l_img: String,
}

/// 用户卡片（精简版）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserCard {
    pub mid: u64,
    pub name: String,
    pub face: String,
    pub sign: String,
    pub rank: i32,
    pub level: i32,
    pub silence: i32,
}

/// 用户详细信息（完整版）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserInfo {
    pub mid: u64,
    pub name: String,
    pub sign: String,
    pub rank: i32,
    pub level: i32,
    pub silence: i32,

    pub sex: Option<String>,
    pub face: String,
    pub vip: Option<UserVip>,
    pub official: Option<UserOfficial>,
    pub is_fake_account: Option<u32>,
    pub expert_info: Option<serde_json::Value>,
}

/// 大会员信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserVip {
    pub r#type: i32,
    pub status: i32,
    pub due_date: i64,
    pub vip_pay_type: i32,
    pub theme_type: i32,
    pub label: Option<VipLabel>,
}

/// 认证信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserOfficial {
    #[serde(default)]
    pub role: i32,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub desc: String,
    #[serde(default)]
    pub r#type: i32,
}

impl BpiClient {
    /// 获取用户空间详细信息
    /// 需要 Wbi 签名认证
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
    ///
    /// - `mid`: 用户 UID
    pub async fn user_space_info(&self, mid: u64) -> Result<BpiResponse<UserSpaceInfo>, BpiError> {
        // 构建查询参数
        let params = vec![("mid", mid.to_string())];

        let params = self.get_wbi_sign2(params).await?;

        self
            .get("https://api.bilibili.com/x/space/wbi/acc/info")
            .query(&params)
            .send_bpi("获取用户空间详细信息").await
    }

    /// 获取用户名片信息
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `mid` | u64 | 用户 UID |
    /// | `photo` | `Option<bool>` | 是否包含主页头图 |
    pub async fn user_card_info(
        &self,
        mid: u64,
        photo: Option<bool>
    ) -> Result<BpiResponse<UserCardInfo>, BpiError> {
        let mut params = vec![("mid", mid.to_string())];

        // 如果指定了photo参数，则添加到请求参数中
        if let Some(photo_value) = photo {
            params.push(("photo", photo_value.to_string()));
        }

        self
            .get("https://api.bilibili.com/x/web-interface/card")
            .query(&params)
            .send_bpi("获取用户名片信息").await
    }

    /// 获取用户名片信息（包含主页头图）
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
    ///
    /// - `mid`: 用户 UID
    pub async fn user_card_info_with_photo(
        &self,
        mid: u64
    ) -> Result<BpiResponse<UserCardInfo>, BpiError> {
        self.user_card_info(mid, Some(true)).await
    }

    /// 获取用户名片信息（不包含主页头图）
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
    ///
    /// - `mid`: 用户 UID
    pub async fn user_card_info_without_photo(
        &self,
        mid: u64
    ) -> Result<BpiResponse<UserCardInfo>, BpiError> {
        self.user_card_info(mid, Some(false)).await
    }

    /// 批量获取用户卡片（精简信息）

    pub async fn user_cards(&self, mids: &[u64]) -> Result<BpiResponse<Vec<UserCard>>, BpiError> {
        let mids_str = mids
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self
            .get("https://api.vc.bilibili.com/account/v1/user/cards")
            .query(&[("uids", mids_str)])
            .send_bpi("批量获取用户卡片").await
    }

    /// 批量获取用户详细信息（带大会员/认证信息）

    pub async fn user_infos(&self, mids: &[u64]) -> Result<BpiResponse<Vec<UserInfo>>, BpiError> {
        let mids_str = mids
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(",");

        self
            .get("https://api.vc.bilibili.com/x/im/user_infos")
            .query(&[("uids", mids_str)])
            .send_bpi("批量获取用户详细信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_user_space_info() {
        tracing::info!("开始测试获取用户空间详细信息");

        let bpi = BpiClient::new();
        let mid = 2; // 测试用户ID

        tracing::info!("测试用户ID: {}", mid);

        let resp = bpi.user_space_info(mid).await;

        match &resp {
            Ok(response) => {
                let data = response.clone().data.unwrap();
                tracing::info!("请求成功，返回码: {}", response.code);
                tracing::info!("用户昵称: {}", data.name);
                tracing::info!("用户等级: {}", data.level);
                tracing::info!("是否为会员: {}", data.vip.vip_type > 0);
                tracing::info!(
                    "粉丝数量: {}",
                    data.fans_medal.as_ref().map_or(0, |_| 1)
                );
            }
            Err(e) => {
                tracing::error!("请求失败: {:?}", e);
            }
        }

        assert!(resp.is_ok());
        tracing::info!("测试完成");
    }

    #[tokio::test]
    async fn test_get_user_space_info_nonexistent() {
        tracing::info!("开始测试获取不存在用户的空间详细信息");

        let bpi = BpiClient::new();
        let mid = 0; // 不存在的用户ID

        tracing::info!("测试用户ID: {}", mid);

        let resp = bpi.user_space_info(mid).await;

        match &resp {
            Ok(response) => {
                tracing::info!("请求返回码: {}", response.code);
                tracing::info!("错误信息: {}", response.message);
                // 应该返回 -404 表示用户不存在
                assert_eq!(response.code, -404);
            }
            Err(e) => {
                tracing::error!("请求失败: {:?}", e);
            }
        }

        tracing::info!("测试完成");
    }

    #[tokio::test]
    async fn test_get_user_card_info() {
        tracing::info!("开始测试获取用户名片信息");

        let bpi = BpiClient::new();
        let mid = 2; // 测试用户ID

        tracing::info!("测试用户ID: {}", mid);

        let resp = bpi.user_card_info(mid, None).await;

        match &resp {
            Ok(response) => {
                let data = response.clone().data.unwrap();

                tracing::info!("请求成功，返回码: {}", response.code);
                tracing::info!("用户昵称: {}", data.card.name);
                tracing::info!("用户性别: {}", data.card.sex);
                tracing::info!("用户等级: {}", data.card.level_info.current_level);
                tracing::info!("是否关注: {}", data.following);
                tracing::info!("稿件数: {}", data.archive_count);
                tracing::info!("粉丝数: {}", data.follower);
                tracing::info!("点赞数: {}", data.like_num);
                tracing::info!("用户签名: {}", data.card.sign);

                // 认证信息
                let official = &data.card.official;
                if official.r#type >= 0 {
                    tracing::info!("认证类型: {}", official.r#type);
                    tracing::info!("认证信息: {}", official.title);
                } else {
                    tracing::info!("用户未认证");
                }

                // VIP信息
                let vip = &data.card.vip;
                if vip.vip_status > 0 {
                    tracing::info!("大会员状态: 已开通");
                    tracing::info!("大会员类型: {}", vip.vip_type);
                } else {
                    tracing::info!("大会员状态: 未开通");
                }
            }
            Err(e) => {
                tracing::error!("请求失败: {:?}", e);
            }
        }

        assert!(resp.is_ok());
        tracing::info!("测试完成");
    }

    #[tokio::test]
    async fn test_get_user_card_info_with_photo() {
        tracing::info!("开始测试获取用户名片信息（包含主页头图）");

        let bpi = BpiClient::new();
        let mid = 2; // 测试用户ID

        tracing::info!("测试用户ID: {}", mid);

        let resp = bpi.user_card_info_with_photo(mid).await;

        match &resp {
            Ok(response) => {
                let data = response.clone().data.unwrap();

                tracing::info!("请求成功，返回码: {}", response.code);
                tracing::info!("用户昵称: {}", data.card.name);

                // 检查主页头图信息
                if let Some(space) = &data.card.space {
                    tracing::info!("主页头图（小）: {}", space.s_img);
                    tracing::info!("主页头图（正常）: {}", space.l_img);
                } else {
                    tracing::info!("用户没有设置主页头图");
                }

                // 挂件信息
                let pendant = &data.card.pendant;
                if pendant.pid > 0 {
                    tracing::info!("挂件名称: {}", pendant.name);
                    tracing::info!("挂件图片: {}", pendant.image);
                } else {
                    tracing::info!("用户没有佩戴挂件");
                }

                // 勋章信息
                let nameplate = &data.card.nameplate;
                if nameplate.nid > 0 {
                    tracing::info!("勋章名称: {}", nameplate.name);
                    tracing::info!("勋章等级: {}", nameplate.level);
                } else {
                    tracing::info!("用户没有佩戴勋章");
                }
            }
            Err(e) => {
                tracing::error!("请求失败: {:?}", e);
            }
        }

        assert!(resp.is_ok());
        tracing::info!("测试完成");
    }

    #[tokio::test]
    async fn test_get_user_card_info_without_photo() {
        tracing::info!("开始测试获取用户名片信息（不包含主页头图）");

        let bpi = BpiClient::new();
        let mid = 123456; // 测试用户ID

        tracing::info!("测试用户ID: {}", mid);

        let resp = bpi.user_card_info_without_photo(mid).await;

        match &resp {
            Ok(response) => {
                let data = response.clone().data.unwrap();

                tracing::info!("请求成功，返回码: {}", response.code);
                tracing::info!("用户昵称: {}", data.card.name);
                tracing::info!("粉丝数: {}", data.card.fans);
                tracing::info!("关注数: {}", data.card.attention);

                // 应该没有主页头图信息
                if data.card.space.is_none() {
                    tracing::info!("正确：没有返回主页头图信息");
                } else {
                    tracing::warn!("注意：返回了主页头图信息");
                }
            }
            Err(e) => {
                tracing::error!("请求失败: {:?}", e);
            }
        }

        assert!(resp.is_ok());
        tracing::info!("测试完成");
    }

    #[tokio::test]
    async fn test_get_user_card_info_invalid_user() {
        tracing::info!("开始测试获取不存在用户的名片信息");

        let bpi = BpiClient::new();
        let mid = 0; // 不存在的用户ID

        tracing::info!("测试用户ID: {}", mid);

        let resp = bpi.user_card_info(mid, None).await;

        match &resp {
            Ok(response) => {
                tracing::info!("请求返回码: {}", response.code);
                tracing::info!("错误信息: {}", response.message);
                // 应该返回错误码
                if response.code != 0 {
                    tracing::info!("正确：返回了错误码 {}", response.code);
                }
            }
            Err(e) => {
                tracing::error!("请求失败: {:?}", e);
            }
        }

        tracing::info!("测试完成");
    }

    #[tokio::test]
    async fn test_get_user_card_info_banned_user() {
        tracing::info!("开始测试获取被封禁用户的名片信息");

        let bpi = BpiClient::new();
        let mid = 999999999; // 假设的被封禁用户ID

        tracing::info!("测试用户ID: {}", mid);

        let resp = bpi.user_card_info(mid, None).await;

        match &resp {
            Ok(response) => {
                tracing::info!("请求成功，返回码: {}", response.code);

                if response.code == 0 {
                    let data = response.clone().data.unwrap();

                    let spacesta = data.card.spacesta;
                    if spacesta == -2 {
                        tracing::info!("用户状态: 被封禁");
                    } else if spacesta == 0 {
                        tracing::info!("用户状态: 正常");
                    } else {
                        tracing::info!("用户状态: 未知 ({})", spacesta);
                    }
                }
            }
            Err(e) => {
                tracing::error!("请求失败: {:?}", e);
            }
        }

        tracing::info!("测试完成");
    }

    #[tokio::test]
    async fn test_user_cards_and_infos() {
        let bpi = BpiClient::new();

        // 测试精简版
        let cards = bpi.user_cards(&[2, 3]).await.unwrap();
        tracing::info!("用户卡片: {:?}", cards.data);

        // 测试完整版
        let infos = bpi.user_infos(&[2, 3]).await.unwrap();
        tracing::info!("用户详细信息: {:?}", infos.data);
    }
}
