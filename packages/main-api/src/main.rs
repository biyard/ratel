use bdk::prelude::*;
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
pub mod utils;

async fn migration(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<()> {
    tracing::info!("Running migration");
    let u = User::get_repository(pool.clone());
    let t = Topic::get_repository(pool.clone());
    let c = Comment::get_repository(pool.clone());
    let v = Vote::get_repository(pool.clone());
    let a = AssemblyMember::get_repository(pool.clone());
    let p = Patron::get_repository(pool.clone());
    let f = Feature::get_repository(pool.clone());

    u.create_this_table().await?;
    t.create_this_table().await?;
    c.create_this_table().await?;
    v.create_this_table().await?;
    a.create_this_table().await?;
    p.create_this_table().await?;
    f.create_this_table().await?;

    u.create_related_tables().await?;
    t.create_related_tables().await?;
    c.create_related_tables().await?;
    v.create_related_tables().await?;
    a.create_related_tables().await?;
    p.create_related_tables().await?;
    f.create_related_tables().await?;

    tracing::info!("Migration done");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
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
    rest_api::set_message(conf.signing_domain.to_string());

    migration(&pool).await?;

    let app = app
        // .nest(
        //     "/v1/patrons",
        //     controllers::patrons::v1::PatronControllerV1::route()?,
        // )
        .nest(
            "/v1/topics",
            controllers::v1::topics::TopicControllerV1::route(pool.clone())?,
        )
        .nest(
            "/v1/users",
            controllers::v1::users::UserControllerV1::route(pool.clone())?,
        )
        .nest(
            "/v1/assembly-members",
            controllers::v1::assembly_members::AssemblyMemberControllerV1::route(pool.clone())?,
        )
        .nest(
            "/v1/patrons",
            controllers::v1::patrons::PatronControllerV1::route(pool.clone())?,
        )
        .nest(
            "/m1",
            controllers::m1::MenaceController::route(pool.clone())?,
        )
        .layer(middleware::from_fn(authorization_middleware));

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
        pub admin_token: String,
        pub now: i64,
        pub id: String,
        pub claims: Claims,
        pub endpoint: String,
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
            )
            .await?;
        tracing::debug!("{:?}", u);

        let user = user.find_one(&UserReadAction::new().user_info()).await?;

        Ok(user)
    }

    pub fn setup_jwt_token(user: User) -> (Claims, String) {
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

    pub async fn setup() -> Result<TestContext> {
        let app = by_axum::new();
        let id = uuid::Uuid::new_v4().to_string();
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

        let app = app
            .nest(
                "/v1/users",
                controllers::v1::users::UserControllerV1::route(pool.clone())?,
            )
            .layer(middleware::from_fn(authorization_middleware));

        let user = setup_test_user(&id, &pool).await.unwrap();
        let (claims, admin_token) = setup_jwt_token(user.clone());

        let app = by_axum::into_api_adapter(app);
        let app = Box::new(app);
        rest_api::set_message(conf.signing_domain.to_string());
        rest_api::set_api_service(app.clone());
        rest_api::add_authorization(&format!("Bearer {}", admin_token));

        Ok(TestContext {
            pool,
            app,
            id,
            user,
            admin_token,
            claims,
            now: now as i64,
            endpoint: format!("http://localhost:3000"),
        })
    }
}
