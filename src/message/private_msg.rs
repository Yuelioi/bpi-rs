use crate::{BilibiliRequest, BpiClient, BpiError, BpiResponse};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use chrono::Utc;
use uuid::Uuid;

// --- API 结构体 ---

/// 未读私信数数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SingleUnreadData {
    pub unfollow_unread: u32,
    pub follow_unread: u32,
    pub unfollow_push_msg: u32,
    pub dustbin_push_msg: u32,
    pub dustbin_unread: u32,
    pub biz_msg_unfollow_unread: u32,
    pub biz_msg_follow_unread: u32,
    pub custom_unread: u32,
}

/// 发送私信的响应数据
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendMsgData {
    pub msg_key: Option<u64>,
    pub e_infos: Option<Vec<EmojiInfo>>,
    pub msg_content: Option<String>,
    pub key_hit_infos: Option<KeyHitInfos>,
}

/// 表情信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmojiInfo {
    pub text: String,
    pub uri: String,
    pub size: u32,
    pub gif_url: Option<String>,
}

/// 触发的提示信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeyHitInfos {
    pub toast: Option<String>,
    pub rule_id: Option<u64>,
    pub high_text: Option<Vec<Value>>, // 具体结构待补充
}

/// 发送的图片格式
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Image {
    pub url: String,
    pub height: u64,
    pub width: u64,
    #[serde(rename = "imageType")]
    pub image_type: Option<String>,
    pub original: Option<u64>, // 1 代表是原图
    pub size: f64,
}

/// 私信消息类型
pub enum MessageType {
    /// 文本消息，内容为纯文本
    Text(String),
    /// 图片消息，内容为JSON字符串
    Image(Image),
}

