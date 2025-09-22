use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

/// 会员信息
#[derive(Debug, Clone, Serialize)]
pub struct Vip {
    /// 会员类型 0：无 1：月大会员 2：年度及以上大会员
    #[serde(alias = "vipType")]
    pub vip_type: u8,
    /// 会员状态 0：无 1：有
    pub vip_status: u8,
    #[serde(alias = "vipDueDate", alias = "due_date")]
    /// 会员过期时间 毫秒时间戳
    pub vip_due_date: u64,
    /// 会员标签
    pub label: VipLabel,
    /// 会员昵称颜色 颜色码，一般为#FB7299
    pub nickname_color: String,

    /// 支付类型 0：未开启自动续费 1：已开启自动续费
    pub vip_pay_type: Option<u8>,

    /// 大角色类型 1：月度大会员 3：年度大会员 7：十年大会员 15：百年大会员
    pub role: Option<u8>,

    /// 是否为tv会员
    pub is_tv_vip: Option<bool>,
    /// 电视大会员状态 0：未开通
    pub tv_vip_status: Option<u8>,
    /// 电视大会员支付类型
    pub tv_vip_pay_type: Option<u8>,
    /// 电视大会员过期时间 秒级时间戳
    pub tv_due_date: Option<u64>,

    /// 用户mid
    pub mid: Option<u64>,
    /// 昵称
    pub name: Option<String>,
}

/// 会员标签结构体
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct VipLabel {
    /// 会员类型文案（大会员/年度大会员/十年大会员/百年大会员/最强绿鲤鱼）
    pub text: String,
    /// 会员标签（vip/annual_vip/ten_annual_vip/hundred_annual_vip/fools_day_hundred_annual_vip）
    pub label_theme: String,
    /// 会员标签文本颜色
    pub text_color: String,
    /// 样式
    pub bg_style: u32,
    /// 会员标签背景颜色（颜色码，一般为#FB7299）
    pub bg_color: String,
}

impl<'de> Deserialize<'de> for Vip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VipVisitor;

        impl<'de> Visitor<'de> for VipVisitor {
            type Value = Vip;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a Vip object")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Vip, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut vip_type = None;
                let mut vip_status = None;
                let mut vip_due_date = None;
                let mut label = None;
                let mut nickname_color = None;
                let mut vip_pay_type = None;
                let mut role = None;
                let mut is_tv_vip = None;
                let mut tv_vip_status = None;
                let mut tv_vip_pay_type = None;
                let mut tv_due_date = None;
                let mut mid = None;
                let mut name = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "vipType" | "vip_type" => {
                            if vip_type.is_some() {
                                return Err(de::Error::duplicate_field("vip_type"));
                            }
                            vip_type = Some(map.next_value()?);
                        }
                        "vip_status" => {
                            if vip_status.is_none() {
                                vip_status = Some(map.next_value()?);
                            } else {
                                let _: serde_json::Value = map.next_value()?;
                            }
                        }
                        "vipStatus" => {
                            if vip_status.is_none() {
                                vip_status = Some(map.next_value()?);
                            } else {
                                let _: serde_json::Value = map.next_value()?;
                            }
                        }
                        "status" => {
                            if vip_status.is_none() {
                                vip_status = Some(map.next_value()?);
                            } else {
                                let _: serde_json::Value = map.next_value()?;
                            }
                        }
                        "vipDueDate" | "due_date" | "vip_due_date" => {
                            if vip_due_date.is_none() {
                                vip_due_date = Some(map.next_value()?);
                            } else {
                                let _: serde_json::Value = map.next_value()?;
                            }
                        }
                        "label" => {
                            if label.is_some() {
                                return Err(de::Error::duplicate_field("label"));
                            }
                            label = Some(map.next_value()?);
                        }
                        "nickname_color" => {
                            if nickname_color.is_some() {
                                return Err(de::Error::duplicate_field("nickname_color"));
                            }
                            nickname_color = Some(map.next_value()?);
                        }
                        "vip_pay_type" => {
                            if vip_pay_type.is_some() {
                                return Err(de::Error::duplicate_field("vip_pay_type"));
                            }
                            vip_pay_type = Some(map.next_value()?);
                        }
                        "role" => {
                            if role.is_some() {
                                return Err(de::Error::duplicate_field("role"));
                            }
                            role = Some(map.next_value()?);
                        }
                        "is_tv_vip" => {
                            if is_tv_vip.is_some() {
                                return Err(de::Error::duplicate_field("is_tv_vip"));
                            }
                            is_tv_vip = Some(map.next_value()?);
                        }
                        "tv_vip_status" => {
                            if tv_vip_status.is_some() {
                                return Err(de::Error::duplicate_field("tv_vip_status"));
                            }
                            tv_vip_status = Some(map.next_value()?);
                        }
                        "tv_vip_pay_type" => {
                            if tv_vip_pay_type.is_some() {
                                return Err(de::Error::duplicate_field("tv_vip_pay_type"));
                            }
                            tv_vip_pay_type = Some(map.next_value()?);
                        }
                        "tv_due_date" => {
                            if tv_due_date.is_some() {
                                return Err(de::Error::duplicate_field("tv_due_date"));
                            }
                            tv_due_date = Some(map.next_value()?);
                        }
                        "mid" => {
                            if mid.is_some() {
                                return Err(de::Error::duplicate_field("mid"));
                            }
                            mid = Some(map.next_value()?);
                        }
                        "name" => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        _ => {
                            // 忽略未知字段
                            let _: serde_json::Value = map.next_value()?;
                        }
                    }
                }

                // 检查必需字段
                let vip_type = vip_type.ok_or_else(|| de::Error::missing_field("vip_type"))?;
                let vip_status =
                    vip_status.ok_or_else(|| de::Error::missing_field("vip_status"))?;
                let vip_due_date =
                    vip_due_date.ok_or_else(|| de::Error::missing_field("vip_due_date"))?;
                let label = label.ok_or_else(|| de::Error::missing_field("label"))?;
                let nickname_color =
                    nickname_color.ok_or_else(|| de::Error::missing_field("nickname_color"))?;

                Ok(Vip {
                    vip_type,
                    vip_status,
                    vip_due_date,
                    label,
                    nickname_color,
                    vip_pay_type,
                    role,
                    is_tv_vip,
                    tv_vip_status,
                    tv_vip_pay_type,
                    tv_due_date,
                    mid,
                    name,
                })
            }
        }

        deserializer.deserialize_map(VipVisitor)
    }
}
