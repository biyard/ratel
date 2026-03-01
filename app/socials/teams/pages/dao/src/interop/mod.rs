use common::wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use crate::*;
#[wasm_bindgen(js_namespace = ["window", "ratel", "ratel_team_dao"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn initialize(config: &JsValue);

    #[wasm_bindgen(js_name = createDAO, catch)]
    pub fn create_dao(
        admins: &web_sys::js_sys::Array,
        network: &str,
        rpc_url: &str,
        block_explorer_url: &str,
    ) -> std::result::Result<Promise, JsValue>;

    #[wasm_bindgen(js_name = copyText, catch)]
    pub fn copy_text(text: &str) -> std::result::Result<Promise, JsValue>;
}