impl BpiClient {
    /// 发送私信。
    ///
    /// # 文档
    /// [查看API文档](https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/message)
    ///
    /// # 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `receiver_id` | u64 | 接收者 ID |
    /// | `receiver_type` | u32 | 接收者类型：1 用户，2 粉丝团 |
    /// | `message_type` | MessageType | 消息类型（文本/图片） |
    pub async fn message_send(
        &self,
        receiver_id: u64,
        receiver_type: u32,
        message_type: MessageType,
    ) -> Result<BpiResponse<SendMsgData>, BpiError> {
        // 1. 获取必需的参数
        let csrf = self.csrf()?;
        let sender_uid = &self
            .get_account()
            .ok_or(BpiError::auth("未登录"))?
            .dede_user_id;
        let dev_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().timestamp();

        let msg_type = match message_type {
            MessageType::Text(_) => 1,
            MessageType::Image(_) => 2,
        };

        // 2. 准备请求体参数
        let mut form = vec![
            ("msg[sender_uid]", sender_uid.to_string()),
            ("msg[receiver_id]", receiver_id.to_string()),
            ("msg[receiver_type]", receiver_type.to_string()),
            ("msg[msg_type]", msg_type.to_string()),
            ("msg[msg_status]", "0".to_string()),
            ("msg[dev_id]", dev_id.clone()),
            ("msg[timestamp]", timestamp.to_string()),
            ("msg[new_face_version]", "1".to_string()),
            ("csrf", csrf.clone()),
            ("csrf_token", csrf.clone()),
            ("build", "0".to_string()),
            ("mobi_app", "web".to_string()),
        ];

        // 3. 构造 msg[content] 参数
        let content = match message_type {
            MessageType::Text(text) => json!({ "content": text }).to_string(),
            MessageType::Image(image) => serde_json::to_string(&image)?,
        };

        form.push(("msg[content]", content));

        let params = vec![
            ("w_sender_uid", sender_uid.to_string()),
            ("w_receiver_id", receiver_id.to_string()),
            ("w_dev_id", dev_id.clone()),
        ];

        let signed_params = self.get_wbi_sign2(params).await?;

        // 发送请求
        self.post("https://api.vc.bilibili.com/web_im/v1/web_im/send_msg")
            .query(&signed_params)
            .form(&form)
            .send_bpi("发送私信")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::params::MessageSingleUnreadParams;
    use crate::probe::contract::HttpMethod;
    use crate::probe::endpoint_contract::EndpointContract;
    use crate::{ApiEnvelope, BpiResult};
    use std::path::Path;
    use tracing::info;

    fn contract() -> BpiResult<EndpointContract> {
        EndpointContract::from_slice(include_bytes!(
            "../../tests/contracts/message/read/single-unread/contract.json"
        ))
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_get_single_unread() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");

        // 默认查询所有未读私信数
        let all_unread_data = bpi
            .message()
            .single_unread(MessageSingleUnreadParams::new())
            .await?;
        println!("所有未读私信数: {:?}", all_unread_data);

        assert_eq!(all_unread_data.dustbin_unread, 0); // show_dustbin为false时，该值为0

        Ok(())
    }

    #[test]
    fn message_single_unread_contract_matches_endpoint_request() -> BpiResult<()> {
        let contract = contract()?;

        assert_eq!(contract.name, "message.single_unread");
        assert_eq!(contract.request.method, HttpMethod::Get);
        assert_eq!(
            contract.request.url.as_str(),
            "https://api.vc.bilibili.com/session_svr/v1/session_svr/single_unread"
        );
        assert_eq!(
            contract.request.query.get("build").map(String::as_str),
            Some("0")
        );
        assert_eq!(
            contract.request.query.get("mobi_app").map(String::as_str),
            Some("web")
        );
        assert_eq!(
            contract
                .request
                .query
                .get("unread_type")
                .map(String::as_str),
            Some("0")
        );
        assert_eq!(
            contract
                .request
                .query
                .get("show_unfollow_list")
                .map(String::as_str),
            Some("0")
        );
        assert_eq!(
            contract
                .request
                .query
                .get("show_dustbin")
                .map(String::as_str),
            Some("0")
        );
        assert_eq!(contract.cases.len(), 3);
        assert_eq!(contract.cases[0].response.api_code, Some(-101));
        assert_eq!(
            contract.cases[0].response.error.as_deref(),
            Some("requires_login")
        );
        assert_eq!(
            contract.cases[1].response.rust_model.as_deref(),
            Some("SingleUnreadData")
        );
        Ok(())
    }

    #[test]
    fn message_single_unread_response_fixtures_parse_declared_model() -> BpiResult<()> {
        let err = ApiEnvelope::<serde_json::Value>::from_slice(include_bytes!(
            "../../tests/contracts/message/read/single-unread/responses/anonymous.requires_login.json"
        ))
        .and_then(ApiEnvelope::ensure_success)
        .unwrap_err();
        assert!(err.requires_login());

        let payload = ApiEnvelope::<SingleUnreadData>::from_slice(include_bytes!(
            "../../tests/contracts/message/read/single-unread/responses/authenticated.success.json"
        ))?
        .into_payload()?;

        assert_eq!(payload.follow_unread, 0);
        assert_eq!(payload.unfollow_unread, 0);
        Ok(())
    }

    fn local_probe_body(profile: &str) -> Option<serde_json::Value> {
        let path =
            format!("target/bpi-probe-runs/message/read/single-unread/{profile}.response.json");
        let bytes = std::fs::read(path).ok()?;
        let value: serde_json::Value = serde_json::from_slice(&bytes).ok()?;
        value
            .get("response")
            .and_then(|response| response.get("body"))
            .cloned()
    }

    #[test]
    fn message_single_unread_model_matches_local_probe_outputs_when_available() -> BpiResult<()> {
        for profile in ["anonymous", "normal", "vip"] {
            let Some(body) = local_probe_body(profile) else {
                continue;
            };

            if profile == "anonymous" {
                let err = serde_json::from_value::<ApiEnvelope<serde_json::Value>>(body)?
                    .ensure_success()
                    .unwrap_err();
                assert!(err.requires_login());
                continue;
            }

            let payload =
                serde_json::from_value::<ApiEnvelope<SingleUnreadData>>(body)?.into_payload()?;
            let _total_unread =
                payload.follow_unread + payload.unfollow_unread + payload.biz_msg_follow_unread;
        }
        Ok(())
    }

    #[ignore = "legacy live API test; requires explicit BPI_LIVE_TEST review"]
    #[tokio::test]
    async fn test_send_text_message() -> Result<(), BpiError> {
        let bpi = BpiClient::new().expect("client should build");
        let receiver_id = 107997089; // 替换为你要发送消息的目标用户mid
        // let message_content = "这是一个测试消息。";
        //
        // let resp = bpi
        //     .send_message(
        //         receiver_id,
        //         1, // 接收者类型：用户
        //         MessageType::Text(message_content),
        //     )
        //     .await?;

        let test_file = Path::new("./assets/test.jpg");
        if !test_file.exists() {
            return Err(BpiError::parse(
                "Test file 'test.jpg' not found.".to_string(),
            ));
        }

        let resp = bpi.dynamic_upload_pic(test_file, None).await?;
        let data = resp.into_data()?;

        info!("上传成功！图片 URL: {}", data.image_url);

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        let resp = bpi
            .message_send(
                receiver_id,
                1, // 接收者类型：用户
                MessageType::Image(Image {
                    url: data.image_url.to_string(),
                    height: data.image_height,
                    width: data.image_width,
                    image_type: None,
                    original: Some(1),
                    size: data.img_size,
                }),
            )
            .await?;

        println!("发送私信响应: {:?}", resp);
        assert_eq!(resp.code, 0);

        Ok(())
    }
}
