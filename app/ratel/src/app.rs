use crate::common::*;
use crate::config;
use crate::features::auth::AuthProvider;
use crate::*;

use dioxus::prelude::*;

pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const MAIN_JS: Asset = asset!("/assets/ratel-app-shell.js");

#[component]
pub fn App() -> Element {
    use_context_provider(|| PopupService::new());
    ToastService::init();
    ThemeService::init();

    let _ = crate::features::auth::Context::init()?;

    // Signal to Playwright that WASM hydration is complete.
    // Placed after Context::init() so the flag is only set once the auth
    // context (which uses `use_loader(...)?` and can suspend) has resolved.
    // This way Playwright's `goto()` — which waits on `__dioxus_hydrated` —
    // will not proceed while the app is still suspended/loading.
    #[cfg(not(feature = "server"))]
    {
        if let Some(window) = web_sys::window() {
            let _ = js_sys::Reflect::set(
                &window,
                &wasm_bindgen::JsValue::from_str("__dioxus_hydrated"),
                &wasm_bindgen::JsValue::TRUE,
            );
        }
    }
    crate::common::contexts::TeamContext::init();
    let conf = config::get();
    let env = conf.common.env;
    use_context_provider(QueryStore::new);

    let keywords = vec![
        "ratel".to_string(),
        "knowledge platform".to_string(),
        "ai knowledge base".to_string(),
        "human knowledge dataset".to_string(),
        "ai training data".to_string(),
        "community intelligence".to_string(),
        "participatory platform".to_string(),
        "survey rewards".to_string(),
        "poll rewards".to_string(),
        "web3 knowledge economy".to_string(),
        "ai memory platform".to_string(),
        "vector knowledge database".to_string(),
        "collective intelligence".to_string(),
    ];

    rsx! {
        document::Link { rel: "icon", href: crate::common::assets::FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/dx-components-theme.css"),
        }
        crate::common::SeoMeta {
            title: "Ratel – AI Knowledge Platform Powered by Human Essences",
            description: "Ratel is a participatory knowledge platform where users share expertise, opinions, and insights as structured “Essences”. AI agents learn from the knowledge base while users earn rewards through surveys, polls, and discussions.",
            image: "https://metadata.ratel.foundation/logos/logo-symbol.png",
            url: "https://ratel.foundation",
            robots: Robots::IndexNofollow,
            keywords,
        }
        document::Script { src: MAIN_JS }
        document::Script { src: "https://cdn.portone.io/v2/browser-sdk.js" }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

        crate::common::Provider {}
        AuthProvider {}

        Router::<Route> {}
        if env == Environment::Local {
            DevTools {}
        }
    }
}
