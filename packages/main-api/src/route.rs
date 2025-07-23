use bdk::prelude::*;

use crate::controllers::{
    self, m2::noncelab::users::register_users::register_users_by_noncelab_handler,
    v2::users::logout::logout_handler,
};

use dto::{Result, by_axum::axum::native_routing::post as npost, by_axum::axum::routing::post};

pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
    Ok(
        by_axum::axum::Router::new()
            .nest("/v1", controllers::v1::route(pool.clone()).await?)
            .nest(
                "/m1",
                controllers::m1::MenaceController::route(pool.clone())?,
            )
            .native_route("/v2/users/logout", npost(logout_handler))
            // Admin APIs
            .route(
                "/m2/noncelab/users",
                post(register_users_by_noncelab_handler),
            ), // End of APIs
    )
}
