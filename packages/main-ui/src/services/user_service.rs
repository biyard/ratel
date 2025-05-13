#![allow(non_snake_case)]
use crate::{
    config,
    utils::phantom::{PhantomAuth, Platform},
};
use bdk::prelude::*;
use dto::*;
use google_wallet::WalletEvent;

pub enum UserEvent {
    Signup(String, String, String, String),
    Login,
    Logout,
    Confirmed,
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
    Phantom,
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
    pub nickname: Option<String>,
    pub profile_url: Option<String>,
}

impl UserInfo {
    pub fn new(principal: String, email: String, nickname: String, profile_url: String) -> Self {
        Self {
            principal,
            email: Some(email),
            nickname: Some(nickname),
            profile_url: Some(profile_url),
        }
    }

    pub fn is_login(&self) -> bool {
        !self.principal.is_empty()
    }
}

impl Default for UserInfo {
    fn default() -> Self {
        Self {
            principal: "".to_string(),
            email: None,
            nickname: None,
            profile_url: None,
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct UserService {
    pub signer: Signal<WalletSigner>,
    pub firebase: Signal<google_wallet::FirebaseWallet>,
    pub phantom: Signal<Option<PhantomAuth>>,
    pub cli: Signal<UserClient>,
    pub user_info: Signal<UserInfo>,
    pub loggedin: Signal<bool>,
}

impl UserService {
    pub fn init() {
        let conf: &config::Config = config::get();

        let firebase = get_firebase_wallet();
        let signer = if firebase.get_login() {
            WalletSigner::Firebase
        } else {
            WalletSigner::None
        };

        let loggedin = if firebase.get_login() { true } else { false };

        #[cfg(feature = "web")]
        let phantom = Some(PhantomAuth::new());
        #[cfg(not(feature = "web"))]
        let phantom = None;

        let mut user = Self {
            signer: use_signal(move || signer),
            firebase: use_signal(move || firebase.clone()),
            phantom: use_signal(move || phantom),
            cli: use_signal(move || User::get_client(&conf.main_api_endpoint)),
            user_info: use_signal(|| UserInfo::default()),
            loggedin: use_signal(|| loggedin),
        };

        use_future(move || async move {
            if loggedin {
                user.get_user_info_from_server().await;
            }
        });

        use_context_provider(move || user);
    }

    pub fn set_signer_type(&mut self, signer: &str) {
        match signer {
            "google" => {
                self.signer.set(WalletSigner::Firebase);
            }
            "phantom" => {
                self.signer.set(WalletSigner::Phantom);
            }
            _ => {
                self.signer.set(WalletSigner::None);
            }
        };
    }

    pub fn get_signer_type(&self) -> String {
        match (self.signer)() {
            WalletSigner::Firebase => "google".to_string(),
            WalletSigner::Phantom => "phantom wallet".to_string(),
            WalletSigner::None => "none".to_string(),
        }
    }

    pub async fn logout(&mut self) {
        match &mut *self.signer.write() {
            WalletSigner::Firebase => self.firebase.write().logout(),
            WalletSigner::Phantom => {
                if let Some(phantom) = self.phantom.write().as_mut() {
                    match phantom.disconnect().await {
                        Ok(_) => {}
                        Err(e) => {
                            tracing::error!("UserService::logout: error={:?}", e);
                        }
                    };
                };
            }
            WalletSigner::None => {
                return;
            }
        };
        self.signer.set(WalletSigner::None);
        self.user_info.set(UserInfo::default());
        self.loggedin.set(false);
    }

    pub async fn get_user_info_from_server(&mut self) {
        let cli = (self.cli)();
        rest_api::set_signer(Box::new(*self));

        let user: User = match cli.user_info().await {
            Ok(v) => v,
            Err(e) => match e {
                Error::NotFound => {
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
            nickname: Some(user.nickname),
            profile_url: Some(user.profile_url),
        });
        self.loggedin.set(true);
    }

    pub fn get_user_info(&self) -> Option<(String, String, String)> {
        let info = (self.user_info)();

        if info.email.is_none() || info.nickname.is_none() {
            return None;
        }

        Some((
            info.nickname.clone().unwrap(),
            info.email.clone().unwrap(),
            // TODO: default image
            info.profile_url.clone().unwrap_or_default(),
        ))
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
                        return Err(Error::Unauthorized);
                    }

                    let (email, name, profile_url) = match firebase.get_user_info() {
                        Some(v) => v,
                        None => {
                            tracing::error!("UserService::login: None");
                            return Err(Error::Unauthorized);
                        }
                    };

                    (evt, principal, email, name, profile_url)
                }
                Err(e) => {
                    tracing::error!("UserService::login: error={:?}", e);
                    return Err(Error::Unauthorized);
                }
            };

        Ok((evt, principal, email, name, profile_url))
    }

    pub async fn login(&mut self) -> UserEvent {
        match (self.signer)() {
            WalletSigner::Firebase => self.login_with_firebase().await,
            WalletSigner::Phantom => self.login_with_phantom().await,
            WalletSigner::None => UserEvent::Logout,
        }
    }

    pub async fn login_or_signup(
        &mut self,
        principal: &str,
        email: &str,
        nickname: &str,
        profile_url: &str,
        term_agreed: bool,
        informed_agreed: bool,
    ) -> Result<()> {
        // match (self.signer)() {
        //     WalletSigner::Phantom => {
        //         let signal = self.phantom.read();
        //         if signal.is_none() {
        //             return Err(ServiceError::Unauthorized.into());
        //         }
        //         let phantom = signal.as_ref().unwrap();

        //         if !phantom.is_connected() {
        //             return Err(ServiceError::Unauthorized.into());
        //         }
        //     }
        //     _ => rest_api::set_signer(Box::new(*self)),
        // }

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
                term_agreed,
                informed_agreed,
            )
            .await
        {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("UserService::signup: error={:?}", e);
                match (self.signer)() {
                    WalletSigner::Phantom => {
                        if let Some(phantom) = self.phantom.write().as_mut() {
                            phantom.remove_signer();
                        }
                    }
                    _ => {}
                };
                rest_api::remove_signer();
                return Err(e);
            }
        };

