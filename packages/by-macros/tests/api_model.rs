#![allow(unused)]
#[cfg(feature = "server")]
use by_axum::aide;

use by_macros::api_model;

pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

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

#[cfg(not(feature = "server"))]
#[api_model(base = "/topics/v1", iter_type=QueryResponse, table = topics)] // rename is supported but usually use default(snake_case)
pub struct Topic {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(read_action = user_info)]
    pub wallet_address: String,
    #[api_model(read_action = [check_email,user_info])]
    pub email: String,
    #[api_model(summary, action = create)]
    pub title: String,
    #[api_model(summary, queryable, query_action = search_by, action = create, action_by_id = update)]
    pub description: String,
    #[api_model(summary, queryable, action_by_id = update, read_action = user_info)]
    pub status: i32,
    #[api_model(summary, query_action = [search_by, date_from], auto  = insert)]
    pub created_at: i64,
    #[api_model(many_to_many = topic_user_likes, foreign_table_name = users, foreign_key = id, foreign_key_type = TEXT)]
    pub is_liked: bool,

    #[api_model(auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(action_by_id = like, related = CommentRequest)]
    #[api_model(action = comment, related = Comment)]
    #[api_model(one_to_many = comments, foreign_key = topic_id)]
    pub comments: Vec<Comment>,

    #[api_model(action_by_id = update)]
    pub tags: Vec<String>,
}

#[cfg(not(feature = "server"))]
#[api_model(base = "/topics/v1/:topic-id/comments", iter_type=QueryResponse)]
pub struct Comment {
    pub id: i64,
    #[api_model(action = comment, related = String, read_action = search_by)]
    pub content: String,
    #[api_model(action_by_id = update, related = i64)]
    pub updated_at: i64,

    #[api_model(many_to_one = topics, foreign_key = id, foreign_key_type = TEXT)]
    pub topic_id: i64,
}

#[cfg(not(feature = "server"))]
#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CommentRequest {
    pub comment_id: i64,
    pub is_liked: bool,
}

#[cfg(not(feature = "server"))]
mod normal {
    use super::*;

    #[test]
    fn test_macro_expansion_topic() {
        let _read_action = TopicReadAction {
            action: Some(TopicReadActionType::CheckEmail),
            wallet_address: None,
            email: Some("test".to_string()),
            status: Some(1),
        };

        let _ = TopicReadAction::new().user_info("wallet".to_string(), "email".to_string(), 1);
        let _ = TopicReadAction::new().check_email("email".to_string());

        let q = TopicQuery {
            action: Some(TopicQueryActionType::DateFrom),
            size: 10,
            bookmark: None,
            description: None,
            status: Some(1),
            created_at: None,
        };

        assert_eq!(q.status, Some(1));
        assert_eq!(q.size, 10);
        assert_eq!(q.bookmark, None);
        assert_eq!(q.description, None);

        let _q = TopicQuery::new(10)
            .with_bookmark("test".to_string())
            .search_by("test".to_string(), 0);

        let q = TopicQuery::new(10)
            .with_bookmark("test".to_string())
            .date_from(1);

        assert_eq!(
            format!("{}", TopicParam::Query(q.clone())),
            "param-type=query&size=10&bookmark=test&action=date-from&created-at=1"
        );

        let summary = TopicSummary::default();
        assert_eq!(summary.id, 0);
        assert_eq!(summary.title, "".to_string());
        assert_eq!(summary.description, "".to_string());
        assert_eq!(summary.status, 0);
        assert_eq!(summary.created_at, 0);

        let create_request = TopicCreateRequest {
            title: "title".to_string(),
            description: "description".to_string(),
        };
        assert_eq!(create_request.title, "title".to_string());
        assert_eq!(create_request.description, "description".to_string());

        let update_request = TopicUpdateRequest {
            description: "description".to_string(),
            status: 1,
            tags: vec!["tag".to_string()],
        };
        assert_eq!(update_request.description, "description".to_string());
        assert_eq!(update_request.status, 1);
        assert_eq!(update_request.tags, vec!["tag".to_string()]);

        let create_request = TopicAction::Create(create_request);
        let update_request = TopicByIdAction::Update(update_request);
        let like_request = TopicByIdAction::Like(CommentRequest {
            comment_id: 1,
            is_liked: true,
        });
        let comment_request = TopicAction::Comment(Comment {
            id: 1,
            content: "content".to_string(),
            updated_at: 0,
            topic_id: 1,
        });

        let cli = Topic::get_client("");
        let _ = cli.get(1);
        let _ = cli.query(q);
        let _ = cli.act(create_request);
        let _ = cli.act(comment_request);
        let _ = cli.act_by_id(1, update_request);
        let _ = cli.act_by_id(1, like_request);
        let _ = cli.user_info("wallet".to_string(), "email".to_string(), 1);
        let _ = cli.check_email("email".to_string());
        // let _ = cli.search_by("test".to_string(), 0);
        let _ = cli.create("title".to_string(), "description".to_string());
        let _ = cli.update(1, "description".to_string(), 1, vec!["tag".to_string()]);
        let _ = cli.search_by(
            1,
            Some("bookmark".to_string()),
            "description".to_string(),
            10,
        );
        let _ = cli.date_from(10, None, 20240101);
    }

