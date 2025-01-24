use tracing::Level;

#[derive(Debug)]
pub struct FirebaseConfig {
    pub api_key: String,
    pub auth_domain: String,
    pub project_id: String,
    pub storage_bucket: String,
    pub messaging_sender_id: String,
    pub app_id: String,
    pub measurement_id: String,
}

#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub domain: &'static str,
    pub log_level: Level,
    pub main_api_endpoint: String,
    pub firebase: FirebaseConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            domain: option_env!("DOMAIN").expect("You must set DOMAIN"),
            log_level: match option_env!("RUST_LOG") {
                Some("trace") => Level::TRACE,
                Some("debug") => Level::DEBUG,
                Some("info") => Level::INFO,
                Some("warn") => Level::WARN,
                Some("error") => Level::ERROR,
                _ => Level::INFO,
            },
            main_api_endpoint: match option_env!("MAIN_API_ENDPOINT") {
                Some(endpoint) => endpoint.to_string(),
                None => format!(
                    "https://api.{}",
                    option_env!("DOMAIN").unwrap_or("dev.democrasee.me")
                ),
            },
            firebase: FirebaseConfig {
                api_key: option_env!("FIREBASE_API_KEY")
                    .expect("You must set FIREBASE_API_KEY")
                    .to_string(),
                auth_domain: option_env!("FIREBASE_AUTH_DOMAIN")
                    .expect("You must set FIREBASE_AUTH_DOMAIN")
                    .to_string(),
                project_id: option_env!("FIREBASE_PROJECT_ID")
                    .expect("You must set FIREBASE_PROJECT_ID")
                    .to_string(),
                storage_bucket: option_env!("FIREBASE_STORAGE_BUCKET")
                    .expect("You must set FIREBASE_STORAGE_BUCKET")
                    .to_string(),
                messaging_sender_id: option_env!("FIREBASE_MESSAGING_SENDER_ID")
                    .expect("You must set FIREBASE_MESSAGING_SENDER_ID")
                    .to_string(),
                app_id: option_env!("FIREBASE_APP_ID")
                    .expect("You must set FIREBASE_APP_ID")
                    .to_string(),
                measurement_id: option_env!("FIREBASE_MEASUREMENT_ID")
                    .expect("You must set FIREBASE_MEASUREMENT_ID")
                    .to_string(),
            },
        }
    }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        CONFIG.as_ref().unwrap()
    }
}
