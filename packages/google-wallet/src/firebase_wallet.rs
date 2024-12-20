#[cfg(feature = "web")]
use dioxus::prelude::*;
#[cfg(feature = "web")]
use dioxus_oauth::prelude::{Credential, FirebaseService};
use gloo_storage::LocalStorage;

pub const IDENTITY_KEY: &str = "identity";

#[derive(Debug, Clone, Copy, Default)]
pub struct FirebaseWallet {
    #[cfg(feature = "web")]
    pub firebase: FirebaseService,
}

impl FirebaseWallet {
    pub fn init(
        api_key: &str,
        auth_domain: &str,
        project_id: &str,
        storage_bucket: &str,
        messaging_sender_id: &str,
        app_id: &str,
        measurement_id: &str,
    ) {
        #[cfg(feature = "web")]
        FirebaseService::init(
            api_key,
            auth_domain,
            project_id,
            storage_bucket,
            messaging_sender_id,
            app_id,
            measurement_id,
        );

        let srv = Self {
            #[cfg(feature = "web")]
            firebase: use_context(),
        };

        use_context_provider(|| srv);
    }

    pub async fn request_wallet_with_google(&self) {
        use crate::drive_api::DriveApi;

        #[cfg(feature = "web")]
        let Credential {
            access_token,
            id_token,
        } = self
            .firebase
            .sign_in_with_popup(vec![
                "https://www.googleapis.com/auth/drive.appdata".to_string()
            ])
            .await;
        #[cfg(not(feature = "web"))]
        let (access_token, id_token) = ("".to_string(), "".to_string());
        let cli = DriveApi::new(access_token);
        let data = match cli.list_files().await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("failed to get file {e}");
                vec![]
            }
        };
        tracing::debug!("data: {data:?}");

        match data
            .iter()
            .find(|x| x.name == option_env!("ENV").unwrap_or("local").to_string())
        {
            Some(v) => match cli.get_file(&v.id).await {
                Ok(v) => {
                    tracing::debug!("file content: {v}");
                    auth.try_setup_from_private_key(v);
                    return;
                }
                Err(e) => {
                    tracing::warn!("failed to get file {e}");
                }
            },
            None => {
                tracing::warn!("file not found");
            }
        };
    }

    pub fn try_setup_from_private_key(&mut self, private_key: String) -> Option<String> {
        use base64::{engine::general_purpose, Engine};
        let id = match general_purpose::STANDARD.decode(private_key.clone()) {
            Ok(key) => {
                tracing::debug!("key setup");
                self.private_key.set(Some(private_key.clone()));
                crate::prelude::init_or_get_identity(Some(key.as_ref()))
            }
            Err(e) => {
                tracing::error!("Decode Error: {e}");
                None
            }
        };

        tracing::debug!("id: {id:?}");

        if id.is_none() {
            return None;
        }
        let id = id.unwrap();
        let principal = id.sender();
        if principal.is_err() {
            return None;
        }
        let principal = principal.unwrap();
        tracing::debug!("principal: {principal:?}");

        self.principal.set(Some(principal.to_text()));
        tracing::debug!("logged in as {}", principal.to_text());
        self.login.set(true);
        let _ = LocalStorage::set(IDENTITY_KEY, private_key);
        Some(principal.to_text())
    }
}
