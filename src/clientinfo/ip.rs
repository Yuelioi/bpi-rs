//! IP 地址归属地查询 API
//!
//! [参考文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/clientinfo/ip.md)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };

// ==========================
// 数据结构
// ==========================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpInfo {
    /// 国家
    pub country: Option<String>,
    /// 省份
    pub province: Option<String>,
    /// 城市
    pub city: Option<String>,
    /// ISP 运营商
    pub isp: Option<String>,
    /// IP 地址
    pub addr: Option<String>,
}

// ==========================
// API 封装
// ==========================

impl BpiClient {
    /// 查询 IP 地址归属地
    ///
    /// 查询指定 IP 地址的地理位置信息，包括国家、省份、城市和 ISP 运营商。
    /// 如果不提供 IP 参数，将返回请求方 IP 的地理信息。
    ///
    /// # 参数
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `ip` | `Option<&str>` | IPv4 或 IPv6 地址，可选。如果留空，返回请求方 IP 信息 |
    ///
    /// # 文档
    /// [IP 地址归属地查询](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/clientinfo/ip.md)
    pub async fn clientinfo_ip(&self, ip: Option<&str>) -> Result<BpiResponse<IpInfo>, BpiError> {
        let mut req = self.get(
            "https://api.live.bilibili.com/ip_service/v1/ip_service/get_ip_addr"
        );

        if let Some(ip) = ip {
            req = req.query(&[("ip", ip)]);
        }

        req.send_bpi("查询 IP 地址归属地").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_IP: &str = "8.8.8.8";

    #[tokio::test]
    async fn test_clientinfo_ip() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();

        let resp = bpi.clientinfo_ip(Some(TEST_IP)).await?;
        if resp.code == 0 {
            if let Some(data) = resp.data {
                tracing::info!(
                    "IP 地址: {}, 省份: {:?}, 城市: {:?}, ISP: {:?}",
                    data.addr.unwrap_or_default(),
                    data.province,
                    data.city,
                    data.isp
                );
            }
        } else {
            tracing::error!("请求失败: code={}, message={}", resp.code, resp.message);
        }

        Ok(())
    }
}
