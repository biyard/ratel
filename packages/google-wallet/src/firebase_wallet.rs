use base64::{engine::general_purpose, Engine};
use dioxus_oauth::prelude::FirebaseService;
use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use ic_agent::{identity::BasicIdentity, Identity};
use ring::{rand::SystemRandom, signature::Ed25519KeyPair};

pub const IDENTITY_KEY: &str = "identity";

#[derive(Debug, Clone)]
pub struct FirebaseWallet {
    pub principal: Option<String>,
    pub firebase: FirebaseService,
}

impl FirebaseWallet {
    pub fn new(
        api_key: &str,
        auth_domain: &str,
        project_id: &str,
        storage_bucket: &str,
        messaging_sender_id: &str,
        app_id: &str,
        measurement_id: &str,
    ) -> Self {
        let firebase = FirebaseService::new(
            api_key,
            auth_domain,
            project_id,
            storage_bucket,
            messaging_sender_id,
            app_id,
            measurement_id,
        );

        // let mut srv = Self {
        // };

        // use_context_provider(|| srv);

        // use_future(move || async move {
        //     srv.try_setup_from_storage();
        // });
        Self::default()
    }

    pub fn get_login(&self) -> bool {
        self.is_some()
    }

    pub fn try_setup_from_storage(&mut self) -> Option<String> {
        if self.get_login() {
            return self.principal;
        }

        tracing::debug!("try_setup_from_storage");
        let key: Result<String, StorageError> = LocalStorage::get(IDENTITY_KEY);
        tracing::debug!("key from storage: {key:?}");

        if let Ok(private_key) = key {
            tracing::debug!("private_key: {private_key}");
            self.try_setup_from_private_key(private_key)
        } else {
            None
        }
    }

    pub async fn request_wallet_with_google(&mut self) -> Result<WalletEvent, String> {
        use crate::drive_api::DriveApi;

        let cred = self
            .firebase
            .sign_in_with_popup(vec![
                "https://www.googleapis.com/auth/drive.appdata".to_string()
            ])
            .await;
        tracing::debug!("cred: {cred:?}");
        let cli = DriveApi::new(cred.access_token);
        let data = match cli.list_files().await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("failed to get file {e}");
                vec![]
            }
        };
        tracing::debug!("data: {data:?}");

        let (evt, private_key) = match data
            .iter()
            .find(|x| x.name == option_env!("ENV").unwrap_or("local").to_string())
        {
            Some(v) => match cli.get_file(&v.id).await {
                Ok(v) => {
                    tracing::debug!("file content: {v}");

                    (WalletEvent::Login, v)
                    // self.try_setup_from_private_key(v);

                    // return Ok(WalletEvent::Login);
                }
                Err(e) => {
                    tracing::warn!("failed to get file {e}");

                    return Err("failed to get file".to_string());
                }
            },
            None => {
                tracing::warn!("file not found");
                let rng = SystemRandom::new();

                let key_pair = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
                let private_key = general_purpose::STANDARD.encode(key_pair.as_ref());

                if let Err(e) = cli.upload_file(&private_key).await {
                    tracing::error!("failed to upload file {e}");
                    return Err("failed to upload file".to_string());
                };

                // self.try_setup_from_private_key(private_key);

                (WalletEvent::Signup, private_key)
            }
        };

        self.try_setup_from_private_key(private_key);
        self.name.set(Some(cred.display_name));
        self.email.set(Some(cred.email));
        self.photo_url.set(Some(cred.photo_url));

        Ok(evt)
    }

    pub fn try_setup_from_private_key(&mut self, private_key: String) -> Option<String> {
        let id = match general_purpose::STANDARD.decode(private_key.clone()) {
            Ok(key) => {
                tracing::debug!("key setup");
                self.private_key.set(Some(private_key.clone()));
                self.init_or_get_identity(Some(key.as_ref()))
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
        {
            use gloo_storage::Storage;
            let _ = gloo_storage::LocalStorage::set(IDENTITY_KEY, private_key);
        }
        Some(principal.to_text())
    }

    pub fn init_or_get_identity(&mut self, id: Option<&[u8]>) -> Option<BasicIdentity> {
        let mut key_pair = self.key_pair.write();

        if key_pair.is_none() && id.is_some() {
            *key_pair = Some(id.unwrap().to_vec().clone());
        }

        if let Some(key_pair) = key_pair.clone() {
            let key = ring::signature::Ed25519KeyPair::from_pkcs8(&key_pair)
                .expect("Could not read the key pair.");
            Some(BasicIdentity::from_key_pair(key))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WalletEvent {
    Login,
    Signup,
    Logout,
}
