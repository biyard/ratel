use common::wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = ["window", "ratel", "{{crate_name}}"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn initialize(config: &JsValue);
}