        let user = res.clone();

        self.user_info.set(UserInfo {
            principal: user.principal,
            email: Some(user.email),
            nickname: Some(user.nickname),
            profile_url: Some(user.profile_url),
        });
        self.loggedin.set(true);

        tracing::debug!("UserService::signup: user={:?}", res);
        Ok(())
    }

    pub fn is_phantom_installed(&self) -> bool {
        self.phantom
            .read()
            .as_ref()
            .map(|v| v.is_installed())
            .unwrap_or(false)
    }

    pub async fn login_with_firebase(&mut self) -> UserEvent {
        tracing::debug!("UserService::login: Firebase");
        let (evt, principal, email, name, profile_url) = match self.request_to_firebase().await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("UserService::login: error={:?}", e);
                return UserEvent::Logout;
            }
        };
        match evt {
            google_wallet::WalletEvent::Signup => {
                tracing::debug!(
                    "UserService::Signup: email={} name={} profile_url={}",
                    email,
                    name,
                    profile_url
                );

                self.user_info.set(UserInfo::new(
                    principal.clone(),
                    email.clone(),
                    name.clone(),
                    profile_url.clone(),
                ));
                self.loggedin.set(true);

                return UserEvent::Signup(principal, email, name, profile_url);
            }
            google_wallet::WalletEvent::Login => {
                tracing::debug!(
                    "UserService::Login: email={} name={} profile_url={}",
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
                        tracing::debug!("UserService::login: error={:?}", e);

                        return UserEvent::Signup(principal, email, name, profile_url);
                    }
                };

                self.user_info.set(UserInfo::new(
                    user.principal,
                    user.email,
                    user.nickname,
                    user.profile_url,
                ));
                self.loggedin.set(true);

                return UserEvent::Login;
            }
            google_wallet::WalletEvent::Logout => {
                tracing::debug!("UserService::login: SignOut");
            }
        }

        return UserEvent::Logout;
    }

    pub async fn login_with_phantom(&mut self) -> UserEvent {
        tracing::debug!("UserService::phantom_wallet login");

        let cli = (self.cli)();

        if self.phantom.read().is_none() {
            tracing::error!("UserService::phantom_wallet: phantom is none");
            return UserEvent::Logout;
        }

        let mut signal = self.phantom.write();
        let phantom = signal.as_mut().unwrap();

        match phantom.detect_platform() {
            Platform::Desktop => {
                tracing::debug!("UserService::phantom_wallet: desktop");

                if phantom.is_logined() {
                    tracing::debug!("UserService::phantom_wallet: connected");
                    let public_key_str = phantom.get_public_key_string();

                    if !phantom.is_signed() {
                        tracing::error!("UserService::phantom_wallet: not signed");
                        match phantom.signin_message().await {
                            Ok(_) => {}
                            Err(e) => {
                                tracing::error!("UserService::phantom_wallet: error={:?}", e);
                                return UserEvent::Logout;
                            }
                        }
                    }

                    match cli.by_principal(public_key_str.clone()).await {
                        Ok(v) => {
                            tracing::debug!("UserService::phantom_wallet: login");
                            self.user_info.set(UserInfo::new(
                                v.principal,
                                v.email,
                                v.nickname,
                                v.profile_url,
                            ));
                            self.loggedin.set(true);

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
                } else {
                    tracing::debug!("UserService::phantom_wallet: connect");
                    match phantom.connect_desktop().await {
                        Ok(_) => {
                            tracing::debug!("UserService::phantom_wallet: connected");
                            return UserEvent::Confirmed;
                        }
                        Err(e) => {
                            tracing::error!("UserService::phantom_wallet: error={:?}", e);
                            return UserEvent::Logout;
                        }
                    }
                };
            }
            Platform::Mobile => {
                tracing::debug!("UserService::phantom_wallet: mobile");
            }
        };
        UserEvent::Logout
    }

    pub async fn wallet_signin_message(&mut self) -> Result<()> {
        if self.phantom.read().is_none() {
            tracing::error!("UserService::phantom_wallet: phantom is none");
        }

        let mut signal = self.phantom.write();
        let phantom = signal.as_mut().unwrap();

        match phantom.signin_message().await {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("UserService::phantom_wallet: error={:?}", e);
                Err(Error::WalletError(e.to_string()))
            }
        }
    }

    pub fn is_logined(&self) -> bool {
        !self.user_info.read().principal.is_empty()
    }
}

