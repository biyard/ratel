use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use crate::*;

#[wasm_bindgen(js_namespace = ["window", "ratel", "app_shell"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn initialize(config: &JsValue);
}

#[macro_export]
macro_rules! define_invoke_js {
    ($fn:ident, $method:expr, $args:ty) => {
        pub async fn $fn(args: &$args) -> crate::Result<()> {
            crate::common::utils::web::invoke_with_args($method, args).await
        }
    };

    ($fn:ident, $method:expr) => {
        pub async fn $fn() -> crate::Result<()> {
            crate::common::utils::web::invoke_with_empty($method).await
        }
    };

    ($fn:ident, $method:expr, res: $res:ty) => {
        pub async fn $fn() -> crate::Result<$res> {
            crate::common::utils::web::invoke_with_empty($method).await
        }
    };
}
