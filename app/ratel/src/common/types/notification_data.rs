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

#[cfg(feature = "server")]
impl NotificationData {
    pub async fn send(&self) -> Result<()> {
        use crate::features::auth::models::EmailTemplate;
        use crate::features::auth::types::email_operation::EmailOperation;

        let cfg = crate::common::CommonConfig::default();
        let ses = cfg.ses();

        match self {
            NotificationData::SendVerificationCode { code, email } => {
                let chars: Vec<char> = code.chars().collect();
                let operation = EmailOperation::SignupSecurityCode {
                    display_name: email.clone(),
                    code_1: chars.first().map(|c| c.to_string()).unwrap_or_default(),
                    code_2: chars.get(1).map(|c| c.to_string()).unwrap_or_default(),
                    code_3: chars.get(2).map(|c| c.to_string()).unwrap_or_default(),
                    code_4: chars.get(3).map(|c| c.to_string()).unwrap_or_default(),
                    code_5: chars.get(4).map(|c| c.to_string()).unwrap_or_default(),
                    code_6: chars.get(5).map(|c| c.to_string()).unwrap_or_default(),
                };

                let template = EmailTemplate {
                    targets: vec![email.clone()],
                    operation,
                };
                template.send_email(ses).await?;
            }
            NotificationData::SendSpaceInvitation {
                emails,
                space_title,
                space_content,
                author_profile_url,
                author_username,
                author_display_name,
                cta_url,
            } => {
                let operation = EmailOperation::SpaceInviteVerification {
                    space_title: space_title.clone(),
                    space_desc: space_content.clone(),
                    author_profile: author_profile_url.clone(),
                    author_display_name: author_display_name.clone(),
                    author_username: author_username.clone(),
                    cta_url: cta_url.clone(),
                };

                let template = EmailTemplate {
                    targets: emails.clone(),
                    operation,
                };
                template.send_email(ses).await?;
            }
            NotificationData::None => {
                tracing::warn!("Received notification with no data, skipping");
            }
        }

        Ok(())
    }
}
