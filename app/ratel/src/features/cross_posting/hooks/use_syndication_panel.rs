//! `UseSyndicationPanel` controller hook for the post-detail author panel.
//!
//! Differs from `UseCrossPosting` in two ways:
//!
//! - **Per-post scope.** A panel instance is bound to one `post_id`; the
//!   hook accepts a `FeedPartition` so its loader re-runs when the route
//!   changes. There is no `provider`/`consumer` split — only the panel
//!   component reads this, so context-caching would buy nothing.
//!
//! - **Author-gated server endpoint.** `get_syndication_panel_handler`
//!   returns `NotAuthorized` for non-authors. The loader catches every
//!   server error and returns `None` so the panel renders blank instead of
//!   propagating a render failure (parent `post_detail` already author-
//!   gates the mount, but defense-in-depth keeps non-authors safe even on
//!   a race).
//!
//! Per `conventions/hooks-and-actions.md`, retry lives as an `async fn`
//! method on the controller — components await `panel.retry(platform)`
//! and decide UX (toast, navigation) on the result. No `use_action`.

use crate::common::*;
use crate::features::cross_posting::controllers::{
    get_syndication_panel_handler, retry_job_handler,
};
use crate::features::cross_posting::types::{SocialPlatform, SyndicationPanelResponse};
use dioxus::fullstack::Loader;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSyndicationPanel {
    /// `Some(panel)` when the loader succeeds; `None` for non-authors and
    /// any server error (NotAuthorized / network / 5xx). The component
    /// short-circuits to an empty render when this is `None`.
    pub panel: Loader<Option<SyndicationPanelResponse>>,

    /// Stored as a Signal so the controller stays `Copy` and `retry`
    /// can read the current id without taking ownership of the original
    /// `FeedPartition` prop.
    post_id: Signal<String>,
}

impl UseSyndicationPanel {
    /// Author-initiated retry of one failed job. The MODIFY event re-runs
    /// the Stage 2 dispatcher Pipe filter (`state=Pending`), so a
    /// successful retry flips the row to `Pending` and back to `Published`
    /// without any further client coordination.
    pub async fn retry(&mut self, platform: SocialPlatform) -> crate::common::Result<()> {
        retry_job_handler(FeedPartition((self.post_id)()), platform.to_string()).await?;
        self.panel.restart();
        Ok(())
    }
}

#[track_caller]
pub fn use_syndication_panel(
    post_id: FeedPartition,
) -> std::result::Result<UseSyndicationPanel, RenderError> {
    let post_id_str = post_id.0.clone();
    let post_id_signal = use_signal(move || post_id_str.clone());

    let panel = use_loader(move || {
        let post_id = post_id.clone();
        async move {
            match get_syndication_panel_handler(post_id).await {
                Ok(p) => Ok::<Option<SyndicationPanelResponse>, crate::common::Error>(Some(p)),
                Err(_) => Ok(None),
            }
        }
    })?;

    Ok(UseSyndicationPanel {
        panel,
        post_id: post_id_signal,
    })
}
