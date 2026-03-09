use crate::common::wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use crate::features::my_follower::*;
#[wasm_bindgen(js_namespace = ["window", "ratel", "ratel_my_follower"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn initialize(config: &JsValue);
}
