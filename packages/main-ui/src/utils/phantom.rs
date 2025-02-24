use dto::ServiceError;
// use ed25519_dalek::Signature as Ed25519Signature;
use hex::encode;
use wallet_adapter::{Wallet, WalletAccount, WalletAdapter, WalletResult};
use web_sys::window;
#[derive(Debug, Clone)]
pub struct PhantomAuth {
    adapter: WalletAdapter,
    wallet: WalletResult<Wallet>,
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

        Self { adapter, wallet }
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

    pub async fn connect_desktop(&mut self) -> Result<WalletAccount, ServiceError> {
        if let Ok(wallet) = self.wallet.clone() {
            return match self.adapter.connect(wallet).await {
                Ok(account) => Ok(account),
                Err(e) => Err(ServiceError::WalletError(
                    format!("Failed to connect wallet: {:?}", e).to_string(),
                )),
            };
        }
        Err(ServiceError::WalletNotFound)
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

    pub fn get_public_key_string(&self) -> String {
        match self.adapter.connected_account() {
            Ok(account) => encode(account.public_key),
            Err(_) => "".to_string(),
        }
    }

    pub fn get_address(&self) -> String {
        match self.adapter.connected_account() {
            Ok(account) => account.address.to_string(),
            Err(_) => "".to_string(),
        }
    }

    pub async fn disconnect(&mut self) -> Result<(), ServiceError> {
        match self.adapter.disconnect().await {
            Ok(_) => Ok(()),
            Err(e) => Err(ServiceError::WalletError(
                format!("Failed to disconnect wallet: {:?}", e).to_string(),
            )),
        }
    }

    pub fn is_connected(&self) -> bool {
        self.adapter.is_connected()
    }

    pub async fn sign(&self, message: &str) -> Option<rest_api::Signature> {
        let message_bytes = message.as_bytes();
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

    // pub async fn connect_mobile(&self) -> Result<WalletAccount, ServiceError> {
    // }
}
