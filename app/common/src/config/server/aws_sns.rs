pub static SNS_CLIENT: dioxus::fullstack::Lazy<crate::utils::aws::SnsClient> =
    dioxus::fullstack::Lazy::new(|| async move {
        let aws_conf = crate::config::aws_config::AwsConfig::default();
        let sns_region =
            std::env::var("SNS_REGION").unwrap_or_else(|_| aws_conf.region.to_string());
        let config = aws_sdk_sns::Config::builder()
            .region(aws_sdk_sns::config::Region::new(sns_region))
            .behavior_version_latest()
            .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
                aws_conf.access_key_id,
                aws_conf.secret_access_key,
                None,
                None,
                "loaded-from-config",
            ))
            .build();
        dioxus::Ok(crate::utils::aws::SnsClient::new(config))
    });
