use crate::common::*;
use crate::config;
use crate::features::auth::AuthProvider;
use crate::*;

use dioxus::prelude::*;

pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const MAIN_JS: Asset = asset!("/assets/ratel-app-shell.js");

#[cfg(feature = "server")]
pub fn app() -> by_axum::axum::AxumRouter {
    dioxus::server::router(App)
}

#[component]
pub fn App() -> Element {
    use_context_provider(|| PopupService::new());
    ToastService::init();
    ThemeService::init();
    let _ = crate::features::auth::Context::init()?;
    crate::common::contexts::TeamContext::init();
    let conf = config::get();
    let env = conf.common.env;
    use_context_provider(QueryStore::new);
    let mut init = use_signal(|| false);

    use_effect(move || {
        if init() {
            return;
        }

        init.set(true);
        interop::initialize(&serde_wasm_bindgen::to_value(&serde_json::json!({})).unwrap());
    });

    rsx! {
        document::Link { rel: "icon", href: crate::common::assets::FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link {
            rel: "stylesheet",
            href: asset!("/assets/dx-components-theme.css"),
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
