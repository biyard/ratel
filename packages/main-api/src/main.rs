use bdk::prelude::{by_axum::axum::Router, *};
#[cfg(test)]
use by_axum::auth::set_auth_config;
use by_axum::{auth::authorization_middleware, axum::middleware};
use by_types::DatabaseConfig;
use dto::*;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

mod controllers {
    pub mod m1;
    pub mod v1;
}

pub mod config;
pub mod models;
pub mod security;
pub mod utils;

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

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    tracing::info!("Running migration");

    migrate!(
        pool,
        User,
        Follower,
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
        FeedComment,
        Space,
        SpaceComment,
        SpaceUser,
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
            .insert("ServiceAdmin".to_string(), 1, 0xffffffffffffffffu64 as i64)
            .await?;
    }

    tracing::info!("Migration done");
    Ok(())
}

async fn api_main() -> Result<Router> {
    let app = by_axum::new();
    let conf = config::get();
    tracing::debug!("config: {:?}", conf);

    let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .connect(url)
            .await?
    } else {
        panic!("Database is not initialized. Call init() first.");
    };

    if conf.migrate {
        migration(&pool).await?;
    }

    let app = app
        .nest("/v1", controllers::v1::route(pool.clone())?)
        .nest(
            "/m1",
            controllers::m1::MenaceController::route(pool.clone())?,
        )
        .layer(middleware::from_fn(authorization_middleware));
    Ok(app)
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = api_main().await?;
    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use by_types::Claims;
    use std::{collections::HashMap, time::SystemTime};

    use super::*;
    use rest_api::ApiService;

    pub struct TestContext {
        pub pool: sqlx::Pool<sqlx::Postgres>,
        pub app: Box<dyn ApiService>,
        pub user: User,
        pub user_token: String,
        pub admin: User,
        pub admin_token: String,
        pub now: i64,
        pub id: String,
        pub claims: Claims,
        pub admin_claims: Claims,
        pub endpoint: String,
    }

    pub async fn setup_test_admin(id: &str, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<User> {
        let user = User::get_repository(pool.clone());
        let nickname = format!("user-{}", id);
        let principal = format!("user-principal-{}", id);
        let email = format!("user-{id}@test.com");
        let profile_url = format!("https://test.com/{id}");
        let mut tx = pool.begin().await?;

        let u = user
            .insert_with_tx(
                &mut *tx,
                nickname.clone(),
                principal.clone(),
                email.clone(),
                profile_url.clone(),
                true,
                true,
                UserType::Individual,
                None,
                email.clone(),
                "".to_string(),
            )
            .await?
            .unwrap();

        GroupMember::get_repository(pool.clone())
            .insert_with_tx(&mut *tx, u.id, 1)
            .await?;

        tx.commit().await?;

        tracing::debug!("{:?}", u);

        Ok(u)
    }

    pub async fn setup_test_user(id: &str, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<User> {
        let user = User::get_repository(pool.clone());
        let nickname = format!("user-{}", id);
        let principal = format!("user-principal-{}", id);
        let email = format!("user-{id}@test.com");
        let profile_url = format!("https://test.com/{id}");

        let u = user
            .insert(
                nickname.clone(),
                principal.clone(),
                email.clone(),
                profile_url.clone(),
                true,
                true,
                UserType::Individual,
                None,
                email.clone(),
                "".to_string(),
            )
            .await?;

        tracing::debug!("{:?}", u);

        Ok(u)
    }

    pub fn setup_jwt_admin_token(user: User) -> (Claims, String) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut claims = Claims {
            sub: user.id.to_string(),
            exp: now + 3600,
            role: by_types::Role::Admin,
            custom: HashMap::new(),
        };
        let token = by_axum::auth::generate_jwt(&mut claims).unwrap();
        (claims, token)
    }

    pub fn setup_jwt_token(user: User) -> (Claims, String) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut claims = Claims {
            sub: user.id.to_string(),
            exp: now + 3600,
            role: by_types::Role::User,
            custom: HashMap::new(),
        };
        let token = by_axum::auth::generate_jwt(&mut claims).unwrap();
        (claims, token)
    }

    pub async fn setup() -> Result<TestContext> {
        let app = api_main().await?;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        let conf = config::get();
        tracing::debug!("config: {:?}", conf);
        set_auth_config(conf.auth.clone());

        let pool = if let DatabaseConfig::Postgres { url, pool_size } = conf.database {
            PgPoolOptions::new()
                .max_connections(pool_size)
                .connect(url)
                .await
                .expect("Failed to connect to Postgres")
        } else {
            panic!("Database is not initialized. Call init() first.");
        };

        let _ = sqlx::query(
            r#"
        CREATE OR REPLACE FUNCTION set_updated_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at := EXTRACT(EPOCH FROM now()); -- seconds
                RETURN NEW;
            END;
        $$ LANGUAGE plpgsql;
        "#,
        )
        .execute(&pool)
        .await;

        let _ = sqlx::query(
            r#"
        CREATE OR REPLACE FUNCTION set_created_at()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.created_at := EXTRACT(EPOCH FROM now()); -- seconds
                RETURN NEW;
            END;
        $$ LANGUAGE plpgsql;
        "#,
        )
        .execute(&pool)
        .await;

        let _ = migration(&pool).await;

        let id = uuid::Uuid::new_v4().to_string();
        let user = setup_test_user(&id, &pool).await.unwrap();

        let id = uuid::Uuid::new_v4().to_string();
        let admin = setup_test_admin(&id, &pool).await.unwrap();
        let (admin_claims, admin_token) = setup_jwt_admin_token(admin.clone());
        let (claims, user_token) = setup_jwt_token(user.clone());

        let app = by_axum::into_api_adapter(app);
        let app = Box::new(app);
        rest_api::set_message(conf.signing_domain.to_string());
        rest_api::set_api_service(app.clone());
        rest_api::add_authorization(&format!("Bearer {}", user_token));

        Ok(TestContext {
            pool,
            app,
            id,
            user,
            user_token,
            admin,
            admin_token,
            claims,
            admin_claims,
            now: now as i64,
            endpoint: format!("http://localhost:3000"),
        })
    }
}
