use std::env;

#[derive(Debug, Clone, Copy)]
pub struct AwsConfig {
    pub region: &'static str,
    pub access_key_id: &'static str,
    pub secret_access_key: &'static str,
    pub account_id: &'static str,
}

impl Default for AwsConfig {
    fn default() -> Self {
        let region = option_env!("AWS_REGION").expect("You must set AWS_REGION");
        let region = env::var("REGION").unwrap_or_else(|_| region.to_string());

        AwsConfig {
            region: Box::leak(region.into_boxed_str()),
            access_key_id: option_env!("AWS_ACCESS_KEY_ID")
                .expect("You must set AWS_ACCESS_KEY_ID"),
            secret_access_key: option_env!("AWS_SECRET_ACCESS_KEY")
                .expect("AWS_SECRET_ACCESS_KEY is required"),
            account_id: option_env!("ACCOUNT_ID").unwrap_or(""),
        }
    }
}
