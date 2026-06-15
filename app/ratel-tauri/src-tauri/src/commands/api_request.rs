//! Native HTTP proxy command for the WebView.
//!
//! The Tauri WebView page lives at `tauri://localhost` (iOS) / `http://tauri.localhost`
//! (Android); the API is `https://*.ratel.foundation` — a cross-site pair.
//! iOS WKWebView (ITP) blocks the cross-site session cookie, so requests made
//! by the in-WebView `fetch` lose the session right after login ("No session
//! found" on every subsequent call).
//!
//! Routing API calls through this native command uses a persistent reqwest
//! cookie jar in the app process instead — no WebView, no ITP, no CORS — so the
//! session cookie set by `/api/auth/login` is stored and sent on every later
//! request. Bonus: native reqwest negotiates HTTP/2 (not WKWebView's QUIC),
//! sidestepping the simulator's broken HTTP/3 too.

use std::sync::OnceLock;

use serde::Serialize;

/// One shared client for the whole app session so its cookie jar persists
/// across requests (login `Set-Cookie` → sent on later calls). `cookie_store`
/// requires reqwest's `cookies` feature.
fn client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .expect("build reqwest client")
    })
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub status: u16,
    pub body: String,
}

/// Perform an HTTP request natively and return the status + raw body.
///
/// Always returns `Ok` — the JS invoke bridge turns an `Err` into `null`, which
/// the WASM caller can't deserialize, so transport failures come back as
/// `status: 0` with the error text in `body` and are handled like any non-2xx.
#[tauri::command]
pub async fn api_request(
    method: String,
    url: String,
    body: Option<String>,
) -> Result<ApiResponse, String> {
    let m = match reqwest::Method::from_bytes(method.as_bytes()) {
        Ok(m) => m,
        Err(e) => {
            return Ok(ApiResponse {
                status: 0,
                body: format!("bad method `{method}`: {e}"),
            });
        }
    };

    let mut req = client().request(m, &url);
    if let Some(b) = body {
        req = req
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(b);
    }

    match req.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let body = resp.text().await.unwrap_or_default();
            Ok(ApiResponse { status, body })
        }
        Err(e) => Ok(ApiResponse {
            status: 0,
            body: format!("send error: {e}"),
        }),
    }
}
