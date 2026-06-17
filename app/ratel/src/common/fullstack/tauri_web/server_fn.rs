//! Plain HTTP helpers that stand in for dioxus-fullstack's RPC transport when
//! running inside the Tauri Android shell.
//!
//! Why: `dioxus/fullstack` pulls `dioxus-web/hydrate` which assumes
//! server-rendered HTML and a matching DOM tree on the client. Tauri serves a
//! static `dist/index.html` with no SSR content, so the hydration code path
//! hits undefined DOM nodes / atob / hydrate_queue / etc. Bypassing the
//! fullstack transport with plain reqwest avoids the whole hydration
//! infrastructure for outbound calls.
//!
//! Usage (PoC — to be wrapped by a proc macro later):
//!
//! ```ignore
//! #[cfg(not(feature = "tauri-web"))]
//! #[get("/api/auth/me", user: OptionalUser)]
//! pub async fn get_me_handler() -> Result<GetMeResponse> { ... }
//!
//! #[cfg(feature = "tauri-web")]
//! pub async fn get_me_handler() -> Result<GetMeResponse> {
//!     server_fn::get("/api/auth/me").await
//! }
//! ```

use reqwest::Method;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::common::Error;


/// Compile-time backend base URL. Baked from `MOBILE_API_URL` by the Makefile.
/// Falls back to dev.ratel.foundation when unset (matches `mobile_endpoint`).
///
/// Public so UI that must surface a real, externally-reachable URL (e.g. the
/// MCP `claude mcp add` command, post share links) can use it instead of the
/// WebView's internal `tauri.localhost` origin. See `use_origin`.
pub fn api_base_url() -> &'static str {
    option_env!("MOBILE_API_URL").unwrap_or("https://dev.ratel.foundation")
}

/// Serialize any `Serialize` value into the textual form we splice into a
/// URL path or query string. Goes through `serde_json` and then strips the
/// surrounding quotes when the result is a JSON string, which makes it
/// round-trip with the server's serde-based deserialization (incl. enums
/// with `#[serde(rename_all = "...")]`). Anything that isn't a JSON string
/// (numbers, bools, objects encoded into the URL) keeps its raw form.
pub fn to_url_value<T: Serialize + ?Sized>(v: &T) -> String {
    let json = serde_json::to_string(v).unwrap_or_default();
    if json.len() >= 2 && json.starts_with('"') && json.ends_with('"') {
        // Unescape the JSON string body — the server splits on `&`/`=`,
        // so we still need percent-encoding from the caller, but the
        // *value* itself should be the unquoted string.
        serde_json::from_str::<String>(&json).unwrap_or_default()
    } else {
        json
    }
}

pub async fn get<R: DeserializeOwned>(path: &str) -> crate::common::Result<R> {
    send::<(), R>(Method::GET, path, None).await
}

pub async fn post<B: Serialize + ?Sized, R: DeserializeOwned>(
    path: &str,
    body: &B,
) -> crate::common::Result<R> {
    send::<B, R>(Method::POST, path, Some(body)).await
}

pub async fn put<B: Serialize + ?Sized, R: DeserializeOwned>(
    path: &str,
    body: &B,
) -> crate::common::Result<R> {
    send::<B, R>(Method::PUT, path, Some(body)).await
}

pub async fn patch<B: Serialize + ?Sized, R: DeserializeOwned>(
    path: &str,
    body: &B,
) -> crate::common::Result<R> {
    send::<B, R>(Method::PATCH, path, Some(body)).await
}

pub async fn delete<R: DeserializeOwned>(path: &str) -> crate::common::Result<R> {
    send::<(), R>(Method::DELETE, path, None).await
}

/// DELETE with a JSON body. Some delete endpoints take a request payload (e.g.
/// the keys to delete), which the dioxus server decoder expects keyed by the
/// handler arg name — same shape as POST/PUT/PATCH.
pub async fn delete_with_body<B: Serialize + ?Sized, R: DeserializeOwned>(
    path: &str,
    body: &B,
) -> crate::common::Result<R> {
    send::<B, R>(Method::DELETE, path, Some(body)).await
}

/// Args sent to the native `api_request` Tauri command.
#[derive(Serialize)]
struct ApiArgs<'a> {
    method: &'a str,
    url: &'a str,
    body: Option<String>,
}

/// Mirror of `ratel_tauri_lib::commands::api_request::ApiResponse`.
#[derive(serde::Deserialize)]
struct ApiResponse {
    status: u16,
    body: String,
}

async fn send<B: Serialize + ?Sized, R: DeserializeOwned>(
    method: Method,
    path: &str,
    body: Option<&B>,
) -> crate::common::Result<R> {
    let url = format!("{}{}", api_base_url(), path);
    let body = body.map(|b| serde_json::to_string(b).unwrap_or_default());

    tracing::debug!("[tauri-web] {} {}", method, url);

    // Route through the native `api_request` command instead of the in-WebView
    // fetch. The WebView page (tauri://localhost / http://tauri.localhost) and
    // the backend (https://*.ratel.foundation) are a cross-site pair, and iOS
    // WKWebView (ITP) strips the session cookie from cross-site fetches —
    // breaking auth right after login. The native command owns a persistent
    // reqwest cookie jar in the app process, so the login `Set-Cookie` is
    // stored and replayed on every later call. It also negotiates HTTP/2 (not
    // WKWebView's QUIC), sidestepping the simulator's broken HTTP/3.
    let resp: ApiResponse = crate::common::utils::web::invoke_tauri(
        "api_request",
        ApiArgs {
            method: method.as_str(),
            url: &url,
            body,
        },
    )
    .await
    .map_err(|e| {
        tracing::error!("[tauri-web] {} {} invoke error: {e:?}", method, url);
        Error::Internal
    })?;

    if !(200..300).contains(&resp.status) {
        tracing::error!(
            "[tauri-web] {} {} -> {} body={}",
            method,
            url,
            resp.status,
            resp.body
        );
        // The backend serializes errors as {"message","code","data"} where
        // `data` is the serialized `common::Error` (e.g. {"Auth":"UserNotFound"}).
        // Recover that first so callers can match on specific variants — the
        // login flow routes `Error::Auth(AuthError::UserNotFound)` into signup,
        // which a blanket 401 -> UnauthorizedAccess mapping would break. Fall
        // back to HTTP-status mapping only when the body isn't that shape.
        #[derive(serde::Deserialize)]
        struct ErrBody {
            data: Error,
        }
        if let Ok(parsed) = serde_json::from_str::<ErrBody>(&resp.body) {
            return Err(parsed.data);
        }
        return Err(match resp.status {
            401 | 403 => Error::UnauthorizedAccess,
            _ => Error::Internal,
        });
    }

    serde_json::from_str::<R>(&resp.body).map_err(|e| {
        tracing::error!("[tauri-web] {} {} decode error: {e}", method, url);
        Error::Internal
    })
}
