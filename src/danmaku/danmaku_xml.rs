//! XML 弹幕
//!
//! 文档入口: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku

use crate::{BpiClient, BpiError};
use flate2::read::DeflateDecoder;
use quick_xml::de::from_str;
use reqwest::Client;
use std::io::Read;

use serde::{Deserialize, Serialize};

// 用于解析 <d> 标签的 p 属性的元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DanmakuMeta {
    pub time: f32,         // 视频内弹幕出现时间（秒）
    pub danmaku_type: i32, // 弹幕类型
    pub font_size: i32,    // 字号
    pub color: i32,        // 颜色（十进制RGB888值）
    pub send_time: i64,    // 发送时间戳
    pub pool_type: i32,    // 弹幕池类型
    pub user_hash: String, // 发送者mid的HASH
    pub dmid: i64,         // 弹幕dmid（唯一标识）
    pub block_level: i32,  // 屏蔽等级
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "d")]
pub struct Danmaku {
    #[serde(rename = "$value")]
    pub content: String, // 弹幕内容

    #[serde(rename = "@p")]
    pub p_value: String, // 原始的 p 属性字符串

    #[serde(skip_serializing)]
    pub meta: Option<DanmakuMeta>,
}

impl Danmaku {
    /// 解析 p 属性并返回 DanmakuMeta
    pub fn parse_p(&mut self) -> Result<(), BpiError> {
        let parts: Vec<&str> = self.p_value.split(',').collect();
        if parts.len() < 8 {
            return Err(BpiError::parse("解析xml失败 弹幕参数不足8"));
        }

        let time: f32 = parts[0].parse().unwrap_or(0.0);
        let danmaku_type: i32 = parts[1].parse().unwrap_or(1);
        let font_size: i32 = parts[2].parse().unwrap_or(25);
        let color: i32 = parts[3].parse().unwrap_or(16777215); // 默认白色
        let send_time: i64 = parts[4].parse().unwrap_or(0);
        let pool_type: i32 = parts[5].parse().unwrap_or(0);
        let user_hash = parts[6].to_string();
        let dmid: i64 = parts[7].parse().unwrap_or(0);

        let block_level = parts[8].parse().unwrap_or(0);
        self.meta = Some(DanmakuMeta {
            time,
            danmaku_type,
            font_size,
            color,
            send_time,
            pool_type,
            user_hash,
            dmid,
            block_level,
        });
        Ok(())
    }
}

// 根标签 i
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "i")]
pub struct DanmakuXml {
    pub chatserver: String,
    pub chatid: String,
    pub mission: i32,
    pub maxlimit: i32,
    pub state: i32, // 0: 正常, 1: 弹幕已关闭
    pub real_name: i32,
    pub source: String,
    #[serde(rename = "d", default)]
    pub danmakus: Vec<Danmaku>,
}

impl BpiClient {
    /// 获取实时弹幕（接口1）
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `oid` | i64 | 视频 oid/cid |
    pub async fn danmaku_xml_list_so(&self, oid: i64) -> Result<DanmakuXml, BpiError> {
        let client = Client::builder()
            .gzip(false)
            .brotli(false)
            .deflate(false) // 禁用自动解压
            .build()?;

        let bytes = client
            .get("https://api.bilibili.com/x/v1/dm/list.so")
            // .header(ACCEPT, "application/xml, text/xml, */*")
            .query(&[("oid", oid.to_string())])
            .send()
            .await
            .map_err(BpiError::from)?
            .bytes()
            .await
            .map_err(|e| BpiError::network(format!("获取响应体失败: {}", e)))?;

        let mut d = DeflateDecoder::new(&bytes[..]);
        let mut xml = String::new();
        d.read_to_string(&mut xml)
            .map_err(|_| BpiError::parse("读取xml失败"))?;

        let mut parsed: DanmakuXml = from_str(&xml).map_err(|_| BpiError::parse("解析xml失败"))?;

        parsed.danmakus.iter_mut().try_for_each(|dm| dm.parse_p())?;

        Ok(parsed)
    }

    /// 获取实时弹幕（接口2）
    /// 使用 deflate 压缩（reqwest 会自动解压），返回 XML 文本
    ///
    /// 文档: https://github.com/SocialSisterYi/bilibili-API-collect/tree/master/docs/danmaku
    ///
    /// 参数
    ///
    /// | 名称 | 类型 | 说明 |
    /// | ---- | ---- | ---- |
    /// | `cid` | i64 | 视频 cid |
    pub async fn danmaku_xml_list(&self, cid: i64) -> Result<DanmakuXml, BpiError> {
        let url = format!("https://comment.bilibili.com/{}.xml", cid);

        let client = Client::builder()
            .gzip(false)
            .brotli(false)
            .deflate(false) // 禁用自动解压
            .build()?;

        let bytes = client.get(url).send().await?.bytes().await?;

        let mut d = DeflateDecoder::new(&bytes[..]);
        let mut xml = String::new();
        d.read_to_string(&mut xml)
            .map_err(|_| BpiError::parse("读取xml失败"))?;

        let mut parsed: DanmakuXml = from_str(&xml).map_err(|_| BpiError::parse("解析xml失败"))?;

        parsed.danmakus.iter_mut().try_for_each(|dm| dm.parse_p())?;

        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;
    use tracing::info;

    #[tokio::test]
    async fn test_get_danmaku_xml_api() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let start = Instant::now();

        let data = bpi.danmaku_xml_list_so(16546).await?;
        let duration = start.elapsed();

        info!(
            "耗时1 {:?} 弹幕装填个数: {:?} ",
            duration,
            data.danmakus.len()
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_get_danmaku_xml_cid() -> Result<(), Box<BpiError>> {
        let bpi = BpiClient::new();
        let start = Instant::now();

        let data = bpi.danmaku_xml_list(16546).await?;
        let duration = start.elapsed();

        info!(
            "耗时2 {:?} 弹幕装填个数: {:?} ",
            duration,
            data.danmakus.len()
        );

        Ok(())
    }
}
