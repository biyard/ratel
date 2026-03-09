use common::wasm_bindgen::prelude::*;
use common::wasm_bindgen_futures::JsFuture;
use common::web_sys::js_sys::Promise;

use super::*;
#[wasm_bindgen(js_namespace = ["window", "ratel", "ratel_team_setting"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn initialize(config: &JsValue);
}
