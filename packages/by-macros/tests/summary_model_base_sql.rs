#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
#[cfg(test)]
pub mod summary_model_base_sql_tests {
    #![allow(unused)]
    use super::*;
    use std::time::SystemTime;

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

    #[api_model(base = "/models", table = summary_base_sql, iter_type=QueryResponse)]
    pub struct SummaryTest {
        #[api_model(summary, primary_key, read_action = find_by_id)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary)]
        pub name: String,

        pub description: String,
    }

    #[tokio::test]
    async fn test_summary_base_sql() {
        let q = SummaryTestSummary::base_sql_with("description = $1");
        assert_eq!(
            q,
            "SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, name FROM summary_base_sql WHERE description = $1 "
        );

        let q = SummaryTestSummary::base_sql_with("WHERE description = $1");
        assert_eq!(
            q,
            "SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, name FROM summary_base_sql WHERE description = $1 "
        );

        let q = SummaryTestSummary::base_sql_with("WHERE description = $1 and name like $2");
        assert_eq!(q, "SELECT COUNT(*) OVER() as total_count, id, created_at, updated_at, name FROM summary_base_sql WHERE description = $1 and name like $2 ");

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

        let repo = SummaryTest::get_repository(pool.clone());
        repo.create_table().await;

        for i in 0..5 {
            repo.insert(format!("test-{}-{}", now, i), format!("desc-{}-{}", now, i))
                .await
                .unwrap();
        }
        let q = SummaryTestSummary::base_sql_with("name = $1");
        let mut total: i64 = 0;

        let rows: Vec<SummaryTestSummary> = sqlx::query(&q)
            .bind(format!("test-{}-1", now))
            .map(|row: sqlx::postgres::PgRow| {
                use sqlx::Row;
                total = row.get("total_count");
                row.into()
            })
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(rows.len(), 1, "{:?}", rows);
        assert_eq!(total, 1);

        let q = SummaryTestSummary::base_sql_with("name ilike $1");
        let rows: Vec<SummaryTestSummary> = sqlx::query(&q)
            .bind(format!("test-{}%", now))
            .map(|row: sqlx::postgres::PgRow| {
                use sqlx::Row;
                total = row.get("total_count");
                row.into()
            })
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(rows.len(), 5);
        assert_eq!(total, 5);
    }
}
