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
fn serve(app: fn() -> Element) {
    let cfg = crate::common::CommonConfig::default();

    let cli = cfg.dynamodb();
    let session_layer =
        crate::common::middlewares::session_layer::get_session_layer(cli, cfg.env.to_string());

    let mcp_router = crate::common::mcp::mcp_router();
    let membership_router = crate::features::membership::server::router();
    let dioxus_router = dioxus::server::router(app)
        .merge(mcp_router)
        .merge(membership_router);
    let app = dioxus_router.layer(session_layer);

    crate::common::mcp::set_app_router(app.clone());

    #[cfg(not(feature = "lambda"))]
    {
        #[cfg(feature = "local-dev")]
        {
            tracing::info!("Starting local-dev DynamoDB Stream poller");
            crate::common::stream_poller::spawn_stream_poller();
        }

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
