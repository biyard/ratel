use crate::common::*;

pub fn run(app: fn() -> Element) {
    let cfg = crate::common::CommonConfig::default();

    crate::common::logger::init(cfg.log_level.into()).expect("logger failed to init");

    debug!("Logger initialized with level {:?}", cfg.log_level);

    // The tauri-web build does NOT use dioxus-fullstack — our
    // `by_macros::server_fn` proc-macro emits reqwest-based stubs that
    // read the backend base URL from `MOBILE_API_URL` at compile time
    // (see `common::fullstack::tauri_web::server_fn::base_url`). So
    // there's no global server URL to set here.

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
            // Android Tauri WebView serves the app from http(s)://tauri.localhost;
            // iOS WKWebView uses the custom scheme origin `tauri://localhost`.
            // With `allow_credentials(true)` the exact origin must be echoed, so
            // the iOS scheme has to be listed explicitly (a wildcard is invalid).
            bytes == b"http://tauri.localhost"
                || bytes == b"https://tauri.localhost"
                || bytes == b"tauri://localhost"
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
    let arcade_router = crate::features::arcade::server::router();
    let cross_posting_router = crate::features::cross_posting::server::router();
    let launchpad_partner_router = crate::features::launchpad_partner::server::router();
    let dioxus_router = dioxus::server::router(app)
        .merge(mcp_router)
        .merge(membership_router)
        .merge(arcade_router)
        .merge(cross_posting_router)
        .merge(launchpad_partner_router);
    // CatchPanicLayer turns any panic in the request future into a 500 response
    // instead of letting it propagate up the spawn_pinned worker thread, which
    // would terminate the worker (and drop the connection). Pairs with the
    // panic hook below so we still capture the backtrace in logs.
    //
    // log_request is the OUTERMOST layer so its post-handler log sees the
    // final response (status, latency) including anything cors / session
    // appended.
    let app = dioxus_router
        .layer(tower_http::catch_panic::CatchPanicLayer::new())
        .layer(cors_layer)
        .layer(session_layer)
        .layer(axum::middleware::from_fn(log_request));

    crate::common::mcp::set_app_router(app.clone());

    // Register arcade realtime channel handlers with the per-process
    // global hub. Each game registers its own channels here so the
    // SSE endpoint can resolve them by kind. Idempotent — re-registers
    // overwrite, which is what tests rely on for mock channels.
    // `serve` is synchronous (called before tokio main spawns the
    // server), so we block on the registration through whichever
    // runtime is available.
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        handle.block_on(
            crate::features::arcade::games::fact_or_fold::realtime::register_channels(),
        );
    } else {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime build for channel registration");
        rt.block_on(crate::features::arcade::games::fact_or_fold::realtime::register_channels());
    }

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

/// Axum middleware that logs every incoming request as a single string,
/// including the request body (buffered into memory, UTF-8 decoded when
/// possible). Authorization / Cookie are reported as flags only to keep
/// secrets out of logs.
///
/// Body is buffered up to 1 MiB; anything bigger is replaced with a
/// length-only summary. The buffered bytes are then rewrapped into a new
/// `Body` so the downstream handler still sees the original payload.
#[cfg(feature = "server")]
async fn log_request(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    const MAX_BODY_LOG: usize = 1024 * 1024;

    let method = req.method().clone();
    let uri = req.uri().clone();
    let headers = req.headers().clone();

    let origin = headers
        .get(axum::http::header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-")
        .to_string();
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-")
        .to_string();
    let content_type = headers
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-")
        .to_string();
    let content_length = headers
        .get(axum::http::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("-")
        .to_string();
    let has_auth = headers.contains_key(axum::http::header::AUTHORIZATION);
    let has_cookie = headers.contains_key(axum::http::header::COOKIE);

    let (parts, body) = req.into_parts();
    let body_bytes = match axum::body::to_bytes(body, MAX_BODY_LOG).await {
        Ok(b) => b,
        Err(e) => {
            tracing::warn!(target: "http", "failed to buffer request body: {e}");
            axum::body::Bytes::new()
        }
    };
    let body_summary = if body_bytes.is_empty() {
        "<empty>".to_string()
    } else {
        match std::str::from_utf8(&body_bytes) {
            Ok(s) if s.len() <= 2048 => s.to_string(),
            Ok(s) => format!("{}...({} bytes total)", &s[..2048], s.len()),
            Err(_) => format!("<binary {} bytes>", body_bytes.len()),
        }
    };
    let req = axum::extract::Request::from_parts(parts, axum::body::Body::from(body_bytes));

    let start = std::time::Instant::now();
    let response = next.run(req).await;
    let elapsed = start.elapsed();

    tracing::info!(
        target: "http",
        "{method} {uri} -> {status} {elapsed:.2?} origin={origin} ua={user_agent} ct={content_type} cl={content_length} auth={auth} cookie={cookie} body={body_summary}",
        method = method,
        uri = uri,
        status = response.status().as_u16(),
        elapsed = elapsed,
        origin = origin,
        user_agent = user_agent,
        content_type = content_type,
        content_length = content_length,
        auth = if has_auth { "yes" } else { "no" },
        cookie = if has_cookie { "yes" } else { "no" },
        body_summary = body_summary,
    );

    response
}
