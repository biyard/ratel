//! Cross-platform theme interop.
//!
//! Uses `dioxus::document::eval` so the same code runs in the browser bundle
//! and inside the Android/iOS WebView spun up by `dioxus/mobile`. The matching
//! JS helpers live in `app/ratel/js/src/common/theme.js` and are exposed on
//! `window.ratel.common.theme` by the bundled `ratel-app-shell.js`.

#[cfg(not(feature = "server"))]
use dioxus::prelude::*;

const LOAD_SCRIPT: &str = r#"
try {
    if (window.ratel && window.ratel.common && window.ratel.common.theme) {
        dioxus.send(window.ratel.common.theme.load_theme());
    } else {
        dioxus.send(null);
    }
} catch (_e) {
    dioxus.send(null);
}
"#;

/// Read the persisted theme from the WebView's `localStorage`.
/// Returns `None` on server or when nothing is saved.
pub async fn load_theme() -> Option<String> {
    #[cfg(feature = "server")]
    {
        return None;
    }

    #[cfg(not(feature = "server"))]
    {
        let mut runner = document::eval(LOAD_SCRIPT);
        runner.recv::<Option<String>>().await.ok().flatten()
    }
}

pub fn save_theme(theme: &str) {
    #[cfg(feature = "server")]
    {
        let _ = theme;
    }

    #[cfg(not(feature = "server"))]
    {
        let script = format!(
            r#"try {{ if (window.ratel && window.ratel.common && window.ratel.common.theme) {{ window.ratel.common.theme.save_theme({theme:?}); }} }} catch (_e) {{}}"#
        );
        let _ = document::eval(&script);
    }
}

pub fn apply_theme(theme: &str) {
    #[cfg(feature = "server")]
    {
        let _ = theme;
    }

    #[cfg(not(feature = "server"))]
    {
        let script = format!(
            r#"try {{ if (window.ratel && window.ratel.common && window.ratel.common.theme) {{ window.ratel.common.theme.apply_theme({theme:?}); }} }} catch (_e) {{}}"#
        );
        let _ = document::eval(&script);
    }
}
