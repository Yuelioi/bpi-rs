use crate::{BpiError, BpiResult};

/// Parameters for `/x/msgfeed/reply`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageReplyFeedParams {
    start_id: Option<u64>,
    start_time: Option<u64>,
    build: String,
    mobi_app: String,
    platform: String,
    web_location: String,
}

impl Default for MessageReplyFeedParams {
    fn default() -> Self {
        Self {
            start_id: None,
            start_time: None,
            build: "0".to_string(),
            mobi_app: "web".to_string(),
            platform: "web".to_string(),
            web_location: String::new(),
        }
    }
}

impl MessageReplyFeedParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the cursor ID returned by the previous page.
    pub fn with_start_id(mut self, start_id: u64) -> BpiResult<Self> {
        self.start_id = Some(validate_positive_u64("id", start_id)?);
        Ok(self)
    }

    /// Sets the cursor timestamp returned by the previous page.
    pub fn with_start_time(mut self, start_time: u64) -> BpiResult<Self> {
        self.start_time = Some(validate_positive_u64("reply_time", start_time)?);
        Ok(self)
    }

    /// Sets Bilibili's raw web-location marker.
    pub fn with_web_location(mut self, web_location: impl Into<String>) -> Self {
        self.web_location = web_location.into();
        self
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = vec![
            ("build", self.build.clone()),
            ("mobi_app", self.mobi_app.clone()),
            ("platform", self.platform.clone()),
            ("web_location", self.web_location.clone()),
        ];

        if let Some(start_id) = self.start_id {
            pairs.push(("id", start_id.to_string()));
        }
        if let Some(start_time) = self.start_time {
            pairs.push(("reply_time", start_time.to_string()));
        }

        pairs
    }
}

/// Unread category accepted by `/session_svr/v1/session_svr/single_unread`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleUnreadType {
    All,
    Follow,
    Unfollow,
    Blocked,
    Custom(u32),
}

impl SingleUnreadType {
    fn as_query_value(self) -> String {
        match self {
            Self::All => "0".to_string(),
            Self::Follow => "1".to_string(),
            Self::Unfollow => "2".to_string(),
            Self::Blocked => "3".to_string(),
            Self::Custom(value) => value.to_string(),
        }
    }
}

/// Parameters for `/session_svr/v1/session_svr/single_unread`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageSingleUnreadParams {
    unread_type: SingleUnreadType,
    show_unfollow_list: bool,
    show_dustbin: bool,
    build: String,
    mobi_app: String,
}

impl Default for MessageSingleUnreadParams {
    fn default() -> Self {
        Self {
            unread_type: SingleUnreadType::All,
            show_unfollow_list: false,
            show_dustbin: false,
            build: "0".to_string(),
            mobi_app: "web".to_string(),
        }
    }
}

impl MessageSingleUnreadParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_unread_type(mut self, unread_type: SingleUnreadType) -> Self {
        if unread_type == SingleUnreadType::Blocked {
            self.show_dustbin = true;
        }
        self.unread_type = unread_type;
        self
    }

    pub fn show_unfollow_list(mut self, show: bool) -> Self {
        self.show_unfollow_list = show;
        self
    }

    pub fn show_dustbin(mut self, show: bool) -> Self {
        self.show_dustbin = show;
        self
    }

    pub fn with_custom_unread_type(mut self, unread_type: u32) -> BpiResult<Self> {
        self.unread_type =
            SingleUnreadType::Custom(validate_positive_u32("unread_type", unread_type)?);
        Ok(self)
    }

    pub(crate) fn query_pairs(&self) -> Vec<(&'static str, String)> {
        vec![
            ("build", self.build.clone()),
            ("mobi_app", self.mobi_app.clone()),
            ("unread_type", self.unread_type.as_query_value()),
            (
                "show_unfollow_list",
                bool_flag(self.show_unfollow_list).to_string(),
            ),
            ("show_dustbin", bool_flag(self.show_dustbin).to_string()),
        ]
    }
}

fn bool_flag(value: bool) -> &'static str {
    if value { "1" } else { "0" }
}

fn validate_positive_u32(field: &'static str, value: u32) -> BpiResult<u32> {
    if value == 0 {
        return Err(BpiError::invalid_parameter(field, "value must be non-zero"));
    }

    Ok(value)
}

fn validate_positive_u64(field: &'static str, value: u64) -> BpiResult<u64> {
    if value == 0 {
        return Err(BpiError::invalid_parameter(field, "value must be non-zero"));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reply_feed_params_serializes_defaults() {
        let params = MessageReplyFeedParams::new();

        assert_eq!(
            params.query_pairs(),
            vec![
                ("build", "0".to_string()),
                ("mobi_app", "web".to_string()),
                ("platform", "web".to_string()),
                ("web_location", String::new()),
            ]
        );
    }

    #[test]
    fn reply_feed_params_serializes_cursor() -> BpiResult<()> {
        let params = MessageReplyFeedParams::new()
            .with_start_id(1001)?
            .with_start_time(1_700_000_000)?;

        assert_eq!(
            params.query_pairs(),
            vec![
                ("build", "0".to_string()),
                ("mobi_app", "web".to_string()),
                ("platform", "web".to_string()),
                ("web_location", String::new()),
                ("id", "1001".to_string()),
                ("reply_time", "1700000000".to_string()),
            ]
        );
        Ok(())
    }

    #[test]
    fn reply_feed_params_rejects_zero_start_id() {
        let err = MessageReplyFeedParams::new().with_start_id(0).unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter { field: "id", .. }
        ));
    }

    #[test]
    fn single_unread_params_serializes_defaults() {
        let params = MessageSingleUnreadParams::new();

        assert_eq!(
            params.query_pairs(),
            vec![
                ("build", "0".to_string()),
                ("mobi_app", "web".to_string()),
                ("unread_type", "0".to_string()),
                ("show_unfollow_list", "0".to_string()),
                ("show_dustbin", "0".to_string()),
            ]
        );
    }

    #[test]
    fn single_unread_params_serializes_flags() {
        let params = MessageSingleUnreadParams::new()
            .with_unread_type(SingleUnreadType::Follow)
            .show_unfollow_list(true)
            .show_dustbin(true);

        assert_eq!(
            params.query_pairs(),
            vec![
                ("build", "0".to_string()),
                ("mobi_app", "web".to_string()),
                ("unread_type", "1".to_string()),
                ("show_unfollow_list", "1".to_string()),
                ("show_dustbin", "1".to_string()),
            ]
        );
    }

    #[test]
    fn single_unread_params_enables_dustbin_for_blocked_type() {
        let params = MessageSingleUnreadParams::new().with_unread_type(SingleUnreadType::Blocked);

        assert_eq!(params.query_pairs()[4], ("show_dustbin", "1".to_string()));
    }

    #[test]
    fn single_unread_params_rejects_zero_custom_type() {
        let err = MessageSingleUnreadParams::new()
            .with_custom_unread_type(0)
            .unwrap_err();

        assert!(matches!(
            err,
            BpiError::InvalidParameter {
                field: "unread_type",
                ..
            }
        ));
    }
}
