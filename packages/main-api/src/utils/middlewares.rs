use std::str::FromStr;

use by_axum::{
    axum::{
        body::Body,
        extract::Request,
        http::{header::AUTHORIZATION, Response, StatusCode},
        middleware::Next,
    },
    log::root,
};

fn now() -> i64 {
    chrono::Utc::now().timestamp()
}

pub async fn authorization_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let log = root().new(slog::o!("middleware" => "authorization_middleware"));

    if let Some(auth_header) = req.headers().get(AUTHORIZATION) {
        if let Ok(auth_value) = auth_header.to_str() {
            let mut auth_value = auth_value.split_whitespace();
            let (scheme, value) = (auth_value.next(), auth_value.next());
            if scheme.unwrap_or_default() == "UserSig" {
                if let Some((timestamp, signature)) = value.unwrap_or_default().split_once(":") {
                    let conf = crate::config::get();
                    let parsed_timestamp: i64 =
                        timestamp.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;
                    if now() - parsed_timestamp >= 3600 {
                        slog::error!(log, "Expired timestamp: {}", timestamp);
                        return Err(StatusCode::UNAUTHORIZED);
                    }

                    let msg = format!("{}-{}", conf.domain, timestamp);
                    let sig = rest_api::Signature::from_str(signature).map_err(|e| {
                        slog::error!(log, "Failed to parse signature: {}", e);
                        StatusCode::UNAUTHORIZED
                    })?;
                    slog::debug!(root(), "SignMessage: {}", msg);
                    let address = sig.verify(&msg).map_err(|e| {
                        slog::error!(log, "Failed to verify signature: {}", e);
                        StatusCode::UNAUTHORIZED
                    })?;

                    if address.is_empty() {
                        return Err(StatusCode::UNAUTHORIZED);
                    }

                    req.extensions_mut().insert(sig);
                    return Ok(next.run(req).await);
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
