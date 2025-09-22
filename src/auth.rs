/// B站账号登录信息
#[derive(Debug, Clone, serde::Deserialize)]
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

    pub fn is_complete(&self) -> bool {
        !self.dede_user_id.is_empty()
            && !self.sessdata.is_empty()
            && !self.bili_jct.is_empty()
            && !self.buvid3.is_empty()
    }
}

impl Account {
    #[cfg(any(test, debug_assertions))]
    pub fn load_test_account() -> Result<Account, Box<dyn std::error::Error>> {
        use config::{Config, File};
        use std::path::Path;

        let config_path = "account.toml";

        // 如果测试配置文件不存在，创建一个模板
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
