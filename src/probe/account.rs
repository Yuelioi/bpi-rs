use std::collections::HashMap;
use std::path::Path;

use config::{Config, File};
use serde::Deserialize;

use crate::{Account, BpiError, BpiResult};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct RawProbeConfig {
    #[serde(default)]
    pub probe: ProbeAccountConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProbeAccountConfig {
    #[serde(default)]
    pub accounts: HashMap<String, ProbeAccountProfile>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProbeAccountProfile {
    pub bili_jct: String,
    pub dede_user_id: String,
    #[serde(default)]
    pub dede_user_id_ckmd5: String,
    pub sessdata: String,
    pub buvid3: String,
}

impl RawProbeConfig {
    pub fn load(path: impl AsRef<Path>) -> BpiResult<Self> {
        let path = path.as_ref();
        let file = path.to_string_lossy();
        Config::builder()
            .add_source(File::with_name(&file))
            .build()
            .and_then(Config::try_deserialize)
            .map_err(|err| BpiError::parse(format!("failed to load probe account config: {err}")))
    }

    pub fn account(&self, profile: &str) -> Option<Account> {
        self.probe
            .accounts
            .get(profile)
            .map(ProbeAccountProfile::to_account)
    }
}

impl ProbeAccountProfile {
    fn to_account(&self) -> Account {
        Account::new(
            self.dede_user_id.clone(),
            self.dede_user_id_ckmd5.clone(),
            self.sessdata.clone(),
            self.bili_jct.clone(),
            self.buvid3.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn account_profile_maps_vip_fields() {
        let raw = RawProbeConfig {
            probe: ProbeAccountConfig {
                accounts: HashMap::from([(
                    "vip".to_string(),
                    ProbeAccountProfile {
                        bili_jct: "csrf".to_string(),
                        dede_user_id: "42".to_string(),
                        dede_user_id_ckmd5: "ck".to_string(),
                        sessdata: "session".to_string(),
                        buvid3: "buvid".to_string(),
                    },
                )]),
            },
        };

        let account = raw.account("vip").expect("vip account should exist");

        assert_eq!(account.dede_user_id, "42");
        assert_eq!(account.bili_jct, "csrf");
    }

    #[test]
    fn account_profile_maps_normal_fields() {
        let raw = RawProbeConfig {
            probe: ProbeAccountConfig {
                accounts: HashMap::from([(
                    "normal".to_string(),
                    ProbeAccountProfile {
                        bili_jct: "csrf2".to_string(),
                        dede_user_id: "43".to_string(),
                        dede_user_id_ckmd5: "ck2".to_string(),
                        sessdata: "session2".to_string(),
                        buvid3: "buvid2".to_string(),
                    },
                )]),
            },
        };

        let account = raw.account("normal").expect("normal account should exist");

        assert_eq!(account.dede_user_id, "43");
        assert_eq!(account.bili_jct, "csrf2");
        assert_eq!(account.buvid3, "buvid2");
    }

    #[test]
    fn account_profile_returns_none_for_unknown_profile() {
        let raw = RawProbeConfig {
            probe: ProbeAccountConfig {
                accounts: HashMap::new(),
            },
        };

        assert!(raw.account("normal").is_none());
    }

    #[test]
    fn account_config_deserializes_semantic_profiles() -> Result<(), BpiError> {
        let raw: RawProbeConfig = Config::builder()
            .add_source(File::from_str(
                r#"
                [probe.accounts.vip]
                bili_jct = "vip-csrf"
                dede_user_id = "42"
                dede_user_id_ckmd5 = "vip-ck"
                sessdata = "vip-session"
                buvid3 = "vip-buvid"

                [probe.accounts.normal]
                bili_jct = "normal-csrf"
                dede_user_id = "43"
                dede_user_id_ckmd5 = "normal-ck"
                sessdata = "normal-session"
                buvid3 = "normal-buvid"
                "#,
                config::FileFormat::Toml,
            ))
            .build()
            .and_then(Config::try_deserialize)
            .map_err(|err| BpiError::parse(err.to_string()))?;

        assert_eq!(raw.account("vip").expect("vip profile").dede_user_id, "42");
        assert_eq!(
            raw.account("normal").expect("normal profile").dede_user_id,
            "43"
        );
        Ok(())
    }
}
