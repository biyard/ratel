use crate::common::hooks::{use_infinite_query, InfiniteQuery};
use crate::common::*;
use crate::features::notifications::controllers::list_inbox::list_inbox_handler;
use crate::features::notifications::types::InboxNotificationResponse;
use crate::notifications::controllers::{mark_all_read_handler, mark_read_handler};
use dioxus::core::provide_root_context;

#[derive(Clone, Copy, DioxusController)]
pub struct UseInbox {
    pub inbox:
        InfiniteQuery<String, InboxNotificationResponse, ListResponse<InboxNotificationResponse>>,
    pub unread_only: Signal<bool>,
    pub unread_count: Signal<i64>,
    pub handle_item_click: Action<(InboxNotificationResponse,), ()>,
    pub handle_mark_all: Action<(), ()>,
}

/// Infinite-query hook for the current user's notification inbox.
///
/// Wraps [`use_infinite_query`] and calls
/// [`list_inbox_handler`] with the supplied `unread_only` flag.
/// Returns an [`InfiniteQuery`] that paginates through
/// `ListResponse<InboxNotificationResponse>` pages.
#[track_caller]
pub fn use_inbox() -> std::result::Result<UseInbox, RenderError> {
    let ctx: Option<UseInbox> = try_use_context();

    if let Some(ctx) = ctx {
        return Ok(ctx);
    }

    let unread_only = use_signal(|| false);

    let mut inbox = use_infinite_query(move |bookmark| {
        let unread_only = unread_only();
        async move { list_inbox_handler(Some(unread_only), bookmark).await }
    })?;

    let nav = use_navigator();

    let handle_item_click = use_action(move |item: InboxNotificationResponse| async move {
        let inbox_id = item.id.clone();
        let cta = item.payload.url().to_string();

        mark_read_handler(inbox_id).await?;

        if !cta.is_empty() {
            nav.push(cta);
        }

        Ok::<(), crate::common::Error>(())
    });

    let mut unread_count = super::use_unread_count();

    let handle_mark_all = use_action(move || async move {
        mark_all_read_handler().await?;
        unread_count.set(0);
        inbox.refresh();
        Ok::<(), crate::common::Error>(())
    });

    Ok(provide_root_context(UseInbox {
        inbox,
        unread_only,
        unread_count,
        handle_item_click,
        handle_mark_all,
    }))
}
