//! Controller for the parent-admin sub-teams list tab.
//!
//! Phase 1 returns a single page (≤ 50 rows) via a bespoke envelope
//! (`SubTeamListResponse`) that includes a `truncated` flag — so we
//! surface it as a `Loader` rather than an `InfiniteQuery`. Deregistration
//! lives in `use_sub_team_deregister` to keep the action shape small.

use crate::features::sub_team::controllers::list_sub_teams_handler;
use crate::features::sub_team::types::SubTeamListResponse;
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamList {
    pub team_id: ReadSignal<TeamPartition>,
    pub teams: Loader<SubTeamListResponse>,
}

#[track_caller]
pub fn use_sub_team_list() -> std::result::Result<UseSubTeamList, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamList>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();

    let teams = use_loader(move || {
        let id = team_id_signal();
        async move { list_sub_teams_handler(id, None).await }
    })?;

    Ok(use_context_provider(|| UseSubTeamList {
        team_id: team_id_signal,
        teams,
    }))
}
