//! `UseFactFoldAdminSettings` — load + persist the admin-tunable
//! game parameters singleton. Backed by the PR1 endpoints
//! `GET/PUT /api/fact-or-fold/admin/settings`.

use crate::features::arcade::games::fact_or_fold::{
    FactOrFoldSettingsResponse, UpdateFactOrFoldSettingsRequest, get_settings_handler,
    update_settings_handler,
};
use crate::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseFactFoldAdminSettings {
    pub settings: Loader<FactOrFoldSettingsResponse>,
}

impl UseFactFoldAdminSettings {
    /// Persist a partial patch and refresh the loader. Returns the
    /// fresh settings on success so the caller can drive UX
    /// (toast / form reset) without a second read.
    pub async fn save(
        &mut self,
        patch: UpdateFactOrFoldSettingsRequest,
    ) -> crate::common::Result<FactOrFoldSettingsResponse> {
        let next = update_settings_handler(patch).await?;
        self.settings.restart();
        Ok(next)
    }
}

/// Provider — call from the Settings page once. Returns the cached
/// instance if a parent already installed it.
pub fn use_fact_fold_admin_settings_provider()
-> std::result::Result<UseFactFoldAdminSettings, RenderError> {
    if let Some(ctx) = try_use_context::<UseFactFoldAdminSettings>() {
        return Ok(ctx);
    }

    let settings = use_loader(move || async move { get_settings_handler().await })?;

    Ok(use_context_provider(|| UseFactFoldAdminSettings { settings }))
}

/// Consumer — assumes provider has been installed by an ancestor
/// (typically the Settings page itself).
#[track_caller]
pub fn use_fact_fold_admin_settings() -> UseFactFoldAdminSettings {
    use_context::<UseFactFoldAdminSettings>()
}
