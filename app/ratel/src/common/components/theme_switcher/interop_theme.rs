use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use crate::common::*;

#[wasm_bindgen(js_namespace = ["window", "ratel", "common", "theme"])]
extern "C" {
    pub fn load_theme() -> Option<String>;
    pub fn save_theme(theme: &str);
    pub fn apply_theme(theme: &str);
    pub fn get_applied_theme() -> String;
}
