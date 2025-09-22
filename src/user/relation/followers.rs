//! B站用户粉丝列表相关接口
//!
//! 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user
use crate::models::Vip;
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};
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

/// 关系列表对象
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RelationListItem {
    /// 用户 mid
    pub mid: u64,
    /// 对方对于自己的关系属性，0: 未关注, 1: 悄悄关注, 2: 已关注, 6: 已互粉, 128: 已拉黑
    pub attribute: u8,
    /// 对方关注目标用户时间，秒级时间戳
    pub mtime: Option<u64>,
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
    pub vip: Vip,
}

/// 用户粉丝明细响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FansListResponseData {
    /// 明细列表
    pub list: Vec<RelationListItem>,
    /// 偏移量供下次请求使用
    pub offset: String,
    pub re_version: u32,
    /// 粉丝总数
    pub total: u64,
}

// --- API 实现 ---

impl BpiClient {
    /// 查询用户粉丝明细
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/user
    ///
    /// # 参数
    /// | 名称            | 类型           | 说明                                   |
    /// | --------------- | --------------| -------------------------------------- |
    /// | `vmid`          | u64           | 目标用户 mid                           |
    /// | `ps`            | Option<u32>   | 每页项数，默认50                       |
    /// | `pn`            | Option<u32>   | 页码，默认1                            |
    /// | `offset`        | Option<&str>  | 偏移量，翻页用                         |
    /// | `last_access_ts`| Option<u64>   | 上次访问时间戳，秒                     |
    /// | `from`          | Option<&str>  | 请求来源，部分场景传"main"             |
    pub async fn user_followers(
        &self,
        vmid: u64,
        ps: Option<u32>,
        pn: Option<u32>,
        offset: Option<&str>,
        last_access_ts: Option<u64>,
        from: Option<&str>,
    ) -> Result<BpiResponse<FansListResponseData>, BpiError> {
        let mut req = self
            .get("https://api.bilibili.com/x/relation/fans")
            .with_bilibili_headers()
            .query(&[("vmid", &vmid.to_string())]);

        if let Some(p) = ps {
            req = req.query(&[("ps", &p.to_string())]);
        }
        if let Some(p) = pn {
            req = req.query(&[("pn", &p.to_string())]);
        }
        if let Some(o) = offset {
            req = req.query(&[("offset", o)]);
        }
        if let Some(l) = last_access_ts {
            req = req.query(&[("last_access_ts", &l.to_string())]);
        }
        if let Some(f) = from {
            req = req.query(&[("from", f)]);
        }

        req.send_bpi("查询用户粉丝明细").await
    }
}

// --- 测试模块 ---

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::info;

    const TEST_VMID: u64 = 4279370;

    #[tokio::test]

    async fn test_user_followers() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let resp = bpi
            .user_followers(TEST_VMID, Some(50), Some(1), None, None, None)
            .await?;
        let data = resp.into_data()?;

        info!("用户粉丝明细: {:?}", data);
        assert!(!data.list.is_empty());
        assert_eq!(data.list.len(), 50);
        assert!(!data.offset.is_empty());
        assert!(data.total > 0);

        Ok(())
    }
}
