use super::ServerConfig;
use aws_sdk_sesv2::Client;
use dioxus::fullstack::Lazy;

use crate::utils::aws::SesClient;

pub static SES_CLIENT: Lazy<SesClient> = Lazy::new(|| async move {
    let config = ServerConfig::default();
    let ses_config = SesConfig::default();
    let aws_sdk_config = config.aws.get_sdk_config();
    let config = aws_sdk_sesv2::Config::from(&aws_sdk_config);

    dioxus::Ok(SesClient::new(config, true, ses_config.from_email))
});

pub struct SesConfig {
    pub from_email: String,
}

impl Default for SesConfig {
    fn default() -> Self {
        let from_email =
            std::env::var("FROM_EMAIL").unwrap_or_else(|_| "no-reply@ratel.foundation".to_string());
        SesConfig { from_email }
    }
}
