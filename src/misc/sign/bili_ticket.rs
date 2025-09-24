//! 用于生成 bili_ticket
//!
//! bili_ticket 位于请求头 Cookie 中, 非必需, 但存在可降低风控概率
//! 是 JWT 令牌，有效时长为 259200 秒，即 3 天。
//!
//! [查看 API 文档](https://github.com/SocialSisterYi/bilibili-API-collect/blob/master/docs/misc/sign/bili_ticket.md)

use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use hmac::{ Hmac, Mac };
use serde::{ Deserialize, Serialize };
use sha2::Sha256;
use std::time::{ SystemTime, UNIX_EPOCH };

type HmacSha256 = Hmac<Sha256>;

/// bili_ticket 响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketData {
    /// bili_ticket JWT 令牌
    pub ticket: String,
    /// 创建时间 UNIX 秒级时间戳
    pub created_at: i64,
    /// 有效时长 259200 秒 (3 天)
    pub ttl: i32,
    /// 空对象
    pub context: serde_json::Value,
    /// WBI 相关信息
    pub nav: NavData,
}

/// WBI 导航数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavData {
    /// img_key 值
    pub img: String,
    /// sub_key 值
    pub sub: String,
}

impl BpiClient {
    /// 生成 bili_ticket
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/misc)
    pub async fn misc_sign_bili_ticket(&self) -> Result<BpiResponse<TicketData>, BpiError> {
        let csrf = self.csrf()?;
        // 获取当前时间戳
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| BpiError::network(format!("获取时间戳失败: {}", e)))?
            .as_secs();

        // 计算 hexsign
        let message = format!("ts{}", timestamp);
        let hexsign = self.hmac_sha256("XgwSnGZ1p", &message)?;

        // let now = Utc::now().timestamp().to_string();

        // 构建请求参数
        let params = [
            ("key_id", "ec02"),
            ("hexsign", &hexsign),
            ("context[ts]", &timestamp.to_string()),
            ("csrf", csrf.as_str()),
        ];

        // 发送请求
        self
            .post("https://api.bilibili.com/bapis/bilibili.api.ticket.v1.Ticket/GenWebTicket")
            .query(&params)
            .send_bpi("生成bili_ticket").await
    }

    /// 仅获取 bili_ticket 字符串
    pub async fn misc_sign_bili_ticket_string(&self) -> Result<String, BpiError> {
        let resp = self.misc_sign_bili_ticket().await?;
        let data = resp.data.ok_or_else(BpiError::missing_data)?;
        Ok(data.ticket)
    }

    /// 使用 HMAC-SHA256 算法计算哈希
    fn hmac_sha256(&self, key: &str, message: &str) -> Result<String, BpiError> {
        let mut mac = HmacSha256::new_from_slice(key.as_bytes()).map_err(|e|
            BpiError::parse(format!("HMAC 密钥错误: {}", e))
        )?;

        mac.update(message.as_bytes());
        let result = mac.finalize();
        Ok(hex::encode(result.into_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hmac_sha256() {
        let bpi = BpiClient::new();
        let result = bpi.hmac_sha256("XgwSnGZ1p", "ts1234567890").unwrap();

        // 验证结果是64位十六进制字符串
        assert_eq!(result.len(), 64);
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
        tracing::info!("HMAC-SHA256 测试通过: {}", result);
    }

    #[tokio::test]
    async fn test_generate_bili_ticket() {
        let bpi = BpiClient::new();

        match bpi.misc_sign_bili_ticket().await {
            Ok(resp) => {
                if resp.code == 0 {
                    let data = resp.data.unwrap();
                    tracing::info!("Ticket: {}", data.ticket);
                    tracing::info!("创建时间: {}", data.created_at);
                    tracing::info!(
                        "有效时长: {} 秒 ({:.1} 天)",
                        data.ttl,
                        (data.ttl as f64) / 86400.0
                    );
                    tracing::info!("WBI img: {}", data.nav.img);
                    tracing::info!("WBI sub: {}", data.nav.sub);

                    // 验证 ticket 是 JWT 格式
                    assert!(data.ticket.contains('.'));
                    assert!(data.ttl > 250000); // 大约 3 天
                } else {
                    panic!("API 返回错误: code={}, message={}", resp.code, resp.message);
                }
            }

            Err(err) => {
                panic!("生成 bili_ticket 失败: {}", err);
            }
        }
    }

    #[tokio::test]
    async fn test_get_bili_ticket_string() {
        let bpi = BpiClient::new();

        match bpi.misc_sign_bili_ticket_string().await {
            Ok(ticket) => {
                tracing::info!("获取到的 bili_ticket: {}", ticket);

                // 验证 ticket 格式
                assert!(!ticket.is_empty());
                assert!(ticket.contains('.'));

                // JWT 应该有 3 部分（header.payload.signature）
                let parts: Vec<&str> = ticket.split('.').collect();
                assert_eq!(parts.len(), 3);
            }
            Err(err) => {
                panic!("获取 bili_ticket 字符串失败: {}", err);
            }
        }
    }

    #[tokio::test]
    async fn test_with_csrf() {
        let bpi = BpiClient::new();

        // 测试带 CSRF 的情况
        match bpi.misc_sign_bili_ticket().await {
            Ok(resp) => {
                tracing::info!("带 CSRF 的 bili_ticket 生成成功: {}", resp.data.unwrap().ticket);
            }
            Err(err) => {
                tracing::info!("带 CSRF 测试失败（预期可能失败）: {}", err);
                // 这里不 panic，因为没有真实的 CSRF token 可能会失败
            }
        }
    }
}
