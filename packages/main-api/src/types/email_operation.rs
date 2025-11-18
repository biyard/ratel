use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmailOperation {
    SpacePostNotification {
        author_profile: String,
        author_display_name: String,
        author_username: String,
        post_title: String,
        post_desc: String,
        connect_link: String,
    },
    TeamInvite {
        team_name: String,
        team_profile: String,
        team_display_name: String,
        url: String,
    },
    SpaceInviteVerification {
        space_title: String,
        space_desc: String,
        author_profile: String,
        author_display_name: String,
        author_username: String,
        cta_url: String,
    },
    SignupSecurityCode {
        email: String,
        username: String,
        code: String,
    },
}

impl Default for EmailOperation {
    fn default() -> Self {
        EmailOperation::SpacePostNotification {
            author_profile: String::new(),
            author_display_name: String::new(),
            author_username: String::new(),
            post_title: String::new(),
            post_desc: String::new(),
            connect_link: String::new(),
        }
    }
}

impl EmailOperation {
    pub fn template_name(&self) -> &'static str {
        match self {
            EmailOperation::SpacePostNotification { .. } => "space_post_notification",
            EmailOperation::TeamInvite { .. } => "team_invite",
            EmailOperation::SpaceInviteVerification { .. } => "email_verification",
            EmailOperation::SignupSecurityCode { .. } => "signup_code",
        }
    }
}
