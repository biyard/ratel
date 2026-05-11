//! Controller for the parent-admin sub-team settings tab.
//!
//! Initial values come turn-key from `TeamArenaContext` — the wall
//! controller already fetches the parent `Team` row when the page
//! mounts, so we don't re-fetch via a dedicated GET. `handle_update`
//! PATCHes and writes the server's echoed response back into the
//! signal so the UI reflects the saved state without a re-load.

use crate::features::social::pages::team_arena::use_team_arena;
use crate::features::sub_team::controllers::update_sub_team_settings_handler;
use crate::features::sub_team::types::{SubTeamSettingsResponse, UpdateSubTeamSettingsRequest};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseSubTeamSettings {
    pub team_id: ReadSignal<TeamPartition>,
    pub settings: Signal<SubTeamSettingsResponse>,
    pub handle_update: Action<(UpdateSubTeamSettingsRequest,), ()>,
}

#[track_caller]
pub fn use_sub_team_settings() -> std::result::Result<UseSubTeamSettings, RenderError> {
    if let Some(ctx) = try_use_context::<UseSubTeamSettings>() {
        return Ok(ctx);
    }

    let team_id: TeamPartition = use_context();
    let team_id_signal: ReadSignal<TeamPartition> = use_signal(|| team_id).into();

    // Seed from `TeamArenaContext` (populated by the wall controller).
    // The fields live on the parent `Team` row, which the wall already
    // loaded — saves an extra round trip per page mount.
    let arena = use_team_arena();
    let mut settings = use_signal(|| SubTeamSettingsResponse {
        is_parent_eligible: arena.is_parent_eligible(),
        min_sub_team_members: arena.min_sub_team_members(),
        min_sub_team_age_days: arena.min_sub_team_age_days(),
    });

    let team_id_for_update = team_id_signal;
    let handle_update =
        use_action(move |req: UpdateSubTeamSettingsRequest| async move {
            let updated = update_sub_team_settings_handler(team_id_for_update(), req).await?;
            settings.set(updated);
            Ok::<(), crate::common::Error>(())
        });

    Ok(use_context_provider(|| UseSubTeamSettings {
        team_id: team_id_signal,
        settings,
        handle_update,
    }))
}
