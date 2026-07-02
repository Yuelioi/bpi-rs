use serde::{Deserialize, Serialize};

use crate::login::params::{LoginLogParams, LoginNoticeParams};
use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};

// --- API 结构体 ---

/// 查询指定登录记录的响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginNoticeData {
    pub mid: u64,
    pub device_name: String,
    pub login_type: String,
    pub login_time: String,
    pub location: String,
    pub ip: String,
}

/// 最近一周登录情况的响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginLogData {
    pub count: u32,
    pub list: Vec<LoginLogEntry>,
}

/// 登录日志列表中的单条记录
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginLogEntry {
    pub ip: String,
    pub time: u64,
    pub time_at: String,
    pub status: bool,
    #[serde(rename = "type")]
    pub login_type: u8,
    #[serde(rename = "geo")]
    pub location: String,
}

impl BpiClient {
    /// 查询指定登录记录。
    ///
    /// # 参数
    /// * `params` - 用户 mid 和可选 buvid。
    pub async fn login_notice(
        &self,
        params: LoginNoticeParams,
    ) -> Result<BpiResponse<LoginNoticeData>, BpiError> {
        self.get("https://api.bilibili.com/x/safecenter/login_notice")
            .query(&params.query_pairs())
            .send_bpi("查询登录记录")
            .await
    }

    /// 查询最近一周的登录情况。
    ///
    /// # 参数
    /// * `params` - JSONP 和 web_location 查询参数。
    pub async fn login_log(
        &self,
        params: LoginLogParams,
    ) -> Result<BpiResponse<LoginLogData>, BpiError> {
        self.get("https://api.bilibili.com/x/member/web/login/log")
            .query(&params.query_pairs())
            .send_bpi("查询最近一周登录情况")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::Mid;

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_login_notice() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let mid = 4279370;

        let resp = bpi
            .login_notice(LoginNoticeParams::new(Mid::new(mid)?))
            .await?;
        let data = resp.into_data()?;

        println!("指定登录记录:");
        println!("  设备名: {}", data.device_name);
        println!("  登录方式: {}", data.login_type);
        println!("  登录时间: {}", data.login_time);
        println!("  登录位置: {}", data.location);
        println!("  IP: {}", data.ip);

        assert_eq!(data.mid, mid);

        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_login_log() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");

        let resp = bpi.login_log(LoginLogParams::new()).await?;
        let data = resp.into_data()?;

        println!("最近一周登录记录 (共 {} 条):", data.count);
        for entry in data.list {
            println!("  时间: {} ({})", entry.time_at, entry.time);
            println!("    IP: {}", entry.ip);
            println!("    位置: {}", entry.location);
            println!("    登录成功: {}", entry.status);
            println!("    登录类型: {}", entry.login_type);
        }

        Ok(())
    }
}
