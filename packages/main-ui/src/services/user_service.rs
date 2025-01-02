#![allow(non_snake_case)]
use dioxus::prelude::*;
use dto::error::ServiceError;

use crate::{
    config,
    utils::rest_api::{Signature, SignatureAlgorithm, Signer},
};

#[derive(Debug, Clone, Copy)]
pub struct UserService {
    #[cfg(feature = "web-only")]
    pub firebase: Signal<google_wallet::FirebaseWallet>,

    pub endpoint: Signal<String>,
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
        });
    }

    pub async fn login(&mut self) {
        tracing::debug!("UserService::login");

        #[cfg(feature = "web-only")]
        {
            let mut firebase = self.firebase.write();
            let evt = match firebase.request_wallet_with_google().await {
                Ok(evt) => {
                    tracing::debug!("UserService::login: cred={:?}", evt);
                    evt
                }
                Err(e) => {
                    tracing::error!("UserService::login: error={:?}", e);
                    return;
                }
            };

            match evt {
                google_wallet::WalletEvent::Signup => {
                    tracing::debug!("UserService::login: SignIn: ");

                    let principal = firebase.get_principal();
                    if principal.is_empty() {
                        tracing::error!("UserService::login: principal is empty");
                        return;
                    }

                    let (email, name, profile_url) = match firebase.get_user_info() {
                        Some(v) => v,
                        None => {
                            tracing::error!("UserService::login: None");
                            return;
                        }
                    };
                    tracing::debug!(
                        "UserService::login: email={} name={} profile_url={}",
                        email,
                        name,
                        profile_url
                    );
                    // self.popup.open(rsx! {
                    //     crate::layouts::user_setup_popup::UserSetupPopup {
                    //         class: "w-[400px]",
                    //         nickname: name,
                    //         profile_url,
                    //         email,
                    //     }
                    // });
                }
                google_wallet::WalletEvent::Login => {
                    tracing::debug!("UserService::login: SignOut");
                }
                google_wallet::WalletEvent::Logout => {
                    tracing::debug!("UserService::login: SignOut");
                }
            }
        }
    }

    // pub async fn signup(&self, email: &str, nickname: &str, profile_url: &str) {}
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
