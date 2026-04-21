use crate::common::*;
use crate::common::models::notification::{UNREAD_SENTINEL, UserInboxNotification};
use crate::features::auth::User;
use crate::features::notifications::types::{MarkAllReadResponse, NotificationsError};

const MAX_PAGES: usize = 5;
const PAGE_LIMIT: i32 = 30;

#[post("/api/inbox/read-all", user: User)]
pub async fn mark_all_read_handler() -> Result<MarkAllReadResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let mut affected = 0i64;
    let mut bookmark: Option<String> = None;
    let mut has_more = false;

    for page in 0..MAX_PAGES {
        // Filter by the sparse GSI sort-key prefix so rows we rewrote to the
        // read sentinel ("R") are skipped — only "U#..." unread keys match.
        let opts = UserInboxNotification::opt_with_bookmark(bookmark)
            .sk("U".to_string())
            .limit(PAGE_LIMIT);
        let (items, next) =
            UserInboxNotification::find_inbox_unread_by_user(cli, user.pk.clone(), opts)
                .await
                .map_err(|e| {
                    crate::error!("read-all GSI failed: {e}");
                    NotificationsError::MarkReadFailed
                })?;

        for item in items {
            if let Err(e) = UserInboxNotification::updater(item.pk, item.sk)
                .with_is_read(true)
                .with_unread_created_at(UNREAD_SENTINEL.to_string())
                .execute(cli)
                .await
            {
                crate::error!("read-all per-row update failed: {e}");
                continue;
            }
            affected += 1;
        }

        match next {
            Some(b) => {
                bookmark = Some(b);
                if page == MAX_PAGES - 1 {
                    has_more = true;
                }
            }
            None => break,
        }
    }

    Ok(MarkAllReadResponse { affected, has_more })
}
