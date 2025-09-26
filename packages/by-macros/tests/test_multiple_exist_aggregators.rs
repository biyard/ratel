#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
mod test_multiple_exist_aggeregators {
    #![allow(unused)]
    use super::*;
    use by_axum::aide;
    use by_macros::api_model;
    use std::time::SystemTime;

    #[api_model(base = "/", table = test_multiple_aggregator_users)]
    pub struct User {
        #[api_model(primary_key)]
        pub id: i64,
        #[api_model(auto = [insert])]
        pub created_at: i64,
        #[api_model(auto = [insert, update])]
        pub updated_at: i64,
        #[api_model(summary, action = signup, unique)]
        pub email: String,
    }

    #[api_model(base = "/", table = test_multiple_aggregator_members)]
    pub struct Member {
        #[api_model(primary_key)]
        pub id: i64,
        #[api_model(auto = [insert])]
        pub created_at: i64,
        #[api_model(auto = [insert, update])]
        pub updated_at: i64,
        #[api_model(summary, action = create, unique)]
        pub name: String,
    }

    // Note: when fields have multiple foreign_primary_key(user_id) with exist aggregator.
    #[api_model(base = "/", table = test_multiple_aggregator_models)]
    pub struct Model {
        #[api_model(primary_key, read_action = find_by_id)]
        pub id: i64,
        #[api_model(auto = insert)]
        pub created_at: i64,
        #[api_model(auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary)]
        pub title: String,

        #[api_model(summary, many_to_many = test_multiple_aggregator_model_user_likes, table_name = test_multiple_aggregator_users, foreign_primary_key = user_id, foreign_reference_key = model_id, aggregator = exist)]
        pub liked: bool,

        #[api_model(summary, many_to_many = test_multiple_aggregator_model_member_followers, table_name = test_multiple_aggregator_member, foreign_primary_key = member_id, foreign_reference_key = model_id, aggregator = exist)]
        pub followed: bool,
    }

    #[api_model(base = "/", table = test_multiple_aggregator_model_user_likes)]
    pub struct ModelUserLikes {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(many_to_one = test_multiple_aggregator_models)]
        pub model_id: i64,
        #[api_model(many_to_one = test_multiple_aggregator_users)]
        pub user_id: i64,
    }

    #[api_model(base = "/", table = test_multiple_aggregator_model_member_followers)]
    pub struct ModelMemberFollows {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(many_to_one = test_multiple_aggregator_models)]
        pub model_id: i64,
        #[api_model(many_to_one = test_multiple_aggregator_members)]
        pub member_id: i64,
    }

    async fn db_setup(pool: &sqlx::Pool<sqlx::Postgres>) {
        let query = r#"
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at := EXTRACT(EPOCH FROM now()); -- seconds
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
    "#;
        sqlx::query(query).execute(pool).await.unwrap();

        let query = r#"
CREATE OR REPLACE FUNCTION set_created_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.created_at := EXTRACT(EPOCH FROM now()); -- seconds
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
    "#;

        sqlx::query(query).execute(pool).await.unwrap();
    }

    #[tokio::test]
    async fn test_user_org() {
        use sqlx::{postgres::PgPoolOptions, Postgres};
        let _ = tracing_subscriber::fmt::try_init();
        let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(
                option_env!("DATABASE_URL")
                    .unwrap_or("postgres://postgres:postgres@localhost:5432/test"),
            )
            .await
            .unwrap();

        db_setup(&pool).await;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let user_repo = User::get_repository(pool.clone());
        let member_repo = Member::get_repository(pool.clone());
        let model_repo = Model::get_repository(pool.clone());
        let like_repo = ModelUserLikes::get_repository(pool.clone());
        let follow_repo = ModelMemberFollows::get_repository(pool.clone());

        user_repo.create_this_table().await;
        member_repo.create_this_table().await;
        model_repo.create_this_table().await;
        like_repo.create_this_table().await;
        follow_repo.create_this_table().await;

        user_repo.create_related_tables().await;
        member_repo.create_related_tables().await;
        model_repo.create_related_tables().await;
        like_repo.create_related_tables().await;
        follow_repo.create_related_tables().await;

        let email = format!("{}@example.com", now);
        let user = user_repo.insert(email.clone()).await;
        assert!(user.is_ok(), "{:?}", user);
        let user = user.unwrap();

        let name = format!("member-{}", now);
        let member = member_repo.insert(name.clone()).await;
        assert!(member.is_ok(), "{:?}", member);
        let member = member.unwrap();

        let model = model_repo.insert("TEST 1".to_string()).await;
        assert!(model.is_ok(), "{:?}", model);
        let model = model.unwrap();

        let model_user_likes = like_repo.insert(model.id, user.id).await;
        assert!(model_user_likes.is_ok(), "{:?}", model_user_likes);
        let r = model_repo
            .find_one(
                user.id,
                member.id,
                &ModelReadAction::new().find_by_id(model.id),
            )
            .await;
        assert!(r.is_ok(), "{:?}", r);
        let r = r.unwrap();
        assert_eq!(r.liked, true);
        assert_eq!(r.followed, false);

        let r: std::result::Result<Model, sqlx::Error> = Model::query_builder(user.id, member.id)
            .id_equals(model.id)
            .query()
            .map(|r: sqlx::postgres::PgRow| {
                use sqlx::Row;
                r.into()
            })
            .fetch_one(&pool)
            .await;
        assert!(r.is_ok(), "{:?}", r);
        let r = r.unwrap();
        assert_eq!(r.liked, true);
        assert_eq!(r.followed, false);

        let model_user_followers = follow_repo.insert(model.id, member.id).await;

        assert!(model_user_followers.is_ok(), "{:?}", model_user_followers);

        let r: std::result::Result<Model, sqlx::Error> = Model::query_builder(user.id, member.id)
            .id_equals(model.id)
            .query()
            .map(|r: sqlx::postgres::PgRow| {
                use sqlx::Row;
                r.into()
            })
            .fetch_one(&pool)
            .await;
        assert!(r.is_ok(), "{:?}", r);
        let r = r.unwrap();
        assert_eq!(r.liked, true);
        assert_eq!(r.followed, true);
    }
}
