use crate::{config, controllers, route::route, utils::referal_code::generate_referral_code};
use bdk::prelude::{by_axum::axum::Router, *};
use by_axum::axum::middleware;
use by_types::DatabaseConfig;
use dto::{
    by_axum::{
        auth::{authorization_middleware, generate_jwt, set_auth_token_key},
        axum::{extract::Request, http::Response, middleware::Next},
    },
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
        AssemblyMember,
        BillWriter,
        Vote,
        Proposer,
        Support,
        Subscription,
        PresidentialCandidate,
        ElectionPledge,
        ElectionPledgeLike,
        Industry,
        Feed,
        FeedUser,
        RedeemCode,
        Space,
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
        TeamMember,
        News,
        Quiz,
        QuizResult,
        ElectionPledgeQuizLike,
        ElectionPledgeQuizDislike,
        Promotion,
        AdvocacyCampaign,
        AdvocacyCampaignAuthor,
        AdvocacyCampaignVoter,
        EventLog,
        Badge,
        UserBadge,
        UserPoint,
        SpaceBadge,
        Onboard,
        Mynetwork,
        Verification,
        Notification,
    );

    if Industry::query_builder()
        .id_equals(1)
        .query()
        .map(Industry::from)
        .fetch_optional(pool)
        .await?
        .is_none()
    {
        Industry::get_repository(pool.clone())
            .insert("Crypto".to_string())
            .await?;
    }

    if User::query_builder()
        .id_equals(1)
        .query()
        .map(User::from)
        .fetch_optional(pool)
        .await?
        .is_none()
    {
        User::get_repository(pool.clone())
            .insert(
                "ServiceAdmin".to_string(),
                "user-principal-1".to_string(),
                "".to_string(),
                "profile_url".to_string(),
                true,
                true,
                UserType::Individual,
                None,
                "admin".to_string(),
                "".to_string(),
                "0x000".to_string(),
                "password".to_string(),
                Membership::Free,
                generate_referral_code(),
            )
            .await?;
    }

    if Group::query_builder()
        .id_equals(1)
        .query()
        .map(Group::from)
        .fetch_optional(pool)
        .await?
        .is_none()
    {
        Group::get_repository(pool.clone())
            .insert(
                "ServiceAdmin".to_string(),
                "".to_string(),
                "".to_string(),
                1,
                0xffffffffffffffffu64 as i64,
            )
            .await?;
    }

    tracing::info!("Migration done");
    Ok(())
}

pub async fn api_main() -> Result<Router> {
    let app = by_axum::new();
    let conf = config::get();
    by_axum::auth::set_auth_config(conf.auth.clone());
    tracing::debug!("config: {:?}", conf);
    let auth_token_key = format!("{}_auth_token", conf.env);
    let auth_token_key = Box::leak(Box::new(auth_token_key));
    set_auth_token_key(auth_token_key);

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?
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
    let mcp_router =
        by_axum::axum::Router::new().nest_service("/mcp", controllers::mcp::route().await?);
    let api_router = route(pool.clone())
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
