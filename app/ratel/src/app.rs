use crate::common::*;
use crate::config;
use crate::*;

use crate::*;

pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const MAIN_JS: Asset = asset!("/assets/ratel-app-shell.js");

#[cfg(feature = "server")]
pub fn app() -> crate::axum::Router {
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

  // Track the soft-keyboard height as `--kb-inset` on <html>. The Android
  // WebView only shrinks the *visual* viewport when the keyboard opens (the
  // layout viewport / `dvh` stays full), so CSS alone can't react. Full-screen
  // editors (`.composer-arena`) subtract this so their bottom toolbar stays
  // above the keyboard. Updates on resize/scroll of the visual viewport.
  (function () {
    var vv = window.visualViewport;
    if (!vv) return;
    function update() {
      var kb = Math.max(0, window.innerHeight - vv.height - vv.offsetTop);
      document.documentElement.style.setProperty("--kb-inset", kb + "px");
      // iOS WKWebView shifts the *visual* viewport up (offsetTop > 0) to keep a
      // focused input above the keyboard, dragging `position:fixed` elements up
      // with it — so the scroll-locked body (topbar) jumps under the status bar
      // when a modal input is focused. Expose the offset so the lock can apply
      // an equal counter-translate and stay visually pinned. 0 on Android.
      document.documentElement.style.setProperty(
        "--vv-offset-top", Math.round(vv.offsetTop) + "px");
    }
    vv.addEventListener("resize", update);
    vv.addEventListener("scroll", update);
    update();
  })();
"#,
        );
    });

    // iOS WKWebView auto-zooms into any focused input whose font-size < 16px
    // (Android has no such behavior), which shoves modals off-screen. In the
    // native Tauri shell pinch-zoom isn't wanted anyway, so `maximum-scale=1 +
    // user-scalable=no` kills the focus-zoom for every input at once. The
    // browser web build keeps user zoom for accessibility.
    #[cfg(feature = "tauri-web")]
    let viewport_content = "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover, interactive-widget=resizes-content";
    #[cfg(not(feature = "tauri-web"))]
    let viewport_content = "width=device-width, initial-scale=1.0, viewport-fit=cover, interactive-widget=resizes-content";

    rsx! {
        document::Meta {
            // `interactive-widget=resizes-content` shrinks the layout viewport
            // (and `dvh`) when the soft keyboard opens, so `100dvh`-based
            // layouts like the composer collapse to the area above the keyboard
            // and their bottom toolbar stays reachable instead of being hidden
            // behind the keyboard. Pairs with `adjustResize` on MainActivity.
            content: viewport_content,
            name: "viewport",
        }
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

        // crate::common::Provider {}
        // Top-level SuspenseBoundary catches suspense propagated from
        // `RootLayout` (which calls `Context::init()?` / `TeamContext::init()?`
        // before its rsx! body). Without a boundary above the Router, the
        // suspended scope never gets re-rendered when its resource task
        // resolves, so the entire UI stays blank.
        SuspenseBoundary { Router::<Route> {} }
    }
}
