use crate::utils::aws::SesClient;
use crate::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};

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
        }
    }
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct EmailTemplate {
    pub targets: Vec<String>,
    pub operation: EmailOperation,
}

impl EmailTemplate {
    pub async fn send_email(&self, ses: &SesClient) -> Result<()> {
        #[cfg(any(test, feature = "no-secret"))]
        {
            let _ = ses;
            for email in &self.targets {
                tracing::warn!("sending email will be skipped for {}", email);
            }
            return Ok(());
        }

        #[cfg(all(not(test), not(feature = "no-secret")))]
        {
            if self.targets.is_empty() {
                return Ok(());
            }

            let template_name = self.operation.template_name();
            let data: JsonValue = serde_json::to_value(&self.operation).map_err(|_| {
                Error::InternalServerError("Failed to serialize email template data".into())
            })?;

            let recipients: Vec<(String, Option<JsonValue>)> = self
                .targets
                .iter()
                .cloned()
                .map(|email| (email, Some(data.clone())))
                .collect();

            ses.send_bulk_with_template(template_name, &recipients)
                .await
                .map_err(|e| Error::AwsSesSendEmailException(e.to_string()))?;

            Ok(())
        }
    }
}
