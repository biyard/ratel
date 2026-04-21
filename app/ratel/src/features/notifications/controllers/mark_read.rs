use crate::common::*;
use crate::common::models::notification::{UNREAD_SENTINEL, UserInboxNotification};
use crate::features::auth::User;
use crate::features::notifications::types::NotificationsError;

#[post("/api/inbox/{inbox_id}/read", user: User)]
pub async fn mark_read_handler(inbox_id: UserInboxNotificationEntityType) -> Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let sk: EntityType = inbox_id.into();
    let existing = UserInboxNotification::get(cli, user.pk.clone(), Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("mark_read get failed: {e}");
            NotificationsError::MarkReadFailed
        })?
        .ok_or(NotificationsError::InboxEntryNotFound)?;

    if existing.is_read {
        return Ok(());
    }

    // Flip is_read and rewrite the GSI sort key to the read sentinel so the
    // row is excluded from `begins_with("U")` unread-list / unread-count
    // queries, without removing the attribute (the entity requires it).
    UserInboxNotification::updater(user.pk.clone(), sk)
        .with_is_read(true)
        .with_unread_created_at(UNREAD_SENTINEL.to_string())
        .execute(cli)
        .await
        .map_err(|e| {
            crate::error!("mark_read update failed: {e}");
            NotificationsError::MarkReadFailed
        })?;

    Ok(())
}
