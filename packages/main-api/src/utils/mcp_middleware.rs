use bdk::prelude::*;
use by_axum::axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use dto::by_axum::auth::verify_jwt;

pub async fn mcp_middleware(mut req: Request<Body>, next: Next) -> Response {
    // Extract the access token from the Authorization header
    let auth_header = req.headers().get("Authorization");
    let token = match auth_header {
        Some(header) => {
            let header_str = header.to_str().unwrap_or("");
            if let Some(stripped) = header_str.strip_prefix("Bearer ") {
                stripped.to_string()
            } else {
                return StatusCode::UNAUTHORIZED.into_response();
            }
        }
        None => {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    };
    match verify_jwt(Some(&token)) {
        Ok(auth) => {
            req.extensions_mut().insert(auth);
            next.run(req).await
        }
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}
