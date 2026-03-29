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

    match envelope.detail_type {
        DetailType::TimelineUpdate => {
            handle_timeline_update(envelope.detail).await?;
        }
        DetailType::PopularPostUpdate => {
            handle_popular_post_update(envelope.detail).await?;
        }
        DetailType::PopularSpaceUpdate => {
            handle_popular_space_update(envelope.detail).await?;
        }
        DetailType::NotificationSend => {
            handle_notification_send(envelope.detail).await?;
        }
        DetailType::Unknown => {
            tracing::warn!(
                "Unhandled EventBridge event: source={}",
                envelope.source,
            );
        }
    }

    Ok(())
}

#[cfg(feature = "lambda")]
async fn handle_timeline_update(
    detail: serde_json::Value,
) -> Result<(), lambda_runtime::Error> {
    use crate::features::posts::models::Post;

    let post: Post = DetailType::parse_detail(&detail)?;

    tracing::info!(
        "Timeline update: post_pk={}, author_pk={}, created_at={}",
        post.pk,
        post.user_pk,
        post.created_at
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    crate::features::timeline::services::fan_out_timeline_entries(
        cli,
        &post.pk,
        &post.user_pk,
        post.created_at,
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
    use crate::features::posts::models::Post;

    let post: Post = DetailType::parse_detail(&detail)?;

    if !crate::features::timeline::services::is_popular(post.likes, post.comments, post.shares) {
        return Ok(());
    }

    tracing::info!(
        "Popular post fan-out: post_pk={}, likes={}, comments={}, shares={}",
        post.pk,
        post.likes,
        post.comments,
        post.shares
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    crate::features::timeline::services::fan_out_popular_post(cli, &post.pk, &post.user_pk, post.created_at)
        .await
        .map_err(|e| {
            tracing::error!("Popular post fan-out failed: {}", e);
            lambda_runtime::Error::from(format!("Popular post fan-out failed: {}", e))
        })?;

    Ok(())
}

#[cfg(feature = "lambda")]
async fn handle_popular_space_update(
    detail: serde_json::Value,
) -> Result<(), lambda_runtime::Error> {
    use crate::common::models::space::SpaceCommon;

    let space: SpaceCommon = DetailType::parse_detail(&detail)?;

    if !crate::features::timeline::services::is_popular_space(space.participants) {
        return Ok(());
    }

    tracing::info!(
        "Popular space fan-out: space_pk={}, participants={}",
        space.pk,
        space.participants
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    crate::features::timeline::services::fan_out_popular_space(
        cli,
        &space.pk,
        &space.post_pk,
        &space.user_pk,
        space.created_at,
    )
    .await
    .map_err(|e| {
        tracing::error!("Popular space fan-out failed: {}", e);
        lambda_runtime::Error::from(format!("Popular space fan-out failed: {}", e))
    })?;

    Ok(())
}

#[cfg(feature = "lambda")]
async fn handle_notification_send(
    detail: serde_json::Value,
) -> Result<(), lambda_runtime::Error> {
    use crate::common::models::notification::Notification;
    use crate::common::types::NotificationData;
    use crate::features::auth::models::EmailTemplate;
    use crate::features::auth::types::email_operation::EmailOperation;

    let notification: Notification = DetailType::parse_detail(&detail)?;

    tracing::info!(
        "Notification send: pk={}, status={:?}, data={:?}",
        notification.pk,
        notification.status,
        notification.data
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let ses = cfg.ses();

    match &notification.data {
        NotificationData::SendVerificationCode { code, email } => {
            let chars: Vec<char> = code.chars().collect();
            let operation = EmailOperation::SignupSecurityCode {
                display_name: email.clone(),
                code_1: chars.first().map(|c| c.to_string()).unwrap_or_default(),
                code_2: chars.get(1).map(|c| c.to_string()).unwrap_or_default(),
                code_3: chars.get(2).map(|c| c.to_string()).unwrap_or_default(),
                code_4: chars.get(3).map(|c| c.to_string()).unwrap_or_default(),
                code_5: chars.get(4).map(|c| c.to_string()).unwrap_or_default(),
                code_6: chars.get(5).map(|c| c.to_string()).unwrap_or_default(),
            };

            let template = EmailTemplate {
                targets: vec![email.clone()],
                operation,
            };
            template.send_email(ses).await.map_err(|e| {
                tracing::error!("Failed to send verification email: {}", e);
                lambda_runtime::Error::from(format!("Failed to send verification email: {}", e))
            })?;
        }
        NotificationData::SendSpaceInvitation {
            emails,
            space_title,
            space_content,
            author_profile_url,
            author_username,
            author_display_name,
            cta_url,
        } => {
            let operation = EmailOperation::SpaceInviteVerification {
                space_title: space_title.clone(),
                space_desc: space_content.clone(),
                author_profile: author_profile_url.clone(),
                author_display_name: author_display_name.clone(),
                author_username: author_username.clone(),
                cta_url: cta_url.clone(),
            };

            let template = EmailTemplate {
                targets: emails.clone(),
                operation,
            };
            template.send_email(ses).await.map_err(|e| {
                tracing::error!("Failed to send space invitation email: {}", e);
                lambda_runtime::Error::from(format!(
                    "Failed to send space invitation email: {}",
                    e
                ))
            })?;
        }
        NotificationData::None => {
            tracing::warn!("Received notification with no data, skipping");
        }
    }

    // Update notification status to Completed
    Notification::updater(notification.pk.clone(), notification.sk.clone())
        .with_status(crate::common::types::EventStatus::Completed)
        .execute(cli)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update notification status: {}", e);
            lambda_runtime::Error::from(format!("Failed to update notification status: {}", e))
        })?;

    Ok(())
}
