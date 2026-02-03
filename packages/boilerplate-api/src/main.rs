use boilerplate_api::*;
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let app = api_main().await?;

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
