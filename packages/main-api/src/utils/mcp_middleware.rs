use bdk::prelude::*;
use by_axum::axum::{
    body::Body,
    http::{
        HeaderValue, Request, StatusCode,
        header::{AUTHORIZATION, WWW_AUTHENTICATE},
    },
    middleware::Next,
    response::{IntoResponse, Response},
};
use by_axum::auth::verify_jwt;

pub async fn mcp_middleware(mut req: Request<Body>, next: Next) -> Response {
    // Extract the access token from the Authorization header
    let auth_header = req.headers().get(AUTHORIZATION);
    let token = match auth_header {
        Some(header) => {
            let header_str = match header.to_str() {
                Ok(s) => s,
                Err(_) => {
                    let mut res = StatusCode::UNAUTHORIZED.into_response();
                    res.headers_mut().insert(
                        WWW_AUTHENTICATE,
                        HeaderValue::from_static(r#"Bearer realm="mcp", error="invalid_token""#),
                    );
                    return res;
                }
            };
            match header_str.split_once(' ') {
                Some((scheme, tok)) if scheme.eq_ignore_ascii_case("bearer") && !tok.is_empty() => {
                    tok
                }
                _ => {
                    let mut res = StatusCode::UNAUTHORIZED.into_response();
                    res.headers_mut().insert(
                        WWW_AUTHENTICATE,
                        HeaderValue::from_static(r#"Bearer realm="mcp", error="invalid_token""#),
                    );
                    return res;
                }
            }
        }

        None => {
            let mut res = StatusCode::UNAUTHORIZED.into_response();
            res.headers_mut().insert(
                WWW_AUTHENTICATE,
                HeaderValue::from_static(r#"Bearer realm="mcp", error="invalid_request""#),
            );
            return res;
        }
    };
    match verify_jwt(Some(token)) {
        Ok(auth) => {
            req.extensions_mut().insert(auth);
            next.run(req).await
        }
        Err(_) => StatusCode::UNAUTHORIZED.into_response(),
    }
}
