use by_axum::{axum::middleware, logger::root};
use controllers::{assets::v1::AssetControllerV1, topic::v1::TopicControllerV1};
use dto::error::ServiceError;
use tokio::net::TcpListener;
use utils::middlewares::authorization_middleware;

mod controllers {
    pub mod topic {
        pub mod v1;
    }
    pub mod users {
        pub mod v1;
    }
    pub mod assembly_members {
        pub mod m1;
        pub mod v1;
    }
    pub mod assets {
        pub mod v1;
    }
}

pub mod config;
pub mod models;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    let log = root();

    easy_dynamodb::init(
        log.clone(),
        option_env!("AWS_ACCESS_KEY_ID")
            .expect("AWS_ACCESS_KEY_ID is required")
            .to_string(),
        option_env!("AWS_SECRET_ACCESS_KEY")
            .expect("AWS_SECRET_ACCESS_KEY is required")
            .to_string(),
        option_env!("AWS_REGION")
            .unwrap_or("ap-northeast-2")
            .to_string(),
        option_env!("TABLE_NAME")
            .expect("TABLE_NAME is required")
            .to_string(),
        "id".to_string(),
        None,
        None,
    );

    let app = by_axum::new()
        .nest("/assets/v1", AssetControllerV1::route()?)
        .nest("/topics/v1", TopicControllerV1::route()?)
        .nest(
            "/users/v1",
            controllers::users::v1::UserControllerV1::route()?,
        )
        .nest(
            "/assembly_members/v1",
            controllers::assembly_members::v1::AssemblyMemberControllerV1::route()?,
        )
        .nest(
            "/assembly_members/m1",
            controllers::assembly_members::m1::AssemblyMemberControllerM1::route()?,
        )
        .layer(middleware::from_fn(authorization_middleware));

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    slog::info!(log, "listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
