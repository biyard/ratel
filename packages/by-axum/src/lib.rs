// pub mod auth;
pub mod axum;

pub use tower_http::cors;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

mod docs;
use std::sync::Arc;

use ::axum::{Extension, Json, Router};

pub use logger as log;
use router::BiyardRouter;

pub mod logger;
pub mod router;
pub use aide;

pub use by_types::ApiError;
pub type Result<T, E> = std::result::Result<Json<T>, ApiError<E>>;
pub use schemars;
use tracing_subscriber::EnvFilter;

pub fn new() -> BiyardRouter {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from(option_env!("RUST_LOG").unwrap_or("info")))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    BiyardRouter::new()
}

pub fn finishing(app: BiyardRouter) -> Router {
    let mut api = app.open_api;
    app.inner
        .finish_api(&mut api)
        .layer(Extension(Arc::new(api)))
}

pub async fn serve(
    _tcp_listener: tokio::net::TcpListener,
    app: BiyardRouter,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let app = app.layer(
        CorsLayer::new()
            .allow_origin(AllowOrigin::mirror_request())
            .allow_credentials(true)
            .allow_methods(AllowMethods::mirror_request())
            .allow_headers(AllowHeaders::mirror_request()),
    );
    serve_wo_cors_layer(_tcp_listener, app).await
}

pub async fn serve_wo_cors_layer(
    _tcp_listener: tokio::net::TcpListener,
    app: BiyardRouter,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut api = app.open_api;
    let app = app
        .inner
        .finish_api(&mut api)
        .layer(Extension(Arc::new(api)));

    axum::serve(_tcp_listener, app).await?;

    Ok(())
}
