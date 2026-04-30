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
    // Hydrate language + cached user session from the WebView's
    // localStorage and keep them in sync on every change. Must run after
    // `Context::init` so we don't overwrite a server-validated user with
    // stale cached data.
    crate::common::services::use_persist_ui_state();
    use_effect(move || {
        document::eval(
            r#"
  const userAgent = navigator.userAgent.toLowerCase();
  const isKakaoInApp = userAgent.includes("kakaotalk");

  if (isKakaoInApp) {
    const targetUrl = window.location.href;
    window.location.replace(
      `kakaotalk://web/openExternal?url=${encodeURIComponent(targetUrl)}`,
    );
  }
"#,
        );
    });

    rsx! {
        document::Link { rel: "icon", href: crate::common::assets::FAVICON }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Orbitron:wght@400;500;600;700;800;900&family=Outfit:wght@300;400;500;600;700&display=swap" }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: asset!("/assets/dx-components-theme.css") }
        // Loaded as a module: Dioxus's `asset!()` post-processes JS into
        // an ES module wrapper (the bundle ends with `export default …`),
        // so a classic `<script>` tag throws `Unexpected token 'export'`
        // and breaks `window.ratel` namespace setup. The bundle's only
        // side effect is populating `window.ratel`, which still happens
        // when loaded as a module.
        document::Script { r#type: "module", src: MAIN_JS }
        document::Script { r#type: "module", src: asset!("/assets/wallet-connect.js") }
        document::Script { src: "https://cdn.portone.io/v2/browser-sdk.js" }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        crate::common::Provider {}

        Router::<Route> {}
    }
}
