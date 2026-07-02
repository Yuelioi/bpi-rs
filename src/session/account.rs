use std::collections::HashMap;
use std::fmt;

use serde::Deserialize;

use crate::{BpiError, BpiResult};

use super::cookie::{CookiePair, parse_cookie_header};

#[derive(Clone, Default, Deserialize)]
pub struct Account {
    pub dede_user_id: String,
    pub dede_user_id_ckmd5: String,
    pub sessdata: String,
    pub bili_jct: String,
    pub buvid3: String,
}

impl Account {
    pub fn new(
        dede_user_id: String,
        dede_user_id_ckmd5: String,
        sessdata: String,
        bili_jct: String,
        buvid3: String,
    ) -> Self {
        Self {
            dede_user_id,
            dede_user_id_ckmd5,
            sessdata,
            bili_jct,
            buvid3,
        }
    }

    pub fn from_cookie_header(cookie_header: &str) -> BpiResult<Self> {
        let pairs = parse_cookie_header(cookie_header)?;
        Ok(Self::from_cookie_pairs(&pairs))
    }

    pub fn from_cookie_pairs(pairs: &[CookiePair]) -> Self {
        let map: HashMap<&str, &str> = pairs
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();

        Self {
            dede_user_id: map
                .get("DedeUserID")
                .copied()
                .unwrap_or_default()
                .to_string(),
            dede_user_id_ckmd5: map
                .get("DedeUserID__ckMd5")
                .copied()
                .unwrap_or_default()
                .to_string(),
            sessdata: map.get("SESSDATA").copied().unwrap_or_default().to_string(),
            bili_jct: map.get("bili_jct").copied().unwrap_or_default().to_string(),
            buvid3: map.get("buvid3").copied().unwrap_or_default().to_string(),
        }
    }

    pub fn cookie_pairs(&self) -> Vec<CookiePair> {
        [
            ("DedeUserID", self.dede_user_id.as_str()),
            ("DedeUserID__ckMd5", self.dede_user_id_ckmd5.as_str()),
            ("SESSDATA", self.sessdata.as_str()),
            ("bili_jct", self.bili_jct.as_str()),
            ("buvid3", self.buvid3.as_str()),
        ]
        .into_iter()
        .filter(|(_, value)| !value.is_empty())
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
    }

    pub fn csrf(&self) -> BpiResult<&str> {
        if self.bili_jct.is_empty() {
            return Err(BpiError::auth("missing csrf token"));
        }

        Ok(&self.bili_jct)
    }

    pub fn is_complete(&self) -> bool {
        !self.dede_user_id.is_empty()
            && !self.sessdata.is_empty()
            && !self.bili_jct.is_empty()
            && !self.buvid3.is_empty()
    }
}

impl fmt::Debug for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Account")
            .field("dede_user_id", &redact_if_present(&self.dede_user_id))
            .field(
                "dede_user_id_ckmd5",
                &redact_if_present(&self.dede_user_id_ckmd5),
            )
            .field("sessdata", &redact_if_present(&self.sessdata))
            .field("bili_jct", &redact_if_present(&self.bili_jct))
            .field("buvid3", &redact_if_present(&self.buvid3))
            .finish()
    }
}

fn redact_if_present(value: &str) -> &'static str {
    if value.is_empty() {
        "<empty>"
    } else {
        "<redacted>"
    }
}

impl Account {
    #[cfg(any(test, debug_assertions))]
    pub fn load_test_account() -> Result<Account, Box<dyn std::error::Error>> {
        use std::path::Path;

        use config::{Config, File};

        let config_path = "account.toml";

        if !Path::new(config_path).exists() {
            create_test_account_template(config_path)?;
            return Err("测试账号配置文件已创建，请填写后重新运行".into());
        }

        let settings = Config::builder()
            .add_source(File::with_name("account"))
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}

#[cfg(any(test, debug_assertions))]
fn create_test_account_template(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    let template = r#"# 测试账号配置文件
# 请填写您的 B站 账号信息用于测试

bili_jct = "your_bili_jct_here"
dede_user_id = "your_dede_user_id_here"
dede_user_id_ckmd5 = "your_dede_user_id_ckmd5_here"
sessdata = "your_sessdata_here"
buvid3 = "your_buvid3_here"

# 注意: 这个文件包含敏感信息，请不要提交到版本控制系统
"#;

    fs::write(path, template)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BpiError;

    #[test]
    fn account_from_cookie_header_extracts_known_fields() -> Result<(), BpiError> {
        let account = Account::from_cookie_header(
            "DedeUserID=42; DedeUserID__ckMd5=ck; SESSDATA=session; bili_jct=csrf; buvid3=buvid",
        )?;

        assert_eq!(account.dede_user_id, "42");
        assert_eq!(account.dede_user_id_ckmd5, "ck");
        assert_eq!(account.sessdata, "session");
        assert_eq!(account.bili_jct, "csrf");
        assert_eq!(account.buvid3, "buvid");
        Ok(())
    }

    #[test]
    fn csrf_returns_token_when_present() -> Result<(), BpiError> {
        let account = Account::from_cookie_header("bili_jct=csrf")?;

        assert_eq!(account.csrf()?, "csrf");
        Ok(())
    }

    #[test]
    fn csrf_returns_auth_error_when_missing() {
        let account = Account::default();

        let err = account.csrf().unwrap_err();
        assert!(matches!(err, BpiError::Auth { .. }));
    }

    #[test]
    fn debug_output_redacts_secret_values() -> Result<(), BpiError> {
        let account = Account::from_cookie_header(
            "DedeUserID=42; SESSDATA=session-secret; bili_jct=csrf-secret; buvid3=buvid-secret",
        )?;

        let debug = format!("{account:?}");
        assert!(!debug.contains("session-secret"));
        assert!(!debug.contains("csrf-secret"));
        assert!(!debug.contains("buvid-secret"));
        Ok(())
    }

    #[test]
    fn complete_account_requires_login_cookie_csrf_and_buvid() -> Result<(), BpiError> {
        let account = Account::from_cookie_header(
            "DedeUserID=42; SESSDATA=session; bili_jct=csrf; buvid3=buvid",
        )?;

        assert!(account.is_complete());
        Ok(())
    }
}
