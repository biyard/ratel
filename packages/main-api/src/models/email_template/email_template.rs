use crate::utils::aws::SesClient;
use crate::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, json};

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
