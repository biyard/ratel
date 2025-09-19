use std::sync::Arc;

use bdk::prelude::*;
use tower_http::trace::TraceLayer;
use tracing::Level;

use by_axum::{
    auth::Authorization,
    axum::{
        body::Body,
        extract::Request,
        http::Response,
        middleware::{self, Next},
    },
};
use reqwest::StatusCode;

use crate::{
    config,
    controllers::{
        self,
        m2::{
            binances::get_merchant_balance::binance_merchant_balance_handler,
            migration::postgres_to_dynamodb::{migrate_users_handler, migration_stats_handler},
            noncelab::users::register_users::{
                RegisterUserResponse, register_users_by_noncelab_handler,
            },
        },
        v2::{
            news::list_news::list_news_handler,
            binances::{
                binance_webhook::binance_webhook_handler,
                create_subscription::create_subscription_handler, unsubscribe::unsubscribe_handler,
            },
            bookmarks::{
                add_bookmark::add_bookmark_handler, list_bookmarks::get_bookmarks_handler,
                remove_bookmark::remove_bookmark_handler,
            },
            connections::{
                follow::connection_follow_handler, network::list_connections_handler,
                search::list_connections_by_keyword_handler,
            },
            conversations::{
                add_conversations::create_conversation_handler,
                get_conversation_by_id::get_conversation_by_id_handler,
                get_conversations::get_conversations_handler,
                messages::{
                    add_messages::add_message_handler, clear_message::clear_message_handler,
                    get_messages::get_messages_handler, poll_messages::poll_messages_handler,
                },
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
                extract_medical_info::{MedicalHandlerState, extract_medical_info_handler},
                extract_passport_info::{PassportHandlerState, extract_passport_info_handler},
                upload_private_image::{UploadPrivateImageState, upload_private_image_handler},
            },
            industries::{industry::list_industries_handler, select_topic::select_topics_handler},
            networks::{
                accept_invitation::accept_invitation_handler,
                accept_suggestion::accept_suggestion_handler, list_networks::list_networks_handler,
                reject_invitation::reject_invitation_handler,
                reject_suggestion::reject_suggestion_handler,
            },
            notifications::{
                get_notifications::get_notifications_handler,
                mark_all_read::mark_all_notifications_read_handler,
            },
            oauth::{
                approve::approve_handler, authorize::authorize_handler,
                oauth_authorization_server::oauth_authorization_server_handler,
                register::register_handler, token::token_handler,
            },
            oracles::create_oracle::create_oracle_handler,
            permissions::has_team_permission::has_team_permission_handler,
            posts::{
                get_post::get_post_handler, list_posts::list_posts_handler,
                update_post::update_post_handler,
            },
            spaces::{delete_space::delete_space_handler, get_my_space::get_my_space_controller},
            telegram::{
                get_telegram_info::get_telegram_info_handler,
                verify_telegram_raw::verify_telegram_raw_handler,
            },
            themes::change_theme::change_theme_handler,
            users::{
                connect_telegram::connect_telegram_handler, find_user::find_user_handler,
                logout::logout_handler,
            },
        },
        v3::users::{
            email_login::v3_login_with_password_handler, email_signup::v3_email_signup_handler,
            request_verification_code::request_verification_code_handler,
            verify_email::email_verification_handler,
        },
        well_known::get_did_document::get_did_document_handler,
        wg::get_home::get_home_handler,
    },
    utils::{
        aws::{BedrockClient, RekognitionClient, S3Client, TextractClient},
        sqs_client::SqsClient,
        telegram::TelegramBot,
    },
};
use by_axum::axum;
use dto::{Result, by_axum::axum::Extension};

