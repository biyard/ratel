use by_axum::logger::root;
use controllers::v1::topic::TopicControllerV1;
use dto::error::ServiceError;
use tokio::net::TcpListener;

mod controllers {
    pub mod v1 {
        pub mod topic;
        pub mod users;
    }
}

pub mod config;

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

    let app = by_axum::new().nest("/v1/topics", TopicControllerV1::route()?);

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    slog::info!(log, "listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
