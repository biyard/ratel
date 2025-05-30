use bdk::prelude::*;
use dto::Error;
// use ed25519_dalek::Signature as Ed25519Signature;
use hex::encode;
use wallet_adapter::{SigninInput, Wallet, WalletAccount, WalletAdapter, WalletResult};
use web_sys::window;
#[derive(Debug, Clone)]
pub struct PhantomAuth {
    adapter: WalletAdapter,
    wallet: WalletResult<Wallet>,
    active_account: Option<WalletAccount>,
    cached_signiture: Option<rest_api::Signature>,
}

pub enum Platform {
    Desktop,
    Mobile,
}

pub enum PhantomDeeplink {
    Connect,
    Disconnect,
    SignMessage,
    SignTransaction,
}

impl PhantomAuth {
    pub fn new() -> Self {
        let adapter = WalletAdapter::init().unwrap();
        let wallet = adapter.get_wallet("Phantom");

        // TODO: Phantom auto login

        Self {
            adapter,
            wallet,
            active_account: None,
            cached_signiture: None,
        }
    }

    pub fn is_installed(&self) -> bool {
        self.wallet.is_ok()
    }

    pub fn detect_platform(&self) -> Platform {
        let window = window().expect("no window");
        let navigator = window.navigator();
        let user_agent = navigator.user_agent().unwrap_or_default().to_lowercase();

        if user_agent.contains("mobi")
            || user_agent.contains("android")
            || user_agent.contains("iphone")
        {
            Platform::Mobile
        } else {
            Platform::Desktop
        }
    }

    pub async fn connect_desktop(&mut self) -> Result<(), Error> {
        if let Ok(wallet) = self.wallet.clone() {
            // background connect
            return match self.adapter.connect(wallet).await {
                Ok(_account) => {
                    let public_key = self.get_public_key_array();
                    let mut signin_input = SigninInput::new();
                    signin_input
                        .set_domain(&self.adapter.window())?
                        .set_statement("Login To Dev Website")
                        .set_chain_id(wallet_adapter::Cluster::DevNet) // TODO: manage this value in env
                        .set_address(&self.get_address())?;
                    match self.adapter.sign_in(&signin_input, public_key).await {
                        Ok(output) => {
                            self.active_account = Some(output.account);
                            Ok(())
                        }
                        Err(e) => Err(Error::WalletError(
                            format!("Failed to connect wallet: {:?}", e).to_string(),
                        )),
                    }
                }
                Err(e) => Err(Error::WalletError(
                    format!("Failed to connect wallet: {:?}", e).to_string(),
                )),
            };
        }
        Err(Error::WalletNotFound)
    }

    pub fn get_account(&self) -> WalletResult<&WalletAccount> {
        self.adapter.connected_account()
    }

    pub fn get_public_key(&self) -> Vec<u8> {
        match self.adapter.connected_account() {
            Ok(account) => account.public_key.to_vec(),
            Err(_) => vec![],
        }
    }

    pub fn get_public_key_array(&self) -> [u8; 32] {
        match self.adapter.connected_account() {
            Ok(account) => account.public_key,
            Err(_) => [0; 32],
        }
    }

    pub fn get_public_key_string(&self) -> String {
        match self.adapter.connected_account() {
            Ok(account) => encode(account.public_key),
            Err(_) => "".to_string(),
        }
    }

    pub fn get_account_name(&self) -> String {
        match self.adapter.connected_account() {
            Ok(account) => account.label.clone().unwrap_or_else(|| "".to_string()),
            Err(_) => "".to_string(),
        }
    }

    pub fn get_address(&self) -> String {
        match self.adapter.connected_account() {
            Ok(account) => account.address.to_string(),
            Err(_) => "".to_string(),
        }
    }

    pub async fn disconnect(&mut self) -> Result<(), Error> {
        match self.adapter.disconnect().await {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::WalletError(
                format!("Failed to disconnect wallet: {:?}", e).to_string(),
            )),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.adapter.is_connected()
    }

    pub async fn sign(&self, message: &str) -> Option<rest_api::Signature> {
        let message_bytes = message.as_bytes();
        if self.adapter.solana_sign_message().is_err() {
            return None;
        }
        match self.adapter.sign_message(message_bytes).await {
            Ok(signed_message) => {
                let sig = signed_message.signature();
                Some(rest_api::Signature {
                    signature: sig.to_bytes().to_vec(),
                    algorithm: rest_api::signature::SignatureAlgorithm::EdDSA,
                    public_key: self.get_public_key(),
                })
            }
            Err(_) => None,
        }
    }

    pub fn remove_signer(&mut self) {
        self.cached_signiture = None;
    }

    pub fn get_signer(&self) -> Option<&rest_api::Signature> {
        self.cached_signiture.as_ref()
    }

    pub fn is_signed(&self) -> bool {
        self.cached_signiture.is_some()
    }

    pub async fn signin_message(&mut self) -> Result<(), Error> {
        match self.adapter.connected_account() {
            Ok(account) => {
                match self.sign(&account.address).await {
                    Some(signature) => {
                        self.cached_signiture = Some(signature);
                    }
                    None => {
                        return Err(Error::WalletError("Failed to sign message".to_string()));
                    }
                }
                Ok(())
            }
            Err(_) => return Err(Error::WalletNotFound),
        }
    }

    pub fn is_logined(&self) -> bool {
        self.active_account.is_some()
    }

    // pub fn get_deeplink(&self, method: &PhantomDeeplink) -> String {
    //     let base_url = "https://phantom.app/ul/v1";

    //     match method {
    //         PhantomDeeplink::Connect => {
    //             let current_url = window().unwrap()
    //                 .location()
    //                 .href()
    //                 .unwrap();
    //             format!("{}/connect?app={}", base_url, current_url)
    //     }
    // }

    // pub fn get_sol_balance(&self, account: WalletAccount) -> WalletResult<u64> {
    //     self.adapter.get_sol_balance(account)
    // }

    // pub async fn connect_mobile(&self) -> Result<WalletAccount, Error> {
    // }
}
