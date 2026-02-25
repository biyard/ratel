use super::ServerConfig;
use crate::utils::aws::SnsClient;
use aws_config::Region;
use dioxus::fullstack::Lazy;

pub static SNS_CLIENT: Lazy<SnsClient> = Lazy::new(|| async move {
    let config = ServerConfig::default();
    let aws_sdk_config = config.aws.get_sdk_config();

    let aws_sdk_config = if let Ok(sns_region) = std::env::var("SNS_REGION") {
        let region = Region::new(sns_region);
        aws_sdk_config.into_builder().region(region).build()
    } else {
        aws_sdk_config
    };

    let config = aws_sdk_sns::Config::from(&aws_sdk_config);
    dioxus::Ok(SnsClient::new(config))
});
