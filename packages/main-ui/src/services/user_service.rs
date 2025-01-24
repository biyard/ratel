#![allow(non_snake_case)]
use dioxus::prelude::*;
use dto::*;

use crate::config;

pub enum UserEvent {
    Signup(String, String, String, String),
    Login,
    Logout,
}

#[derive(Debug, Clone, Copy)]
pub struct UserService {
    #[cfg(feature = "web-only")]
    pub firebase: Signal<google_wallet::FirebaseWallet>,

    pub cli: Signal<UserClient>,
    pub email: Signal<String>,
    pub nickname: Signal<String>,
    pub profile_url: Signal<String>,
    pub principal: Signal<String>,
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
        let _ = firebase.try_setup_from_storage();
        let cli = User::get_client(&conf.main_api_endpoint);

        use_context_provider(|| Self {
            #[cfg(feature = "web-only")]
            firebase: Signal::new(firebase),
            cli: Signal::new(cli),
            email: Signal::new("".to_string()),
            nickname: Signal::new("".to_string()),
            profile_url: Signal::new("".to_string()),
            principal: Signal::new("".to_string()),
        });

        #[cfg(feature = "web-only")]
        {
            let mut user = use_context::<UserService>();
            let firebase = (user.firebase)();
            if firebase.get_login() {
                tracing::debug!("UserService::init: wallet={:?}", firebase);
                spawn(async move {
                    user.get_user_info_from_server().await;
                });
            }
        }
    }

    pub fn logout(&mut self) {
        #[cfg(feature = "web-only")]
        {
            self.firebase.write().logout();
        }

        self.email.set("".to_string());
        self.nickname.set("".to_string());
        self.profile_url.set("".to_string());
        self.principal.set("".to_string());
    }

    #[cfg(feature = "web-only")]
    pub async fn get_user_info_from_server(&mut self) {
        let cli = (self.cli)();
        rest_api::set_signer(Box::new(*self));

        let user: User = match cli.user_info().await {
            Ok(v) => v,
            Err(e) => match e {
                ServiceError::NotFound => {
                    return;
                }
                e => {
                    tracing::error!("UserService::get_user_info_from_server: error={:?}", e);
                    return;
                }
            },
        };

        self.nickname.set(user.nickname);
        self.profile_url.set(user.profile_url);
    }

    pub fn get_user_info(&self) -> Option<(String, String)> {
        let nickname = (self.nickname)();
        let profile_url = (self.profile_url)();

        if profile_url.is_empty() || nickname.is_empty() {
            return None;
        }

        Some((nickname, profile_url))
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
            let (evt, principal, email, name, profile_url) =
                self.request_to_firebase().await.unwrap();
            match evt {
                google_wallet::WalletEvent::Signup => {
                    tracing::debug!(
                        "UserService::Signup: email={} name={} profile_url={}",
                        email,
                        name,
                        profile_url
                    );

                    return UserEvent::Signup(principal, email, name, profile_url);
                }
                google_wallet::WalletEvent::Login => {
                    tracing::debug!(
                        "UserService::Signup: email={} name={} profile_url={}",
                        email,
                        name,
                        profile_url
                    );
                    rest_api::set_signer(Box::new(*self));
                    let cli = (self.cli)();

                    let user: User = match cli.check_email(email.clone()).await {
                        // Login
                        Ok(v) => v,
                        Err(e) => {
                            // Signup
                            rest_api::remove_signer();

                            match e {
                                ServiceError::NotFound => {
                                    return UserEvent::Signup(principal, email, name, profile_url);
                                }
                                e => {
                                    tracing::error!("UserService::login: error={:?}", e);
                                    return UserEvent::Logout;
                                }
                            }
                        }
                    };

                    self.email.set(email);
                    self.nickname.set(user.nickname);
                    self.profile_url.set(user.profile_url);
                    self.principal.set(principal);

                    return UserEvent::Login;
                }
                google_wallet::WalletEvent::Logout => {
                    tracing::debug!("UserService::login: SignOut");
                }
            }
        }

        UserEvent::Logout
    }

    pub async fn login_or_signup(
        &self,
        principal: &str,
        email: &str,
        nickname: &str,
        profile_url: &str,
    ) -> Result<()> {
        #[cfg(feature = "web-only")]
        rest_api::set_signer(Box::new(*self));

        tracing::debug!(
            "UserService::signup: principal={} email={} nickname={} profile_url={}",
            principal,
            email,
            nickname,
            profile_url
        );

        let cli = (self.cli)();

        let res: User = match cli
            .signup(
                nickname.to_string(),
                email.to_string(),
                profile_url.to_string(),
            )
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("UserService::signup: error={:?}", e);
                rest_api::remove_signer();
                return Err(e);
            }
        };

        tracing::debug!("UserService::signup: user={:?}", res);
        Ok(())
    }
}

#[cfg(feature = "web-only")]
impl rest_api::Signer for UserService {
    fn signer(&self) -> String {
        (self.firebase)().get_principal()
    }

    fn sign(
        &self,
        msg: &str,
    ) -> std::result::Result<rest_api::Signature, Box<dyn std::error::Error>> {
        tracing::debug!("UserService::sign: msg={}", msg);
        let firebase = (self.firebase)();

        if !firebase.get_login() {
            tracing::debug!("UserService::sign: not login {firebase:?}");
            return Err(Box::<ServiceException>::new(
                ServiceError::Unauthorized.into(),
            ));
        }

        let sig = firebase.sign(msg);
        if sig.is_none() {
            return Err(Box::<ServiceException>::new(
                ServiceError::Unauthorized.into(),
            ));
        }
        let sig = rest_api::Signature {
            signature: sig.unwrap().as_ref().to_vec(),
            public_key: firebase.public_key().unwrap_or_default(),
            algorithm: rest_api::signature::SignatureAlgorithm::EdDSA,
        };

        Ok(sig)
    }
}
