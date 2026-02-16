#[derive(Debug, Clone, Copy)]
pub struct FirebaseConfig {
    pub api_key: &'static str,
    pub auth_domain: &'static str,
    pub project_id: &'static str,
    pub storage_bucket: &'static str,
    pub messaging_sender_id: &'static str,
    pub app_id: &'static str,
    pub measurement_id: &'static str,
}

impl Default for FirebaseConfig {
    fn default() -> Self {
        FirebaseConfig {
            api_key: option_env!("VITE_FIREBASE_API_KEY").unwrap_or(""),
            auth_domain: option_env!("VITE_FIREBASE_AUTH_DOMAIN").unwrap_or(""),
            project_id: option_env!("VITE_FIREBASE_PROJECT_ID").unwrap_or(""),
            storage_bucket: option_env!("VITE_FIREBASE_STORAGE_BUCKET").unwrap_or(""),
            messaging_sender_id: option_env!("VITE_FIREBASE_MESSAGING_SENDER_ID").unwrap_or(""),
            app_id: option_env!("VITE_FIREBASE_APP_ID").unwrap_or(""),
            measurement_id: option_env!("VITE_FIREBASE_MEASUREMENT_ID").unwrap_or(""),
        }
    }
}