impl rest_api::Signer for UserService {
    fn signer(&self) -> String {
        match (self.signer)() {
            WalletSigner::Firebase => (self.firebase)().get_principal(),
            WalletSigner::Phantom => match (self.phantom)() {
                Some(phantom) => phantom.get_public_key_string(),
                None => "".to_string(),
            },
            WalletSigner::None => "".to_string(),
        }
    }

    fn sign(
        &self,
        msg: &str,
    ) -> std::result::Result<rest_api::Signature, Box<dyn std::error::Error>> {
        tracing::debug!("UserService::sign: msg={}", msg);
        match (self.signer)() {
            WalletSigner::Firebase => {
                let firebase = (self.firebase)();

                if !firebase.get_login() {
                    tracing::debug!("UserService::sign: not login {firebase:?}");
                    return Err(Box::<ServiceException>::new(Error::Unauthorized.into()));
                }

                let sig = firebase.sign(msg);
                if sig.is_none() {
                    return Err(Box::<ServiceException>::new(Error::Unauthorized.into()));
                }
                let sig = rest_api::Signature {
                    signature: sig.unwrap().as_ref().to_vec(),
                    public_key: firebase.public_key().unwrap_or_default(),
                    algorithm: rest_api::signature::SignatureAlgorithm::EdDSA,
                };

                return Ok(sig);
            }
            WalletSigner::Phantom => {
                let signal = self.phantom.read();

                if signal.is_none() {
                    tracing::debug!("UserService::sign: not login {signal:?}");
                    return Err(Box::<ServiceException>::new(Error::Unauthorized.into()));
                }

                let phantom = signal.as_ref().unwrap();

                if !phantom.is_connected() {
                    tracing::debug!("UserService::sign: not login {phantom:?}");
                    return Err(Box::<ServiceException>::new(Error::Unauthorized.into()));
                }

                let sig = match phantom.get_signer() {
                    Some(v) => v,
                    None => {
                        return Err(Box::<ServiceException>::new(Error::Unauthorized.into()));
                    }
                };

                return Ok(sig.clone());
            }
            WalletSigner::None => {
                return Err(Box::<ServiceException>::new(Error::Unauthorized.into()));
            }
        }
    }
}
