#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn OAuthPopup() -> Element {
    #[cfg(feature = "web")]
    {
        use url::Url;
        let window = match web_sys::window() {
            Some(window) => window,
            None => {
                tracing::error!("Window not found");
                return rsx! {
                    div { "Window not found" }
                };
            }
        };
        let url = window.location().href().unwrap_or_default();
        tracing::debug!("url={:?}", url);
        let opener: web_sys::Window = match window.opener() {
            Ok(opener) => opener.into(),
            Err(e) => {
                tracing::debug!("e={e:?}");
                return rsx! {
                    div { "opener not found" }
                };
            }
        };
        let url = Url::parse(&url).expect("Failed to parse URL");
        if let Some(code) = url.query_pairs().find(|(key, _)| key == "code") {
            tracing::debug!("Code query parameter: {}", code.1);

            if let Err(e) = opener.post_message(
                &wasm_bindgen::JsValue::from_str(&format!("code={}", code.1)),
                "*",
            ) {
                tracing::error!("");
                return rsx! { "{e:?}" };
            } else {
                let _ = window.close();
            };
        }
    }

    rsx! {
        div { "" }
    }
}
