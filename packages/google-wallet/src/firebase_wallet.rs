use base64::{engine::general_purpose, Engine};
use dioxus_oauth::prelude::FirebaseService;
use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use ring::{
    rand::SystemRandom,
    signature::{Ed25519KeyPair, KeyPair, Signature},
};
use simple_asn1::{
    oid, to_der,
    ASN1Block::{BitString, ObjectIdentifier, Sequence},
};

pub const IDENTITY_KEY: &str = "identity";

#[derive(Debug, Clone)]
pub struct FirebaseWallet {
    pub principal: Option<String>,
    pub firebase: FirebaseService,
    pub private_key: Option<String>,
    pub public_key: Option<Vec<u8>>,
    pub pkcs8: Option<Vec<u8>>,

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
            pkcs8: None,

            email: None,
            name: None,
            photo_url: None,
        }
    }

    pub fn get_login(&self) -> bool {
        self.principal.is_some()
    }

    pub fn get_principal(&self) -> String {
        let public_key = self.public_key.clone().unwrap_or_default();

        let id_ed25519 = oid!(1, 3, 101, 112);
        let algorithm = Sequence(0, vec![ObjectIdentifier(0, id_ed25519)]);
        let subject_public_key = BitString(0, public_key.len() * 8, public_key);
        let subject_public_key_info = Sequence(0, vec![algorithm, subject_public_key]);
        let der_public_key = to_der(&subject_public_key_info).unwrap();
        let wallet_address = candid::Principal::self_authenticating(der_public_key);
        wallet_address.to_text()
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

    pub fn logout(&mut self) {
        self.private_key = None;
        self.public_key = None;
        self.principal = None;
        self.email = None;
        self.name = None;
        self.photo_url = None;

        let _ = LocalStorage::delete(IDENTITY_KEY);
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

        let (evt, pkcs8) = match data
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

                (WalletEvent::Signup, private_key)
            }
        };

        self.try_setup_from_private_key(pkcs8);
        self.name = Some(cred.display_name);
        self.email = Some(cred.email);
        self.photo_url = Some(cred.photo_url);

        Ok(evt)
    }

    pub fn sign(&self, msg: &str) -> Option<Signature> {
        tracing::debug!("sign: {msg}");
        let key_pair = self.get_identity()?;

        Some(key_pair.sign(msg.as_bytes()))
    }

    pub fn public_key(&self) -> Option<Vec<u8>> {
        let key_pair = self.get_identity()?;

        Some(key_pair.public_key().as_ref().to_vec())
    }

    pub fn try_setup_from_private_key(&mut self, private_key: String) -> Option<String> {
        match general_purpose::STANDARD.decode(&private_key) {
            Ok(key) => {
                tracing::debug!("key setup");
                self.private_key = Some(private_key.clone());
                if let Some(key_pair) = self.init_or_get_identity(Some(key.as_ref())) {
                    self.public_key = Some(key_pair.public_key().as_ref().to_vec());
                    self.principal = Some(self.get_principal());
                    tracing::debug!("wallet initialized");
                }
            }
            Err(e) => {
                tracing::error!("Decode Error: {e}");

                return None;
            }
        };

        use gloo_storage::Storage;
        let _ = gloo_storage::LocalStorage::set(IDENTITY_KEY, private_key);

        Some(self.get_principal())
    }

    pub fn init_or_get_identity(&mut self, pkcs8: Option<&[u8]>) -> Option<Ed25519KeyPair> {
        if self.pkcs8.is_none() && pkcs8.is_some() {
            self.pkcs8 = Some(pkcs8.unwrap().to_vec());
        }

        self.get_identity()
    }

    pub fn get_identity(&self) -> Option<Ed25519KeyPair> {
        if let Some(pkcs8) = &self.pkcs8 {
            let key = ring::signature::Ed25519KeyPair::from_pkcs8(pkcs8)
                .expect("Could not read the key pair.");
            Some(key)
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
