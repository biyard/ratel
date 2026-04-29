//! `UseCrossPosting` controller hook.
//!
//! Per `conventions/hooks-and-actions.md`, every interactive feature
//! exposes a single controller bundling its loaders, signals, and
//! actions. Components destructure from this hook — they MUST NOT
//! call server-function `_handler`s directly.
//!
//! ## Two-function pattern (UseInbox precedent)
//!
//! Dioxus's rules of hooks require the hook-call sequence to be
//! identical across re-renders. The earlier "single function with
//! `try_use_context` early-return" pattern (still shown in
//! `hooks-and-actions.md`) violated this — first render registered
//! every hook slot, second render bailed out via context-cache and
//! registered zero. Splitting installer / consumer keeps the
//! sequence stable.
//!
//! - [`use_provide_cross_posting`] — runs every signal / loader /
//!   action and installs the `UseCrossPosting` value via
//!   `use_context_provider`. Call **exactly once** from a long-lived
//!   ancestor (e.g. the Settings/Connections page route, or a
//!   ComposeBootstrap if used inside compose).
//! - [`use_cross_posting`] — pure context read. Safe from any
//!   transient component. Panics if no ancestor provided.

use crate::common::*;
use crate::features::cross_posting::controllers::{
    connect_bluesky_handler, disconnect_handler, list_connections_handler, toggle_auto_post_handler,
};
use crate::features::cross_posting::models::ConnectionStatus;
use crate::features::cross_posting::types::{
    ConnectBlueskyRequest, ConnectionResponse, SocialPlatform, ToggleAutoPostRequest,
};
use dioxus::fullstack::Loader;
use std::collections::HashMap;

#[derive(Clone, Copy, DioxusController)]
pub struct UseCrossPosting {
    /// Server-loaded list of the user's social connections (Bluesky /
    /// LinkedIn / Threads). Populated from `GET /api/cross-posting/connections`.
    /// Empty Vec when the user is logged out (the loader short-circuits).
    pub connections: Loader<Vec<ConnectionResponse>>,

    /// Count of connections in `Connected` state — drives the Settings
    /// page header stat ("X Connected").
    pub connected_count: Memo<usize>,

    /// "Y posts this month" header stat. Phase 1 placeholder = 0; the
    /// dedicated server endpoint that aggregates published syndications
    /// in the current calendar month is deferred to a later PR.
    pub posts_this_month: Signal<i64>,

    /// Compose-time per-post platform toggle map. Keyed by platform,
    /// `true` = include this platform in the next publish, `false` =
    /// skip. The compose sidebar reads/writes this signal; the
    /// `update_post_handler` Publish call serializes its `true` keys
    /// into `enabled_platforms`. Persistent connection settings
    /// (`auto_post_enabled` on `SocialConnection`) are NOT mutated by
    /// this signal — only the per-post override.
    pub per_post_enabled: Signal<HashMap<SocialPlatform, bool>>,

    /// Count of platforms currently toggled `true` in `per_post_enabled`.
    /// Drives the compose sidebar's "Reaching N networks" summary and
    /// the Publish button's "Publish to N" label.
    pub reach_count: Memo<usize>,

    // ── Actions (mutations that hit the server) ───────────────────────

    /// Bluesky app-password connect. Inputs: (handle, app_password).
    /// Discards the app_password after `createSession`; only the
    /// returned session JWTs are persisted (AEAD-sealed).
    pub handle_connect_bluesky: Action<(String, String), ()>,

    /// Toggle `auto_post_enabled` on an existing connection. Inputs:
    /// (platform, new_value).
    pub handle_toggle_auto_post: Action<(SocialPlatform, bool), ()>,

    /// Disconnect (soft-delete: status=Revoked, ciphertext zeroed).
    /// Inputs: (platform,).
    pub handle_disconnect: Action<(SocialPlatform,), ()>,
}

/// Installer — runs every cross-posting hook in this scope and installs
/// the resulting `UseCrossPosting` via `use_context_provider`. Call
/// exactly once from a long-lived ancestor; consumer components use
/// [`use_cross_posting`].
#[track_caller]
pub fn use_provide_cross_posting() -> std::result::Result<UseCrossPosting, RenderError> {
    // Read login state reactively so the connections loader re-runs on
    // login/logout. Logged-out users see an empty list without hitting
    // the API, mirroring the inbox-loader pattern.
    let user_ctx = crate::features::auth::hooks::use_user_context();

    let mut connections = use_loader(move || {
        let logged_in = user_ctx().is_logged_in();
        async move {
            if !logged_in {
                return Ok(Vec::<ConnectionResponse>::new());
            }
            list_connections_handler().await
        }
    })?;

    let connected_count = use_memo(move || {
        connections()
            .iter()
            .filter(|c| c.status == ConnectionStatus::Connected)
            .count()
    });

    let posts_this_month = use_signal(|| 0i64);
    let per_post_enabled = use_signal(HashMap::<SocialPlatform, bool>::new);

    let reach_count = use_memo(move || per_post_enabled().values().filter(|v| **v).count());

    let handle_connect_bluesky = use_action(move |handle: String, app_password: String| async move {
        connect_bluesky_handler(ConnectBlueskyRequest { handle, app_password }).await?;
        connections.restart();
        Ok::<(), crate::common::Error>(())
    });

    let handle_toggle_auto_post =
        use_action(move |platform: SocialPlatform, enabled: bool| async move {
            toggle_auto_post_handler(
                platform.to_string(),
                ToggleAutoPostRequest { auto_post_enabled: enabled },
            )
            .await?;
            connections.restart();
            Ok::<(), crate::common::Error>(())
        });

    let handle_disconnect = use_action(move |platform: SocialPlatform| async move {
        disconnect_handler(platform.to_string()).await?;
        connections.restart();
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseCrossPosting {
        connections,
        connected_count,
        posts_this_month,
        per_post_enabled,
        reach_count,
        handle_connect_bluesky,
        handle_toggle_auto_post,
        handle_disconnect,
    }))
}

/// Consumer — reads the `UseCrossPosting` controller that an ancestor
/// installed via [`use_provide_cross_posting`]. Pure context read (one
/// stable hook slot). Panics if no ancestor provided — indicating a
/// missing installer in the route tree.
#[track_caller]
pub fn use_cross_posting() -> UseCrossPosting {
    use_context::<UseCrossPosting>()
}
