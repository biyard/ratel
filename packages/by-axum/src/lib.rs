// pub mod auth;
pub mod axum;

#[cfg(feature = "lambda")]
pub mod lambda_adapter;

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
        .with_env_filter(
            EnvFilter::from(option_env!("RUST_LOG").unwrap_or("debug"))
                .add_directive("rustls=info".parse().unwrap())
                .add_directive("aws_smithy_http_client=info".parse().unwrap())
                .add_directive("aws_smithy_runtime_api=info".parse().unwrap())
                .add_directive("aws_sdk_dynamodb=info".parse().unwrap())
                .add_directive("hyper_util=info".parse().unwrap())
                .add_directive("tower_http=info".parse().unwrap())
                .add_directive("aws_smithy_runtime=info".parse().unwrap()),
        )
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_ansi(
            option_env!("DISABLE_ANSI")
                .map(|e| e.to_lowercase() != "true")
                .unwrap_or(true),
        )
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

    #[cfg(not(feature = "lambda"))]
    axum::serve(_tcp_listener, app).await?;

    #[cfg(feature = "lambda")]
    {
        lambda_runtime::run(lambda_adapter::LambdaAdapter::from(app.into_service()))
            .await
            .unwrap();
    }

    Ok(())
}
