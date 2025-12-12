use crate::{
    features::notification::{MarkAsReadRequest, MarkAsReadResponse, Notification},
    types::{EntityType, Partition, notification_status::NotificationStatus},
    utils::time::get_now_timestamp_millis,
    *,
};

pub async fn mark_as_read_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<MarkAsReadRequest>,
) -> std::result::Result<Json<MarkAsReadResponse>, Error> {
    let user_pk = &user.pk;
    let now = get_now_timestamp_millis();

    let mut updated_count = 0;

    // Build transactions for marking notifications as read
    let mut transactions = Vec::new();

    for notification_id in req.notification_ids.iter() {
        let pk = Partition::Notification(user_pk.to_string());
        let sk: EntityType = notification_id.clone().into();

        // Verify the notification belongs to the user by attempting to get it
        if let Some(_) =
            Notification::get(&dynamo.client, pk.to_string(), Some(sk.to_string())).await?
        {
            let update_tx = Notification::updater(pk, sk)
                .with_status(NotificationStatus::Read)
                .with_readed_at(now)
                .transact_write_item();

            transactions.push(update_tx);
            updated_count += 1;
        }
    }

    // Execute all updates in a transaction
    if !transactions.is_empty() {
        dynamo
            .client
            .transact_write_items()
            .set_transact_items(Some(transactions))
            .send()
            .await
            .map_err(Into::<aws_sdk_dynamodb::Error>::into)?;
    }

    Ok(Json(MarkAsReadResponse {
        success: true,
        updated_count,
    }))
}
