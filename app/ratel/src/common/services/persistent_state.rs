//! App-level hook that hydrates UI preferences from the WebView's
//! `localStorage` and persists further changes back to storage.
//!
//! Call `use_persist_ui_state()` once from a high-level provider that
//! wraps the entire app so the persisted values are loaded before any
//! descendant component reads them.
//!
//! Right now this covers Language only. UserContext caching was tried and
//! removed — restoring a cached user without also restoring the server
//! session cookie produced a misleading "signed in" UI that immediately
//! 401'd on the next authenticated call. See `clear_cached_session` for
//! the defensive cleanup callable from places that detect a 401.

use dioxus::prelude::*;
use dioxus_translate::{set_initial_language, Language, STORAGE_KEY as LANGUAGE_STORAGE_KEY};
use std::str::FromStr;

use crate::common::utils::storage;

/// Storage key reserved for any future cached user-session blob (refresh
/// token, etc.). Used today only to know what to wipe on 401.
pub const USER_CONTEXT_STORAGE_KEY: &str = "user_context";

/// Root-level hydrator. Wire this into a provider rendered above any
/// component that reads the persisted state.
///
/// Function-name prefix `use_` is required by the Dioxus lint so it can
/// see this is a hook composition site — without it, the inner
/// `use_future` / `use_effect` calls fail `dx check` with
/// "hook called outside component or hook".
pub fn use_persist_ui_state() {
    use_hydrate_language();
    use_persist_language_on_change();
}

/// Defensive cleanup. Call from anywhere that observes a session-gone
/// signal (NoSessionFound, 401 from `get_me_handler`, etc.) so we don't
/// keep replaying a dead session on the next render or restart.
pub fn clear_cached_session() {
    storage::remove(USER_CONTEXT_STORAGE_KEY);
}

// ── Language ────────────────────────────────────────────────────────────

fn use_hydrate_language() {
    use_future(move || async move {
        if let Some(raw) = storage::load(LANGUAGE_STORAGE_KEY).await {
            if let Ok(lang) = Language::from_str(&raw) {
                set_initial_language(lang);
            }
        }
    });
}

fn use_persist_language_on_change() {
    let lang = dioxus_translate::use_language();
    use_effect(move || {
        let current = lang();
        storage::save(LANGUAGE_STORAGE_KEY, &current.to_string());
    });
}
