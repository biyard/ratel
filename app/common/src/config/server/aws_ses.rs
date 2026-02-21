pub static SES_CLIENT: dioxus::fullstack::Lazy<crate::utils::aws::SesClient> =
    dioxus::fullstack::Lazy::new(|| async move {
        let aws_conf = crate::config::aws_config::AwsConfig::default();
        let config = aws_sdk_sesv2::Config::builder()
            .region(aws_sdk_sesv2::config::Region::new(aws_conf.region))
            .behavior_version_latest()
            .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
                aws_conf.access_key_id,
                aws_conf.secret_access_key,
                None,
                None,
                "loaded-from-config",
            ))
            .build();
        let from_email =
            std::env::var("FROM_EMAIL").unwrap_or_else(|_| "no-reply@ratel.foundation".to_string());
        dioxus::Ok(crate::utils::aws::SesClient::new(config, true, from_email))
    });
