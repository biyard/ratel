use crate::common::*;

pub fn run(app: fn() -> Element) {
    let cfg = crate::common::CommonConfig::default();

    crate::common::logger::init(cfg.log_level.into()).expect("logger failed to init");

    #[cfg(not(feature = "server"))]
    launch(app);

    #[cfg(feature = "server")]
    serve(app);
}

#[cfg(not(feature = "server"))]
fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}

#[cfg(feature = "server")]
fn install_panic_hook() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let payload = info
                .payload()
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| info.payload().downcast_ref::<String>().map(|s| s.as_str()))
                .unwrap_or("<non-string panic payload>");
            let location = info
                .location()
                .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
                .unwrap_or_else(|| "<unknown location>".to_string());
            tracing::error!(panic = %payload, location = %location, "server thread panicked");
            // Delegate to the previous hook so backtraces still print when
            // RUST_BACKTRACE=1 and tests still observe the panic normally.
            prev(info);
        }));
    });
}

#[cfg(feature = "server")]
fn serve(app: fn() -> Element) {
    install_panic_hook();

    let cfg = crate::common::CommonConfig::default();

    let cli = cfg.dynamodb();

    // Run pending migrations. No-op unless `MIGRATE=true` is set in the env.
    // Blocks server startup until done; set on exactly one instance per
    // release. The conditional UpdateItem in `LastBackfillVersion::advance_to`
    // is the safety net for accidental double-set.
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        if let Err(e) = handle.block_on(crate::common::migrations::run_migrations(cli)) {
            tracing::error!(error = %e, "migration runner failed; aborting startup");
            std::process::exit(1);
        }
    } else {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime build for migrations");
        if let Err(e) = rt.block_on(crate::common::migrations::run_migrations(cli)) {
            tracing::error!(error = %e, "migration runner failed; aborting startup");
            std::process::exit(1);
        }
    }

    let session_layer =
        crate::common::middlewares::session_layer::get_session_layer(cli, cfg.env.to_string());

    // Allow cross-origin requests from the Tauri Android/iOS WebView
    // (http://tauri.localhost / https://tauri.localhost) and from the
    // production frontend (https://ratel.foundation and its subdomains).
    // Web traffic is same-origin so it never triggers CORS — these entries
    // exist only for the Tauri shell and any subdomain frontends.
    //
    // CORS layer is placed OUTSIDE the session layer so preflight OPTIONS
    // requests never hit session lookup (they carry no credentials).
    let cors_layer = {
        use tower_http::cors::{AllowOrigin, CorsLayer};
        let allow = AllowOrigin::predicate(|origin, _req_parts| {
            let bytes = origin.as_bytes();
            bytes == b"http://tauri.localhost"
                || bytes == b"https://tauri.localhost"
                || bytes == b"https://ratel.foundation"
                || (bytes.starts_with(b"https://") && bytes.ends_with(b".ratel.foundation"))
        });
        CorsLayer::new()
            .allow_origin(allow)
            .allow_credentials(true)
            .allow_methods([
                axum::http::Method::GET,
                axum::http::Method::POST,
                axum::http::Method::PUT,
                axum::http::Method::PATCH,
                axum::http::Method::DELETE,
                axum::http::Method::OPTIONS,
            ])
            .allow_headers([
                axum::http::header::CONTENT_TYPE,
                axum::http::header::AUTHORIZATION,
                axum::http::header::ACCEPT,
                axum::http::header::COOKIE,
            ])
    };

    let mcp_router = crate::common::mcp::mcp_router();
    let membership_router = crate::features::membership::server::router();
    let dioxus_router = dioxus::server::router(app)
        .merge(mcp_router)
        .merge(membership_router);
    // CatchPanicLayer turns any panic in the request future into a 500 response
    // instead of letting it propagate up the spawn_pinned worker thread, which
    // would terminate the worker (and drop the connection). Pairs with the
    // panic hook below so we still capture the backtrace in logs.
    let app = dioxus_router
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .layer(cors_layer)
        .layer(session_layer);

    crate::common::mcp::set_app_router(app.clone());

    #[cfg(not(feature = "lambda"))]
    {
        #[cfg(feature = "local-dev")]
        {
            tracing::info!("Starting local-dev DynamoDB Stream poller");
            crate::common::stream_poller::spawn_stream_poller();
        }

        #[cfg(feature = "local-dev")]
        let app = {
            let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let design_dir = manifest_dir.join("assets/design");
            let app = crate::common::design_preview::merge_design_routes(app, &design_dir);

            // Repo root is app/ratel/../.. — mount the roadmap/ spec folder as /roadmap.
            let roadmap_dir = manifest_dir.join("../../roadmap");
            if roadmap_dir.exists() {
                crate::common::design_preview::merge_roadmap_routes(app, &roadmap_dir)
            } else {
                app
            }
        };

        dioxus::serve(move || {
            let app = app.clone();

            async move { Ok(app) }
        });
    }

    #[cfg(feature = "lambda")]
    {
        use lambda_http::tower::ServiceExt;
        use lambda_http::Service;
        use lambda_runtime::LambdaEvent;

        let app_future = async move {
            lambda_runtime::run(lambda_runtime::service_fn(
                move |event: LambdaEvent<serde_json::Value>| {
                    let app = app.clone();
                    async move {
                        let (payload, ctx) = event.into_parts();

                        if payload.get("source").is_some()
                            && payload.get("detail-type").is_some()
                            && payload.get("detail").is_some()
                        {
                            let eb_event: EventBridgeEnvelope = serde_json::from_value(payload)
                                .map_err(lambda_runtime::Error::from)?;
                            eb_event.proc().await?;
                            Ok::<serde_json::Value, lambda_runtime::Error>(
                                serde_json::json!({"statusCode": 200}),
                            )
                        } else {
                            let lambda_request: lambda_http::request::LambdaRequest =
                                serde_json::from_value(payload)
                                    .map_err(lambda_runtime::Error::from)?;
                            let mut adapter = lambda_http::Adapter::from(app);
                            let svc = adapter.ready().await.map_err(lambda_runtime::Error::from)?;

                            let resp = svc
                                .call(LambdaEvent::new(lambda_request, ctx))
                                .await
                                .map_err(lambda_runtime::Error::from)?;
                            serde_json::to_value(resp).map_err(lambda_runtime::Error::from)
                        }
                    }
                },
            ))
            .await
        };

        info!("Starting server in Lambda environment");
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            let _ = handle.block_on(app_future);
        } else {
            let _ = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(app_future);
        }
    }
}
