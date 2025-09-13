use std::env;

use crate::{
    config, controllers,
    route::route,
    utils::{
        aws::{BedrockClient, RekognitionClient, S3Client, TextractClient},
        // dynamo_migrate::{create_dynamo_tables, get_user_tables},
        mcp_middleware::mcp_middleware,
        sqs_client,
        telegram::TelegramBot,
    },
};

use bdk::prelude::{by_axum::axum::Router, *};
use by_axum::axum::middleware;
use by_types::DatabaseConfig;
use dto::{
    by_axum::{
        auth::{authorization_middleware, generate_jwt, set_auth_token_key},
        axum::{extract::Request, http::Response, middleware::Next},
    },
    sqlx::PgPool,
    *,
};
use sqlx::postgres::PgPoolOptions;
use tower_sessions::{
    Session, SessionManagerLayer,
    cookie::time::{Duration, OffsetDateTime},
};
use tower_sessions_sqlx_store::PostgresStore;

macro_rules! migrate {
    ($pool:ident, $($table:ident),* $(,)?) => {
        {
            $(
                let t = $table::get_repository($pool.clone());
                t.create_this_table().await?;
            )*
            $(
                let t = $table::get_repository($pool.clone());
                t.create_related_tables().await?;
            )*
        }
    };
}

pub async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    tracing::info!("Running migration");

    migrate!(
        pool,
        User,
        Group,
        GroupMember,
        // AssemblyMember,
        // BillWriter,
        // Vote,
        // Proposer,
        // Support,
        Subscription,
        // PresidentialCandidate,
        // ElectionPledge,
        // ElectionPledgeLike,
        Industry,
        UserIndustry,
        Feed,
        FeedUser,
        FeedShare,
        RedeemCode,
        Space,
        SpaceLikeUser,
        FeedBookmarkUser,
        SpaceShareUser,
        Survey,
        SurveyResponse,
        SpaceDraft,
        Discussion,
        DiscussionParticipant,
        DiscussionMember,
        Elearning,
        SpaceUser,
        SpaceContract,
        SpaceHolder,
        SpaceGroup,
        SpaceMember,
        SprintLeague,
        SprintLeaguePlayer,
        SprintLeagueVote,
        TeamMember,
        News,
        // Quiz,
        // QuizResult,
        // ElectionPledgeQuizLike,
        // ElectionPledgeQuizDislike,
        Promotion,
        // AdvocacyCampaign,
        // AdvocacyCampaignAuthor,
        // AdvocacyCampaignVoter,
        EventLog,
        Badge,
        UserBadge,
        UserPoint,
        SpaceBadge,
        Onboard,
        Mynetwork,
        ConnectionInvitationDecline,
        UserSuggestionDismissal,
        Verification,
        Notification,
        NoticeQuizAnswer,
        NoticeQuizAttempt,
        Dagit,
        Artwork,
        Oracle,
        DagitOracle,
        DagitArtwork,
        Consensus,
        ConsensusVote,
        ArtworkCertification,
        ArtworkDetail,
        Conversation,
        Message,
        ConversationParticipant,
        AuthClient,
        AuthCode,
        Post,
        TelegramChannel,
        TelegramToken,
    );

    // Create DynamoDB tables
    // tracing::info!("Creating DynamoDB tables");
    // let dynamo_tables = get_user_tables();
    // create_dynamo_tables(dynamo_tables).await?;
    // tracing::info!("DynamoDB tables created successfully");

    tracing::info!("Migration done");
    Ok(())
}

pub async fn db_init(url: &'static str, max_conn: u32) -> Result<PgPool> {
    let url = if let Ok(host) = env::var("PGHOST") {
        let url = if let Some(at_pos) = url.rfind('@') {
            let (before_at, after_at) = url.split_at(at_pos + 1);
            if let Some(slash_pos) = after_at.find('/') {
                let (_, after_slash) = after_at.split_at(slash_pos);
                format!("{}{}{}", before_at, host, after_slash)
            } else {
                url.to_string()
            }
        } else {
            url.to_string()
        };
        url
    } else {
        url.to_string()
    };

    tracing::debug!("Connecting to database at {}", url);

    let pool = PgPoolOptions::new()
        .max_connections(max_conn)
        .connect(&url)
        .await?;

    Ok(pool)
}

