#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
#[cfg(test)]
pub mod update_into_tests {
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

    #[api_model(base = "/models", table = upinto, iter_type=QueryResponse)]
    pub struct UpdateInto {
        #[api_model(summary, primary_key, read_action = find_by_id)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary, action_by_id = update_desc)]
        pub name: String,

        #[api_model(action_by_id = update_desc)]
        pub description: String,

        #[api_model(action_by_id = update_status)]
        pub status: i32,
    }

    #[tokio::test]
    async fn test_update_into() {
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
        let desc = format!("desc-{}", now);
        let status = 1;

        let repo = UpdateInto::get_repository(pool.clone());
        repo.create_table().await;

        let doc = repo.insert(name.clone(), desc, status).await.unwrap();

        assert_eq!(doc.name, name);

        let new_name = format!("new-name-{}", now);
        let new_desc = format!("new-desc-{}", now);

        let req = UpdateIntoByIdAction::UpdateDesc(UpdateIntoUpdateDescRequest {
            name: new_name.clone(),
            description: new_desc.clone(),
        });

        repo.update(doc.id, req.into()).await.unwrap();

        let res: UpdateInto = repo
            .find_one(&UpdateIntoReadAction::new().find_by_id(doc.id.clone()))
            .await
            .unwrap();

        assert_eq!(res.name, new_name);
        assert_eq!(res.description, new_desc);
        assert_eq!(res.status, status);

        let req = UpdateIntoByIdAction::UpdateStatus(UpdateIntoUpdateStatusRequest { status: 2 });
        repo.update(doc.id, req.into()).await.unwrap();
        let res: UpdateInto = repo
            .find_one(&UpdateIntoReadAction::new().find_by_id(doc.id.clone()))
            .await
            .unwrap();

        assert_eq!(res.name, new_name);
        assert_eq!(res.description, new_desc);
        assert_eq!(res.status, 2);

        let req = UpdateIntoByIdAction::UpdateStatus(UpdateIntoUpdateStatusRequest { status: 2 });
        let req: UpdateIntoRepositoryUpdateRequest = req.into();
        repo.update(doc.id, req.with_name(name.clone()))
            .await
            .unwrap();

        let res: UpdateInto = repo
            .find_one(&UpdateIntoReadAction::new().find_by_id(doc.id.clone()))
            .await
            .unwrap();

        assert_eq!(res.name, name);
        assert_eq!(res.description, new_desc);
        assert_eq!(res.status, 2);
    }
}
