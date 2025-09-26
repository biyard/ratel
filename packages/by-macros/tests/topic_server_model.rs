#![allow(unused)]
#[cfg(feature = "server")]
use by_axum::aide;

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

#[cfg(feature = "server")]
mod server_tests {
    use by_macros::api_model;
    use by_types::Role;
    use lazy_static::lazy_static;
    use std::time::SystemTime;
    use validator::ValidationError;

    use super::*;
    fn validate_nickname(nickname: &str) -> std::result::Result<(), ValidationError> {
        lazy_static! {
            static ref NICKNAME_REGEX: regex::Regex =
                regex::Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-_]{1,20}$").unwrap();
        }

        if !NICKNAME_REGEX.is_match(nickname) {
            return Err(ValidationError::new("Nickname must be started with alphabet or number and only allow alphabet, number, hyphen and underscore, maximum 20 characters"));
        }

        Ok(())
    }

    #[api_model(base = "/topics/v1/:topic-id/votes", table = votes, iter_type=QueryResponse)]
    pub struct Vote {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(many_to_one = topics)]
        pub topic_id: i64,

        #[api_model(many_to_one = users)]
        pub user_id: i64,

        #[api_model(summary, action = support)]
        pub amount: i64,

        #[api_model(summary, action = support)]
        pub name: String,

        #[api_model(action_by_id = confirm)]
        pub confirmed: Option<bool>,
    }

    #[derive(validator::Validate)]
    #[api_model(base = "/users/v1", read_action = user_info, table = users, iter_type=QueryResponse)]
    pub struct User {
        #[api_model(primary_key)]
        pub id: i64,
        #[api_model(auto = insert)]
        pub created_at: u64,
        #[api_model(auto = [insert, update])]
        pub updated_at: u64,

        #[api_model(action = signup)]
        #[validate(custom(function = "validate_nickname"))]
        pub nickname: String,
        #[api_model(action = signup, read_action = [check_email], unique)]
        #[validate(email)]
        pub email: String,
        #[api_model(action = signup)]
        #[validate(url)]
        pub profile_url: String,

        #[api_model(type = INTEGER)]
        pub role: Role,

        #[api_model(version = v0.1, unique, action = [login], read_action = [find_by_kakao_id])]
        pub kakao_id: String,
    }

    #[api_model(base = "/topics/v1", read_action = [get_topic] , table = topics, iter_type=QueryResponse)]
    pub struct Topic {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary, action = create, action_by_id = update)]
        pub title: String,
        #[api_model(summary, action = create, action_by_id = update)]
        pub content: String,

        #[api_model(summary, action = create, action_by_id = update)]
        pub started_at: i64,
        // The end time of the vote
        #[api_model(summary, action = create, action_by_id = update)]
        pub ended_at: i64,
        // The number of required votes
        #[api_model(summary, action = create, action_by_id = update)]
        pub requirement: i64,
        // The number of voters
        #[api_model(summary, one_to_many = votes, foreign_key = topic_id, aggregator = count)]
        pub voters: i64,
        // // Donation amount
        #[api_model(summary, one_to_many = votes, foreign_key = topic_id, aggregator = sum(amount))]
        pub amount: i64,

        #[api_model(summary, many_to_many = votes, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = topic_id, aggregator = exist, unique)]
        pub voted: bool,
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
    async fn test_db_create() {
        use sqlx::{postgres::PgPoolOptions, Postgres};
        let _ = tracing_subscriber::fmt::try_init();

        db_setup().await;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let email = format!("test-{}@test.com", now);
        let _principal = format!("{}-principal", now);

        let pool: sqlx::Pool<Postgres> = PgPoolOptions::new()
            .max_connections(5)
            .connect(
                option_env!("DATABASE_URL")
                    .unwrap_or("postgres://postgres:postgres@localhost:5432/test"),
            )
            .await
            .unwrap();

        let u = User::get_repository(pool.clone());
        u.create_table().await.unwrap();
        let repo = Topic::get_repository(pool.clone());
        repo.create_table().await.unwrap();
        let v = Vote::get_repository(pool.clone());
        v.create_table().await.unwrap();
    }
}
