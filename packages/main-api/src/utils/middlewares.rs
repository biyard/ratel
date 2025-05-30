use bdk::prelude::*;
use std::str::FromStr;

use by_axum::axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode, header::AUTHORIZATION},
    middleware::Next,
};
use rest_api::Signature;

fn now() -> i64 {
    chrono::Utc::now().timestamp()
}

pub async fn authorization_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_value) = auth_header.to_str() {
            let mut auth_value = auth_value.split_whitespace();
            let (scheme, value) = (auth_value.next(), auth_value.next());
            match scheme.unwrap_or_default().to_lowercase().as_str() {
                "usersig" => {
                    let sig = verify_usersig(value)?;
                    req.extensions_mut().insert(Some(sig));
                    return Ok(next.run(req).await);
                }
                _ => {}
            }
        }
    }

    req.extensions_mut().insert(None::<Signature>);

    return Ok(next.run(req).await);
}

pub fn verify_usersig(value: Option<&str>) -> Result<Signature, StatusCode> {
    if let Some((timestamp, signature)) = value.unwrap_or_default().split_once(":") {
        let conf = crate::config::get();
        let parsed_timestamp: i64 = timestamp.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;
        if now() - parsed_timestamp >= 3600 {
            tracing::error!("Expired timestamp: {}", timestamp);
            return Err(StatusCode::UNAUTHORIZED);
        }

        let msg = format!("{}-{}", conf.signing_domain, timestamp);
        let sig = rest_api::Signature::from_str(signature).map_err(|e| {
            tracing::error!("Failed to parse signature: {}", e);
            StatusCode::UNAUTHORIZED
        })?;
        tracing::debug!("SignMessage: {}", msg);
        let address = sig.verify(&msg).map_err(|e| {
            tracing::error!("Failed to verify signature: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

        if address.is_empty() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        return Ok(sig);
    }

    Err(StatusCode::UNAUTHORIZED)
}
