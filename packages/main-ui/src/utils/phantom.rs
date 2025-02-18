use hex::encode;
use wallet_adapter::{Wallet, WalletAccount, WalletAdapter, WalletError, WalletResult};
use web_sys::window;

pub struct PhantomAuth {
    adapter: WalletAdapter,
    wallet: Wallet,
}

pub enum Platform {
    Desktop,
    Mobile,
}

impl PhantomAuth {
    pub fn new() -> Self {
        let adapter = WalletAdapter::init().unwrap();
        let wallet = adapter.get_wallet("Phantom").unwrap();

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

    pub async fn connect_wallet(&mut self) -> Result<WalletAccount, WalletError> {
        self.adapter.connect(self.wallet.clone()).await
    }

    pub fn get_account(&self) -> WalletResult<&WalletAccount> {
        self.adapter.connected_account()
    }

    pub fn get_public_key(&self, account: WalletAccount) -> String {
        encode(account.public_key)
    }
}
