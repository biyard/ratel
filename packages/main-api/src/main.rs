use std::env;

use bdk::prelude::*;
use dto::Result;
use main_api::api_main::api_main;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let app = api_main().await?;

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
