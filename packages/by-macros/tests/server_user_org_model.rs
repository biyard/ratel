#![allow(unused)]
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct QueryResponse<T> {
    pub items: Vec<T>,
    pub total_count: i64,
}

impl<T> From<(Vec<T>, i64)> for QueryResponse<T> {
    fn from((items, total_count): (Vec<T>, i64)) -> Self {
        QueryResponse { items, total_count }
    }
}

pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;
use std::time::SystemTime;
use validator::Validate;

#[cfg(feature = "server")]
mod server_tests {
    use super::*;

    #[api_model(base = "/auth/v1", action = [signup(code = String), reset(code = String)], table = uo_users, iter_type=QueryResponse)]
    pub struct User {
        #[api_model(primary_key)]
        pub id: i64,
        #[api_model(auto = insert)]
        pub created_at: i64,
        #[api_model(auto = [insert, update])]
        pub updated_at: i64,
        #[api_model(action = [signup, login, reset], unique, read_action = get_user)]
        pub email: String,
        #[api_model(action = [signup, login, reset], read_action = get_user)]
        pub password: String,

        #[api_model(many_to_many = uo_user_orgs, foreign_table_name = uo_organizations, foreign_primary_key = org_id, foreign_reference_key = user_id)]
        #[serde(default)]
        pub orgs: Vec<Organization>,
    }

    #[api_model(base = "/auth/v1/organizations", table = uo_organizations, iter_type=QueryResponse)]
    pub struct Organization {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = insert)]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary)]
        pub name: String,
        #[api_model(many_to_many = uo_user_orgs, foreign_table_name = uo_users, foreign_primary_key = user_id, foreign_reference_key = org_id)]
        #[serde(default)]
        pub users: Vec<User>,
    }

    async fn db_setup() {
        let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(
                option_env!("DATABASE_URL")
                    .unwrap_or("postgres://postgres:postgres@localhost:5432/test"),
            )
            .await
            .unwrap();
        let query = r#"
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at := EXTRACT(EPOCH FROM now()); -- seconds
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
    "#;
    sqlx::query(query).execute(&pool).await.unwrap();


        let query = r#"
CREATE OR REPLACE FUNCTION set_created_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.created_at := EXTRACT(EPOCH FROM now()); -- seconds
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
    "#;

        sqlx::query(query).execute(&pool).await.unwrap();
    }

    #[tokio::test]
    async fn test_user_org() {
        use sqlx::{postgres::PgPoolOptions, Postgres};
        let _ = tracing_subscriber::fmt::try_init();
        db_setup().await;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let email = format!("test-{}@test.com", now);
        let password = "password".to_string();

        let pool: sqlx::Pool<Postgres> = PgPoolOptions::new()
            .max_connections(5)
            .connect(
                option_env!("DATABASE_URL")
                    .unwrap_or("postgres://postgres:postgres@localhost:5432/test"),
            )
            .await
            .unwrap();

        let u = User::get_repository(pool.clone());
        u.create_this_table().await.unwrap();

        let o = Organization::get_repository(pool.clone());
        o.create_this_table().await.unwrap();
        u.create_table().await.unwrap();
        o.create_table().await.unwrap();

        let user = u.insert(email.clone(), password.clone()).await;
        assert!(user.is_ok(), "{:?}", user);
        let user = user.unwrap();

        let name = "org".to_string();
        let res = o.insert_with_dependency(user.id, name.to_string()).await;

        assert!(res.is_ok(), "{:?}", res);

        let res = u
            .find_one(&UserReadAction::new().get_user(email.clone(), password.clone()))
            .await;
        assert!(res.is_ok());
        let found_user = res.unwrap();
        assert_eq!(found_user.email, email);
        assert_eq!(found_user.password, password);
        assert_eq!(found_user.id, user.id);
        assert_eq!(found_user.orgs.len(), 1);
        assert_eq!(found_user.orgs[0].name, name);

        let res = o.insert_with_dependency(user.id, name.to_string()).await;
        assert!(res.is_ok(), "{:?}", res);

        let res = u
            .find_one(&UserReadAction::new().get_user(email.clone(), password.clone()))
            .await;
        assert!(res.is_ok());
        let found_user = res.unwrap();
        assert_eq!(found_user.orgs.len(), 2);
    }
}
