//! 导航栏用户信息
//!
//! [查看 API 文档](https://socialsisteryi.github.io/bilibili-API-collect/docs/login/login_info_info.html#导航栏用户信息)
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// ============ 导航栏用户信息 ============

/// 用户信息数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavData {
    /// 是否已登录 false：未登录 true：已登录
    #[serde(rename = "isLogin")]
    pub is_login: bool,

    /// Wbi 签名实时口令（该字段即使用户未登录也存在）
    pub wbi_img: WbiImg,

    /// 是否验证邮箱地址 0：未验证 1：已验证
    pub email_verified: i32,

    /// 用户头像 url
    pub face: String,

    /// 头像 NFT 类型
    pub face_nft: i32,

    /// 等级信息
    pub level_info: LevelInfo,

    /// 用户 mid
    pub mid: u64,

    /// 是否验证手机号 0：未验证 1：已验证
    pub mobile_verified: i32,

    /// 拥有硬币数
    pub money: f64,

    /// 当前节操值，上限为70
    pub moral: i32,

    /// 认证信息
    pub official: Official,

    /// 认证信息 2
    #[serde(rename = "officialVerify")]
    pub official_verify: OfficialVerify,

    /// 头像框信息
    pub pendant: Pendant,

    /// 未知字段
    pub scores: i32,

    /// 用户昵称
    pub uname: String,

    /// 会员到期时间毫秒时间戳
    #[serde(rename = "vipDueDate")]
    pub vip_due_date: u64,

    /// 大会员状态
    /// - 1：正常
    /// - 2：IP频繁更换，服务被冻结
    /// - 3：大会员账号风险过高，功能锁定
    #[serde(rename = "vipStatus")]
    pub vip_status: i32,

    /// 会员类型 0：无 1：月度大会员 2：年度及以上大会员
    #[serde(rename = "vipType")]
    pub vip_type: i32,

    /// 会员开通状态 0：无 1：有
    pub vip_pay_type: i32,

    /// 未知字段
    pub vip_theme_type: i32,

    /// 会员标签
    pub vip_label: VipLabel,

    /// 是否显示会员图标 0：不显示 1：显示
    pub vip_avatar_subscript: i32,

    /// 会员昵称颜色（颜色码）
    pub vip_nickname_color: String,

    /// 会员信息
    pub vip: Vip,

    /// B币钱包信息
    pub wallet: Wallet,

    /// 是否拥有推广商品 false：无 true：有
    pub has_shop: bool,

    /// 商品推广页面 url
    pub shop_url: String,

    /// 是否硬核会员 0：非硬核会员 1：硬核会员
    pub is_senior_member: i32,

    /// 是否风纪委员 true：风纪委员 false：非风纪委员
    pub is_jury: bool,

    /// 用户名渲染信息
    pub name_render: Option<serde_json::Value>,
}

/// 钱包信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub mid: u64,
    pub bcoin_balance: i64,
    pub coupon_balance: i64,
    pub coupon_due_time: i64,
}

/// Wbi 图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WbiImg {
    pub img_url: String,
    pub sub_url: String,
}

use crate::models::{ LevelInfo, Official, OfficialVerify, Pendant, Vip, VipLabel };

#[derive(Debug, Clone, Serialize)]
pub struct User {
    is_login: bool, // 是否登录
    face: String, // 头像
    mid: u64, // 用户id
    money: f64, // 硬币
    uname: String, // 用户昵称
    is_vip: bool, // 是否vip
}

impl BpiClient {
    /// 获取导航栏用户信息
    pub async fn login_info_nav_info(&self) -> Result<BpiResponse<NavData>, BpiError> {
        self
            .get("https://api.bilibili.com/x/web-interface/nav")
            .send_bpi("获取导航栏用户信息").await
    }

    /// 检查是否已登录
    pub async fn is_logged_in(&self) -> bool {
        self.login_info_nav_info().await.is_ok()
    }

    /// 获取用户基本信息
    pub async fn login_info_user_info(&self) -> Result<User, BpiError> {
        let nav_response = self.login_info_nav_info().await;

        match nav_response {
            Ok(nav_response) =>
                Ok(
                    if let Some(data) = nav_response.data {
                        User {
                            is_login: data.is_login,
                            face: data.face,
                            mid: data.mid,
                            money: data.money,
                            uname: data.uname,
                            is_vip: data.vip.vip_status == 1,
                        }
                    } else {
                        User {
                            is_login: false,
                            face: String::new(),
                            mid: 0,
                            money: 0.0,
                            uname: String::new(),
                            is_vip: false,
                        }
                    }
                ),
            _ => Err(BpiError::auth("账号未登录".to_string())),
        }
    }
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    #[tokio::test]
    /// 测试登录
    async fn test_bilibili_uinfo() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let resp = bpi.login_info_nav_info().await?;

        if resp.code == 0 {
            let data = resp.data.unwrap();
            info!("登录成功！UID={} 昵称={} ", data.mid, data.uname);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_user_info() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let user_info = bpi.login_info_user_info().await?;

        info!("用户信息：{:?}", user_info);

        Ok(())
    }
}
