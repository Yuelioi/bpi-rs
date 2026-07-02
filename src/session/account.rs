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
    pub fn load_test_account() -> BpiResult<Account> {
        Self::load_test_account_from("account.toml")
    }

    #[cfg(any(test, debug_assertions))]
    pub fn load_test_account_from(path: impl AsRef<std::path::Path>) -> BpiResult<Account> {
        use config::{Config, File};

        let path = path.as_ref();

        if !path.exists() {
            return Err(BpiError::invalid_parameter(
                "account_path",
                "account config file does not exist",
            ));
        }

        let settings = Config::builder()
            .add_source(File::from(path.to_path_buf()))
            .build()
            .map_err(|err| BpiError::parse(format!("failed to load account config: {err}")))?;

        settings
            .try_deserialize()
            .map_err(|err| BpiError::parse(format!("failed to parse account config: {err}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BpiError;
    use std::path::PathBuf;

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

    #[test]
    fn load_test_account_from_missing_path_does_not_create_file() {
        let path = unique_test_account_path("missing");
        assert!(!path.exists());

        let err = Account::load_test_account_from(&path).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "account_path",
                ..
            }
        ));
        assert!(!path.exists());
    }

    #[test]
    fn load_test_account_from_reads_explicit_path() -> Result<(), BpiError> {
        let path = unique_test_account_path("valid");
        std::fs::write(
            &path,
            r#"
            bili_jct = "csrf"
            dede_user_id = "42"
            dede_user_id_ckmd5 = "ck"
            sessdata = "session"
            buvid3 = "buvid"
            "#,
        )
        .map_err(|err| BpiError::parse(err.to_string()))?;

        let account = Account::load_test_account_from(&path)?;

        std::fs::remove_file(&path).map_err(|err| BpiError::parse(err.to_string()))?;
        assert_eq!(account.dede_user_id, "42");
        assert_eq!(account.bili_jct, "csrf");
        Ok(())
    }

    fn unique_test_account_path(label: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "bpi-rs-{label}-account-{}-{nanos}.toml",
            std::process::id(),
        ))
    }
}
