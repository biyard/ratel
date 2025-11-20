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

    // Verify the notification belongs to the user
    let notification = Notification::get(&dynamo.client, pk.to_string(), Some(sk.to_string()))
        .await?
        .ok_or(Error::NotFound("Notification not found".to_string()))?;

    // Verify ownership
    if notification.user_pk != *user_pk {
        return Err(Error::Unauthorized(
            "You don't have permission to delete this notification".to_string(),
        ));
    }

    // Delete the notification
    Notification::delete(&dynamo.client, pk.to_string(), Some(sk.to_string())).await?;

    Ok(Json(DeleteNotificationResponse { success: true }))
}
