use crate::utils::aws::S3Client;
use crate::{AppState, Error, models::user::User, types::*};
use axum::extract::State;
use bdk::prelude::*;

// pub mod did;
pub mod networks;
pub mod notifications;
mod payments;

pub mod promotions {
    pub mod get_top_promotion;
}
pub mod me;

pub mod users;

pub mod assets {
    pub mod complete_multipart_upload;
    pub mod get_put_multi_object_uri;
    pub mod get_put_object_uri;
}

pub mod auth;
pub mod posts;
pub mod presence;
pub mod reports;
pub mod rewards;
pub mod spaces;
pub mod teams;

use crate::*;
use crate::{
    assets::{
        complete_multipart_upload::complete_multipart_upload,
        get_put_multi_object_uri::get_put_multi_object_uri, get_put_object_uri::get_put_object_uri,
    },
    promotions::get_top_promotion::get_top_promotion_handler,
    spaces::{
        create_space::create_space_handler, delete_space::delete_space_handler, get_space_handler,
        list_spaces_handler, update_space::update_space_handler,
    },
    utils::{
        aws::{DynamoClient, SesClient},
        telegram::ArcTelegramBot,
    },
};
use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::axum::*;

pub struct RouteDeps {
    pub dynamo_client: DynamoClient,
    pub ses_client: SesClient,
    pub bot: Option<ArcTelegramBot>,
    pub s3: S3Client,
}

pub fn route(bot: Option<ArcTelegramBot>) -> Result<Router> {
    Ok(Router::new()
        .nest("/payments", payments::route()?)
        .nest("/presence", presence::route()?)
        // .nest("/did", did::route()?)
        .nest(
            "/networks",
            Router::new().route(
                "/suggestions",
                get(crate::controllers::v3::networks::get_suggestions_handler),
            ),
        )
        .route("/promotions/top", get(get_top_promotion_handler))
        .nest("/reports", reports::route()?)
        .nest("/me", me::route()?)
        .nest("/users", users::route()?)
        .nest("/posts", posts::route()?)
        .nest("/auth", auth::route()?)
        .nest("/teams", teams::route()?)
        .nest("/spaces", spaces::route()?)
        .nest("/rewards", rewards::route()?)
        .nest("/notifications", notifications::route()?)
        .nest(
            "/assets",
            Router::new()
                .route("/", get(get_put_object_uri))
                .route("/multiparts", get(get_put_multi_object_uri))
                .route("/multiparts", post(complete_multipart_upload)),
        )
        .layer(Extension(bot))
        .with_state(AppState::default()))
}
