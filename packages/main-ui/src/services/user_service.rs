#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::config;

#[derive(Debug, Clone, Copy)]
pub struct UserService {
    #[cfg(feature = "web-only")]
    pub firebase: Signal<google_wallet::FirebaseWallet>,
}

impl UserService {
    pub fn init() {
        let conf = config::get();

        #[cfg(feature = "web-only")]
        let firebase = google_wallet::FirebaseWallet::new(
            conf.firebase.api_key.clone(),
            conf.firebase.auth_domain.clone(),
            conf.firebase.project_id.clone(),
            conf.firebase.storage_bucket.clone(),
            conf.firebase.messaging_sender_id.clone(),
            conf.firebase.app_id.clone(),
            conf.firebase.measurement_id.clone(),
        );

        use_context_provider(|| Self {
            #[cfg(feature = "web-only")]
            firebase: Signal::new(firebase),
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
}
