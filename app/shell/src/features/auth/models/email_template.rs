#[cfg(feature = "server")]
use crate::common::utils::aws::SesClient;

// Migrated from packages/main-api/src/models/email_template/email_template.rs
use crate::features::auth::types::email_operation::EmailOperation;
use crate::features::auth::*;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub targets: Vec<String>,
    pub operation: EmailOperation,
}

#[cfg(feature = "server")]
impl EmailTemplate {
    #[allow(unused_variables)]
    pub async fn send_email(&self, ses: &SesClient) -> Result<()> {
        if self.targets.is_empty() {
            return Ok(());
        }

        #[cfg(any(test, feature = "bypass"))]
        {
            let _ = ses;
            for email in &self.targets {
                tracing::warn!("sending email will be skipped for {}", email);
            }
            return Ok(());
        }

        #[cfg(all(not(test), not(feature = "bypass")))]
        {
            let template_name = self.operation.template_name();
            let data: serde_json::Value = serde_json::to_value(&self.operation).map_err(|_| {
                Error::InternalServerError("Failed to serialize email template data".into())
            })?;

            let recipients: Vec<(String, Option<serde_json::Value>)> = self
                .targets
                .iter()
                .cloned()
                .map(|email| (email, Some(data.clone())))
                .collect();

            ses.send_bulk_with_template(template_name, &recipients)
                .await
                .map_err(|e| Error::SendSmsFailed(e.to_string()))?;

            Ok(())
        }
    }
}
