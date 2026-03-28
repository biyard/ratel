use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum NotificationData {
    #[default]
    None,
    SendVerificationCode {
        code: String,
        email: String,
    },
    SendSpaceInvitation {
        emails: Vec<String>,
        space_title: String,
        space_content: String,
        author_profile_url: String,
        author_username: String,
        author_display_name: String,
        cta_url: String,
    },
}
