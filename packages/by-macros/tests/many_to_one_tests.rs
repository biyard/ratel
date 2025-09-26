#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
pub mod many_to_one_tests {
    #![allow(unused)]
    use super::*;
    use std::time::SystemTime;

    #[cfg(feature = "server")]
    use by_axum::aide;
    use by_macros::api_model;
    use sqlx::postgres::PgRow;

    #[api_model(base = "/v1/topics", table = topics_mto)]
    pub struct Topic {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: u64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: u64,

        #[api_model(summary, action = create)]
        pub title: String,

        #[api_model(summary, one_to_many = votes_mto, foreign_key = topic_id, aggregator = sum(amount))]
        #[serde(default)]
        pub volume: i64,
    }

    #[api_model(base = "/v1/topics/:topic-id/votes", table = votes_mto)]
    pub struct Vote {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary, action = vote)]
        pub amount: i64,

        #[api_model(many_to_one = topics_mto)]
        pub topic_id: i64,
    }

    #[tokio::test]
    async fn test_many_to_one_insert() {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_target(false)
            .try_init()
            .unwrap();

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

        let title = format!("{}-title", now);
        let password = "password".to_string();
        let org_name = format!("org-{}", now);
        let member_name = format!("member-{}", now);
        let contact = Some("contact".to_string());

        let t = Topic::get_repository(pool.clone());
        let v = Vote::get_repository(pool.clone());

        t.create_this_table().await;
        v.create_this_table().await;

        t.create_table().await;
        v.create_table().await;

        let doc = t.insert(title.clone()).await.unwrap();
        assert_eq!(doc.title, title);

        t.insert_without_returning(title.clone()).await.unwrap();
        let res = v.insert(10, doc.id).await;
        assert!(res.is_ok(), "topic id: {}", doc.id);

        v.insert(50, doc.id + 1).await.unwrap();

        let mut q = TopicSummary::base_sql_with("title ilike $1");
        q.push_str(" order by id asc");
        assert_eq!(
            q,
            "SELECT  COUNT(*) OVER() as total_count, p.id, p.created_at, p.updated_at, p.title, COALESCE(MAX(volume.value), 0) AS volume FROM topics_mto p \nLEFT JOIN (\n    SELECT topic_id, SUM(amount) AS value\n    FROM votes_mto \n    GROUP BY topic_id\n) volume ON p.id = volume.topic_id\n WHERE title ilike $1 GROUP BY p.id order by id asc"
        );

        let docs: Vec<TopicSummary> = sqlx::query(&q)
            .bind(format!("{}%", now))
            .map(|row: PgRow| row.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 2);

        assert_eq!(docs[0].volume, 10, "topic id: {}", docs[0].id);
        assert_eq!(docs[1].volume, 50, "topic id: {}", docs[1].id);
        v.insert(100, doc.id).await.unwrap();

        let docs: Vec<TopicSummary> = sqlx::query(&q)
            .bind(format!("{}%", now))
            .map(|row: PgRow| row.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs[0].volume, 110);
    }
}