use axum::native_routing::get as nget;
use axum::native_routing::post as npost;
use axum::routing::{get_with, post_with};

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
    bot: Option<TelegramBot>,
) -> Result<by_axum::axum::Router> {
    let conf = config::get();

    let dynamo_conf = aws_sdk_dynamodb::config::Config::builder()
        .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
            conf.aws.access_key_id,
            conf.aws.secret_access_key,
            None,
            None,
            "dynamo",
        ))
        .behavior_version_latest()
        .build();

    let dynamo_client = aws_sdk_dynamodb::Client::from_conf(dynamo_conf);
    let dynamo_client = Arc::new(dynamo_client);

    Ok(by_axum::axum::Router::new()
        // For Admin routes
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
        // Migration routes
        .route(
            "/m2/migration/users",
            get_with(
                migrate_users_handler,
                api_docs!(
                    "Migrate Users",
                    r#"Migrate users from PostgreSQL to DynamoDB.
                    
                    **Query Parameters:**
                    - `batch_size`: Number of users to migrate (default: 100, max: 1000)
                    - `start_user_id`: Starting user ID for batch migration
                    - `user_id`: Specific user ID to migrate (overrides batch)
                    - `dry_run`: Validate migration without writing to DynamoDB
                    
                    **Authorization header required**
                    
                    `Authorization: Bearer <token>`"#
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/m2/migration/stats",
            get_with(
                migration_stats_handler,
                api_docs!(
                    "Migration Statistics",
                    r#"Get migration statistics including:
                    - Total users in PostgreSQL
                    - Total users in DynamoDB
                    - Pending migration count
                    - Last migrated user ID
                    
                    **Authorization header required**
                    
                    `Authorization: Bearer <token>`"#
                ),
            )
            .with_state(pool.clone()),
        )
        .layer(middleware::from_fn(authorize_admin))
        // For user routes
        .nest(
            "/v1",
            controllers::v1::route(pool.clone())
                .await?
                .layer(Extension(bot.map(Arc::new))),
        )
        .native_route("/v2/users/logout", npost(logout_handler))
        .route(
            "/v2/binances/subscriptions",
            post_with(
                create_subscription_handler,
                api_docs!(
                    "Create Subscription",
                    "Create subscription in ratel and get a QR code"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/binances/webhooks",
            post_with(
                binance_webhook_handler,
                api_docs!(
                    "Create Webhook",
                    "Create binance payment api webhook handler"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/binances/unsubscribe",
            post_with(
                unsubscribe_handler,
                api_docs!("Unsubscribe Service", "Unsubscribe service in ratel"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/conversations",
            post_with(
                create_conversation_handler,
                api_docs!(
                    "Create Conversation",
                    "Create a new group or channel conversation"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/conversations",
            get_with(
                get_conversations_handler,
                api_docs!(
                    "Get Conversations",
                    "Retrieve user's conversations with pagination"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/conversations/:conversation_id",
            get_with(
                get_conversation_by_id_handler,
                api_docs!(
                    "Get Conversation by ID",
                    "Retrieve a specific conversation by ID"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/conversations/:conversation_id/messages",
            post_with(
                add_message_handler,
                api_docs!("Add Message", "Add a new message to a conversation"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/conversations/:conversation_id/messages",
            get_with(
                get_messages_handler,
                api_docs!(
                    "Get Messages",
                    "Retrieve messages from a conversation with pagination"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/conversations/:conversation_id/messages/poll",
            get_with(
                poll_messages_handler,
                api_docs!(
                    "Poll Messages",
                    "Long poll for new messages in a conversation"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/messages/:message_id/clear",
            post_with(
                clear_message_handler,
                api_docs!(
                    "Clear Message",
                    "Clear the content of a message (soft delete)"
                ),
            )
            .with_state(pool.clone()),
        )
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
            "/v2/news",
            get_with(
                list_news_handler,
                api_docs!(
                    "List News",
                    "List latest news ordered by created_at descending; supports optional limit query param"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/connections",
            get_with(
                list_connections_handler,
                api_docs!(
                    "List Connections",
                    "List connections based on recommendation algorithm"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/connections/search",
            get_with(
                list_connections_by_keyword_handler,
                api_docs!(
                    "List Connections by keyword",
                    "List connections by search keyword"
                ),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/connections/follow",
            post_with(
                connection_follow_handler,
                api_docs!("Follow Users", "Follow users with follower IDs"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/networks",
            get_with(
                list_networks_handler,
                api_docs!("List Networks", "List Networks with user ID"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/networks/invitations/accept",
            post_with(
                accept_invitation_handler,
                api_docs!("Accept invitation", "Accept Invitation from followee ID"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/networks/invitations/reject",
            post_with(
                reject_invitation_handler,
                api_docs!("Reject invitation", "Reject Invitation from followee ID"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/networks/suggestions/accept",
            post_with(
                accept_suggestion_handler,
                api_docs!("Accept suggestion", "Accept Suggestion from followee ID"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/networks/suggestions/reject",
            post_with(
                reject_suggestion_handler,
                api_docs!("Reject suggestion", "Reject Suggestion from followee ID"),
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
            "/v2/notifications",
            get_with(
                get_notifications_handler,
                api_docs!((), "Get Notifications", "Retrieve a notifications"),
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
            "/v2/permissions",
            get_with(
                has_team_permission_handler,
                api_docs!("Has Permission", "Check user group permission"),
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
            "/v2/users/telegram",
            post_with(
                connect_telegram_handler,
                api_docs!("Update User Telegram Id", "Connect User with Telegram"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/telegram",
            post_with(
                verify_telegram_raw_handler,
                api_docs!(
                    "Verify Telegram Raw Data",
                    "Verify Telegram Raw Data and return token for future connection"
                ),
            )
            .get_with(
                get_telegram_info_handler,
                api_docs!("Get Telegram Info", "Get Telegram Info from token"),
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
            "/v2/themes",
            post_with(
                change_theme_handler,
                api_docs!("Change Theme", "Change Users Theme Information"),
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
            "/v2/verifiable-credentials/medical",
            post_with(
                extract_medical_info_handler,
                api_docs!(
                    "Extract Information from Medical Image",
                    r#"This endpoint allows you to extract medical information from an image."#
                ),
            )
            .with_state(MedicalHandlerState {
                pool: pool.clone(),
                bedrock_client: bedrock_client.clone(),
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
            "/v2/feeds/:id",
            post_with(
                update_post_handler,
                api_docs!("Update Post", "Update an existing post with new details"),
            )
            .get_with(
                get_post_handler,
                api_docs!("Get Post", "Retrieve a specific post by ID"),
            )
            .with_state(pool.clone()),
        )
        .route(
            "/v2/feeds",
            get_with(
                list_posts_handler,
                api_docs!(
                    "List Posts",
                    "Retrieve a paginated list of posts with optional filters"
                ),
            )
            .with_state(pool.clone()),
        )
        .native_route(
            "/v2/oauth/register",
            npost(register_handler)
                .options(register_handler)
                .with_state(pool.clone()),
        )
        .native_route(
            "/v2/oauth/approve",
            npost(approve_handler)
                .options(approve_handler)
                .with_state(pool.clone()),
        )
        .native_route(
            "/v2/oauth/authorize",
            nget(authorize_handler).with_state(pool.clone()),
        )
        .native_route(
            "/v2/oauth/token",
            npost(token_handler)
                .options(token_handler)
                .with_state(pool.clone()),
        )
        .nest(
            "/v3",
            axum::Router::new()
                .nest(
                    "/users",
                    axum::Router::new()
                        .route(
                            "/signup",
                            post_with(
                                v3_email_signup_handler,
                                api_docs!(
                                    "V3 User Signup",
                                    "Register a new user with email and password using V3 API"
                                ),
                            ),
                        )
                        .route(
                            "/login",
                            post_with(
                                v3_login_with_password_handler,
                                api_docs!(
                                    "V3 User Login",
                                    "Login user with email and password using V3 API"
                                ),
                            ),
                        )
                        .route(
                            "/verifications",
                            post_with(
                                email_verification_handler,
                                api_docs!(
                                    "V3 Email Verification",
                                    "Verify user's email address with verification code"
                                ),
                            )
                            .get_with(
                                request_verification_code_handler,
                                api_docs!(
                                    "V3 Request Verification Code",
                                    "Send verification code to user's email address"
                                ),
                            ),
                        ),
                )
                .with_state(dynamo_client),
        )
        .route(
            "/.well-known/oauth-authorization-server",
            get_with(
                oauth_authorization_server_handler,
                api_docs!(
                    "Authorization Server Metadata",
                    "Retrieve OAuth 2.0 Authorization Server Metadata"
                ),
            )
            .options(oauth_authorization_server_handler)
            .with_state(pool.clone()),
        )
        .route(
            "/m2/binances/balance",
            get_with(
                binance_merchant_balance_handler,
                api_docs!(
                    "Query Owner Balance",
                    "Query Owner Balance from inner owner wallet address"
                ),
            )
            .with_state(pool.clone()),
        )
        .native_route("/.well-known/did.json", nget(get_did_document_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::span!(
                        Level::INFO,
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version()
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        tracing::info!(
                            status = %response.status(),
                            latency = ?latency,
                            "response generated"
                        )
                    },
                ),
        )
        .route(
            "/wg/home",
            get_with(
                get_home_handler,
                api_docs!(
                    (),
                    "Get Home Data",
                    "Retrieve home data including feeds, promotions, and news"
                ),
            )
            .with_state(pool.clone()),
        ))
}

pub async fn authorize_admin(
    req: Request,
    next: Next,
) -> std::result::Result<Response<Body>, StatusCode> {
    tracing::debug!("Authorization admin");
    match req.extensions().get::<Option<Authorization>>() {
        Some(Some(Authorization::SecretApiKey)) => Ok(next.run(req).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
