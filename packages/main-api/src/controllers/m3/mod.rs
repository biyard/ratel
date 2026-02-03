use bdk::prelude::*;

use by_axum::axum::{
    body::Body,
    extract::{FromRequestParts, Request, State},
    http::Response,
    middleware::{self, Next},
};
use reqwest::StatusCode;

use crate::{AppState, models::dynamo_tables::main::user::User};

pub mod admin;
mod attribute_codes;
pub mod memberships;
pub mod migrations;
pub mod rewards;

pub fn route() -> crate::Result<by_axum::axum::Router> {
    let app_state = AppState::default();
    Ok(axum::Router::new()
        .nest("/memberships", memberships::route()?)
        .nest("/attribute-codes", attribute_codes::route()?)
        .nest("/admin", admin::route()?)
        .nest("/rewards", rewards::route()?)
        .nest("/migrations", migrations::route()?)
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            authorize_service_admin,
        ))
        .with_state(app_state))
}

pub async fn authorize_service_admin(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, StatusCode> {
    tracing::debug!("ServiceAdmin authorization middleware");

    // Split the request into parts to extract the user
    let (mut parts, body) = req.into_parts();

    // Try to extract the user from the request
    let user_result = User::from_request_parts(&mut parts, &state).await;

    // Reconstruct the request
    req = Request::from_parts(parts, body);

    match user_result {
        Ok(user) => {
            if user.is_admin() {
                tracing::debug!("User is admin, allowing request to /m3/*");
                Ok(next.run(req).await)
            } else {
                tracing::warn!(
                    "User {} is not admin, denying request to /m3/*",
                    user.username
                );
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        Err(e) => {
            tracing::warn!("Failed to extract user for /m3/* request: {:?}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
