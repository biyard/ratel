// Migrated from packages/main-api/src/types/email_operation.rs
use crate::features::auth::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum EmailOperation {
    SignupSecurityCode {
        display_name: String,
        code_1: String,
        code_2: String,
        code_3: String,
        code_4: String,
        code_5: String,
        code_6: String,
    },
    SpaceInviteVerification {
        space_title: String,
        space_desc: String,
        author_profile: String,
        author_display_name: String,
        author_username: String,
        cta_url: String,
    },
    SpaceStatusNotification {
        headline: String,
        body: String,
        space_title: String,
        cta_url: String,
    },
    MentionNotification {
        mentioned_by_name: String,
        comment_preview: String,
        cta_url: String,
    },
    ReplyOnCommentNotification {
        replier_name: String,
        comment_preview: String,
        reply_preview: String,
        cta_url: String,
    },
    SpaceActionOngoingNotification {
        space_title: String,
        action_title: String,
        action_type_label: String,
        cta_url: String,
    },
}

impl Default for EmailOperation {
    fn default() -> Self {
        EmailOperation::SignupSecurityCode {
            display_name: String::new(),
            code_1: String::new(),
            code_2: String::new(),
            code_3: String::new(),
            code_4: String::new(),
            code_5: String::new(),
            code_6: String::new(),
        }
    }
}

impl EmailOperation {
    pub fn template_name(&self) -> &'static str {
        match self {
            EmailOperation::SignupSecurityCode { .. } => "signup_code",
            EmailOperation::SpaceInviteVerification { .. } => "email_verification",
            EmailOperation::SpaceStatusNotification { .. } => "space_status_notification",
            EmailOperation::MentionNotification { .. } => "mention_notification",
            EmailOperation::ReplyOnCommentNotification { .. } => "reply_on_comment_notification",
            EmailOperation::SpaceActionOngoingNotification { .. } => {
                "space_action_ongoing_notification"
            }
        }
    }
}
