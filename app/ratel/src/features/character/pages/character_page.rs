use crate::components::{RatelArenaTopbar, RatelArenaTopbarSection};
use dioxus::prelude::*;

/// Placeholder for the Character page (XP & skill tree).
///
/// Real content (hero with Level/XP/SP, skill grid) is implemented in a
/// follow-up dispatch (Tasks 34-36 of the
/// `2026-05-01-character-xp-skills` plan). This stub exists so the
/// `/me/character` route resolves and the shared [`ArenaTopbar`] can be
/// rendered with the Character section highlighted.
#[component]
pub fn CharacterPage() -> Element {
    rsx! {
        div { class: "character-arena",
            RatelArenaTopbar { active: Some(RatelArenaTopbarSection::Character) }
            main { class: "character-page", "Character (coming soon)" }
        }
    }
}
