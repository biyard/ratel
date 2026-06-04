//! Axum router for Launchpad callbacks. Routes are unauthenticated
//! (no session) and instead verified by HMAC signature, matching
//! Launchpad's `demo_preview/server.rs` contract.

#![cfg(feature = "server")]

use crate::common::axum::{
    Router,
    body::Bytes,
    extract::Json,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
};
use crate::features::launchpad_partner::config::LaunchpadPartnerConfig;
use crate::features::launchpad_partner::controllers;
use crate::features::launchpad_partner::crypto::verify_signature;
use crate::features::launchpad_partner::error::PartnerError;
use crate::features::launchpad_partner::types::{DeductBody, HealthBody, LookupBody};
use serde::Serialize;

pub fn router() -> Router {
    Router::new()
        .route("/launchpad/health", post(health))
        .route("/launchpad/points/lookup", post(lookup))
        .route("/launchpad/points/deduct", post(deduct))
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

fn err_response(e: PartnerError) -> Response {
    let status = StatusCode::from_u16(e.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, Json(ErrorBody { error: e.to_string() })).into_response()
}

/// Verify HMAC + project id; returns the raw body parsed as T.
fn verify_and_parse<T: serde::de::DeserializeOwned>(
    headers: &HeaderMap,
    body: &Bytes,
    project_id_in_body: impl Fn(&T) -> String,
) -> Result<T, PartnerError> {
    let cfg = LaunchpadPartnerConfig::default();
    let raw = std::str::from_utf8(body.as_ref()).map_err(|_| PartnerError::Server)?;
    let ts = header(headers, "x-launchpad-timestamp");
    let sig = header(headers, "x-launchpad-signature");
    if !verify_signature(cfg.shared_secret, &ts, &sig, raw) {
        return Err(PartnerError::InvalidSignature);
    }
    let parsed: T = serde_json::from_str(raw).map_err(|_| PartnerError::Server)?;
    // Project id may arrive in a header or the body; require body match.
    let header_pid = header(headers, "x-launchpad-project-id");
    let body_pid = project_id_in_body(&parsed);
    let effective = if header_pid.is_empty() {
        body_pid
    } else {
        header_pid
    };
    if effective != cfg.project_id {
        return Err(PartnerError::ProjectMismatch);
    }
    Ok(parsed)
}

fn header(headers: &HeaderMap, name: &str) -> String {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string()
}

async fn health(headers: HeaderMap, body: Bytes) -> Response {
    match verify_and_parse::<HealthBody>(&headers, &body, |b| b.project_id.clone()) {
        Ok(_) => (StatusCode::OK, Json(controllers::health())).into_response(),
        Err(e) => err_response(e),
    }
}

async fn lookup(headers: HeaderMap, body: Bytes) -> Response {
    let parsed = match verify_and_parse::<LookupBody>(&headers, &body, |b| b.project_id.clone()) {
        Ok(p) => p,
        Err(e) => return err_response(e),
    };
    match controllers::lookup(&parsed.company_user_key).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(e) => err_response(e),
    }
}

async fn deduct(headers: HeaderMap, body: Bytes) -> Response {
    let parsed = match verify_and_parse::<DeductBody>(&headers, &body, |b| b.project_id.clone()) {
        Ok(p) => p,
        Err(e) => return err_response(e),
    };
    match controllers::deduct(&parsed).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(e) => err_response(e),
    }
}
