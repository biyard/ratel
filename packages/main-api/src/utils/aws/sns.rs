use aws_config::SdkConfig;
use aws_sdk_sns::{Client, config::Config};

#[derive(Clone)]
pub struct SnsClient {
    client: Client,
    allow_error: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum SnsServiceError {
    #[error("Send SMS Failed: {0}")]
    SendSmsFailed(String),
}

impl SnsClient {
    pub fn new(config: SdkConfig, allow_error: bool) -> Self {
        let aws_config = Config::from(&config);
        let client = Client::from_conf(aws_config);

        Self {
            client,
            allow_error,
        }
    }

    pub async fn send_sms(
        &self,
        phone_number: &str,
        message: &str,
    ) -> Result<(), SnsServiceError> {
        let result = self
            .client
            .publish()
            .phone_number(phone_number)
            .message(message)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("SNS Send SMS Error: {:?}", e);
                SnsServiceError::SendSmsFailed(e.to_string())
            });

        if let Err(e) = result {
            if !self.allow_error {
                return Err(e);
            }
            tracing::warn!("SNS Send SMS Error Ignored: {:?}", e);
        }

        Ok(())
    }

    #[cfg(test)]
    pub fn mock(config: SdkConfig) -> Self {
        Self {
            client: Client::from_conf(Config::from(&config)),
            allow_error: true,
        }
    }
}
