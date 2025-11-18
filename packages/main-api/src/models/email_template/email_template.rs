use crate::utils::aws::SesClient;
use crate::*;

// FIXME: fix to redefine template model
#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct EmailTemplate {
    pub email: String,
    pub title: String,
    pub html_contents: String,
    pub fallback_contents: String,
}

impl EmailTemplate {
    pub async fn send_email(&self, ses: &SesClient) -> Result<()> {
        #[cfg(any(test, feature = "no-secret"))]
        {
            let _ = ses;
            tracing::warn!("sending email will be skipped for {}", self.email,);
        }

        #[cfg(all(not(test), not(feature = "no-secret")))]
        {
            use crate::utils::html::signup_html;

            let mut i = 0;
            while let Err(e) = ses
                .send_mail_html(
                    &self.email,
                    &self.title,
                    &self.html_contents,
                    Some(&self.fallback_contents),
                )
                .await
            {
                btracing::notify!(
                    crate::config::get().slack_channel_monitor,
                    &format!("Failed to send email: {:?}", e)
                );
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                i += 1;
                if i >= 3 {
                    return Err(Error::AwsSesSendEmailException(e.to_string()));
                }
            }
        }

        Ok(())
    }
}
