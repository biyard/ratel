use crate::common::wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use super::*;
#[wasm_bindgen(js_namespace = ["window", "ratel", "ratel_user_reward"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn initialize(config: &JsValue);
}

#[wasm_bindgen(js_namespace = ["window", "ratel", "tokens", "claim"])]
extern "C" {
    #[wasm_bindgen(js_name = getWalletAddress)]
    fn get_wallet_address_promise() -> Promise;

    #[wasm_bindgen(js_name = connectWallet)]
    fn connect_wallet_promise(chain_id: u32) -> Promise;

    #[wasm_bindgen(js_name = claimTokens)]
    fn claim_tokens_promise(params: &JsValue) -> Promise;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimTokensParams {
    pub contract_address: String,
    pub chain_id: u64,
    pub month_index: String,
    pub amount: String,
    pub max_claimable: String,
    pub nonce: String,
    pub deadline: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaimTokensResult {
    pub tx_hash: String,
    pub address: String,
}

pub async fn get_wallet_address() -> Option<String> {
    let js_value = JsFuture::from(get_wallet_address_promise()).await.ok()?;
    js_value.as_string()
}

pub async fn connect_wallet(chain_id: u64) -> crate::common::Result<String> {
    use super::controllers::ExchangePointsError;

    let js_value = JsFuture::from(connect_wallet_promise(chain_id as u32))
        .await
        .map_err(|_| ExchangePointsError::NoWalletConnected)?;

    js_value
        .as_string()
        .ok_or_else(|| ExchangePointsError::NoWalletConnected.into())
}

pub async fn claim_tokens(
    params: ClaimTokensParams,
) -> crate::common::Result<ClaimTokensResult> {
    use super::controllers::ExchangePointsError;

    let js_params = serde_wasm_bindgen::to_value(&params)
        .map_err(|_| ExchangePointsError::ClaimFailed)?;

    let js_value = JsFuture::from(claim_tokens_promise(&js_params))
        .await
        .map_err(|_| ExchangePointsError::ClaimFailed)?;

    serde_wasm_bindgen::from_value(js_value)
        .map_err(|_| ExchangePointsError::ClaimFailed.into())
}
