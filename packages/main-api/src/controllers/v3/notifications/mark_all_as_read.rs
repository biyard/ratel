use crate::{
    features::notification::{MarkAsReadResponse, Notification, NotificationQueryOption},
    types::notification_status::NotificationStatus,
    utils::time::get_now_timestamp_millis,
    *,
};

pub async fn mark_all_as_read_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
) -> std::result::Result<Json<MarkAsReadResponse>, Error> {
    let user_pk = &user.pk;
    let now = get_now_timestamp_millis();

    // Get all unread notifications
    let opt = NotificationQueryOption::builder()
        .limit(100)
        .scan_index_forward(false);

    let (notifications, _) =
        Notification::find_by_user_notifications(&dynamo.client, user_pk.to_string(), opt)
            .await?;

    let unread_notifications: Vec<_> = notifications
        .into_iter()
        .filter(|n| n.status == NotificationStatus::Unread)
        .collect();

    let mut updated_count = 0;

    // Build transactions for marking all unread notifications as read
    if !unread_notifications.is_empty() {
        // Process in batches of 100 (DynamoDB transaction limit)
        for chunk in unread_notifications.chunks(100) {
            let mut transactions = Vec::new();

            for notification in chunk {
                let update_tx = Notification::updater(&notification.pk, &notification.sk)
                    .with_status(NotificationStatus::Read)
                    .with_readed_at(now)
                    .transact_write_item();

                transactions.push(update_tx);
                updated_count += 1;
            }

            dynamo
                .client
                .transact_write_items()
                .set_transact_items(Some(transactions))
                .send()
                .await
                .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
        }
    }

    Ok(Json(MarkAsReadResponse {
        success: true,
        updated_count,
    }))
}
