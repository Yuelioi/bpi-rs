use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

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
    /// * `mid` - 用户mid，必须是自己的mid。
    /// * `buvid` - 可选的设备虚拟ID（web端为buvid3）。
    pub async fn login_notice(
        &self,
        mid: u64,
        buvid: Option<&str>
    ) -> Result<BpiResponse<LoginNoticeData>, BpiError> {
        let mut params = HashMap::new();
        params.insert("mid", mid.to_string());
        if let Some(buvid_val) = buvid {
            params.insert("buvid", buvid_val.to_string());
        }

        self
            .get("https://api.bilibili.com/x/safecenter/login_notice")
            .query(&params)
            .send_bpi("查询登录记录").await
    }

    /// 查询最近一周的登录情况。
    ///
    /// # 参数
    /// 无。该接口自动使用当前登录用户的Session。
    pub async fn login_log(&self) -> Result<BpiResponse<LoginLogData>, BpiError> {
        self
            .get("https://api.bilibili.com/x/member/web/login/log")
            .query(
                &[
                    ("jsonp", "jsonp"),
                    ("web_location", "333.33"),
                ]
            )
            .send_bpi("查询最近一周登录情况").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_login_notice() -> Result<(), BpiError> {
        let bpi = BpiClient::new();
        let mid = 4279370;

        let resp = bpi.login_notice(mid, None).await?;
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

    #[tokio::test]
    async fn test_get_login_log() -> Result<(), BpiError> {
        let bpi = BpiClient::new();

        let resp = bpi.login_log().await?;
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
