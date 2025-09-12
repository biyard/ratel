#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
#[cfg(test)]
pub mod update_into_tests {
    #![allow(unused)]
    use super::*;
    use std::{fmt::Arguments, time::SystemTime};

    #[cfg(feature = "server")]
    use by_axum::aide;
    use by_macros::api_model;
    use sqlx::{postgres::PgRow, Execute};

    #[api_model(base = "/models", table = query_builder_test)]
    pub struct QueryModel {
        #[api_model(summary, primary_key, read_action = find_by_id)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary, query_action = search)]
        pub name: String,

        #[api_model(query_action = search)]
        pub description: String,

        #[api_model(query_action = list_by_status)]
        pub status: i32,

        #[api_model(summary)]
        pub num: i64,

        pub is_like: bool,

        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id)]
        pub children: Vec<ChildModels>,

        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id, aggregator=sum(volumes))]
        pub volume_of_children: i64,

        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id, aggregator=max(volumes))]
        pub max_volume_of_children: i64,

        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id, aggregator=min(volumes))]
        pub min_volume_of_children: i64,

        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id, aggregator=avg(volumes))]
        pub avg_volume_of_children: f64,

        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id, aggregator=exist)]
        pub has_children: bool,

        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id, aggregator=count)]
        pub num_of_children: i64,
    }

    #[api_model(base = "/models", table = query_builder_test)]
    pub struct QueryModelWithMany {
        #[api_model(summary, primary_key, read_action = find_by_id)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary, query_action = search)]
        pub name: String,

        #[api_model(query_action = search)]
        pub description: String,

        #[api_model(query_action = list_by_status)]
        pub status: i32,

        #[api_model(summary)]
        pub num: i64,

        pub is_like: bool,

        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id)]
        pub children: Vec<ChildModels>,

        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id, aggregator=sum(volumes))]
        pub volume_of_children: i64,

        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id, aggregator=max(volumes))]
        pub max_volume_of_children: i64,

        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id, aggregator=min(volumes))]
        pub min_volume_of_children: i64,
        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id, aggregator=avg(volumes))]
        pub avg_volume_of_children: i64,

        // #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id, aggregator=exist)]
        // pub has_children: bool,
        #[api_model(summary, one_to_many = child_model_query_builder_test, foreign_key = model_id, aggregator=count)]
        pub num_of_children: i64,

        #[api_model(many_to_many = many_join_table, table_name = many_model_query_builder_test, foreign_reference_key = model_id, foreign_primary_key = many_id, aggregator=exist)]
        pub has_many: bool,

        #[api_model(many_to_many = many_join_table, foreign_table_name = many_model_query_builder_test, foreign_reference_key = model_id, foreign_primary_key = many_id)]
        pub many: Vec<ManyModelsSummary>,
    }

    #[api_model(base = "/:model-id/models", table = child_model_query_builder_test)]
    pub struct ChildModels {
        #[api_model(summary, primary_key, read_action = find_by_id)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary, many_to_one = query_builder_test)]
        pub model_id: i64,

        pub name: String,

        pub volumes: i64,
    }

    #[api_model(base = "/many-models", table = many_model_query_builder_test)]
    pub struct ManyModels {
        #[api_model(summary, primary_key, read_action = find_by_id)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(many_to_many = many_join_table, foreign_table_name = query_builder_test, foreign_reference_key = many_id, foreign_primary_key = model_id)]
        pub models: Vec<QueryModelWithManySummary>,

        #[api_model(summary)]
        pub name: String,
    }

    #[tokio::test]
    async fn test_query_builder_basic() {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_target(false)
            .try_init();

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

        let name = format!("name-{}", now);
        let description = format!("desc-{}", now);
        let status = 1;

        let repo = QueryModel::get_repository(pool.clone());
        let repo_child = ChildModels::get_repository(pool.clone());

        repo.create_this_table().await;
        repo_child.create_this_table().await;

        repo.create_table().await;
        repo_child.create_table().await;

        let mut ids = vec![];

        for b in [true, false].iter() {
            for n in 0..10 {
                let doc = repo
                    .insert(
                        format!("{} {}-{}", name, n, b),
                        description.clone(),
                        status,
                        n,
                        *b,
                    )
                    .await
                    .unwrap();
                ids.push(doc.id);
            }
        }

        let mut q = QueryModel::query_builder()
            .num_between(0, 5)
            .name_contains(name.clone())
            .order_by_id_asc();

        //         assert_eq!(q.sql(), r#"SELECT p.*,
        // COALESCE(
        //     json_agg(to_jsonb(children)) FILTER (WHERE f.id IS NOT NULL), '[]'
        // ) AS children
        // , COALESCE(volume_of_children.value, 0) AS volume_of_children, COALESCE(num_of_children.value, 0) AS num_of_children FROM query_builder_test p
        // LEFT JOIN child_model_query_builder_test children ON p.id = children.model_id

        // LEFT JOIN (
        //     SELECT model_id, SUM(volumes) AS value
        //     FROM child_model_query_builder_test
        //     GROUP BY model_id
        // ) volume_of_children ON p.id = volume_of_children.model_id

        // LEFT JOIN (
        //     SELECT model_id, COUNT(id) AS value
        //     FROM child_model_query_builder_test
        //     GROUP BY model_id
        // ) num_of_children ON p.id = num_of_children.model_id
        //  WHERE num BETWEEN $1 AND $2 AND name ILIKE $3 ORDER BY id ASC"#);

        let mut q = q.with_count();

        // "#
        //         assert_eq!(
        //             q.sql(),
        //             "SELECT COUNT(*) OVER() as total_count, p.*, COALESCE(volume_of_children.value, 0) AS volume_of_children, COALESCE(num_of_children.value, 0) AS num_of_children FROM query_builder_test p \nLEFT JOIN (\n    SELECT model_id, SUM(volumes) AS value\n    FROM child_model_query_builder_test \n    GROUP BY model_id\n) volume_of_children ON p.id = volume_of_children.model_id\n \nLEFT JOIN (\n    SELECT model_id, COUNT(id) AS value\n    FROM child_model_query_builder_test \n    GROUP BY model_id\n) num_of_children ON p.id = num_of_children.model_id\n WHERE num BETWEEN $1 AND $2 AND name ILIKE $3 ORDER BY id ASC"
        //         );

        //         let sq = QueryModelSummary::query_builder().sql();
        //         assert_eq!(
        //             sq,
        //             "SELECT COUNT(*) OVER() as total_count,  COUNT(*) OVER() as total_count, p.id, p.created_at, p.updated_at, p.name, p.num, p.children, COALESCE(volume_of_children.value, 0) AS volume_of_children, COALESCE(num_of_children.value, 0) AS num_of_children FROM query_builder_test p \nLEFT JOIN (\n    SELECT model_id, SUM(volumes) AS value\n    FROM child_model_query_builder_test \n    GROUP BY model_id\n) volume_of_children ON p.id = volume_of_children.model_id\n \nLEFT JOIN (\n    SELECT model_id, COUNT(id) AS value\n    FROM child_model_query_builder_test \n    GROUP BY model_id\n) num_of_children ON p.id = num_of_children.model_id\n "
        //         );

        let docs: Vec<QueryModel> = QueryModel::query_builder()
            .name_contains(name.clone())
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 20);

        let doc: QueryModel = q
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(doc.name, format!("{} 0-true", name));

        let docs: Vec<QueryModel> = q
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 12);
        let mut total: i64 = 0;

        let docs: Vec<QueryModel> = q
            .clone()
            .limit(3)
            .query()
            .map(|r: PgRow| {
                use sqlx::Row;
                total = r.get("total_count");
                r.into()
            })
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 3);
        assert_eq!(total, 12);
        assert_eq!(docs[0].name, format!("{} 0-true", name));
        assert_eq!(docs[1].name, format!("{} 1-true", name));
        assert_eq!(docs[2].name, format!("{} 2-true", name));

        let docs: Vec<QueryModel> = q
            .clone()
            .limit(6)
            .page(2)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 6);
        assert_eq!(docs[0].name, format!("{} 0-false", name));
        assert_eq!(docs[1].name, format!("{} 1-false", name));
        assert_eq!(docs[2].name, format!("{} 2-false", name));

        let docs: QueryModel = QueryModel::query_builder()
            .name_contains(name.clone())
            .is_like_is_false()
            .order_by_id_desc()
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(docs.name, format!("{} 9-false", name));

        let docs: QueryModel = QueryModel::query_builder()
            .name_contains(name.clone())
            .is_like_is_true()
            .order_by_id_asc()
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(docs.name, format!("{} 0-true", name));

        let docs: Vec<QueryModel> = QueryModel::query_builder()
            .name_contains(name.clone())
            .num_greater_than(5)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 8);

        for doc in docs {
            assert!(doc.num > 5);
        }

        let docs: Vec<QueryModel> = QueryModel::query_builder()
            .name_contains(name.clone())
            .num_greater_than_equals(5)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 10);

        for doc in docs {
            assert!(doc.num >= 5);
        }

        let docs: Vec<QueryModel> = QueryModel::query_builder()
            .name_contains(name.clone())
            .num_less_than(5)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 10);

        for doc in docs {
            assert!(doc.num < 5);
        }

        let docs: Vec<QueryModel> = QueryModel::query_builder()
            .name_contains(name.clone())
            .num_less_than_equals(5)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 12);

        for doc in docs {
            assert!(doc.num <= 5);
        }

        let docs: Vec<QueryModel> = QueryModel::query_builder()
            .name_contains(name.clone())
            .num_equals(5)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 2);

        for doc in docs {
            assert!(doc.num == 5);
        }

        let docs: Vec<QueryModel> = QueryModel::query_builder()
            .name_contains(name.clone())
            .num_not_equals(5)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(docs.len(), 18);

        for doc in docs {
            assert!(doc.num != 5);
        }

        let test_name = format!("test-{}", now);

        let doc = repo
            .insert(test_name.clone(), description, status, 10, false)
            .await
            .unwrap();

        let docs: QueryModel = QueryModel::query_builder()
            .name_not_contains("name".to_string())
            .name_contains(format!("{}", now))
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(docs.name, test_name);

        let child_name = format!("child-{}", now);

        repo_child
            .insert(doc.id, child_name.clone(), 5)
            .await
            .unwrap();
        repo_child
            .insert(doc.id, child_name.clone(), 10)
            .await
            .unwrap();
        repo_child
            .insert(doc.id, child_name.clone(), 15)
            .await
            .unwrap();

        let docs: QueryModel = QueryModel::query_builder()
            .name_contains(test_name.clone())
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(docs.volume_of_children, 30);
        assert_eq!(docs.num_of_children, 3);
        assert_eq!(docs.max_volume_of_children, 15, "{:?}", test_name);
        assert_eq!(docs.min_volume_of_children, 5);
        assert_eq!(docs.has_children, true);
        assert_eq!(docs.avg_volume_of_children, 10.0);
        assert_eq!(docs.children.len(), 3);

        // Many to Many tests
        let many_repo = ManyModels::get_repository(pool.clone());
        let res = many_repo.create_table().await;
        assert!(res.is_ok(), "{:?}", res);

        let summary: QueryModelWithManySummary = QueryModelWithManySummary::query_builder()
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&pool)
            .await
            .unwrap();

        let data: QueryModelWithMany = QueryModelWithMany::query_builder(0)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(data.many.len(), 0);
        assert_eq!(data.has_many, false);

        many_repo
            .insert_with_dependency(doc.id, "many-1".to_string())
            .await
            .unwrap();
        many_repo
            .insert_with_dependency(doc.id, "many-2".to_string())
            .await
            .unwrap();
        let m = many_repo
            .insert_with_dependency(doc.id, "many-3".to_string())
            .await
            .unwrap();

        let data: QueryModelWithMany = QueryModelWithMany::query_builder(m.id)
            .id_equals(doc.id)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(
            data.many.len(),
            3,
            "many_id: {:?} model_id {:?}",
            m.id,
            doc.id
        );
        assert_eq!(data.has_many, true);

        let data: QueryModelWithMany = QueryModelWithMany::query_builder(m.id)
            .id_equals(doc.id)
            .query()
            .map(|r: PgRow| r.into())
            .fetch_one(&pool)
            .await
            .unwrap();

        let mut q = QueryModel::query_builder();
        let q = q.clone().id_equals(ids[0]) | q.clone().id_equals(ids[1]);
        let docs = q
            .clone()
            .order_by_id_asc()
            .query()
            .map(QueryModel::from)
            .fetch_all(&pool)
            .await
            .unwrap();

        let s = q.clone().order_by_id_asc().sql();
        assert_eq!(docs.len(), 2, "{}", s);
        assert_eq!(docs[0].id, ids[0], "{:?} {:?} {}", docs, ids, s);
        assert_eq!(docs[1].id, ids[1], "{:?} {:?} {}", docs, ids, s);
    }
}
