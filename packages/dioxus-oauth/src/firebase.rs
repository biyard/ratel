use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/firebase.js")]
extern "C" {
    #[wasm_bindgen(js_name = initializeApp)]
    fn initialize_app(
        api_key: &str,
        auth_domain: &str,
        project_id: &str,
        storage_bucket: &str,
        messaging_sender_id: &str,
        app_id: &str,
        measurement_id: &str,
    );

    #[wasm_bindgen(catch, js_name = signInWithPopup)]
    async fn sign_in_with_popup(scopes: Vec<String>) -> Result<JsValue, JsValue>;
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
pub struct Credential {
    pub id_token: String,
    pub access_token: String,
    pub display_name: String,
    pub email: String,
    pub photo_url: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FirebaseService {}

impl FirebaseService {
    pub fn new(
        api_key: &str,
        auth_domain: &str,
        project_id: &str,
        storage_bucket: &str,
        messaging_sender_id: &str,
        app_id: &str,
        measurement_id: &str,
    ) -> Self {
        tracing::debug!("FirebaseService::init: {api_key}, {auth_domain}, {project_id}, {storage_bucket}, {messaging_sender_id}, {app_id}, {measurement_id}");

        initialize_app(
            api_key,
            auth_domain,
            project_id,
            storage_bucket,
            messaging_sender_id,
            app_id,
            measurement_id,
        );

        Self {}
    }

    #[cfg(feature = "web")]
    pub async fn sign_in_with_popup(&self, scopes: Vec<String>) -> Credential {
        tracing::debug!("FirebaseService::sign_in_with_popup: {scopes:?}");
        let cred: Credential = match sign_in_with_popup(scopes).await {
            Ok(v) => {
                let c = v.as_string().unwrap_or_default();
                serde_json::from_str(&c).unwrap_or_default()
            }
            _ => Credential::default(),
        };

        cred
    }
}
