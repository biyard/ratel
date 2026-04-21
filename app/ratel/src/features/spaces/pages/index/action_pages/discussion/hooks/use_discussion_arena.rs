use crate::common::*;
use crate::features::spaces::pages::actions::actions::discussion::controllers::{
    get_comment, list_replies,
};
use crate::features::spaces::pages::actions::actions::discussion::DiscussionCommentResponse;
use dioxus::fullstack::Loader;

/// Per-discussion controller bundling the state that needs to survive
/// across the comments-list ↔ thread-view swap. The two loaders read
/// `active_reply_thread` inside their closures, so changing the signal
/// triggers a background refresh — the loader's previous value stays
/// visible during the fetch so consumers (`ReplyThreadView`) never see
/// a `Loading` suspense, which is what was causing the flash where
/// `DiscussionArenaPage` was being replaced by the overlay's empty
/// `SuspenseBoundary` fallback when the user tapped Reply.
#[derive(Clone, Copy, DioxusController)]
pub struct UseDiscussionArena {
    /// `Some(comment_id)` swaps the comments panel over to the in-sheet
    /// thread drill-down for that comment; `None` shows the normal
    /// comments list. Set on mobile Reply tap, cleared by the Back
    /// button inside the thread view.
    pub active_reply_thread: Signal<Option<String>>,

    /// Mirrors the bottom-sheet `.expanded` state. Owned by Dioxus (via
    /// this signal + `data-expanded` attribute on `.comments-panel`)
    /// instead of JS so the expand state survives panel re-renders that
    /// happen when the thread view swaps in.
    pub sheet_expanded: Signal<bool>,

    /// Parent comment for the currently active thread. Returns a default
    /// (empty) `DiscussionCommentResponse` when no thread is active so
    /// the initial mount resolves synchronously without an actual
    /// network request.
    pub parent_loader: Loader<DiscussionCommentResponse>,

    /// Replies for the currently active thread. Same default-when-None
    /// trick as `parent_loader` to avoid an unnecessary first-mount
    /// fetch.
    pub replies_loader: Loader<ListResponse<DiscussionCommentResponse>>,
}

/// Per-`DiscussionArenaPage` controller. Build once at the arena page,
/// reuse from any descendant (e.g. `CommentItem`, the thread view).
///
/// We `provide_context` (not `provide_root_context`) so the controller
/// is scoped to the current arena instance — navigating to a different
/// discussion unmounts this scope and the next call rebuilds against
/// the new `space_id` / `discussion_id`.
#[track_caller]
pub fn use_discussion_arena(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> std::result::Result<UseDiscussionArena, RenderError> {
    if let Some(ctx) = try_use_context::<UseDiscussionArena>() {
        return Ok(ctx);
    }

    let active_reply_thread: Signal<Option<String>> = use_signal(|| None);
    let sheet_expanded: Signal<bool> = use_signal(|| false);

    let parent_loader = use_loader(move || async move {
        let Some(id) = active_reply_thread() else {
            return Ok::<DiscussionCommentResponse, crate::common::Error>(
                DiscussionCommentResponse::default(),
            );
        };
        get_comment(
            space_id(),
            discussion_id(),
            SpacePostCommentEntityType(id),
        )
        .await
    })?;

    let replies_loader = use_loader(move || async move {
        let Some(id) = active_reply_thread() else {
            return Ok::<ListResponse<DiscussionCommentResponse>, crate::common::Error>(
                ListResponse::<DiscussionCommentResponse>::default(),
            );
        };
        list_replies(
            space_id(),
            discussion_id(),
            SpacePostCommentEntityType(id),
            None,
        )
        .await
    })?;

    Ok(use_context_provider(|| UseDiscussionArena {
        active_reply_thread,
        sheet_expanded,
        parent_loader,
        replies_loader,
    }))
}
