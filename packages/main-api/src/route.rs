use bdk::prelude::*;

use crate::controllers::{
    self,
    m2::noncelab::users::register_users::{
        RegisterUserResponse, register_users_by_noncelab_handler,
    },
    v2::users::logout::logout_handler,
};

use dto::{
    Result,
    aide::axum::routing::post_with,
    by_axum::axum::{Json, native_routing::post as npost},
};

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
                post_with(register_users_by_noncelab_handler, |op| {
                    op.response_with::<200, Json<RegisterUserResponse>, _>(|res| {
                        res.description("Success response")
                    })
                    .response_range_with::<4, Json<dto::Error>, _>(|res| {
                        res.description("Incorrect or invalid requests")
                            .example(dto::Error::UserAlreadyExists)
                    })
                    .summary("Register users by Noncelab")
                    .description("This endpoint allows you to register users by Noncelab.")
                }),
            ), // End of APIs
    )
}
