use bdk::prelude::*;

use crate::controllers::{
    self,
    m2::noncelab::users::register_users::{
        RegisterUserResponse, register_users_by_noncelab_handler,
    },
    v2::telegram::subscribe::telegram_subscribe_handler,
    v2::users::logout::logout_handler,
};
use by_axum::axum;
use dto::Result;

use axum::native_routing::post as npost;

macro_rules! wrap_api {
    (
        $method:expr,
        $handler:expr,
        $success_ty:ty,
        $summary:expr,
        $description:expr,
    ) => {
        $method($handler, |op| {
            op.summary($summary)
                .description($description)
                .response_with::<200, axum::Json<$success_ty>, _>(|res| {
                    res.description("Success response")
                })
                .response_with::<400, axum::Json<dto::Error>, _>(|res| {
                    res.description("Incorrect or invalid requests")
                        .example(dto::Error::UserAlreadyExists)
                })
        })
    };
}

macro_rules! post_api {
    (
        $handler:expr,
        $success_ty:ty,
        $summary:expr,
        $description:expr,
    ) => {
        wrap_api!(
            axum::routing::post_with,
            $handler,
            $success_ty,
            $summary,
            $description,
        )
    };
}

pub async fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
    Ok(by_axum::axum::Router::new()
        .nest("/v1", controllers::v1::route(pool.clone()).await?)
        .nest(
            "/m1",
            controllers::m1::MenaceController::route(pool.clone())?,
        )
        .native_route("/v2/users/logout", npost(logout_handler))
        // Admin APIs
        .route(
            "/v2/telegram/subscribe",
            post_api!(
                telegram_subscribe_handler,
                (),
                "Subscribe to Telegram",
                "This endpoint allows users to subscribe to Telegram notifications.",
            )
            .with_state(pool.clone()),
        )
        .route(
            "/m2/noncelab/users",
            post_api!(
                register_users_by_noncelab_handler,
                RegisterUserResponse,
                "Register users by Noncelab",
                "This endpoint allows you to register users by Noncelab.\n\n**Authorization header required**\n\n`Authorization: Bearer <token>`",
            )
            .with_state(pool.clone()),
        ))
}
