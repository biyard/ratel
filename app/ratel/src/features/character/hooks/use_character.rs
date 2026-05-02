use crate::features::character::controllers::{get_character_handler, level_up_handler};
use crate::features::character::dto::CharacterResponse;
use crate::features::character::types::SkillId;
use crate::*;

/// Controller for the Character page (XP, level, skill points, skills).
///
/// Bundles the character `Loader<CharacterResponse>` plus a single
/// `level_up_action` that takes a [`SkillId`] and refreshes the loader on
/// success. Components consume this via [`use_character`] and never call
/// the server `_handler` functions directly (per
/// `conventions/hooks-and-actions.md`).
#[derive(Clone, Copy, DioxusController)]
pub struct UseCharacter {
    pub character: Loader<CharacterResponse>,
    pub level_up_action: Action<(SkillId,), ()>,
}

/// Provider — installs the controller into context (or returns the
/// already-installed instance). Returns `Result<UseCharacter, Loading>`
/// so that callers can `?`-bubble it from a `#[component]` body.
#[track_caller]
pub fn use_character() -> std::result::Result<UseCharacter, Loading> {
    if let Some(ctx) = try_use_context::<UseCharacter>() {
        return Ok(ctx);
    }

    let user_ctx = crate::features::auth::hooks::use_user_context();

    let mut character = use_loader(move || {
        let logged_in = user_ctx().is_logged_in();
        async move {
            if !logged_in {
                return Ok(CharacterResponse::default());
            }
            get_character_handler().await
        }
    })?;

    let level_up_action = use_action(move |id: SkillId| async move {
        level_up_handler(id.as_str().to_string()).await?;
        character.restart();
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(move || UseCharacter {
        character,
        level_up_action,
    }))
}
