use std::sync::Arc;

use bdk::prelude::*;

use crate::{
    controllers::{
        self,
        m2::noncelab::users::register_users::{
            RegisterUserResponse, register_users_by_noncelab_handler,
        },
        v2::{
            dagits::{
                add_oracle::add_oracle_handler,
                artworks::{
                    create_artwork::create_artwork_handler,
                    get_artwork_certificate::get_artwork_certificate_handler,
                    get_artwork_detail::get_artwork_detail_handler,
                },
                consensus::{
                    create_consensus::create_consensus_handler, vote::consensus_vote_handler,
                },
                get_dagit::get_dagit_handler,
            },
            industries::{industry::list_industries_handler, select_topic::select_topics_handler},
            networks::{
                follow::follow_handler, network::list_networks_handler,
                search::list_networks_by_keyword_handler,
            },
            oracles::create_oracle::create_oracle_handler,
            telegram::subscribe::telegram_subscribe_handler,
            users::{find_user::find_user_handler, logout::logout_handler},
        },
    },
    utils::rds_client::RdsClient,
};
use by_axum::axum;
use dto::Result;

use axum::native_routing::get as nget;
use axum::native_routing::post as npost;
use axum::routing::{get_with, post_with};

macro_rules! wrap_api {
    (
        $method:expr,
        $handler:expr,
        $success_ty:ty,
        $summary:expr,
        $description:expr
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
        $description:expr
    ) => {
        wrap_api!(
            axum::routing::post_with,
            $handler,
            $success_ty,
            $summary,
            $description
        )
    };
}

macro_rules! api_docs {
    ($success_ty:ty, $summary:expr, $description:expr) => {
        |op| {
            op.summary($summary)
                .description($description)
                .response_with::<200, axum::Json<$success_ty>, _>(|res| {
                    res.description("Success response")
                })
                .response_with::<400, axum::Json<dto::Error>, _>(|res| {
                    res.description("Incorrect or invalid requests")
                        .example(dto::Error::UserAlreadyExists)
                })
        }
    };

    ($summary:expr, $description:expr) => {
        |op| {
            op.summary($summary)
                .description($description)
                .response_with::<400, axum::Json<dto::Error>, _>(|res| {
                    res.description("Incorrect or invalid requests")
                        .example(dto::Error::UserAlreadyExists)
                })
        }
    };
}

pub async fn route(
    pool: sqlx::Pool<sqlx::Postgres>,
    rds_client: Arc<RdsClient>,
) -> Result<by_axum::axum::Router> {
    Ok(by_axum::axum::Router::new()
        .nest("/v1", controllers::v1::route(pool.clone()).await?)
        .nest(
            "/m1",
            controllers::m1::MenaceController::route(pool.clone())?,
        )
        .native_route("/v2/users/logout", npost(logout_handler))
        .native_route(
            "/v2/industries/select-topics",
            npost(select_topics_handler).with_state(pool.clone()),
        )
        .native_route(
            "/v2/industries",
            nget(list_industries_handler).with_state(pool.clone()),
        )
        .native_route(
            "/v2/networks",
            nget(list_networks_handler).with_state(pool.clone()),
        )
        .native_route(
            "/v2/networks/search",
            nget(list_networks_by_keyword_handler).with_state(pool.clone()),
        )
        .native_route(
            "/v2/networks/follow",
            npost(follow_handler).with_state(pool.clone()),
        )
        .native_route(
            "/v2/users",
            nget(find_user_handler).with_state(pool.clone()),
        )
        .route(
            "/v2/dagits/:space_id",
            get_with(
                get_dagit_handler,
                api_docs!("Get Dagit by space ID", "Retrieve dagit in a space"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/dagits/:space_id/oracles",
            post_with(
                add_oracle_handler,
                api_docs!("Add Oracle", "Add a new oracle to a dagit"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/dagits/:space_id/artworks",
            post_with(
                create_artwork_handler,
                api_docs!("Create Artwork", "Create a new artwork for a dagit"),
            )
            .with_state((pool.clone(), rds_client.clone())),
        )
        .route(
            "/v2/dagits/:space_id/consensus",
            post_with(
                create_consensus_handler,
                api_docs!("Start Dagit Consensus", "Start a new consensus for a dagit"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/artworks/:artwork_id",
            get_with(
                get_artwork_detail_handler,
                api_docs!("Get Artwork", "Retrieve a specific artwork"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/dagits/:space_id/artworks/:artwork_id/vote",
            post_with(
                consensus_vote_handler,
                api_docs!(
                    "Vote on Dagit Consensus",
                    "Submit a vote for a specific dagit consensus"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/artworks/:artwork_id/certificate",
            get_with(
                get_artwork_certificate_handler,
                api_docs!("Get Artwork", "Retrieve a specific artwork"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/m2/oracles",
            post_with(
                create_oracle_handler,
                api_docs!("Create Oracle", "Create a new oracle"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/telegram/subscribe",
            post_api!(
                telegram_subscribe_handler,
                (),
                "Subscribe to Telegram",
                "This endpoint allows users to subscribe to Telegram notifications."
            )
            .with_state(pool.clone()),
        )
        .route(
            "/m2/noncelab/users",
            post_api!(
                register_users_by_noncelab_handler,
                RegisterUserResponse,
                "Register users by Noncelab",
                //NOTE: This text blocking `rustfmt`
                concat!(
                    "This endpoint allows you to register users by Noncelab.\n\n",
                    "**Authorization header required**\n\n",
                    "`Authorization: Bearer <token>`"
                )
            )
            .with_state(pool.clone()),
        ))
}
