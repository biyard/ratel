#[derive(Debug, Clone, Copy)]
pub struct GoogleCloudConfig {
    pub project_id: &'static str,
    pub enable_fcm: bool,
}

impl Default for GoogleCloudConfig {
    fn default() -> Self {
        GoogleCloudConfig {
            project_id: option_env!("RATEL_PROJECT_ID").unwrap_or(""),
            enable_fcm: option_env!("FCM_ENABLED")
                .map(|s| s.parse::<bool>().unwrap_or(false))
                .unwrap_or(false),
        }
    }
}
