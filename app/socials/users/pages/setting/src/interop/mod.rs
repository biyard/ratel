use common::wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use crate::*;
#[wasm_bindgen(js_namespace = ["window", "ratel", "ratel_user_setting"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn initialize(config: &JsValue);

    #[wasm_bindgen(js_name = connectWallet, catch)]
    pub fn connect_wallet() -> std::result::Result<Promise, JsValue>;
}
