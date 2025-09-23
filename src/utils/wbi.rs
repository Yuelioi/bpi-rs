use serde::{ Deserialize, Serialize };
use std::collections::{ BTreeMap, HashMap };
use std::sync::{ RwLock, LazyLock };
use chrono::Local;

use crate::models::WbiData;
use crate::{ BilibiliRequest, BpiClient, BpiError, BpiResponse };
use std::time::{ SystemTime, UNIX_EPOCH };

const MIXIN_KEY_TAB: [usize; 64] = [
    46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35, 27, 43, 5, 49, 33, 9, 42, 19, 29, 28,
    14, 39, 12, 38, 41, 13, 37, 48, 7, 16, 24, 55, 40, 61, 26, 17, 0, 1, 60, 51, 30, 4, 22, 25, 54, 21,
    56, 59, 6, 63, 57, 62, 11, 36, 20, 34, 44, 52,
];

pub static WBI_KEY_MAP: LazyLock<RwLock<HashMap<String, String>>> = LazyLock::new(||
    RwLock::new(HashMap::new())
);

fn get_mixin_key(orig: &str) -> String {
    let bytes = orig.as_bytes();
    let mut s = Vec::new();
    for &i in &MIXIN_KEY_TAB {
        if i < bytes.len() {
            s.push(bytes[i] as char);
        }
    }
    s.into_iter().take(32).collect()
}

fn url_encode(s: &str) -> String {
    let mut result = String::new();
    for byte in s.bytes() {
        match byte {
            // 不编码的字符（字母数字和部分特殊字符）
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            // 空格编码为 %20
            b' ' => result.push_str("%20"),
            // 其他字符进行百分号编码，字母大写
            _ => result.push_str(&format!("%{:02X}", byte)),
        }
    }
    result
}

fn enc_wbi(params: &mut BTreeMap<String, String>, img_key: &str, sub_key: &str) {
    let mixin_key = get_mixin_key(&(img_key.to_owned() + sub_key));
    let wts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    params.insert("wts".to_string(), wts.to_string());

    // 过滤 value 中的 !'()* 字符
    for value in params.values_mut() {
        *value = value
            .chars()
            .filter(|c| !"!'()*".contains(*c))
            .collect();
    }

    // 按 key 排序 (BTreeMap 默认排序)
    let query = params
        .iter()
        .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
        .collect::<Vec<String>>()
        .join("&");

    let digest = md5::compute(format!("{}{}", query, mixin_key));
    let w_rid = format!("{:x}", digest);
    params.insert("w_rid".to_string(), w_rid);
}

#[derive(Deserialize, Serialize)]
struct WbiImgData {
    img_url: String,
    sub_url: String,
}

#[derive(Deserialize, Serialize)]
struct NavData {
    wbi_img: WbiImgData,
}

impl BpiClient {
    pub async fn get_wbi_sign(&self) -> Result<WbiData, BpiError> {
        let mut params = BTreeMap::new();

        let resp: BpiResponse<NavData> = self
            .get("https://api.bilibili.com/x/web-interface/nav")
            .with_bilibili_headers()
            .send_bpi("获取 wbi 签名").await?;

        let data = resp.data.ok_or_else(|| BpiError::parse("获取 wbi 签名失败"))?;

        let img_key = data.wbi_img.img_url.rsplit('/').next().unwrap().split('.').next().unwrap();
        let sub_key = data.wbi_img.sub_url.rsplit('/').next().unwrap().split('.').next().unwrap();

        enc_wbi(&mut params, img_key, sub_key);

        Ok(WbiData {
            wts: params
                .get("wts")
                .ok_or_else(|| BpiError::parse("缺少 wts"))?
                .parse::<u64>()
                .map_err(|_| BpiError::parse("wts 转换失败"))?,
            w_rid: params
                .get("w_rid")
                .ok_or_else(|| BpiError::parse("缺少 w_rid"))?
                .to_string(),
        })
    }

    pub async fn get_wbi_sign2<I, K, V>(&self, params: I) -> Result<Vec<(String, String)>, BpiError>
        where I: IntoIterator<Item = (K, V)>, K: ToString, V: ToString
    {
        let now = Local::now();
        let s = now.format("%Y-%m-%d %H").to_string();

        let img_key_key = format!("{}img_key", s);
        let sub_key_key = format!("{}sub_key", s);

        // 先尝试从缓存读取
        let (img_key, sub_key) = {
            let map = WBI_KEY_MAP.read().unwrap();
            if let (Some(img), Some(sub)) = (map.get(&img_key_key), map.get(&sub_key_key)) {
                (img.clone(), sub.clone())
            } else {
                drop(map); // 释放读锁再去写

                // 缓存没有 -> 请求 API
                let resp: BpiResponse<NavData> = self
                    .get("https://api.bilibili.com/x/web-interface/nav")
                    .send_bpi("获取 wbi 签名").await?;

                let data = resp.data.ok_or_else(|| BpiError::parse("获取 wbi 签名失败"))?;

                let img = data.wbi_img.img_url
                    .rsplit('/')
                    .next()
                    .unwrap()
                    .split('.')
                    .next()
                    .unwrap()
                    .to_string();

                let sub = data.wbi_img.sub_url
                    .rsplit('/')
                    .next()
                    .unwrap()
                    .split('.')
                    .next()
                    .unwrap()
                    .to_string();

                // 插入缓存
                let mut map = WBI_KEY_MAP.write().unwrap();
                map.insert(img_key_key.clone(), img.clone());
                map.insert(sub_key_key.clone(), sub.clone());

                (img, sub)
            }
        };

        // 构造参数
        let mut params: BTreeMap<String, String> = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        enc_wbi(&mut params, &img_key, &sub_key);

        Ok(params.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_wts_and_rid2() {
        let bpi = BpiClient::new();

        let params = vec![
            ("bvid", "BV18x411c74j".to_string()),
            ("cid", "21448".to_string()),
            ("up_mid", "46473".to_string()),
            ("web_location", "0.0".to_string())
        ];

        let wbi = bpi.get_wbi_sign2(params.clone()).await.unwrap();
        tracing::info!("{:?}", wbi);
        tracing::info!("{:?}", WBI_KEY_MAP);

        let wbi = bpi.get_wbi_sign2(params).await.unwrap();
        tracing::info!("{:?}", wbi);
    }
}
