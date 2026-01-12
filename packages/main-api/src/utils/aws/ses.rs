use std::{env, sync::Arc};

use crate::config;
use aws_config::SdkConfig;
use aws_sdk_sesv2::{
    Client,
    config::Config,
    types::{
        Body, BulkEmailContent, BulkEmailEntry, Content, Destination, EmailContent,
        EmailTemplateContent, Message, ReplacementEmailContent, ReplacementTemplate,
        Template as SesTemplate,
    },
};
use serde_json::Value as JsonValue;

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
    pub fn new(config: SdkConfig, allow_error: bool, enable_config: bool) -> Self {
        let aws_config = Config::from(&config);
        let client = Client::from_conf(aws_config);
        let from_email = if enable_config {
            config::get().from_email.to_string()
        } else {
            env::var("FROM_EMAIL").unwrap_or_else(|_| "no-reply@ratel.foundation".to_string())
        };

        Self {
            client,
            from: Arc::new(from_email),
            allow_error,
        }
    }

    fn sanitize_subject(subject: &str) -> String {
        subject
            .chars()
            .filter(|&c| c != '\r' && c != '\n')
            .collect()
    }

    pub async fn create_template(
        &self,
        template_name: &str,
        subject: &str,
        html: &str,
    ) -> Result<(), SesServiceError> {
        let tpl_content = EmailTemplateContent::builder()
            .subject(Self::sanitize_subject(subject))
            .html(html.to_string())
            .build();

        self.client
            .create_email_template()
            .template_name(template_name)
            .template_content(tpl_content)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("SES Create Template Error: {:?}", e);
                SesServiceError::SendEmailFailed(e.to_string())
            })?;

        Ok(())
    }

    pub async fn send_bulk_with_template(
        &self,
        template_name: &str,
        recipients: &[(String, Option<JsonValue>)],
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

    pub async fn send_mail_html(
        &self,
        to: &str,
        subject: &str,
        html: &str,
        text_fallback: Option<&str>,
    ) -> Result<(), SesServiceError> {
        let destination = Destination::builder().to_addresses(to).build();

        let mut body_builder = Body::builder().html(
            Content::builder()
                .data(html.to_string())
                .charset("UTF-8")
                .build()
                .map_err(|e| SesServiceError::InvalidContent(e.to_string()))?,
        );

        if let Some(text) = text_fallback {
            body_builder = body_builder.text(
                Content::builder()
                    .data(text.to_string())
                    .charset("UTF-8")
                    .build()
                    .map_err(|e| SesServiceError::InvalidContent(e.to_string()))?,
            );
        }

        let msg = Message::builder()
            .subject(
                Content::builder()
                    .data(Self::sanitize_subject(subject))
                    .charset("UTF-8")
                    .build()
                    .map_err(|e| SesServiceError::InvalidContent(e.to_string()))?,
            )
            .body(body_builder.build())
            .build();

        let content = EmailContent::builder().simple(msg).build();

        self.client
            .send_email()
            .from_email_address(self.from.as_ref())
            .destination(destination)
            .content(content)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("SES Send Email Error: {:?}", e);
                SesServiceError::SendEmailFailed(e.to_string())
            })?;

        Ok(())
    }

    pub async fn send_mail(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), SesServiceError> {
        let destination = Destination::builder()
            .set_to_addresses(Some(vec![to.to_string()]))
            .build();

        let body = Content::builder()
            .data(body)
            .build()
            .map_err(|e| SesServiceError::InvalidContent(e.to_string()))?;

        let body = Body::builder().text(body).build();

        let subject = Content::builder()
            .data(subject)
            .build()
            .map_err(|e| SesServiceError::InvalidContent(e.to_string()))?;
        let msg = Message::builder().subject(subject).body(body).build();

        let content = EmailContent::builder().simple(msg).build();
        let from = self.from.as_ref();
        let result = self
            .client
            .send_email()
            .from_email_address(from)
            .destination(destination)
            .content(content)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("SES Send Email Error: {:?}", e);
                SesServiceError::SendEmailFailed(e.to_string())
            });
        if let Err(e) = result {
            tracing::warn!("SES Send Email Error Ignored: {:?}", e);
            if !self.allow_error {
                return Err(e);
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub fn mock(config: SdkConfig) -> Self {
        Self {
            client: Client::from_conf(Config::from(&config)),
            from: Arc::new("no@rep.ly".to_string()),
            allow_error: true,
        }
    }
}
