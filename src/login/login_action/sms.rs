use serde::{Deserialize, Serialize};

use crate::{BpiError, BpiResult};

#[derive(Debug, Deserialize, Serialize)]
pub struct SMSSendData {
    captcha_key: String, // 短信登录 token
}

#[derive(Debug, Deserialize, Serialize)]
struct SMSLoginData {
    is_new: bool, // 是否为新用户
    status: i32,  // 0:成功
    url: String,  // 跳转url
}

/// Parameters for sending a web SMS login verification code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginSmsCodeParams {
    cid: u32,
    tel: String,
    source: String,
    token: String,
    challenge: String,
    validate: String,
    seccode: String,
}

impl LoginSmsCodeParams {
    /// Creates SMS-code request parameters.
    pub fn new(
        cid: u32,
        tel: impl Into<String>,
        token: impl Into<String>,
        challenge: impl Into<String>,
        validate: impl Into<String>,
        seccode: impl Into<String>,
    ) -> BpiResult<Self> {
        let params = Self {
            cid,
            tel: tel.into(),
            source: "main_web".to_string(),
            token: token.into(),
            challenge: challenge.into(),
            validate: validate.into(),
            seccode: seccode.into(),
        };
        params.validate()?;
        Ok(params)
    }

    /// Sets the Bilibili login source marker. Defaults to `main_web`.
    pub fn source(mut self, source: impl Into<String>) -> BpiResult<Self> {
        self.source = source.into();
        self.validate()?;
        Ok(self)
    }

    fn validate(&self) -> BpiResult<()> {
        if self.cid == 0 {
            return Err(BpiError::invalid_parameter("cid", "cid must be non-zero"));
        }
        validate_required("tel", &self.tel)?;
        validate_required("source", &self.source)?;
        validate_required("token", &self.token)?;
        validate_required("challenge", &self.challenge)?;
        validate_required("validate", &self.validate)?;
        validate_required("seccode", &self.seccode)?;
        Ok(())
    }
}

fn validate_required(field: &'static str, value: &str) -> BpiResult<()> {
    if value.trim().is_empty() {
        return Err(BpiError::invalid_parameter(field, "value cannot be blank"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_sms_code_params_rejects_blank_token() {
        let err = LoginSmsCodeParams::new(
            86,
            "13800138000",
            " ",
            "challenge",
            "validate",
            "validate|jordan",
        )
        .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "token", .. }
        ));
    }
}
