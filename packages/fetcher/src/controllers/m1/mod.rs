// mod assembly_members;
mod bills;

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
use by_types::Role;
use dto::*;
use reqwest::StatusCode;

pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
    Ok(by_axum::axum::Router::new()
        .nest(
            "/bills",
            bills::BillWriterController::new(pool.clone()).route()?,
        )
        // .nest(
        //     "/assembly-members",
        //     assembly_members::AssemblyMemberController::new(pool).route()?,
        // )
        .layer(middleware::from_fn(authorize_router)))
}

pub async fn authorize_router(
    req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, StatusCode> {
    tracing::debug!("Authorization m1 middleware");
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

    match auth {
        Authorization::Bearer { claims } => {
            if claims.role != Role::Admin {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
        Authorization::ServerKey => {
            tracing::info!("call by server key authorization");
        }
        _ => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    return Ok(next.run(req).await);
}
