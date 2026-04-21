use crate::common::*;
use crate::common::models::notification::UserInboxNotification;
use crate::features::auth::User;
use crate::features::notifications::types::{NotificationsError, UnreadCountResponse};

const UNREAD_COUNT_CAP: i64 = 100;

#[mcp_tool(
    name = "get_unread_count",
    description = "Return the count of unread notifications in the current user's inbox (capped at 100)."
)]
#[get("/api/inbox/unread-count", user: User)]
pub async fn get_unread_count_handler() -> Result<UnreadCountResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let mut count: i64 = 0;
    let mut bookmark: Option<String> = None;
    for _ in 0..4 {
        // Filter by the sparse GSI sort-key prefix so rows we rewrote to the
        // read sentinel ("R") are skipped — only "U#..." unread keys match.
        let opts = UserInboxNotification::opt_with_bookmark(bookmark)
            .sk("U".to_string())
            .limit(30);
        let (items, next) =
            UserInboxNotification::find_inbox_unread_by_user(cli, user.pk.clone(), opts)
                .await
                .map_err(|e| {
                    crate::error!("unread-count GSI query failed: {e}");
                    NotificationsError::ListFailed
                })?;
        count += items.len() as i64;
        if count >= UNREAD_COUNT_CAP {
            return Ok(UnreadCountResponse {
                count: UNREAD_COUNT_CAP,
            });
        }
        if next.is_none() {
            break;
        }
        bookmark = next;
    }
    Ok(UnreadCountResponse { count })
}
