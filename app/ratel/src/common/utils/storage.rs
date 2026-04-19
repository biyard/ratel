//! Cross-platform persistent key-value storage backed by the WebView's
//! `localStorage`.
//!
//! On web (browser) and on mobile (Dioxus mobile runs a WebView), a single
//! JS bridge — the snippets below invoked via `dioxus::document::eval` —
//! gives us persistent storage that survives page reloads, navigations,
//! and Android app restarts.
//!
//! On the server (SSR) there is no storage; both functions degrade to no-ops
//! so callers can use them unconditionally.
//!
//! This is the shared primitive behind Theme, Language, and cached
//! `UserContext` persistence — route any new persistence need through here
//! instead of re-rolling another `document::eval` snippet.

#[cfg(not(feature = "server"))]
use dioxus::prelude::*;

fn quote(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_string())
}

/// Read a previously saved string by key. Returns `None` when the key is
/// absent, when storage isn't available (private mode, SSR, etc.), or when
/// the JS bridge throws.
pub async fn load(key: &str) -> Option<String> {
    #[cfg(feature = "server")]
    {
        let _ = key;
        None
    }

    #[cfg(not(feature = "server"))]
    {
        let script = format!(
            r#"try {{
                var v = window.localStorage.getItem({k});
                dioxus.send(v === null ? null : v);
            }} catch (_e) {{
                dioxus.send(null);
            }}"#,
            k = quote(key),
        );
        let mut runner = document::eval(&script);
        runner.recv::<Option<String>>().await.ok().flatten()
    }
}

/// Write a string value. Fire-and-forget; failures (quota, SSR) are
/// swallowed because callers routinely want to save in event handlers where
/// there's nowhere to surface the error.
pub fn save(key: &str, value: &str) {
    #[cfg(feature = "server")]
    {
        let _ = (key, value);
    }

    #[cfg(not(feature = "server"))]
    {
        let script = format!(
            r#"try {{ window.localStorage.setItem({k}, {v}); }} catch (_e) {{}}"#,
            k = quote(key),
            v = quote(value),
        );
        let _ = document::eval(&script);
    }
}

/// Remove a key entirely. Useful when invalidating a cached session.
pub fn remove(key: &str) {
    #[cfg(feature = "server")]
    {
        let _ = key;
    }

    #[cfg(not(feature = "server"))]
    {
        let script = format!(
            r#"try {{ window.localStorage.removeItem({k}); }} catch (_e) {{}}"#,
            k = quote(key),
        );
        let _ = document::eval(&script);
    }
}
