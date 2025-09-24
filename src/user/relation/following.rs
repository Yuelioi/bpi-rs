//! B站用户关注列表相关接口
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// --- 响应数据结构体 ---

/// 用户认证信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OfficialVerify {
    /// 用户认证类型，-1: 无, 0: UP 主认证, 1: 机构认证
    #[serde(rename = "type")]
    pub verify_type: i8,
    /// 用户认证信息，无则为空
    pub desc: String,
}

/// 大会员标签
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VipLabel {
    pub path: String,
}

/// 会员信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VipInfo {
    /// 会员类型，0: 无, 1: 月度大会员, 2: 年度以上大会员
    #[serde(rename = "vipType")]
    pub vip_type: u8,
    /// 会员到期时间，毫秒级时间戳
    #[serde(rename = "vipDueDate")]
    pub vip_due_date: u64,
    #[serde(rename = "dueRemark")]
    pub due_remark: String,
    #[serde(rename = "accessStatus")]
    pub access_status: u8,
    /// 大会员状态，0: 无, 1: 有
    #[serde(rename = "vipStatus")]
    pub vip_status: u8,
    #[serde(rename = "vipStatusWarn")]
    pub vip_status_warn: String,
    #[serde(rename = "themeType")]
    pub theme_type: u8,
    pub label: VipLabel,
}

/// 关系列表对象
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelationListItem {
    /// 用户 mid
    pub mid: u64,
    /// 对方对于自己的关系属性，0: 未关注, 1: 悄悄关注, 2: 已关注, 6: 已互粉, 128: 已拉黑
    pub attribute: u8,
    /// 对方关注目标用户时间，秒级时间戳
    pub mtime: u64,
    /// 目标用户将对方分组到的 id
    pub tag: Option<Vec<u64>>,
    /// 目标用户特别关注对方标识，0: 否, 1: 是
    pub special: u8,
    pub contract_info: Option<serde_json::Value>,
    /// 用户昵称
    pub uname: String,
    /// 用户头像 url
    pub face: String,
    /// 用户签名
    pub sign: String,
    /// 是否为 NFT 头像
    pub face_nft: u8,
    /// 认证信息
    pub official_verify: OfficialVerify,
    /// 会员信息
    pub vip: VipInfo,
    #[serde(rename = "name_render")]
    pub name_render: Option<serde_json::Value>,
    #[serde(rename = "nft_icon")]
    pub nft_icon: Option<String>,
    /// 推荐该用户的原因
    pub rec_reason: Option<String>,
    #[serde(rename = "track_id")]
    pub track_id: Option<String>,
    #[serde(rename = "follow_time")]
    pub follow_time: Option<String>,
}

/// 用户关注明细响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FollowingListResponseData {
    /// 明细列表
    pub list: Vec<RelationListItem>,
    pub re_version: u32,
    /// 关注总数
    pub total: u64,
}

// --- API 实现 ---

impl BpiClient {
    /// 查询用户关注明细
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user)
    ///
    /// # 参数
    /// | 名称        | 类型           | 说明                                   |
    /// | ----------- | --------------| -------------------------------------- |
    /// | `vmid`      | u64           | 目标用户 mid                           |
    /// | `order_type`| `Option<&str>`  | 排序方式，可选                         |
    /// | `ps`        | `Option<u32>`   | 每页项数，默认50                       |
    /// | `pn`        | `Option<u32>`   | 页码，默认1                            |
    pub async fn user_followings(
        &self,
        vmid: u64,
        order_type: Option<&str>,
        ps: Option<u32>,
        pn: Option<u32>
    ) -> Result<BpiResponse<FollowingListResponseData>, BpiError> {
        let mut req = self
            .get("https://api.bilibili.com/x/relation/followings")
            .with_bilibili_headers()
            .query(&[("vmid", &vmid.to_string())]);

        if let Some(o) = order_type {
            req = req.query(&[("order_type", o)]);
        }
        if let Some(p) = ps {
            req = req.query(&[("ps", &p.to_string())]);
        }
        if let Some(p) = pn {
            req = req.query(&[("pn", &p.to_string())]);
        }

        req.send_bpi("查询用户关注明细").await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    const TEST_VMID: u64 = 293793435;

    #[tokio::test]
    async fn test_user_followings() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi.user_followings(TEST_VMID, None, Some(50), Some(1)).await?;
        let data = resp.into_data()?;

        info!("用户关注明细: {:?}", data);
        assert!(!data.list.is_empty());
        assert_eq!(data.list.len(), 50);

        Ok(())
    }
}
