pub mod assembly_members;
use bdk::prelude::*;

use by_axum::{
    auth::Authorization,
    axum::{
        body::Body,
        extract::Request,
        http::Response,
        middleware::{self, Next},
    },
};
use dto::*;
use reqwest::StatusCode;

#[derive(Clone, Debug)]
pub struct MenaceController {}

impl MenaceController {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .nest(
                "/assembly-members",
                assembly_members::AssemblyMemberControllerM1::route(pool)?,
            )
            .layer(middleware::from_fn(authorize_organization)))
    }
}

pub async fn authorize_organization(
    req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, StatusCode> {
    tracing::debug!("Authorization middleware");
    let auth = req.extensions().get::<Option<Authorization>>();
    if auth.is_none() {
        tracing::debug!("No Authorization header");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth = auth.unwrap();

    if auth.is_none() {
        tracing::debug!("No Authorization header");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth = auth.clone().unwrap();

    if auth != Authorization::SecretApiKey {
        tracing::debug!("Authorization header is not Secret");
        return Err(StatusCode::UNAUTHORIZED);
    }

    return Ok(next.run(req).await);
}