    #[test]
    fn test_macro_expansion_comment() {
        let cli = Comment::get_client("");
        let _ = cli.get(1, 1);
        let _ = cli.query(1, CommentQuery::new(10));
        let _ = cli.act(1, CommentAction::Comment("content".to_string()));
        let _ = cli.comment(1, "content".to_string());
        let _ = cli.search_by(1, "content".to_string());
        let _ = cli.update(1, 1, 100);
        let _ = cli.act_by_id(1, 1, CommentByIdAction::Update(100));
    }
}

#[cfg(feature = "server")]
mod server_tests {
    use sqlx::Postgres;
    use std::time::SystemTime;
    use validator::Validate;

    use super::*;

    #[derive(validator::Validate)]
    #[api_model(base = "/users/v1", read_action = user_info, table = test_users, iter_type=QueryResponse)]
    pub struct User {
        #[api_model(primary_key)]
        pub id: i64,
        #[api_model(auto = insert)]
        pub created_at: u64,
        #[api_model(auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(action = signup)]
        pub nickname: String,
        #[api_model(unique, read_action=check_principal)]
        pub principal: String,
        #[api_model(action = signup, read_action = check_email, unique, queryable, action_by_id = update)]
        #[validate(email)]
        pub email: String,
        #[api_model(action = signup)]
        pub profile_url: String,
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

    #[test]
    fn test_macro_expansion_user() {
        let _ = User {
            id: 1,
            created_at: 0,
            updated_at: 0,
            principal: "principal".to_string(),
            nickname: "nickname".to_string(),
            email: "email".to_string(),
            profile_url: "profile_url".to_string(),
        };

        let mut read = UserReadAction {
            action: Some(UserReadActionType::CheckEmail),
            email: Some("email".to_string()),
            principal: Some("principal".to_string()),
        };

        assert!(read.validate().is_err());

        read.email = Some("abc@test.com".to_string());
        assert!(read.validate().is_ok());

        let mut q = UserQuery {
            size: 10,
            bookmark: None,
            email: Some("email".to_string()),
        };
        assert!(q.validate().is_err());

        q.email = Some("abc@test.com".to_string());
        assert!(q.validate().is_ok());

        let mut action = UserSignupRequest {
            nickname: "nickname".to_string(),
            email: "email".to_string(),
            profile_url: "profile_url".to_string(),
        };
        assert!(action.validate().is_err());

        action.email = "abc@test.com".to_string();
        assert!(action.validate().is_ok());

        assert!(UserAction::Signup(action).validate().is_ok());

        let mut action = UserUpdateRequest {
            email: "email".to_string(),
        };
        assert!(action.validate().is_err());

        action.email = "abc@test.com".to_string();
        assert!(action.validate().is_ok());

        assert!(UserByIdAction::Update(action).validate().is_ok());

        let cli = User::get_client("");
        let _ = cli.user_info();
        let _ = cli.check_email("email".to_string());
    }

    #[tokio::test]
    async fn test_db_create() {
        db_setup().await;
        use sqlx::{postgres::PgPoolOptions, Postgres};
        let _ = tracing_subscriber::fmt::try_init();

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let email = format!("test-{}@test.com", now);
        let principal = format!("{}-principal", now);

        let pool: sqlx::Pool<Postgres> = PgPoolOptions::new()
            .max_connections(5)
            .connect(
                option_env!("DATABASE_URL")
                    .unwrap_or("postgres://postgres:postgres@localhost:5432/test"),
            )
            .await
            .unwrap();

        let repo = User::get_repository(pool.clone());
        repo.create_table().await.unwrap();
        repo.insert(
            "nickname".to_string(),
            principal.clone(),
            email.clone(),
            "profile_url".to_string(),
        )
        .await
        .unwrap();

        match repo
            .insert(
                "nickname".to_string(),
                principal.clone(),
                email.clone(),
                "profile_url".to_string(),
            )
            .await
        {
            Ok(_) => assert!(false, "Unique constraint violation"),
            Err(e) => {
                println!("{}", e);
            }
        };

        let user = match repo
            .find_one(&UserReadAction::new().check_email(email.clone()))
            .await
        {
            Ok(v) => v,
            Err(e) => {
                assert!(false, "Failed to fetch {e}");
                return;
            }
        };

        assert_eq!(user.principal, principal);
        assert_eq!(user.email, email);

        let QueryResponse {
            items: users_1,
            total_count: total,
        } = match repo.find(&UserQuery::new(5).with_page(1)).await {
            Ok(v) => v,
            Err(e) => {
                assert!(false, "Failed to fetch {e}");
                return;
            }
        };
        assert_eq!(total > 0, true);
        assert!(
            users_1.len() == (total as usize) || users_1.len() == 5,
            "incorrect length"
        );

        repo.insert(
            "nickname".to_string(),
            format!("2{}", principal),
            format!("2{}", email),
            "profile_url".to_string(),
        )
        .await
        .unwrap();

        repo.insert(
            "nickname".to_string(),
            format!("3{}", principal),
            format!("3{}", email),
            "profile_url".to_string(),
        )
        .await
        .unwrap();

        let QueryResponse {
            items: users_2,
            total_count: total2,
        } = match repo.find(&UserQuery::new(2).with_page(1)).await {
            Ok(v) => v,
            Err(e) => {
                assert!(false, "Failed to fetch {e}");
                return;
            }
        };

        assert_eq!(total2 > 0, true);
        assert_eq!(users_2.len(), 2, "incorrect length; it must be two");
    }
}
