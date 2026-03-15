use crate::common::wasm_bindgen::prelude::*;
use crate::common::wasm_bindgen_futures::JsFuture;
use crate::common::web_sys::js_sys::Promise;

use super::*;
#[wasm_bindgen(js_namespace = ["window", "ratel", "ratel_team_member"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn initialize(config: &JsValue);
}
