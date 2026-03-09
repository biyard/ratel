use crate::common::wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use crate::features::spaces::actions::subscription::*;
#[wasm_bindgen(js_namespace = ["window", "ratel", "space_action_subscription"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn initialize(config: &JsValue);
}
