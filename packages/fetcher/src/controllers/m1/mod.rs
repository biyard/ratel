// mod assembly_members;
// mod bills;
// mod ch;
// mod eu;
// mod hk;
// mod us;

mod log;
mod spaces;

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
use log::logs;
use reqwest::StatusCode;

pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
    Ok(by_axum::axum::Router::new()
        // .nest(
        //     "/bills",
        //     bills::BillWriterController::new(pool.clone())
        //         .await
        //         .route()?,
        // )
        .nest("/logs", logs::LogController::new(pool.clone()).route()?)
        // .nest(
        //     "/assembly-members",
        //     assembly_members::AssemblyMemberController::new(pool).route()?,
        // )
        // .nest(
        //     "/us/bills",
        //     us::bills::USBillWriterController::new(pool.clone())
        //         .await
        //         .route()?,
        // )
        // .nest(
        //     "/hk/bills",
        //     hk::bills::HKBillWriterController::new(pool.clone())
        //         .await
        //         .route()?,
        // )
        // .nest(
        //     "/ch/bills",
        //     ch::bills::CHBillWriterController::new(pool.clone())
        //         .await
        //         .route()?,
        // )
        // .nest(
        //     "/eu/bills",
        //     eu::bills::EUBillWriterController::new(pool.clone())
        //         .await
        //         .route()?,
        // )
        .nest(
            "/spaces",
            spaces::SpaceController::new(pool.clone()).await.route()?,
        )
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
