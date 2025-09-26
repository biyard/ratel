#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
pub mod test_one_to_many_with_filter {
    #![allow(unused)]
    use std::time::SystemTime;

    #[cfg(feature = "server")]
    use by_axum::aide;
    use by_macros::api_model;
    use sqlx::postgres::PgRow;
    use sqlx::Row;
    use tokio::time::Instant;

    #[api_model(base = "", table = test_otm_bills)]
    pub struct Bill {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = insert)]
        pub created_at: i64,

        #[api_model(summary)]
        pub title: String,

        #[api_model(one_to_many = test_otm_votes, type = JSONB, foreign_key = bill_id, filter_by = user_id )]
        pub user_vote: Vote,
    }

    #[api_model(base = "", table = test_otm_votes)]
    pub struct Vote {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,
        #[api_model(many_to_one = test_otm_users)]
        pub user_id: i64,
        #[api_model(many_to_one = test_otm_bills)]
        pub bill_id: i64,
    }

    #[api_model(base = "/v1/users", table = test_otm_users)]
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

        let user = u.insert("test1@gmail.com".to_string()).await.unwrap();
        let bill = b.insert("BILL".to_string()).await.unwrap();
        let vote = v.insert(user.id, bill.id).await.unwrap();

        let r: std::result::Result<Bill, sqlx::Error> = Bill::query_builder(user.id)
            .id_equals(bill.id)
            .query()
            .map(Bill::from)
            .fetch_one(&pool)
            .await;
        assert!(r.is_ok());
        let bill = r.unwrap();
        tracing::debug!("user_id: {}", user.id);

        assert!(bill.user_vote.user_id == user.id);
        tracing::debug!("BILL: {bill:?}");
        let r: std::result::Result<Bill, sqlx::Error> = Bill::query_builder(0)
            .id_equals(bill.id)
            .query()
            .map(Bill::from)
            .fetch_one(&pool)
            .await;
        assert!(r.is_ok());
        let bill = r.unwrap();
        tracing::debug!("BILL: {bill:?}");
        assert!(bill.user_vote.user_id != user.id);
    }
}
