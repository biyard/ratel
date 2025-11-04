use std::sync::Arc;

use crate::config;

use aws_config::SdkConfig;
use aws_sdk_sesv2::{
    Client,
    config::Config,
    types::{Body, Content, Destination, EmailContent, Message},
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
    pub fn new(config: SdkConfig, allow_error: bool) -> Self {
        let aws_config = Config::from(&config);
        let client = Client::from_conf(aws_config);

        Self {
            client,
            from: Arc::new(config::get().from_email.to_string()),
            allow_error,
        }
    }

    fn sanitize_subject(subject: &str) -> String {
        subject
            .chars()
            .filter(|&c| c != '\r' && c != '\n')
            .collect()
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
