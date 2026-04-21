use crate::common::*;
use crate::common::hooks::{InfiniteQuery, use_infinite_query};
use crate::features::notifications::controllers::list_inbox::list_inbox_handler;
use crate::features::notifications::types::InboxNotificationResponse;

/// Infinite-query hook for the current user's notification inbox.
///
/// Wraps [`use_infinite_query`] and calls
/// [`list_inbox_handler`] with the supplied `unread_only` flag.
/// Returns an [`InfiniteQuery`] that paginates through
/// `ListResponse<InboxNotificationResponse>` pages.
pub fn use_inbox(
    unread_only: bool,
) -> dioxus::prelude::Result<
    InfiniteQuery<String, InboxNotificationResponse, ListResponse<InboxNotificationResponse>>,
    RenderError,
> {
    let unread_only_signal = use_signal(|| unread_only);

    use_infinite_query(move |bookmark| {
        let unread_only = unread_only_signal();
        async move { list_inbox_handler(Some(unread_only), bookmark).await }
    })
}
