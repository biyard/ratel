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
    let dioxus_router = dioxus::server::router(app).merge(mcp_router);
    let app = dioxus_router.layer(session_layer);

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
    pub detail_type: DetailType,
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DetailType {
    TimelineUpdate,
    PopularPostUpdate,
    PopularSpaceUpdate,
    NotificationSend,
    PostVectorIndex,
    PostVectorDelete,
    AiModeratorReplyCheck,
    AiModeratorReplyIndex,
    #[serde(other)]
    Unknown,
}

#[cfg(feature = "lambda")]
impl DetailType {
    fn parse_detail<T: serde::de::DeserializeOwned>(
        detail: &serde_json::Value,
    ) -> Result<T, lambda_runtime::Error> {
        let new_image = detail
            .get("newImage")
            .ok_or("missing newImage in detail")?;

        let item: std::collections::HashMap<String, serde_dynamo::AttributeValue> =
            serde_json::from_value(new_image.clone())
                .map_err(|e| format!("failed to parse DynamoDB image: {}", e))?;

        serde_dynamo::from_item(item)
            .map_err(|e| lambda_runtime::Error::from(format!("failed to deserialize: {}", e)))
    }
}

#[cfg(feature = "lambda")]
async fn event_bridge_handler(
    event: lambda_runtime::LambdaEvent<EventBridgeEnvelope>,
) -> Result<(), lambda_runtime::Error> {
    let (envelope, _ctx) = event.into_parts();
    tracing::info!(
        detail_type = ?envelope.detail_type,
        "Received EventBridge event"
    );

    let result = match envelope.detail_type {
        DetailType::TimelineUpdate => {
            crate::features::timeline::services::fan_out_timeline_entries(DetailType::parse_detail(&envelope.detail)?).await
        }
        DetailType::PopularPostUpdate => {
            crate::features::timeline::services::fan_out_popular_post(DetailType::parse_detail(&envelope.detail)?).await
        }
        DetailType::PopularSpaceUpdate => {
            crate::features::timeline::services::fan_out_popular_space(DetailType::parse_detail(&envelope.detail)?).await
        }
        DetailType::NotificationSend => {
            let notification: crate::common::models::notification::Notification =
                DetailType::parse_detail(&envelope.detail)?;
            notification.process().await
        }
        DetailType::PostVectorIndex => {
            let post: crate::features::posts::models::Post =
                DetailType::parse_detail(&envelope.detail)?;
            crate::features::rag::qdrant::indexers::post_indexer::index_post(post).await
        }
        DetailType::PostVectorDelete => {
            let post: crate::features::posts::models::Post =
                DetailType::parse_detail(&envelope.detail)?;
            crate::features::rag::qdrant::indexers::post_indexer::delete_post_index(post).await
        }
        DetailType::AiModeratorReplyCheck => {
            let post: crate::features::spaces::pages::actions::actions::discussion::SpacePost =
                DetailType::parse_detail(&envelope.detail)?;
            crate::features::ai_moderator::services::event_handler::handle_ai_moderator_event(post).await
        }
        DetailType::AiModeratorReplyIndex => {
            let comment: crate::features::spaces::pages::actions::actions::discussion::SpacePostComment =
                DetailType::parse_detail(&envelope.detail)?;
            crate::features::rag::qdrant::indexers::reply_indexer::index_reply(comment).await
        }
        DetailType::Unknown => {
            tracing::warn!(
                "Unhandled EventBridge event: source={}",
                envelope.source,
            );
            Ok(())
        }
    };

    if let Err(ref e) = result {
        tracing::error!(
            detail_type = ?envelope.detail_type,
            source = %envelope.source,
            error = %e,
            "EventBridge handler failed"
        );
    }

    result.map_err(lambda_runtime::Error::from)?;

    Ok(())
}
