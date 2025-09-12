#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
pub mod test_many_to_many_single_field {
    #![allow(unused)]
    use std::time::SystemTime;

    #[cfg(feature = "server")]
    use by_axum::aide;
    use by_macros::api_model;
    use sqlx::postgres::PgRow;
    use sqlx::Row;
    use tokio::time::Instant;

    #[api_model(base = "", table = test_mtm_single_bills)]
    pub struct Bill {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = insert)]
        pub created_at: i64,

        #[api_model(summary)]
        pub title: String,

        #[api_model(many_to_many = test_mtm_single_votes, type = JSONB, foreign_table_name = test_mtm_single_users, foreign_primary_key = user_id, foreign_reference_key = model_id, target_table = join)]
        pub user_vote: Vote,
    }

    #[api_model(base = "", table = test_mtm_single_votes)]
    pub struct Vote {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,
        #[api_model(many_to_one = test_mtm_single_users)]
        pub user_id: i64,
        #[api_model(many_to_one = test_mtm_single_bills)]
        pub model_id: i64,
    }

    #[api_model(base = "/v1/users", table = test_mtm_single_users)]
    pub struct User {
        #[api_model(primary_key)]
        pub id: i64,
        #[api_model(auto = insert)]
        pub created_at: i64,
        #[api_model(auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary, action = signup)]
        pub email: String,
    }

    #[tokio::test]
    async fn test_target_table_parameter() {
        let _ = tracing_subscriber::fmt::try_init();
        let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(
                option_env!("DATABASE_URL")
                    .unwrap_or("postgres://postgres:postgres@localhost:5432/test"),
            )
            .await
            .unwrap();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let u = User::get_repository(pool.clone());
        let v = Vote::get_repository(pool.clone());
        let b = Bill::get_repository(pool.clone());

        b.create_this_table().await;
        u.create_this_table().await;
        v.create_this_table().await;

        b.create_related_tables().await;
        u.create_related_tables().await;
        v.create_related_tables().await;

        let user = u.insert("test1@gmail.com".to_string()).await.unwrap().id;

        let bill = b.insert("BILL".to_string()).await.unwrap();

        let vote = v.insert(user, bill.id).await.unwrap();

        let r: std::result::Result<Bill, sqlx::Error> = Bill::query_builder()
            .id_equals(bill.id)
            .query()
            .map(Bill::from)
            .fetch_one(&pool)
            .await;
        tracing::debug!("{:?}", r);
        assert!(r.is_ok());
        let bill = r.unwrap();
        assert_eq!(bill.user_vote, vote);
    }
}
