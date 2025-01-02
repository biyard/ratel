#![allow(non_snake_case)]
use dioxus::prelude::*;
use dto::*;

use crate::{
    config,
    utils::rest_api::{self, Signature, SignatureAlgorithm, Signer},
};

pub enum UserEvent {
    Signup(String, String, String),
    Login(String, String, String),
    Logout,
}

#[derive(Debug, Clone, Copy)]
pub struct UserService {
    #[cfg(feature = "web-only")]
    pub firebase: Signal<google_wallet::FirebaseWallet>,

    pub endpoint: Signal<String>,
    pub email: Signal<String>,
    pub nickname: Signal<String>,
    pub profile_url: Signal<String>,
}

impl UserService {
    pub fn init() {
        let conf = config::get();

        #[cfg(feature = "web-only")]
        let mut firebase = google_wallet::FirebaseWallet::new(
            conf.firebase.api_key.clone(),
            conf.firebase.auth_domain.clone(),
            conf.firebase.project_id.clone(),
            conf.firebase.storage_bucket.clone(),
            conf.firebase.messaging_sender_id.clone(),
            conf.firebase.app_id.clone(),
            conf.firebase.measurement_id.clone(),
        );

        #[cfg(feature = "web-only")]
        {
            let w = firebase.try_setup_from_storage();
            if w.is_some() {
                tracing::debug!("UserService::init: wallet={:?}", w);
            }
        }

        use_context_provider(|| Self {
            #[cfg(feature = "web-only")]
            firebase: Signal::new(firebase),
            endpoint: Signal::new(conf.main_api_endpoint.clone()),
            email: Signal::new("".to_string()),
            nickname: Signal::new("".to_string()),
            profile_url: Signal::new("".to_string()),
        });
    }

    #[cfg(feature = "web-only")]
    async fn request_to_firebase(
        &mut self,
    ) -> Result<(google_wallet::WalletEvent, String, String, String, String)> {
        let mut firebase = self.firebase.write();
        let (evt, principal, email, name, profile_url) =
            match firebase.request_wallet_with_google().await {
                Ok(evt) => {
                    tracing::debug!("UserService::login: cred={:?}", evt);
                    let principal = firebase.get_principal();
                    if principal.is_empty() {
                        tracing::error!("UserService::login: principal is empty");
                        return Err(ServiceError::Unauthorized);
                    }

                    let (email, name, profile_url) = match firebase.get_user_info() {
                        Some(v) => v,
                        None => {
                            tracing::error!("UserService::login: None");
                            return Err(ServiceError::Unauthorized);
                        }
                    };

                    (evt, principal, email, name, profile_url)
                }
                Err(e) => {
                    tracing::error!("UserService::login: error={:?}", e);
                    return Err(ServiceError::Unauthorized);
                }
            };

        Ok((evt, principal, email, name, profile_url))
    }

    pub async fn login(&mut self) -> UserEvent {
        tracing::debug!("UserService::login");
        #[cfg(feature = "web-only")]
        {
            let (evt, _principal, email, name, profile_url) =
                self.request_to_firebase().await.unwrap();
            match evt {
                google_wallet::WalletEvent::Signup => {
                    tracing::debug!(
                        "UserService::Signup: email={} name={} profile_url={}",
                        email,
                        name,
                        profile_url
                    );

                    return UserEvent::Signup(email, name, profile_url);
                }
                google_wallet::WalletEvent::Login => {
                    tracing::debug!(
                        "UserService::Signup: email={} name={} profile_url={}",
                        email,
                        name,
                        profile_url
                    );

                    return UserEvent::Login(email, name, profile_url);
                }
                google_wallet::WalletEvent::Logout => {
                    tracing::debug!("UserService::login: SignOut");
                }
            }
        }

        UserEvent::Logout
    }

    pub async fn signup(
        &self,
        principal: &str,
        email: &str,
        nickname: &str,
        profile_url: &str,
    ) -> Result<()> {
        tracing::debug!(
            "UserService::signup: principal={} email={} nickname={} profile_url={}",
            principal,
            email,
            nickname,
            profile_url
        );

        let endpoint = (self.endpoint)();
        let url = format!("{}/users/signup", endpoint);

        let body = dto::UserActionRequest::Signup(SignupRequest {
            email: email.to_string(),
            nickname: nickname.to_string(),
            profile_url: profile_url.to_string(),
        });

        let res: Result<User> = rest_api::post(&url, &body).await;

        tracing::debug!("UserService::signup: user={:?}", res);
        Ok(())
    }
}

#[cfg(feature = "web-only")]
impl Signer for UserService {
    fn signer(&self) -> String {
        (self.firebase)().get_principal()
    }

    fn sign(&self, msg: &str) -> dto::Result<Signature> {
        let firebase = (self.firebase)();

        if !firebase.get_login() {
            return Err(ServiceError::Unauthorized);
        }

        let sig = firebase.sign(msg);
        if sig.is_none() {
            return Err(ServiceError::SignException);
        }

        Ok(Signature {
            signature: sig.unwrap().as_ref().to_vec(),
            public_key: firebase.public_key.clone().unwrap_or_default(),
            algorithm: SignatureAlgorithm::EdDSA,
        })
    }
}
