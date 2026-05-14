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
fn base_url() -> &'static str {
    option_env!("MOBILE_API_URL").unwrap_or("https://dev.ratel.foundation")
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

async fn send<B: Serialize + ?Sized, R: DeserializeOwned>(
    method: Method,
    path: &str,
    body: Option<&B>,
) -> crate::common::Result<R> {
    let url = format!("{}{}", base_url(), path);

    // A fresh client per call is fine — reqwest's wasm backend reuses the
    // browser's fetch internals and the builder-level config (cookie_store,
    // pool, etc.) is a no-op on wasm anyway. Cross-origin cookie handling on
    // wasm is driven by `fetch_credentials_include` below, not by client
    // config.
    let client = reqwest::Client::new();
    let mut req = client.request(method.clone(), &url);

    // The Tauri WebView page lives at http://tauri.localhost and the backend
    // at https://ratel.foundation — that's a cross-origin pair, so the
    // browser defaults to `credentials: 'same-origin'` and strips the
    // session cookie. Forcing `include` lets the cookie ride along, paired
    // with the server's CORS allow-credentials + `SameSite=None; Secure`
    // cookie attributes that we already configured.
    #[cfg(target_arch = "wasm32")]
    {
        req = req.fetch_credentials_include();
    }

    if let Some(b) = body {
        req = req.json(b);
    }

    tracing::debug!("[tauri-web] {} {}", method, url);

    let resp = req.send().await.map_err(|e| {
        tracing::error!("[tauri-web] {} {} request error: {e}", method, url);
        Error::Internal
    })?;

    let status = resp.status();
    if !status.is_success() {
        let body_text = resp.text().await.unwrap_or_default();
        tracing::error!(
            "[tauri-web] {} {} -> {} body={}",
            method,
            url,
            status,
            body_text
        );
        // Map HTTP semantics to existing Error variants where it matters.
        return Err(match status.as_u16() {
            401 | 403 => Error::UnauthorizedAccess,
            _ => Error::Internal,
        });
    }

    resp.json::<R>().await.map_err(|e| {
        tracing::error!("[tauri-web] {} {} decode error: {e}", method, url);
        Error::Internal
    })
}
