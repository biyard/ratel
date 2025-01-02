use base64::{engine::general_purpose, Engine};
use dioxus_oauth::prelude::FirebaseService;
use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use ic_agent::{identity::BasicIdentity, Identity};
use ring::{
    rand::SystemRandom,
    signature::{Ed25519KeyPair, Signature},
};

pub const IDENTITY_KEY: &str = "identity";

#[derive(Debug, Clone)]
pub struct FirebaseWallet {
    pub principal: Option<String>,
    pub firebase: FirebaseService,
    pub private_key: Option<String>,
    pub public_key: Option<String>,
    pub key_pair: Option<Vec<u8>>,

    pub email: Option<String>,
    pub name: Option<String>,
    pub photo_url: Option<String>,
}

impl FirebaseWallet {
    pub fn new(
        api_key: String,
        auth_domain: String,
        project_id: String,
        storage_bucket: String,
        messaging_sender_id: String,
        app_id: String,
        measurement_id: String,
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

        Self {
            firebase,
            principal: None,
            private_key: None,
            public_key: None,
            key_pair: None,

            email: None,
            name: None,
            photo_url: None,
        }
    }

    pub fn get_login(&self) -> bool {
        self.principal.is_some()
    }

    pub fn get_principal(&self) -> String {
        self.principal.clone().unwrap_or_default()
    }

    pub fn get_user_info(&self) -> Option<(String, String, String)> {
        if self.email.is_none() || self.name.is_none() || self.photo_url.is_none() {
            return None;
        }

        Some((
            self.email.clone().unwrap(),
            self.name.clone().unwrap(),
            self.photo_url.clone().unwrap(),
        ))
    }

    pub fn try_setup_from_storage(&mut self) -> Option<String> {
        if self.get_login() {
            return self.principal.clone();
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
                return Err(format!("{e:?}"));
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

                self.try_setup_from_private_key(private_key.clone());

                (WalletEvent::Signup, private_key)
            }
        };

        self.try_setup_from_private_key(private_key);
        self.name = Some(cred.display_name);
        self.email = Some(cred.email);
        self.photo_url = Some(cred.photo_url);

        Ok(evt)
    }

    pub fn sign(&self, msg: &str) -> Option<Signature> {
        let private_key_bytes = general_purpose::STANDARD
            .decode(self.private_key.clone()?)
            .unwrap_or_default();

        let key_pair =
            Ed25519KeyPair::from_pkcs8(&private_key_bytes).expect("invalid private key format");

        Some(key_pair.sign(msg.as_bytes()))
    }

    pub fn try_setup_from_private_key(&mut self, private_key: String) -> Option<String> {
        let id = match general_purpose::STANDARD.decode(&private_key) {
            Ok(key) => {
                tracing::debug!("key setup");
                self.private_key = Some(private_key.clone());
                match self.init_or_get_identity(Some(key.as_ref())) {
                    Some(id) => {
                        let public_key = general_purpose::STANDARD.encode(id.public_key().unwrap());
                        self.public_key = Some(public_key);
                        Some(id)
                    }
                    None => None,
                }
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

        self.principal = Some(principal.to_text());
        tracing::debug!("logged in as {}", principal.to_text());
        {
            use gloo_storage::Storage;
            let _ = gloo_storage::LocalStorage::set(IDENTITY_KEY, private_key);
        }
        Some(principal.to_text())
    }

    pub fn init_or_get_identity(&mut self, id: Option<&[u8]>) -> Option<BasicIdentity> {
        if self.key_pair.is_none() && id.is_some() {
            self.key_pair = Some(id.unwrap().to_vec().clone());
        }

        if self.key_pair.is_some() {
            let key = ring::signature::Ed25519KeyPair::from_pkcs8(
                self.key_pair.clone().unwrap().as_ref(),
            )
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
