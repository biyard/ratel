//! Native S3 upload command for the WebView.
//!
//! Image/file uploads hand the WebView an S3 **presigned PUT URL** and the JS
//! side `fetch`es the bytes straight to `s3.<region>.amazonaws.com`. From the
//! Tauri WebView that fetch is cross-origin (`tauri://localhost` →
//! `s3.amazonaws.com`), so the browser fires a CORS **preflight** the S3 bucket
//! doesn't allow → `403 Preflight response is not successful`, and the upload
//! never happens (logo upload, paste-image, etc.).
//!
//! Routing the PUT through this native command sidesteps CORS entirely: the
//! request originates from the app process (reqwest), not the WebView, so no
//! preflight is involved. The presigned URL is self-authenticating, so no
//! cookie jar is needed here (unlike `api_request`).

use base64::Engine;

use super::api_request::ApiResponse;

/// PUT raw bytes to a presigned S3 URL natively.
///
/// `body_b64` is the file contents base64-encoded (the JS bridge can't pass raw
/// binary cleanly). Always returns `Ok` — transport/decoding failures come back
/// as `status: 0` with the error in `body`, matching `api_request`'s contract
/// so the WASM caller handles them like any non-2xx.
#[tauri::command]
pub async fn s3_put_object(
    url: String,
    content_type: String,
    body_b64: String,
) -> Result<ApiResponse, String> {
    let bytes = match base64::engine::general_purpose::STANDARD.decode(body_b64.as_bytes()) {
        Ok(b) => b,
        Err(e) => {
            return Ok(ApiResponse {
                status: 0,
                body: format!("base64 decode error: {e}"),
            });
        }
    };

    let client = reqwest::Client::new();
    let mut req = client.put(&url).body(bytes);
    if !content_type.is_empty() {
        req = req.header(reqwest::header::CONTENT_TYPE, content_type);
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
