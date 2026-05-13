//! Server-only axum routes for cross-posting that don't fit Dioxus'
//! `Result<JSON>` server-function shape.
//!
//! Currently just the LinkedIn OAuth callback, which LinkedIn redirects
//! the user's browser to with `?code=&state=` query params. The handler
//! consumes those, persists the new SocialConnection row, and 302s the
//! user back to the connections page — none of which fits the
//! JSON-response contract that `#[get]/#[post]` macros generate.

use crate::common::axum::{
    AxumRouter, Extension,
    extract::Query,
    http::StatusCode,
    native_routing::get,
    response::{IntoResponse, Redirect, Response},
};
use crate::common::config::site_base_url;
use crate::common::models::auth::SESSION_KEY_USER_ID;
use crate::features::cross_posting::services::adapters::{DecryptedCredentials, LinkedInAdapter};
use crate::features::cross_posting::services::connection::{
    ConnectionUpsert, seal_and_upsert_connection,
};
use crate::features::cross_posting::services::oauth_state;
use crate::features::cross_posting::types::SocialPlatform;
use serde::Deserialize;

/// LinkedIn callback query string. LinkedIn either sends `code+state` on
/// success, or `error+error_description+state` on user-denial / invalid
/// scope / etc — we accept both shapes via `Option`s and surface the
/// appropriate redirect query.
#[derive(Deserialize)]
struct LinkedInCallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    #[serde(rename = "error_description")]
    error_description: Option<String>,
}

/// `GET /api/cross-posting/connections/linkedin/callback`. The browser
/// arrives here from LinkedIn's redirect after the user clicks Allow (or
/// Deny). We:
///   1. Reject if the user isn't signed in (no session) — mid-flow
///      sign-out is rare but recoverable: redirect to login with a
///      reason.
///   2. Surface user-cancel / LinkedIn errors via `?linkedin=denied|error`.
///   3. Verify the state HMAC + user_pk match.
///   4. Exchange the code for tokens, derive the member URN.
///   5. Seal + upsert the SocialConnection row via the shared helper.
///   6. 302 to `/settings/connections?linkedin=ok`.
async fn linkedin_callback(
    Extension(session): Extension<tower_sessions::Session>,
    Query(q): Query<LinkedInCallbackQuery>,
) -> Response {
    // (1) Auth check.
    let user_pk_str: String = match session.get(SESSION_KEY_USER_ID).await {
        Ok(Some(s)) => s,
        _ => {
            return Redirect::to(&format!(
                "{}/login?return_to=/settings/connections",
                site_base_url()
            ))
            .into_response();
        }
    };

    // (2) LinkedIn-side error. Either the user clicked Cancel
    // (`error=user_cancelled_login`) or the OAuth params were rejected
    // (`error=invalid_scope` etc). Surface both as `linkedin=denied` so
    // the connections page shows the same recoverable banner.
    if let Some(err) = q.error.as_deref() {
        tracing::info!(
            error = err,
            error_description = ?q.error_description,
            "linkedin callback received error from provider"
        );
        return Redirect::to(&connections_page_url("denied")).into_response();
    }

    let code = match q.code {
        Some(c) if !c.is_empty() => c,
        _ => {
            return Redirect::to(&connections_page_url("error")).into_response();
        }
    };
    let state = match q.state {
        Some(s) if !s.is_empty() => s,
        _ => {
            return Redirect::to(&connections_page_url("error")).into_response();
        }
    };

    // (3) HMAC + user-mismatch check.
    let decoded = match oauth_state::decode_and_verify(&state) {
        Ok(d) => d,
        Err(e) => {
            tracing::warn!(error = %e, "linkedin callback: state verify failed");
            return Redirect::to(&connections_page_url("error")).into_response();
        }
    };
    if decoded.user_pk.to_string() != user_pk_str {
        tracing::warn!(
            "linkedin callback: state.user_pk does not match session user — possible cross-user replay"
        );
        return Redirect::to(&connections_page_url("error")).into_response();
    }

    // (4) Code exchange → token + member URN. Redirect URI MUST match
    // the one used at /init byte-for-byte.
    let adapter = LinkedInAdapter::new();
    let redirect_uri =
        crate::features::cross_posting::controllers::connect_linkedin_init::linkedin_redirect_uri();
    let session_data = match adapter.exchange_code(&code, &redirect_uri).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(error = %e, "linkedin callback: code exchange failed");
            return Redirect::to(&connections_page_url("error")).into_response();
        }
    };

    // (5) Seal + upsert via shared helper. LinkedIn's access tokens are
    // typically 60-day-lived; we don't currently parse `expires_in`
    // from the response (LinkedInAdapter::TokenResponse drops it) so we
    // pass `None`. The dispatcher's `try_refresh_credentials` handles
    // expiry-driven refresh based on AuthExpired errors.
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let user_pk: crate::common::Partition = match user_pk_str.parse() {
        Ok(p) => p,
        Err(_) => {
            tracing::error!(
                user_pk_str,
                "linkedin callback: session user_pk failed to parse"
            );
            return Redirect::to(&connections_page_url("error")).into_response();
        }
    };

    let upsert = ConnectionUpsert {
        user_pk,
        platform: SocialPlatform::LinkedIn,
        decrypted: DecryptedCredentials::LinkedIn {
            access_token: session_data.access_token.clone(),
            refresh_token: session_data.refresh_token.clone(),
            member_urn: session_data.member_urn.clone(),
        },
        // LinkedIn `/v2/userinfo` includes a `name` field, but we drop
        // it on the floor here — the URN itself is what we need. The
        // connections page already displays "@member URN" as the
        // identity; a follow-up polish PR can pull `name` for nicer UX.
        external_handle: session_data.member_urn.clone(),
        external_user_id: session_data.member_urn,
        token_expires_at: None,
    };

    if let Err(e) = seal_and_upsert_connection(cli, upsert).await {
        tracing::error!(error = %e, "linkedin callback: seal+upsert failed");
        return Redirect::to(&connections_page_url("error")).into_response();
    }

    // (6) Success — bounce the user back to the connections page with a
    // success marker. The page reads `?linkedin=ok` and shows a toast.
    Redirect::to(&connections_page_url("ok")).into_response()
}

fn connections_page_url(linkedin: &str) -> String {
    format!("{}/settings/connections?linkedin={linkedin}", site_base_url())
}

pub fn router() -> AxumRouter {
    AxumRouter::new().route(
        "/api/cross-posting/connections/linkedin/callback",
        get(linkedin_callback),
    )
}

// Silence unused-import lints when this module is compiled but the
// callback route isn't hit (e.g., during cargo check on web target —
// though `server` feature gates the whole module via parent mod).
#[allow(dead_code)]
fn _unused_status_code() -> StatusCode {
    StatusCode::OK
}
