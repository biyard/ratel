use bdk::prelude::*;

use by_axum::axum::{
    body::Body,
    extract::Request,
    http::Response,
    middleware::{self, Next},
};
use reqwest::StatusCode;

use crate::AppState;

pub mod memberships;

pub fn route(app_state: AppState) -> crate::Result<by_axum::axum::Router> {
    Ok(axum::Router::new()
        .nest("/memberships", memberships::route()?)
        .layer(middleware::from_fn(authorize_service_admin))
        .with_state(app_state))
}

pub async fn authorize_service_admin(
    req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, StatusCode> {
    tracing::debug!("Authorization middleware");
    // let user: User = req.extract_parts_with_state(&state).await.map_err(|err| {
    //     tracing::error!("Failed to extract user from request: {}", err);
    //     StatusCode::UNAUTHORIZED
    // })?;

    return Ok(next.run(req).await);
}
