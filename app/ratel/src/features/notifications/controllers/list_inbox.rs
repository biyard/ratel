use crate::common::*;
use crate::common::models::notification::UserInboxNotification;
use crate::features::auth::User;
use crate::features::notifications::types::{InboxNotificationResponse, NotificationsError};

/// Target number of items to accumulate per response when `space_id`
/// filtering is active, and the hard cap on how many raw DynamoDB pages we
/// scan to reach it (so a user whose space-related notifications are sparse
/// among many global ones doesn't trigger an unbounded scan).
const SPACE_PAGE_TARGET: usize = 30;
const SPACE_MAX_PAGES: usize = 5;
const PAGE_LIMIT: i32 = 30;

#[mcp_tool(
    name = "list_inbox",
    description = "List current user's notification inbox. Returns paginated results ordered newest-first."
)]
#[get("/api/inbox?unread_only&space_id&bookmark", user: User)]
pub async fn list_inbox_handler(
    #[mcp(description = "If true, return only unread notifications.")] unread_only: Option<bool>,
    #[mcp(
        description = "If set, return only notifications belonging to this space (id without the SPACE# prefix)."
    )]
    space_id: Option<SpacePartition>,
    #[mcp(description = "Opaque pagination bookmark from a previous response. Omit for first page.")]
    bookmark: Option<String>,
) -> Result<ListResponse<InboxNotificationResponse>> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let unread = unread_only.unwrap_or(false);

    // Unfiltered path: preserve the original single-page behavior exactly.
    if space_id.is_none() {
        let (items, next) = fetch_inbox_page(cli, &user.pk, unread, bookmark).await?;
        let items: Vec<InboxNotificationResponse> = items.into_iter().map(Into::into).collect();
        return Ok((items, next).into());
    }

    // Space-scoped path: notifications carry their space only inside the
    // payload, so we filter post-fetch. A single raw page may yield few (or
    // zero) matches, so keep paging on the bookmark until we've collected
    // enough or run out — capped at SPACE_MAX_PAGES. The returned bookmark is
    // the last raw page's cursor, letting the client's infinite query resume.
    let space_id = space_id.unwrap();
    let mut acc: Vec<InboxNotificationResponse> = Vec::new();
    let mut cursor = bookmark;
    let mut next_out: Option<String> = None;

    for _ in 0..SPACE_MAX_PAGES {
        let (items, next) = fetch_inbox_page(cli, &user.pk, unread, cursor.clone()).await?;
        for it in items {
            let resp: InboxNotificationResponse = it.into();
            if resp.payload.space_id().as_ref() == Some(&space_id) {
                acc.push(resp);
            }
        }
        next_out = next.clone();
        match next {
            Some(b) => cursor = Some(b),
            None => break,
        }
        if acc.len() >= SPACE_PAGE_TARGET {
            break;
        }
    }

    Ok((acc, next_out).into())
}

/// Fetch one raw page of the user's inbox newest-first. `unread` selects the
/// sparse unread GSI ("U#…" keys) vs the full partition scan (filtered to the
/// notification sk prefix so the User record isn't picked up).
#[cfg(feature = "server")]
async fn fetch_inbox_page(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
    unread: bool,
    bookmark: Option<String>,
) -> Result<(Vec<UserInboxNotification>, Option<String>)> {
    if unread {
        let opts = UserInboxNotification::opt_with_bookmark(bookmark)
            .sk("U".to_string())
            .scan_index_forward(false)
            .limit(PAGE_LIMIT);
        UserInboxNotification::find_inbox_unread_by_user(cli, user_pk.clone(), opts)
            .await
            .map_err(|e| {
                crate::error!("list_inbox unread GSI failed: {e}");
                NotificationsError::ListFailed.into()
            })
    } else {
        let opts = UserInboxNotification::opt_with_bookmark(bookmark)
            .sk("USER_INBOX_NOTIFICATION".to_string())
            .scan_index_forward(false)
            .limit(PAGE_LIMIT);
        UserInboxNotification::query(cli, user_pk.clone(), opts)
            .await
            .map_err(|e| {
                crate::error!("list_inbox pk scan failed: {e}");
                NotificationsError::ListFailed.into()
            })
    }
}
