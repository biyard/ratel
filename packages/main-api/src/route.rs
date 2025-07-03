use bdk::prelude::*;

use crate::controllers::{self, v2::users::logout::logout_handler};

use dto::{Result, by_axum::axum::native_routing::post};

pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
    Ok(by_axum::axum::Router::new()
        .nest("/v1", controllers::v1::route(pool.clone()).await?)
        .nest(
            "/m1",
            controllers::m1::MenaceController::route(pool.clone())?,
        )
        .native_route("/v2/users/logout", post(logout_handler)))
}
