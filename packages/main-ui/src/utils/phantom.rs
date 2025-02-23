use dto::ServiceError;
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

    pub fn get_public_key(&self, account: WalletAccount) -> String {
        encode(account.public_key)
    }

    pub fn get_address(&self, account: WalletAccount) -> String {
        account.address
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
