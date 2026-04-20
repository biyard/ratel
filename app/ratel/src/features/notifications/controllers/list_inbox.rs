use crate::common::*;
use crate::common::models::notification::UserInboxNotification;
use crate::features::auth::User;
use crate::features::notifications::types::{InboxNotificationResponse, NotificationsError};

#[get("/api/inbox?unread_only&bookmark", user: User)]
pub async fn list_inbox_handler(
    unread_only: Option<bool>,
    bookmark: Option<String>,
) -> Result<ListResponse<InboxNotificationResponse>> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let (items, next) = if unread_only.unwrap_or(false) {
        let opts = UserInboxNotification::opt_with_bookmark(bookmark)
            .scan_index_forward(false)
            .limit(30);
        UserInboxNotification::find_inbox_unread_by_user(cli, user.pk.clone(), opts)
            .await
            .map_err(|e| {
                crate::error!("list_inbox unread GSI failed: {e}");
                NotificationsError::ListFailed
            })?
    } else {
        // Filter sk by prefix so the partition scan doesn't pick up the User
        // record or any other entity sharing this user's partition key.
        let opts = UserInboxNotification::opt_with_bookmark(bookmark)
            .sk("USER_INBOX_NOTIFICATION".to_string())
            .scan_index_forward(false)
            .limit(30);
        UserInboxNotification::query(cli, user.pk.clone(), opts)
            .await
            .map_err(|e| {
                crate::error!("list_inbox pk scan failed: {e}");
                NotificationsError::ListFailed
            })?
    };

    let items: Vec<InboxNotificationResponse> = items.into_iter().map(Into::into).collect();
    Ok((items, next).into())
}
