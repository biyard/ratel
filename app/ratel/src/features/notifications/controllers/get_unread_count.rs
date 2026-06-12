use crate::common::*;
use crate::common::models::notification::UserInboxNotification;
use crate::features::auth::User;
use crate::features::notifications::types::{NotificationsError, UnreadCountResponse};

const UNREAD_COUNT_CAP: i64 = 100;

#[mcp_tool(
    name = "get_unread_count",
    description = "Return the count of unread notifications in the current user's inbox (capped at 100)."
)]
#[get("/api/inbox/unread-count?space_id", user: User)]
pub async fn get_unread_count_handler(
    #[mcp(
        description = "If set, count only unread notifications belonging to this space (id without the SPACE# prefix)."
    )]
    space_id: Option<SpacePartition>,
) -> Result<UnreadCountResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // When scoping to a space, notifications carry their space only inside the
    // payload, so scan more pages (capped) and count only matching rows.
    let max_pages = if space_id.is_some() { 5 } else { 4 };
    let mut count: i64 = 0;
    let mut bookmark: Option<String> = None;
    for _ in 0..max_pages {
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
        count += match &space_id {
            Some(sid) => items
                .iter()
                .filter(|it| it.payload.space_id().as_ref() == Some(sid))
                .count() as i64,
            None => items.len() as i64,
        };
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
