//! Controller for the parent-admin sub-team settings tab.
//!
//! Hydrates `settings` from `get_sub_team_settings_handler` on mount
//! so the activation switch + stepper values survive a page refresh.
//! `handle_update` PATCHes and writes the server's echoed response
//! back into the signal.

use crate::features::sub_team::controllers::{
    get_sub_team_settings_handler, update_sub_team_settings_handler,
};
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

    // Server-side hydration — fetches the persisted settings (Team.
    // is_parent_eligible / min_* fields) on mount and re-uses the
    // result as the `settings` signal seed.
    let loader = use_loader(move || {
        let id = team_id_signal();
        async move { get_sub_team_settings_handler(id).await }
    })?;
    let mut settings = use_signal(SubTeamSettingsResponse::default);
    let loaded = loader();
    use_effect(move || {
        settings.set(loaded.clone());
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
