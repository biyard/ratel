//! Controller for the parent-admin sub-team settings tab.
//!
//! The backend exposes no dedicated GET for sub-team settings — the
//! `SubTeamSettingsResponse` fields live directly on the parent `Team`
//! record, which is already fetched elsewhere in the management page
//! (via `Team::get`). The page component seeds `settings` via
//! `use_context_provider(initial)` before invoking this hook; the
//! update action replaces that signal with the server's echoed response.

use crate::features::sub_team::controllers::update_sub_team_settings_handler;
use crate::*;
use crate::features::sub_team::types::{SubTeamSettingsResponse, UpdateSubTeamSettingsRequest};

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

    // Initial snapshot — consumed from ancestor context if the page seeded
    // one, otherwise default (zeros). The caller should always seed; the
    // default branch exists so the hook never panics when only the team id
    // has been plumbed through.
    let seeded: SubTeamSettingsResponse =
        try_consume_context().unwrap_or_default();
    let mut settings = use_signal(|| seeded);

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