pub async fn api_main() -> Result<Router> {
    let app = by_axum::new();
    let conf = config::get();
    by_axum::auth::set_auth_config(conf.auth.clone());

    let auth_token_key = format!("{}_auth_token", conf.env);
    let auth_token_key = Box::leak(Box::new(auth_token_key));
    set_auth_token_key(auth_token_key);

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        let pool = db_init(url, pool_size).await?;
        tracing::info!(
            "Connected to Postgres at {}",
            pool.connect_options().get_host()
        );
        pool
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    let session_store = PostgresStore::new(pool.clone());
    if conf.migrate {
        migration(&pool).await?;
        let res = session_store.migrate().await;
        if let Err(e) = res {
            tracing::error!("Failed to migrate session store: {}", e);
            return Err(e.into());
        }
    }

    let sqs_client = sqs_client::SqsClient::new().await;
    let bedrock_client = BedrockClient::new();
    let rek_client = RekognitionClient::new();
    let textract_client = TextractClient::new();
    let private_s3_client = S3Client::new(conf.private_bucket_name);
    let metadata_s3_client = S3Client::new(conf.bucket.name);
    let is_local = conf.env == "local";
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(!is_local)
        .with_http_only(!is_local)
        .with_same_site(if is_local {
            tower_sessions::cookie::SameSite::Lax
        } else {
            tower_sessions::cookie::SameSite::None
        })
        .with_name(format!("{}_sid", conf.env))
        .with_path("/")
        .with_expiry(tower_sessions::Expiry::AtDateTime(
            OffsetDateTime::now_utc()
                .checked_add(Duration::days(30))
                .unwrap(),
        ));
    let mcp_router = by_axum::axum::Router::new()
        .nest_service("/mcp", controllers::mcp::route(pool.clone()).await?)
        .layer(middleware::from_fn(mcp_middleware));
    let bot = if let Some(token) = conf.telegram_token {
        Some(TelegramBot::new(token).await?)
    } else {
        None
    };
    // FIXME: Is this the correct way to inject and pass the states into the route?
    // find better way to  management Axum's state or dependency injection for better modularity and testability.
    let api_router = route(
        pool.clone(),
        sqs_client,
        bedrock_client,
        rek_client,
        textract_client,
        metadata_s3_client,
        private_s3_client,
        bot,
    )
    .await?
    .layer(middleware::from_fn(authorization_middleware))
    .layer(session_layer)
    .layer(middleware::from_fn(cookie_middleware));

    let app = app.merge(mcp_router).merge(api_router);
    Ok(app)
}

pub async fn cookie_middleware(
    req: Request,
    next: Next,
) -> std::result::Result<Response<by_axum::axum::body::Body>, by_axum::axum::http::StatusCode> {
    tracing::debug!("Authorization middleware {:?}", req.uri());
    let session_initialized = if let Some(session) = req.extensions().get::<Session>() {
        if let Ok(Some(_)) = session
            .get::<by_axum::auth::UserSession>(by_axum::auth::USER_SESSION_KEY)
            .await
        {
            true
        } else {
            false
        }
    } else {
        false
    };

    let mut res = next.run(req).await;
    tracing::debug!("Authorization middleware response: {:?}", res.status());
    if session_initialized {
        tracing::debug!("Session not initialized, skipping cookie generation.");
        return Ok(res);
    }

    if let Some(ref session) = res.extensions().get::<Session>() {
        tracing::debug!("Checking for user session in response...");
        if let Ok(Some(user_session)) = session
            .get::<by_axum::auth::UserSession>(by_axum::auth::USER_SESSION_KEY)
            .await
        {
            tracing::debug!("User session found in response: {:?}", user_session);
            let mut claims = by_types::Claims {
                sub: user_session.user_id.to_string(),
                ..Default::default()
            };

            let token = generate_jwt(&mut claims)?;

            res.headers_mut().append(
                reqwest::header::SET_COOKIE,
                format!(
                    "{}_auth_token={}; SameSite=Lax; Path=/; Max-Age=2586226; HttpOnly; Secure;",
                    config::get().env,
                    token,
                )
                .parse()
                .unwrap(),
            );
        }
    }

    return Ok(res);
}
