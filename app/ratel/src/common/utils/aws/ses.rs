use std::sync::Arc;

use aws_sdk_sesv2::{
    Client,
    config::Config,
    types::{
        BulkEmailContent, BulkEmailEntry, Destination, ReplacementEmailContent,
        ReplacementTemplate, Template as SesTemplate,
    },
};

#[derive(Clone)]
pub struct SesClient {
    client: Client,
    from: Arc<String>,
    allow_error: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum SesServiceError {
    #[error("Invalid email content: {0}")]
    InvalidContent(String),
    #[error("Send Email Failed: {0}")]
    SendEmailFailed(String),
}
impl SesClient {
    pub fn new(config: Config, allow_error: bool, from_email: String) -> Self {
        let client = Client::from_conf(config);

        Self {
            client,
            from: Arc::new(from_email),
            allow_error,
        }
    }

    pub async fn send_bulk_with_template(
        &self,
        template_name: &str,
        recipients: &[(String, Option<serde_json::Value>)],
    ) -> Result<(), SesServiceError> {
        const MAX_PER_CALL: usize = 50;

        for chunk in recipients.chunks(MAX_PER_CALL) {
            let mut entries = Vec::with_capacity(chunk.len());

            for (to, data) in chunk {
                let destination = Destination::builder().to_addresses(to).build();

                let mut entry_builder = BulkEmailEntry::builder().destination(destination);

                if let Some(json) = data {
                    let json_str = json.to_string();

                    let replacement_template = ReplacementTemplate::builder()
                        .replacement_template_data(json_str)
                        .build();

                    let replacement_email_content = ReplacementEmailContent::builder()
                        .replacement_template(replacement_template)
                        .build();

                    entry_builder =
                        entry_builder.replacement_email_content(replacement_email_content);
                }

                entries.push(entry_builder.build());
            }

            let default_template = SesTemplate::builder()
                .template_name(template_name)
                .template_data("{}".to_string())
                .build();

            let default_content = BulkEmailContent::builder()
                .template(default_template)
                .build();

            let result = self
                .client
                .send_bulk_email()
                .from_email_address(self.from.as_ref())
                .default_content(default_content)
                .set_bulk_email_entries(Some(entries))
                .send()
                .await
                .map_err(|e| {
                    tracing::error!("SES SendBulkEmail Error: {:?}", e);
                    SesServiceError::SendEmailFailed(e.to_string())
                });

            if let Err(e) = result {
                if !self.allow_error {
                    return Err(e);
                }
                tracing::warn!("SES SendBulkEmail Error Ignored: {:?}", e);
            }
        }

        Ok(())
    }
}
