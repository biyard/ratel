//! `UseCrossPosting` controller hook.
//!
//! Per `conventions/hooks-and-actions.md`:
//!
//! - **Two-function pattern** — provider installs context, consumer reads it
//!   back. The earlier "single function with `try_use_context` early-return"
//!   pattern violated Dioxus's rules of hooks (first render registered every
//!   hook slot, subsequent renders bailed via the cache and registered zero,
//!   producing slot-mismatch panics). Splitting keeps the call sequence stable.
//!     - [`use_cross_posting_provider`] — runs every signal / loader and
//!       installs `UseCrossPosting` via `use_context_provider`. Call **once**
//!       from a long-lived ancestor (Settings/Connections page or post-edit).
//!     - [`use_cross_posting`] — pure `use_context` read, safe from any
//!       transient component.
//!
//! - **Mutations are `async fn` methods on the controller**, not
//!   `use_action` fields. Components `await` the result and decide UX
//!   (close popup, toast, navigate). The few legitimate `use_action`
//!   cases are when UI binds to `.pending()` / `.error()` / `.value()`
//!   directly — none of cross-posting's button handlers do that.

use crate::common::*;
use crate::features::cross_posting::controllers::{
    connect_bluesky_handler, connect_linkedin_init_handler, disconnect_handler,
    list_connections_handler, toggle_auto_post_handler,
};
use crate::features::cross_posting::models::ConnectionStatus;
use crate::features::cross_posting::types::{
    ConnectBlueskyRequest, ConnectionResponse, SocialPlatform, ToggleAutoPostRequest,
};
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
}

impl UseCrossPosting {
    /// Bluesky app-password connect. Discards the app_password after
    /// `createSession`; only the returned session JWTs are persisted
    /// (AEAD-sealed). Refreshes the connections loader on success so the
    /// Settings page row flips to "Connected" without a manual refetch.
    pub async fn connect_bluesky(
        &mut self,
        handle: String,
        app_password: String,
    ) -> crate::common::Result<()> {
        connect_bluesky_handler(ConnectBlueskyRequest {
            handle,
            app_password,
        })
        .await?;
        self.connections.restart();
        Ok(())
    }

    /// Toggle `auto_post_enabled` on an existing connection.
    pub async fn toggle_auto_post(
        &mut self,
        platform: SocialPlatform,
        enabled: bool,
    ) -> crate::common::Result<()> {
        toggle_auto_post_handler(
            platform,
            ToggleAutoPostRequest {
                auto_post_enabled: enabled,
            },
        )
        .await?;
        self.connections.restart();
        Ok(())
    }

    /// Disconnect (soft-delete: status=Revoked, ciphertext zeroed).
    pub async fn disconnect(&mut self, platform: SocialPlatform) -> crate::common::Result<()> {
        disconnect_handler(platform).await?;
        self.connections.restart();
        Ok(())
    }

    /// Kick off the LinkedIn OAuth flow. Calls the init endpoint to mint
    /// a signed state token + LinkedIn authorize URL, then bounces the
    /// browser there. `nav.push(Route)` is SPA-internal and would lose
    /// the redirect, so we use the interop helper which goes through
    /// `dioxus::document::eval` (see `conventions/dioxus-app.md` §
    /// JS Interop).
    ///
    /// `return_to` is an optional SPA-internal path the OAuth callback
    /// will bounce the user back to on success (e.g. the post-edit URL
    /// they came from). `None` ⇒ default to the connections page.
    ///
    /// Server-side seal/upsert happens in the `/callback` handler after
    /// LinkedIn bounces the user back; this method does NOT refresh
    /// `connections` because the page is about to leave anyway.
    pub async fn connect_linkedin(
        &mut self,
        return_to: Option<String>,
    ) -> crate::common::Result<()> {
        let req = crate::features::cross_posting::types::LinkedInOauthInitRequest { return_to };
        let resp = connect_linkedin_init_handler(req).await?;
        crate::features::cross_posting::interop::redirect_to_external(&resp.authorize_url);
        Ok(())
    }
}

/// Provider — runs every cross-posting signal / loader once and installs
/// the resulting `UseCrossPosting` via `use_context_provider`. Call
/// exactly once from a long-lived ancestor; consumer components use
/// [`use_cross_posting`].
#[track_caller]
pub fn use_cross_posting_provider() -> std::result::Result<UseCrossPosting, RenderError> {
    if let Some(ctx) = try_use_context::<UseCrossPosting>() {
        return Ok(ctx);
    }

    // Read login state reactively so the connections loader re-runs on
    // login/logout. Logged-out users see an empty list without hitting
    // the API, mirroring the inbox-loader pattern.
    let user_ctx = crate::features::auth::hooks::use_user_context();

    let connections = use_loader(move || {
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

    Ok(use_context_provider(|| UseCrossPosting {
        connections,
        connected_count,
        posts_this_month,
        per_post_enabled,
        reach_count,
    }))
}

/// Consumer — reads the `UseCrossPosting` controller that an ancestor
/// installed via [`use_cross_posting_provider`]. Pure context read (one
/// stable hook slot). Panics if no ancestor provided.
#[track_caller]
pub fn use_cross_posting() -> UseCrossPosting {
    use_context::<UseCrossPosting>()
}
