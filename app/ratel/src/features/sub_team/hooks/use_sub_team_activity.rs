//! Controller for the parent-admin sub-team detail (activity dashboard) page.
//!
//! Loads the sub-team overview + a window-scoped counts view + a paginated
//! per-member activity list. Window is a `Signal<ActivityWindow>` so the
//! dashboard's Weekly/Monthly toggle re-runs the counts/member queries.

use crate::common::hooks::{use_infinite_query, InfiniteQuery};
use crate::features::sub_team::controllers::{
    get_sub_team_activity_handler, get_sub_team_detail_handler,
    get_sub_team_member_activity_handler, list_direct_messages_handler,
    send_direct_message_handler,
};
use crate::features::sub_team::types::{
    ActivityCountsResponse, ActivityWindow, MemberActivityResponse, SendDirectMessageRequest,
    SubTeamAnnouncementResponse, SubTeamDetailResponse,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamActivity {
    pub team_id: ReadSignal<TeamPartition>,
    pub sub_team_id: ReadSignal<String>,
    pub window: Signal<ActivityWindow>,
    pub detail: Loader<SubTeamDetailResponse>,
    pub counts: Loader<ActivityCountsResponse>,
    pub members:
        InfiniteQuery<String, MemberActivityResponse, ListResponse<MemberActivityResponse>>,
    /// History of direct ("이 하위팀에만 공지") announcements this parent
    /// has sent to the sub-team in view. Refreshes after each successful
    /// send via `handle_send_direct`.
    pub direct_messages: Loader<ListResponse<SubTeamAnnouncementResponse>>,
    /// Fire-and-await action that posts a single direct announcement and
    /// re-loads `direct_messages` on success.
    pub handle_send_direct: Action<(SendDirectMessageRequest,), ()>,
}

#[track_caller]
pub fn use_sub_team_activity() -> std::result::Result<UseSubTeamActivity, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamActivity>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();
    // sub_team_id is seeded as String via context by the page wrapper.
    let initial_sub_team_id: String = try_consume_context().unwrap_or_default();
    let sub_team_id_signal: ReadSignal<String> = use_signal(|| initial_sub_team_id).into();
    let window: Signal<ActivityWindow> = use_signal(|| ActivityWindow::Monthly);

    let detail = use_loader(move || {
        let tid = team_id_signal();
        let sid = sub_team_id_signal();
        let w = window();
        async move { get_sub_team_detail_handler(tid, sid, Some(w)).await }
    })?;

    let counts = use_loader(move || {
        let tid = team_id_signal();
        let sid = sub_team_id_signal();
        let w = window();
        async move { get_sub_team_activity_handler(tid, sid, Some(w)).await }
    })?;

    let members = use_infinite_query(move |bookmark| {
        let tid = team_id_signal();
        let sid = sub_team_id_signal();
        let w = window();
        async move { get_sub_team_member_activity_handler(tid, sid, Some(w), bookmark).await }
    })?;

    let mut direct_messages = use_loader(move || {
        let tid = team_id_signal();
        let sid = sub_team_id_signal();
        async move { list_direct_messages_handler(tid, sid, None).await }
    })?;

    let handle_send_direct = use_action(move |req: SendDirectMessageRequest| async move {
        let tid = team_id_signal();
        let sid = sub_team_id_signal();
        send_direct_message_handler(tid, sid, req).await?;
        direct_messages.restart();
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseSubTeamActivity {
        team_id: team_id_signal,
        sub_team_id: sub_team_id_signal,
        window,
        detail,
        counts,
        members,
        direct_messages,
        handle_send_direct,
    }))
}
