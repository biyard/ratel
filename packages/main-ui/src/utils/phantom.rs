use dto::*;
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsValue};
use web_sys::window; //Window};

pub const PHANTOM_KEY: &str = "phantom_auth";

#[derive(Debug, Clone)]
pub struct PhantomAuth {
    pub address: Option<String>,
    pub signature: Option<String>,
    pub platform: Platform,
}

#[derive(Debug, Serialize, Deserialize)]
struct PhantomResponse {
    public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SignMessageResponse {
    signature: String,
    public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Platform {
    Desktop,
    Mobile,
}

impl PhantomAuth {
    pub fn new() -> Self {
        Self {
            address: None,
            signature: None,
            platform: Self::detect_platform(),
        }
    }

    fn detect_platform() -> Platform {
        let window = window().expect("no window");
        let navigator = window.navigator();
        let user_agent = navigator.user_agent().unwrap_or_default();

        if user_agent.contains("Mobile") {
            Platform::Mobile
        } else {
            Platform::Desktop
        }
    }

    pub fn is_phantom_installed() -> bool {
        if let Some(window) = window() {
            if let Some(solana) = window.get("solana") {
                if let Ok(is_phantom) =
                    js_sys::Reflect::get(&solana, &JsValue::from_str("isPhantom"))
                {
                    return is_phantom.as_bool().unwrap_or(false);
                }
            }
        }
        false
    }

    pub async fn connect_phantom(&mut self) -> Result<String> {
        match self.platform {
            Platform::Desktop => self.connect_desktop().await,
            Platform::Mobile => self.connect_mobile().await,
        }
    }

    async fn connect_desktop(&mut self) -> Result<String> {
        if !Self::is_phantom_installed() {
            return Err(ServiceError::WalletError(
                "Phantom not installed".to_string(),
            ));
        }

        let window = window()
            .ok_or("No window object found")
            .map_err(|e| ServiceError::WalletError(e.to_string()))?;
        let solana = window
            .get("solana")
            .ok_or("No Solana object found")
            .map_err(|e| ServiceError::WalletError(e.to_string()))?;

        let connect_fn = js_sys::Reflect::get(&solana, &JsValue::from_str("connect"))
            .map_err(|_| ServiceError::WalletError("connect not found".to_string()))?
            .dyn_ref::<js_sys::Function>()
            .ok_or("connect is not a function")
            .map_err(|e| ServiceError::WalletError(e.to_string()))?
            .clone();

        let result = connect_fn
            .call0(&solana)
            .map_err(|_| ServiceError::WalletError("connect call failed".to_string()))?;
        let response: PhantomResponse = serde_wasm_bindgen::from_value(result)
            .map_err(|e| ServiceError::WalletError(e.to_string()))?;

        self.address = Some(response.public_key.clone());

        LocalStorage::set(PHANTOM_KEY, &response.public_key)?;

        Ok(response.public_key)
    }

    async fn connect_mobile(&mut self) -> Result<String> {
        Ok("".to_string())
    }
    // fn create_mobile_deep_link() -> String {
    //     let redirect_url = window().unwrap().location().href().unwrap();
    //     format!("https://phantom.app/ul/browse/{}", redirect_url)
    // }
}
