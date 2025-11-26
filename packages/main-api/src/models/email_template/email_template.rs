use crate::utils::aws::SesClient;
use crate::*;
use crate::{
    email_operation::EmailOperation, features::notification::Notification, models::user::User,
    utils::aws::DynamoClient,
};
use aws_sdk_dynamodb::types::TransactWriteItem;
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct EmailTemplate {
    pub targets: Vec<String>,
    pub operation: EmailOperation,
}

impl EmailTemplate {
    fn should_create_notification(&self) -> bool {
        !matches!(self.operation, EmailOperation::SignupSecurityCode { .. })
    }

    async fn create_notifications(&self, dynamo: &DynamoClient) -> Result<()> {
        if self.targets.is_empty() || !self.should_create_notification() {
            return Ok(());
        }

        let mut tx_items: Vec<TransactWriteItem> = Vec::new();

        for email in &self.targets {
            let opt = User::opt_one();
            let (users, _) = User::find_by_email(&dynamo.client, email.to_string(), opt).await?;

            if let Some(user) = users.into_iter().next() {
                let noti = Notification::new(self.operation.clone(), user);
                tx_items.push(noti.create_transact_write_item());
            } else {
                tracing::warn!(
                    "no user found for email={}, skip notification creation",
                    email
                );
            }
        }

        if tx_items.is_empty() {
            return Ok(());
        }

        for chunk in tx_items.chunks(25) {
            dynamo
                .client
                .transact_write_items()
                .set_transact_items(Some(chunk.to_vec()))
                .send()
                .await
                .map_err(|e| Error::DynamoDbError(e.into()))?;
        }

        Ok(())
    }

    #[allow(unused_variables)]
    pub async fn send_email(
        &self,
        dynamo: &DynamoClient,
        ses: &SesClient,
        require_email_success: bool,
    ) -> Result<()> {
        if self.targets.is_empty() {
            return Ok(());
        }

        #[cfg(any(test, feature = "no-secret"))]
        {
            let _ = ses;
            for email in &self.targets {
                tracing::warn!("sending email will be skipped for {}", email);
            }

            return self.create_notifications(dynamo).await;
        }

        #[cfg(all(not(test), not(feature = "no-secret")))]
        {
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

            // Send email via SES
            let email_result = ses.send_bulk_with_template(template_name, &recipients).await;

            // Handle email errors based on require_email_success flag
            if let Err(e) = email_result {
                if require_email_success {
                    // Critical flows (e.g., auth) need to propagate errors
                    return Err(Error::AwsSesSendEmailException(e.to_string()));
                } else {
                    // Non-critical flows can log and continue to create notifications
                    tracing::error!("Failed to send email via SES (non-critical): {:?}", e);
                }
            }


            self.create_notifications(dynamo).await

        }
    }
}
