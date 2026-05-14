//! Controller for the sub-team announcement compose page (new + edit).
//!
//! Bundles every signal the composer UI needs: title, html_contents,
//! tags, space toggle + space type, plus the autosave lifecycle status
//! and the publish/discard actions. Components consume this hook —
//! they never call the announcement server handlers directly.

use crate::features::posts::types::SpaceType;
use crate::features::sub_team::controllers::{
    create_announcement_handler, delete_announcement_handler, get_announcement_handler,
    publish_announcement_handler, update_announcement_handler,
};
use crate::features::sub_team::types::{
    CreateSubTeamAnnouncementRequest, SubTeamAnnouncementResponse,
    UpdateSubTeamAnnouncementRequest,
};
use crate::*;

/// Lifecycle of the in-progress draft, surfaced to the composer UI so it
/// can render the "방금 저장됨" / "저장 중…" / "저장 안 됨" indicator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BroadcastDraftStatus {
    /// Composer just opened — nothing to save yet.
    Idle,
    /// User typed; debounce timer hasn't fired.
    Dirty,
    /// Network request in flight.
    Saving,
    /// Last request succeeded; signals == last_saved.
    Saved,
    /// Last request errored.
    Error,
}

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamBroadcastCompose {
    pub team_id: ReadSignal<TeamPartition>,
    pub announcement_id: Signal<Option<String>>,
    pub announcement: Loader<Option<SubTeamAnnouncementResponse>>,

    // Composer signals — bound to inputs.
    pub title: Signal<String>,
    pub html_contents: Signal<String>,
    pub tags: Signal<Vec<String>>,
    pub space_enabled: Signal<bool>,
    pub space_type: Signal<Option<SpaceType>>,

    // Draft autosave lifecycle.
    pub draft_status: Signal<BroadcastDraftStatus>,
    pub last_saved_at: Signal<Option<i64>>,

    // Autosave mutations — fired from inside a use_effect, so the
    // calling component stays mounted long enough for the action's
    // spawned future to resolve. Action lifecycle is exposed via
    // `.value()` / `.pending()` so the editor can promote the new id
    // after `handle_save_new` resolves.
    pub handle_save_new: Action<(CreateSubTeamAnnouncementRequest,), String>,
    pub handle_save_existing: Action<(String, UpdateSubTeamAnnouncementRequest), ()>,
}

impl UseSubTeamBroadcastCompose {
    /// Publish the draft and wait for the server round-trip to finish.
    /// Components await this before navigating — using `Action::call` here
    /// would detach the future from the component, and the SPA nav.push
    /// that follows would drop the future before the request completes,
    /// leaving the draft stuck in `작성중 · DRAFTS`.
    pub async fn publish_announcement(&mut self, id: String) -> crate::common::Result<()> {
        let team_id = self.team_id;
        publish_announcement_handler(team_id(), id).await?;
        Ok(())
    }

    /// Delete the draft and wait for the server to confirm before the
    /// caller navigates away (same rationale as `publish_announcement`).
    pub async fn delete_announcement(&mut self, id: String) -> crate::common::Result<()> {
        let team_id = self.team_id;
        delete_announcement_handler(team_id(), id).await?;
        Ok(())
    }
}

#[track_caller]
pub fn use_sub_team_broadcast_compose(
) -> std::result::Result<UseSubTeamBroadcastCompose, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamBroadcastCompose>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();
    let initial_announcement_id: Option<String> = try_consume_context().unwrap_or(None);
    let announcement_id: Signal<Option<String>> = use_signal(|| initial_announcement_id);

    let announcement_loader = use_loader(move || {
        let tid = team_id_signal();
        let aid = announcement_id();
        async move {
            let Some(aid) = aid else {
                return Ok::<Option<SubTeamAnnouncementResponse>, crate::common::Error>(None);
            };
            let row = get_announcement_handler(tid, aid).await?;
            Ok(Some(row))
        }
    })?;

    // Seed editor signals from the loaded draft (if any).
    let loaded = announcement_loader();
    let initial_title = loaded.as_ref().map(|a| a.title.clone()).unwrap_or_default();
    let initial_html = loaded
        .as_ref()
        .map(|a| {
            if !a.html_contents.is_empty() {
                a.html_contents.clone()
            } else {
                a.body.clone()
            }
        })
        .unwrap_or_default();
    let initial_tags = loaded
        .as_ref()
        .map(|a| a.tags.clone())
        .unwrap_or_default();
    let initial_space_enabled = loaded.as_ref().map(|a| a.space_enabled).unwrap_or(false);
    let initial_space_type = loaded.as_ref().and_then(|a| a.space_type);

    let title: Signal<String> = use_signal(|| initial_title);
    let html_contents: Signal<String> = use_signal(|| initial_html);
    let tags: Signal<Vec<String>> = use_signal(|| initial_tags);
    let space_enabled: Signal<bool> = use_signal(|| initial_space_enabled);
    let space_type: Signal<Option<SpaceType>> = use_signal(|| initial_space_type);

    let draft_status: Signal<BroadcastDraftStatus> =
        use_signal(|| BroadcastDraftStatus::Idle);
    let last_saved_at: Signal<Option<i64>> = use_signal(|| None);

    // Mutations.
    let team_id_for_new = team_id_signal;
    let handle_save_new =
        use_action(move |req: CreateSubTeamAnnouncementRequest| async move {
            let created = create_announcement_handler(team_id_for_new(), req).await?;
            Ok::<String, crate::common::Error>(created.id)
        });

    let team_id_for_update = team_id_signal;
    let handle_save_existing = use_action(
        move |aid: String, req: UpdateSubTeamAnnouncementRequest| async move {
            update_announcement_handler(team_id_for_update(), aid, req).await?;
            Ok::<(), crate::common::Error>(())
        },
    );

    Ok(use_context_provider(|| UseSubTeamBroadcastCompose {
        team_id: team_id_signal,
        announcement_id,
        announcement: announcement_loader,
        title,
        html_contents,
        tags,
        space_enabled,
        space_type,
        draft_status,
        last_saved_at,
        handle_save_new,
        handle_save_existing,
    }))
}
