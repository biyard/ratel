use dto::ServiceError;
use hex::encode;
use wallet_adapter::{Wallet, WalletAccount, WalletAdapter, WalletResult};
use web_sys::window;
pub struct PhantomAuth {
    adapter: WalletAdapter,
    wallet: WalletResult<Wallet>,
}

pub enum Platform {
    Desktop,
    Mobile,
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

    pub fn get_deeplink(&self, redirect_url: &str) -> String {
        format!("https://phantom.app/ul/browse/{}", redirect_url)
    }

    pub async fn connect_mobile(&self) -> Result<WalletAccount, ServiceError> {
        
    }
}
