use crate::{
    features::notification::{DeleteNotificationResponse, Notification},
    types::{EntityType, NotificationEntityType, Partition},
    *,
};

pub async fn delete_notification_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(notification_id): Path<NotificationEntityType>,
) -> std::result::Result<Json<DeleteNotificationResponse>, Error> {
    let user_pk = &user.pk;
    let pk = Partition::Notification(user_pk.to_string());
    let sk: EntityType = notification_id.into();

    match Notification::delete(&dynamo.client, pk.to_string(), Some(sk.to_string())).await {
        Ok(_) => Ok(Json(DeleteNotificationResponse { success: true })),
        Err(e) => Err(e),
    }
}
