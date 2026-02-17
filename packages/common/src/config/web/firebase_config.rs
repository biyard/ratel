use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirebaseConfig {
    pub api_key: &'static str,
    pub auth_domain: &'static str,
    pub project_id: &'static str,
    pub storage_bucket: &'static str,
    pub messaging_sender_id: &'static str,
    pub app_id: &'static str,
    pub measurement_id: &'static str,
}

impl Into<JsValue> for FirebaseConfig {
    fn into(self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap_or_else(|err| {
            error!("Failed to convert FirebaseConfig to JsValue: {}", err);
            JsValue::NULL
        })
    }
}

impl Default for FirebaseConfig {
    fn default() -> Self {
        FirebaseConfig {
            api_key: option_env!("FIREBASE_API_KEY").unwrap_or_else(|| {
                warn!("FIREBASE_API_KEY is not set in the environment variables. Using empty string as default.");
                ""
            }),
            auth_domain: option_env!("FIREBASE_AUTH_DOMAIN").unwrap_or_else(|| {
                warn!("FIREBASE_AUTH_DOMAIN is not set in the environment variables. Using empty string as default.");
                ""
            }),
            project_id: option_env!("FIREBASE_PROJECT_ID").unwrap_or_else(|| {
                warn!("FIREBASE_PROJECT_ID is not set in the environment variables. Using empty string as default.");
                ""
            }),
            storage_bucket: option_env!("FIREBASE_STORAGE_BUCKET").unwrap_or_else(|| {
                warn!("FIREBASE_STORAGE_BUCKET is not set in the environment variables. Using empty string as default.");
                ""
            }),
            messaging_sender_id: option_env!("FIREBASE_MESSAGING_SENDER_ID").unwrap_or_else(|| {
                warn!("FIREBASE_MESSAGING_SENDER_ID is not set in the environment variables. Using empty string as default.");
                ""
            }),
            app_id: option_env!("FIREBASE_APP_ID").unwrap_or_else(|| {
                warn!("FIREBASE_APP_ID is not set in the environment variables. Using empty string as default.");
                ""
            }),
            measurement_id: option_env!("FIREBASE_MEASUREMENT_ID").unwrap_or_else(|| {
                warn!("FIREBASE_MEASUREMENT_ID is not set in the environment variables. Using empty string as default.");
                ""
            }),
        }
    }
}
