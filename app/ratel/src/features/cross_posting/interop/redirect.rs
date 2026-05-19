//! Top-level browser navigation helper for cross-posting OAuth flows.
//!
//! Uses the `dioxus::document::eval` channel pattern (see
//! `conventions/dioxus-app.md` § JS Interop) so the same call compiles
//! on every target — the runner is a no-op outside web, no per-platform
//! `cfg` gates needed at the call site.

use dioxus::document::eval as dx_eval;

/// Navigate the top-level browser window to `url`. Use this instead of
/// `nav.push(...)` when the destination leaves the SPA (OAuth consent
/// pages on linkedin.com, threads.net, etc.) — `nav.push` is router-
/// internal and would lose the redirect.
pub fn redirect_to_external(url: &str) {
    let runner = dx_eval(include_str!("web/redirect_external.js"));
    let _ = runner.send(serde_json::json!(url));
}
