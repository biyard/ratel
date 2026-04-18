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
    let session_layer =
        crate::common::middlewares::session_layer::get_session_layer(cli, cfg.env.to_string());

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
            let design_dir =
                std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/design");
            crate::common::design_preview::merge_design_routes(app, &design_dir)
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
