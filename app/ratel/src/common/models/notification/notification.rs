use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct Notification {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub status: EventStatus,

    pub data: NotificationData,
}

#[cfg(feature = "server")]
impl Notification {
    pub fn new(data: NotificationData) -> Self {
        let uid = uuid::Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string();
        let now = crate::common::utils::time::get_now_timestamp_millis();

        Self {
            pk: Partition::Notification(uid.clone()),
            sk: EntityType::Notification(uid),
            created_at: now,
            status: EventStatus::Requested,
            data,
        }
    }

    pub async fn process(&self) -> Result<()> {
        use crate::features::auth::models::EmailTemplate;
        use crate::features::auth::types::email_operation::EmailOperation;

        tracing::info!(
            "Notification send: pk={}, status={:?}, data={:?}",
            self.pk,
            self.status,
            self.data
        );

        let cfg = crate::common::CommonConfig::default();
        let cli = cfg.dynamodb();
        let ses = cfg.ses();

        match &self.data {
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

        // Update notification status to Completed
        Notification::updater(self.pk.clone(), self.sk.clone())
            .with_status(EventStatus::Completed)
            .execute(cli)
            .await?;

        Ok(())
    }
}
