#![allow(non_snake_case)]
use crate::{
    config,
    utils::phantom::{PhantomAuth, Platform},
};
use dioxus::prelude::*;
use dto::*;
use google_wallet::WalletEvent;

pub enum UserEvent {
    Signup(String, String, String, String),
    Login,
    Logout,
}

#[allow(async_fn_in_trait)]
pub trait WalletProvider {
    async fn connect(&mut self) -> Result<(WalletEvent, UserInfo)>;
    fn get_principal(&self) -> String;
    async fn login(&mut self) -> Result<(UserEvent, UserInfo)>;
    fn logout(&mut self);
    fn get_login(&self) -> bool;
    fn get_public_key(&self) -> Option<Vec<u8>>;
}

#[derive(Debug, Clone)]
pub enum WalletSigner {
    Firebase,
    // Phantom,
    None,
}

pub fn get_firebase_wallet() -> google_wallet::FirebaseWallet {
    #[allow(unused_variables)]
    let conf = config::get();

    #[cfg(not(feature = "web"))]
    let firebase = google_wallet::FirebaseWallet::default();

    #[cfg(feature = "web")]
    let mut firebase = google_wallet::FirebaseWallet::new(
        conf.firebase.api_key.clone(),
        conf.firebase.auth_domain.clone(),
        conf.firebase.project_id.clone(),
        conf.firebase.storage_bucket.clone(),
        conf.firebase.messaging_sender_id.clone(),
        conf.firebase.app_id.clone(),
        conf.firebase.measurement_id.clone(),
    );

    #[cfg(feature = "web")]
    let _ = firebase.try_setup_from_storage();
    tracing::debug!("get_firebase_wallet: firebase={:?}", firebase);
    firebase
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub principal: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub profile_url: Option<String>,
}

impl UserInfo {
    pub fn new(principal: String, email: String, name: String, profile_url: String) -> Self {
        Self {
            principal,
            email: Some(email),
            name: Some(name),
            profile_url: Some(profile_url),
        }
    }
}

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            principal: "".to_string(),
            email: None,
            name: None,
            profile_url: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UserService {
    pub signer: Signal<WalletSigner>,
    pub firebase: Signal<google_wallet::FirebaseWallet>,
    pub cli: Signal<UserClient>,
    pub user_info: Signal<UserInfo>,
}

impl UserService {
    pub fn init() {
        let conf: &config::Config = config::get();

        let firebase = get_firebase_wallet();

        let cli = User::get_client(&conf.main_api_endpoint);

        use_context_provider(|| Self {
            signer: Signal::new(WalletSigner::None),
            firebase: Signal::new(firebase.clone()),
            cli: Signal::new(cli),
            user_info: Signal::new(UserInfo::default()),
        });

        let mut user = use_context::<UserService>();
        let signer = (user.signer)();

        let is_login = match &signer {
            WalletSigner::Firebase => firebase.get_login(),
            // WalletSigner::Phantom(auth) => auth.read().get_login(),
            WalletSigner::None => false,
        };

        if is_login {
            tracing::debug!("UserService::init: wallet={:?}", signer);
            spawn(async move {
                user.get_user_info_from_server().await;
            });
        }
    }

    pub fn set_signer_type(&mut self, signer: &str) {
        match signer {
            "google" => {
                self.signer.set(WalletSigner::Firebase);
            }
            "phantom" => {
                self.signer.set(WalletSigner::None);
            }
            _ => {
                self.signer.set(WalletSigner::None);
            }
        };
    }

    pub fn logout(&mut self) {
        match &mut *self.signer.write() {
            WalletSigner::Firebase => self.firebase.write().logout(),
            // WalletSigner::Phantom(auth) => auth.write().logout(),
            WalletSigner::None => {
                return;
            }
        };
        self.signer.set(WalletSigner::None);
        self.user_info.set(UserInfo::default());
    }

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

        self.user_info.set(UserInfo {
            principal: user.principal,
            email: Some(user.email),
            name: Some(user.nickname),
            profile_url: Some(user.profile_url),
        });
    }

    pub fn get_user_info(&self) -> Option<(String, String)> {
        let info = (self.user_info)();

        if info.email.is_none() || info.name.is_none() {
            return None;
        }

        Some((info.email.clone().unwrap(), info.name.clone().unwrap()))
    }

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
        match (self.signer)() {
            WalletSigner::Firebase => {
                tracing::debug!("UserService::login: Firebase");
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
                                        return UserEvent::Signup(
                                            principal,
                                            email,
                                            name,
                                            profile_url,
                                        );
                                    }
                                    e => {
                                        tracing::error!("UserService::login: error={:?}", e);
                                        return UserEvent::Logout;
                                    }
                                }
                            }
                        };

                        self.user_info.set(UserInfo::new(
                            user.principal,
                            user.email,
                            user.nickname,
                            user.profile_url,
                        ));

                        return UserEvent::Login;
                    }
                    google_wallet::WalletEvent::Logout => {
                        tracing::debug!("UserService::login: SignOut");
                    }
                }

                return UserEvent::Logout;
            }
            WalletSigner::None => {
                return UserEvent::Logout;
            }
        }
    }

    pub async fn login_or_signup(
        &self,
        principal: &str,
        email: &str,
        nickname: &str,
        profile_url: &str,
    ) -> Result<()> {
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

    pub async fn phantom_login(&mut self) -> UserEvent {
        tracing::debug!("UserService::phantom_wallet login");

        let cli = (self.cli)();
        let mut phantom = PhantomAuth::new();

        match phantom.detect_platform() {
            Platform::Desktop => {
                tracing::debug!("UserService::phantom_wallet: desktop");
                match phantom.connect_desktop().await {
                    Ok(account) => {
                        let public_key_str = phantom.get_public_key(account);

                        match cli.by_principal(public_key_str.clone()).await {
                            Ok(v) => {
                                tracing::debug!("UserService::phantom_wallet: login");
                                self.user_info.set(UserInfo::new(
                                    v.principal,
                                    v.email,
                                    v.nickname,
                                    v.profile_url,
                                ));
                                return UserEvent::Login;
                            }
                            Err(_) => {
                                tracing::debug!("UserService::phantom_wallet: signup");
                                return UserEvent::Signup(
                                    public_key_str,
                                    "".to_string(),
                                    "".to_string(),
                                    "".to_string(),
                                );
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("UserService::phantom_wallet: error={:?}", e);
                    }
                };
            }
            Platform::Mobile => {
                tracing::debug!("UserService::phantom_wallet: mobile");
            }
        };
        UserEvent::Logout
    }
}

impl rest_api::Signer for UserService {
    fn signer(&self) -> String {
        match (self.signer)() {
            WalletSigner::Firebase => (self.firebase)().get_principal(),
            // WalletSigner::Phantom => auth.get_principal(),
            WalletSigner::None => "".to_string(),
        }
    }

    fn sign(
        &self,
        msg: &str,
    ) -> std::result::Result<rest_api::Signature, Box<dyn std::error::Error>> {
        tracing::debug!("UserService::sign: msg={}", msg);
        let signer = (self.signer)();

        match signer {
            WalletSigner::Firebase => {
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

                return Ok(sig);
            }
            WalletSigner::None => {
                return Err(Box::<ServiceException>::new(
                    ServiceError::Unauthorized.into(),
                ));
            }
        }
    }
}
