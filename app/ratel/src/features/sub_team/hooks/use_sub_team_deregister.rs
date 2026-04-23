//! Controller for the parent-admin "deregister sub-team" confirmation
//! page. Isolated from `UseSubTeamList` so the deregister page can mount
//! without fetching the full sub-teams list, and so the form-side state
//! (reason text, confirmation) stays off the list controller.

use crate::features::sub_team::controllers::deregister_sub_team_handler;
use crate::features::sub_team::types::DeregisterRequest;
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamDeregister {
    pub team_id: ReadSignal<TeamPartition>,
    pub sub_team_id: ReadSignal<String>,
    pub reason: Signal<String>,
    pub handle_deregister: Action<(DeregisterRequest,), ()>,
}

#[track_caller]
pub fn use_sub_team_deregister() -> std::result::Result<UseSubTeamDeregister, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamDeregister>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();
    // sub_team_id comes from the route; caller provides via context.
    let initial_sub_team_id: String = try_consume_context().unwrap_or_default();
    let sub_team_id_signal: ReadSignal<String> = use_signal(|| initial_sub_team_id).into();

    let reason: Signal<String> = use_signal(String::new);

    let team_id_for_dereg = team_id_signal;
    let sub_team_id_for_dereg = sub_team_id_signal;
    let nav = use_navigator();
    let handle_deregister = use_action(move |req: DeregisterRequest| async move {
        deregister_sub_team_handler(team_id_for_dereg(), sub_team_id_for_dereg(), req).await?;
        // After a successful deregister, return to the previous screen —
        // the sub-teams list typically sits one level up in the browser
        // history, so `go_back` is the lowest-friction choice.
        nav.go_back();
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(|| UseSubTeamDeregister {
        team_id: team_id_signal,
        sub_team_id: sub_team_id_signal,
        reason,
        handle_deregister,
    }))
}
