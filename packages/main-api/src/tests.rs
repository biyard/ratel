use by_types::Claims;
use dto::{
    by_axum::auth::set_auth_config, by_types::DatabaseConfig, sqlx::postgres::PgPoolOptions, *,
};
use std::{collections::HashMap, time::SystemTime};

use crate::api_main::migration;

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
            format!("0x{}", id),
            "".to_string(), // password
            Membership::Free,
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
            format!("0x{}", id),
            "".to_string(), // password
            Membership::Free,
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
    let app = api_main::api_main().await?;
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
