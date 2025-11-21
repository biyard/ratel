use crate::{
    features::notification::{DeleteNotificationResponse, Notification},
    types::{EntityType, Partition},
    *,
};
use aide::NoApi;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use bdk::prelude::*;

pub async fn delete_notification_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(notification_id): Path<String>,
) -> std::result::Result<Json<DeleteNotificationResponse>, Error> {
    let user_pk = &user.pk;
    let pk = Partition::Notification(user_pk.to_string());
    let sk = EntityType::Notification(notification_id);

    // Verify the notification exists and belongs to the user
    // Note: Because we use the user's PK in the query, we can only find notifications
    // that belong to this user. This provides implicit authorization.
    let _notification = Notification::get(&dynamo.client, pk.to_string(), Some(sk.to_string()))
        .await?
        .ok_or(Error::NotFound("Notification not found".to_string()))?;

    // Delete the notification
    Notification::delete(&dynamo.client, pk.to_string(), Some(sk.to_string())).await?;

    Ok(Json(DeleteNotificationResponse { success: true }))
}
