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

    let dioxus_router = dioxus::server::router(app);
    let app = dioxus_router.layer(session_layer);

    #[cfg(not(feature = "lambda"))]
    dioxus::serve(move || {
        let app = app.clone();

        async move { Ok(app) }
    });

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
                            event_bridge_handler(LambdaEvent::new(eb_event, ctx)).await?;
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

#[cfg(feature = "lambda")]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventBridgeEnvelope {
    pub source: String,
    #[serde(rename = "detail-type")]
    pub detail_type: String,
    pub detail: serde_json::Value,
    #[serde(default)]
    pub account: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub time: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub resources: Vec<String>,
}

#[cfg(feature = "lambda")]
async fn event_bridge_handler(
    event: lambda_runtime::LambdaEvent<EventBridgeEnvelope>,
) -> Result<(), lambda_runtime::Error> {
    let (envelope, _ctx) = event.into_parts();
    tracing::info!(
        source = %envelope.source,
        detail_type = %envelope.detail_type,
        "Received EventBridge event"
    );

    match (envelope.source.as_str(), envelope.detail_type.as_str()) {
        ("ratel.dynamodb.stream", "TimelineUpdate") => {
            handle_timeline_update(envelope.detail).await?;
        }
        ("ratel.dynamodb.stream", "PopularPostUpdate") => {
            handle_popular_post_update(envelope.detail).await?;
        }
        _ => {
            tracing::warn!(
                "Unhandled EventBridge event: source={}, detail-type={}",
                envelope.source,
                envelope.detail_type
            );
        }
    }

    Ok(())
}

#[cfg(feature = "lambda")]
async fn handle_timeline_update(
    detail: serde_json::Value,
) -> Result<(), lambda_runtime::Error> {
    use crate::common::types::{EntityType, Partition};

    // Parse the DynamoDB stream record from the EventBridge detail
    // Expected format: { "post_pk": "FEED#...", "author_pk": "USER#...", "created_at": 123456 }
    let post_pk_str = detail
        .get("post_pk")
        .and_then(|v| v.as_str())
        .ok_or("missing post_pk in detail")?;
    let author_pk_str = detail
        .get("author_pk")
        .and_then(|v| v.as_str())
        .ok_or("missing author_pk in detail")?;
    let created_at = detail
        .get("created_at")
        .and_then(|v| v.as_i64())
        .ok_or("missing created_at in detail")?;

    let post_pk: Partition = post_pk_str
        .parse()
        .map_err(|e| format!("invalid post_pk: {}", e))?;
    let author_pk: Partition = author_pk_str
        .parse()
        .map_err(|e| format!("invalid author_pk: {}", e))?;

    tracing::info!(
        "Timeline update: post_pk={}, author_pk={}, created_at={}",
        post_pk,
        author_pk,
        created_at
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    crate::features::timeline::services::fan_out_timeline_entries(
        cli,
        &post_pk,
        &author_pk,
        created_at,
    )
    .await
    .map_err(|e| {
        tracing::error!("Timeline fan-out failed: {}", e);
        lambda_runtime::Error::from(format!("Timeline fan-out failed: {}", e))
    })?;

    Ok(())
}

#[cfg(feature = "lambda")]
async fn handle_popular_post_update(
    detail: serde_json::Value,
) -> Result<(), lambda_runtime::Error> {
    use crate::common::types::Partition;

    let post_pk_str = detail
        .get("post_pk")
        .and_then(|v| v.as_str())
        .ok_or("missing post_pk in detail")?;
    let author_pk_str = detail
        .get("author_pk")
        .and_then(|v| v.as_str())
        .ok_or("missing author_pk in detail")?;
    let created_at = detail
        .get("created_at")
        .and_then(|v| v.as_i64())
        .ok_or("missing created_at in detail")?;
    let likes = detail.get("likes").and_then(|v| v.as_i64()).unwrap_or(0);
    let comments = detail
        .get("comments")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let shares = detail.get("shares").and_then(|v| v.as_i64()).unwrap_or(0);

    if !crate::features::timeline::services::is_popular(likes, comments, shares) {
        return Ok(());
    }

    let post_pk: Partition = post_pk_str
        .parse()
        .map_err(|e| format!("invalid post_pk: {}", e))?;
    let author_pk: Partition = author_pk_str
        .parse()
        .map_err(|e| format!("invalid author_pk: {}", e))?;

    tracing::info!(
        "Popular post fan-out: post_pk={}, likes={}, comments={}, shares={}",
        post_pk,
        likes,
        comments,
        shares
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    crate::features::timeline::services::fan_out_popular_post(cli, &post_pk, &author_pk, created_at)
        .await
        .map_err(|e| {
            tracing::error!("Popular post fan-out failed: {}", e);
            lambda_runtime::Error::from(format!("Popular post fan-out failed: {}", e))
        })?;

    Ok(())
}
