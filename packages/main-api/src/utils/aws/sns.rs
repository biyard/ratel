use crate::*;
use aws_config::SdkConfig;
use aws_sdk_sns::{Client, config::Config};

#[derive(Clone)]
pub struct SnsClient {
    client: Client,
}

#[cfg(not(test))]
impl SnsClient {
    pub fn new(config: SdkConfig) -> Self {
        let aws_config = Config::from(&config);
        let client = Client::from_conf(aws_config);

        Self { client }
    }

    pub async fn send_sms(&self, phone_number: &str, message: &str) -> Result<()> {
        self.client
            .publish()
            .phone_number(phone_number)
            .message(message)
            .send()
            .await
            .map_err(|e| Error::SendSmsFailed(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
impl SnsClient {
    pub fn new(config: SdkConfig) -> Self {
        let aws_config = Config::from(&config);
        let client = Client::from_conf(aws_config);

        Self { client }
    }

    pub async fn send_sms(&self, _phone_number: &str, _message: &str) -> Result<()> {
        Ok(())
    }

    pub fn mock(config: SdkConfig) -> Self {
        Self {
            client: Client::from_conf(Config::from(&config)),
        }
    }
}
