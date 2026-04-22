use crate::common::hooks::{use_infinite_query, InfiniteQuery};
use crate::common::*;
use crate::features::notifications::controllers::list_inbox::list_inbox_handler;
use crate::features::notifications::types::InboxNotificationResponse;
use crate::notifications::controllers::{mark_all_read_handler, mark_read_handler};

#[derive(Clone, Copy, DioxusController)]
pub struct UseInbox {
    pub inbox:
        InfiniteQuery<String, InboxNotificationResponse, ListResponse<InboxNotificationResponse>>,
    pub unread_only: Signal<bool>,
    pub unread_count: Signal<i64>,
    pub handle_item_click: Action<(InboxNotificationResponse,), ()>,
    pub handle_mark_all: Action<(), ()>,
}

/// Installer — runs every notification hook (signals, loader, actions) in
/// this scope and installs the resulting `UseInbox` via
/// `use_context_provider`. Call this **exactly once** from a long-lived
/// ancestor (the app's `NotificationsBootstrap`).
///
/// Named `use_provide_inbox` rather than `provide_inbox` because
/// Dioxus's linter (`dx check`) requires any function that calls hooks
/// to itself be a hook — i.e. its name must start with `use_` or it
/// must be a `#[component]`. Dropping the `use_` prefix tripped
/// "hook called outside component or hook" errors.
///
/// The earlier single-function pattern (`use_inbox` that conditionally
/// installed on first call and early-returned on subsequent calls via
/// `try_use_context`) violated Dioxus's rules of hooks: first render
/// registered ~dozen hook slots, second render hit the early return and
/// registered zero — mismatch → "Unable to retrieve the hook that was
/// initialized at this index" panic. Splitting installer/consumer keeps
/// the hook sequence identical on every render of this scope.
#[track_caller]
pub fn use_provide_inbox() -> std::result::Result<UseInbox, RenderError> {
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

    Ok(use_context_provider(|| UseInbox {
        inbox,
        unread_only,
        unread_count,
        handle_item_click,
        handle_mark_all,
    }))
}

/// Consumer — reads the `UseInbox` controller that a parent scope
/// installed via `provide_inbox`. Pure context read (1 stable hook slot),
/// safe to call from any transient component. Panics if no ancestor has
/// called `provide_inbox` — indicating a missing `NotificationsBootstrap`
/// in the route tree.
#[track_caller]
pub fn use_inbox() -> UseInbox {
    use_context::<UseInbox>()
}
