use std::sync::Arc;

use bdk::prelude::*;

use crate::{
    controllers::{
        self,
        m2::noncelab::users::register_users::{
            RegisterUserResponse, register_users_by_noncelab_handler,
        },
        v2::{
            bookmarks::{
                add_bookmark::add_bookmark_handler, list_bookmarks::get_bookmarks_handler,
                remove_bookmark::remove_bookmark_handler,
            },
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
            dashboards::get_dashboard::get_dashboard_handler,
            documents::{
                extract_passport_info::{PassportHandlerState, extract_passport_info_handler},
                upload_private_image::{UploadPrivateImageState, upload_private_image_handler},
            },
            industries::{industry::list_industries_handler, select_topic::select_topics_handler},
            networks::{
                follow::follow_handler, network::list_networks_handler,
                search::list_networks_by_keyword_handler,
            },
            notifications::mark_all_read::mark_all_notifications_read_handler,
            oracles::create_oracle::create_oracle_handler,
            spaces::{delete_space::delete_space_handler, get_my_space::get_my_space_controller},
            telegram::subscribe::telegram_subscribe_handler,
            users::{find_user::find_user_handler, logout::logout_handler},
        },
        well_known::get_did_document::get_did_document_handler,
    },
    utils::{
        aws::{BedrockClient, RekognitionClient, S3Client, TextractClient},
        sqs_client::SqsClient,
    },
};
use by_axum::axum;
use dto::Result;

use axum::native_routing::get as nget;
use axum::native_routing::post as npost;
use axum::routing::{get_with, post_with};

// macro_rules! wrap_api {
//     (
//         $method:expr,
//         $handler:expr,
//         $success_ty:ty,
//         $summary:expr,
//         $description:expr
//     ) => {
//         $method($handler, |op| {
//             op.summary($summary)
//                 .description($description)
//                 .response_with::<200, axum::Json<$success_ty>, _>(|res| {
//                     res.description("Success response")
//                 })
//                 .response_with::<400, axum::Json<dto::Error>, _>(|res| {
//                     res.description("Incorrect or invalid requests")
//                         .example(dto::Error::UserAlreadyExists)
//                 })
//         })
//     };
// }

// macro_rules! post_api {
//     (
//         $handler:expr,
//         $success_ty:ty,
//         $summary:expr,
//         $description:expr
//     ) => {
//         wrap_api!(
//             axum::routing::post_with,
//             $handler,
//             $success_ty,
//             $summary,
//             $description
//         )
//     };
// }

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
    sqs_client: Arc<SqsClient>,
    bedrock_client: BedrockClient,
    rek_client: RekognitionClient,
    textract_client: TextractClient,
    _metadata_s3_client: S3Client,
    private_s3_client: S3Client,
) -> Result<by_axum::axum::Router> {
    Ok(by_axum::axum::Router::new()
        .nest("/v1", controllers::v1::route(pool.clone()).await?)
        .nest(
            "/m1",
            controllers::m1::MenaceController::route(pool.clone())?,
        )
        .native_route("/v2/users/logout", npost(logout_handler))
        .route(
            "/v2/industries/select-topics",
            post_with(
                select_topics_handler,
                api_docs!("Select Topics", "Select interesting topics"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/industries",
            get_with(
                list_industries_handler,
                api_docs!("List Industries", "List industry types"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/networks",
            get_with(
                list_networks_handler,
                api_docs!(
                    "List Networks",
                    "List networks based on recommendation algorithm"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/networks/search",
            get_with(
                list_networks_by_keyword_handler,
                api_docs!(
                    "List Networks by keyword",
                    "List networks by search keyword"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/networks/follow",
            post_with(
                follow_handler,
                api_docs!("Follow Users", "Follow users with follower IDs"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/bookmarks/add",
            post_with(
                add_bookmark_handler,
                api_docs!("Add Bookmarks", "Add Feed Bookmarks with user ID"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/bookmarks/remove",
            post_with(
                remove_bookmark_handler,
                api_docs!("Remove Bookmarks", "Remove Feed Bookmarks with user ID"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/bookmarks",
            get_with(
                get_bookmarks_handler,
                api_docs!("List Bookmarks", "Retrieve bookmarked feed with user ID"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/users",
            get_with(
                find_user_handler,
                api_docs!(
                    "Get User",
                    "Retrieve users with username or phone number or email"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/dashboards",
            get_with(
                get_dashboard_handler,
                api_docs!("Get Dashboards", "Retrieve dashboard in a service"),
            )
            .with_state(pool.clone()),
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
            .with_state((pool.clone(), sqs_client.clone())),
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
            "/v2/my-spaces",
            get_with(
                get_my_space_controller,
                api_docs!("Get My Space", "Retrieve a spaces"),
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
            "/v2/documents",
            get_with(
                upload_private_image_handler,
                api_docs!(
                    "Get S3 Presigned URL for Uploading Private Image",
                    r#"This endpoint provides presigned URLs for uploading private images to S3.
                **Authorization header required**
                `Authorization: Bearer <token>`"#
                ),
            )
            .with_state(UploadPrivateImageState {
                s3_client: private_s3_client.clone(),
            }),
        )
        .route(
            "/v2/documents/passport",
            post_with(
                extract_passport_info_handler,
                api_docs!(
                    "Extract Information from Passport Image",
                    r#"This endpoint allows you to extract passport information from an image.
                
                **Authorization header required**"#
                ),
            )
            .with_state(PassportHandlerState {
                pool: pool.clone(),
                bedrock_client: bedrock_client.clone(),
                rek_client: rek_client.clone(),
                textract_client: textract_client.clone(),
                s3_client: private_s3_client.clone(),
            }),
        )
        .route(
            "/v2/telegram/subscribe",
            post_with(
                telegram_subscribe_handler,
                api_docs!(
                    "Subscribe to Telegram",
                    "This endpoint allows users to subscribe to Telegram notifications."
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/spaces/:space_id/delete",
            post_with(
                delete_space_handler,
                api_docs!(
                    (),
                    "Delete Space",
                    "Delete a space and all its related resources after confirmation"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/notifications/mark-all-read",
            post_with(
                mark_all_notifications_read_handler,
                api_docs!(
                    "Mark All Notifications Read",
                    "Mark all notifications as read for the authenticated user."
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/m2/noncelab/users",
            post_with(
                register_users_by_noncelab_handler,
                api_docs!(
                    RegisterUserResponse,
                    "Register users by Noncelab",
                    r#"This endpoint allows you to register users by Noncelab.
                    
                    **Authorization header required**
                    
                    `Authorization: Bearer <token>`"#
                ),
            )
            .with_state(pool.clone()),
        )
        .native_route("/.well-known/did.json", nget(get_did_document_handler)))
}
