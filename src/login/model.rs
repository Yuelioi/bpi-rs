use serde::de;
use serde::{Deserialize, Deserializer, Serialize};

use crate::ids::Mid;

/// Login/navigation state returned by `/x/web-interface/nav`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginNav {
    /// Whether the current session is logged in.
    #[serde(rename = "isLogin")]
    pub is_login: bool,
    /// Logged-in user ID. Guest responses return `0`, exposed as `None`.
    #[serde(default, deserialize_with = "deserialize_optional_mid")]
    pub mid: Option<Mid>,
    /// Logged-in display name. Empty guest values are exposed as `None`.
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub uname: Option<String>,
    /// Logged-in avatar URL. Empty guest values are exposed as `None`.
    #[serde(default, deserialize_with = "deserialize_optional_string")]
    pub face: Option<String>,
    /// WBI image keys. Bilibili returns these for guest sessions too.
    pub wbi_img: LoginWbiImg,
}

/// WBI image key URLs embedded in the login nav response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginWbiImg {
    /// URL containing the img key filename.
    pub img_url: String,
    /// URL containing the sub key filename.
    pub sub_url: String,
}

/// Authenticated user's social counters returned by `/x/web-interface/nav/stat`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginStats {
    /// Number of followed users.
    pub following: u64,
    /// Number of followers.
    pub follower: u64,
    /// Number of published dynamic posts.
    pub dynamic_count: u64,
}

/// Current authenticated account coin balance.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LoginCoinBalance {
    /// Current coin balance.
    pub money: f64,
}

/// Today's experience gained from coin operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LoginTodayCoinExp {
    /// Experience gained today.
    pub value: u32,
}

/// Daily reward completion state returned by `/x/member/web/exp/reward`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginDailyReward {
    /// Whether the daily login reward is complete.
    pub login: bool,
    /// Whether the daily watch reward is complete.
    pub watch: bool,
    /// Experience gained from daily coin operations.
    pub coins: u32,
    /// Whether the daily share reward is complete.
    pub share: bool,
    /// Whether the email-binding reward is complete.
    pub email: bool,
    /// Whether the phone-binding reward is complete.
    pub tel: bool,
    /// Whether the safe-question reward is complete.
    pub safe_question: bool,
    /// Whether the real-name verification reward is complete.
    pub identify_card: bool,
}

/// Authenticated account profile returned by `/x/member/web/account`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginAccountInfo {
    /// Current user's ID.
    pub mid: Mid,
    /// Current user's display name.
    pub uname: String,
    /// Login username, which may differ from the display name.
    pub userid: String,
    /// Current profile signature.
    pub sign: String,
    /// Birthday string returned by Bilibili, usually `YYYY-MM-DD`.
    pub birthday: String,
    /// Sex label returned by Bilibili.
    pub sex: String,
    /// Whether the account has not set a custom nickname.
    pub nick_free: bool,
    /// Membership rank string returned by Bilibili.
    pub rank: String,
}

/// Authenticated account VIP state returned by `/x/vip/web/user/info`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoginVipInfo {
    /// Current user's ID.
    pub mid: Mid,
    /// VIP type returned by Bilibili.
    pub vip_type: u8,
    /// VIP status returned by Bilibili.
    pub vip_status: u8,
    /// VIP expiry timestamp in milliseconds.
    pub vip_due_date: u64,
    /// VIP payment type returned by Bilibili.
    pub vip_pay_type: u8,
    /// VIP theme type returned by Bilibili.
    pub theme_type: u8,
}

impl LoginVipInfo {
    /// Returns whether the account currently has an active VIP status.
    pub fn is_active(self) -> bool {
        self.vip_status == 1 && self.vip_due_date > 0
    }
}

fn deserialize_optional_mid<'de, D>(deserializer: D) -> Result<Option<Mid>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<u64>::deserialize(deserializer)? {
        Some(0) | None => Ok(None),
        Some(value) => Mid::new(value).map(Some).map_err(de::Error::custom),
    }
}

fn deserialize_optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<String>::deserialize(deserializer)?
        .and_then(|value| (!value.trim().is_empty()).then_some(value)))
}
