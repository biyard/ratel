//! Platform-agnostic logout flow.
//!
//! Every logout site must do three things:
//!   1. Flush the server-side session (`logout_handler`).
//!   2. Clear the in-memory `UserContext` so the UI reflects the signed-out
//!      state on every platform — web, desktop, mobile.
//!   3. On web only, reload the window to drop any SSR-hydrated user data.
//!
//! Six call sites used to duplicate this, and all of them only cleared state
//! via `window.location().reload()` — which is a no-op on mobile/desktop,
//! leaving the UI stuck in a signed-in state even after the server session
//! was flushed. Route every logout through this helper instead.
//!
//! The caller must pass in the `UserContext` store obtained from
//! `use_user_context()` during render — Dioxus hooks cannot be invoked from a
//! detached async task.

use crate::features::auth::context::UserContext;
use crate::features::auth::controllers::logout_handler;
use crate::features::auth::*;

pub async fn sign_out(mut user_ctx: Store<UserContext>) {
    if let Err(e) = logout_handler().await {
        crate::error!("logout_handler failed: {e}");
    }

    user_ctx.set(UserContext::default());

    // Drop persistent restore-session state too, otherwise the next app
    // launch would refresh straight back into the just-signed-out account.
    crate::features::auth::services::restore_session::clear_refresh_token();
    crate::common::services::persistent_state::clear_cached_session();

    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let _ = window.location().reload();
        }
    }
}
