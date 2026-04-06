use crate::common::aws_sdk_dynamodb;
use crate::common::mcp::mcp_router;
use crate::common::models::auth::User;
use crate::common::types::UserType;
use crate::common::utils::password::hash_password;
use axum::Router;

use crate::App;

#[derive(Clone)]
pub struct TestContext {
    pub app: Router,
    pub ddb: aws_sdk_dynamodb::Client,
    pub test_user: (User, axum::http::HeaderMap),
}

impl TestContext {
    pub async fn setup() -> Self {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .try_init();

        let config = crate::config::get();
        let cli = config.common.dynamodb();

        let session_layer = crate::common::middlewares::session_layer::get_session_layer(
            cli,
            config.common.env.to_string(),
        );

        let mcp_router = crate::common::mcp::mcp_router();
        let dioxus_router = dioxus::server::router(App).merge(mcp_router);
        let app = dioxus_router.layer(session_layer);
        crate::common::mcp::set_app_router(app.clone());

        let ddb = cli.clone();
        let test_user = create_user_session(app.clone(), &ddb).await;

        TestContext {
            app,
            ddb,
            test_user,
        }
    }

    pub async fn create_another_user(&self) -> (User, axum::http::HeaderMap) {
        create_user_session(self.app.clone(), &self.ddb).await
    }
}

pub fn create_user_name() -> String {
    format!("user{}", uuid::Uuid::new_v4())
}

pub fn create_nick_name() -> String {
    let short_uuid = &uuid::Uuid::new_v4().simple().to_string()[..6];
    format!("nickname{}", short_uuid)
}

pub async fn create_test_user(cli: &aws_sdk_dynamodb::Client) -> User {
    let profile = "https://ratel.foundation/images/default-profile.png".to_string();
    let username = create_user_name();
    let nickname = create_nick_name();
    let email = format!("a+{}@example.com", nickname);

    let user = User::new(
        nickname,
        email,
        profile,
        true,
        true,
        UserType::Individual,
        username,
        Some("password".to_string()),
    );
    user.create(cli).await.unwrap();

    user
}

pub async fn create_user_session(
    app: Router,
    cli: &aws_sdk_dynamodb::Client,
) -> (User, axum::http::HeaderMap) {
    let uid = uuid::Uuid::new_v4().to_string();
    let email = format!("{}@example.com", uid);
    let password = hash_password(&uid);
    let user = User::new(
        format!("displayName{}", uid),
        email.clone(),
        "https://metadata.ratel.foundation/ratel/default-profile.png".to_string(),
        true,
        true,
        UserType::Individual,
        uid.clone(),
        Some(password),
    );

    user.create(cli).await.expect("Failed to create user");

    let (_, header, _) = crate::test_post! {
        app: app,
        path: "/api/auth/login",
        body: {
            "req": {
                "email": email,
                "password": uid,
                "code": "000000"
            }
        }
    };
    let session_cookie = header
        .get("set-cookie")
        .expect("No set-cookie header found")
        .to_str()
        .expect("Failed to convert set-cookie header to str")
        .to_string();
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("cookie", session_cookie.parse().unwrap());
    (user, headers)
}
