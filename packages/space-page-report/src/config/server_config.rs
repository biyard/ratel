#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub env: &'static str,
    pub bucket_name: &'static str,
    pub bedrock_model_id: &'static str,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            env: option_env!("ENV").expect("You must set ENV"),
            bucket_name: option_env!("BUCKET_NAME").expect("You must set BUCKET_NAME"),
            bedrock_model_id: option_env!("BEDROCK_MODEL_ID")
                .expect("You must set BEDROCK_MODEL_ID"),
        }
    }
}

static mut CONFIG: Option<ServerConfig> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static ServerConfig {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(ServerConfig::default());
        }
        CONFIG.as_ref().unwrap()
    }
}
