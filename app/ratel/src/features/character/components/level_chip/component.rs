use super::LevelChipTranslate;
use crate::features::character::controllers::get_public_character_handler;
use crate::*;

/// Public profile "Level X" chip (visitor view).
///
/// Class names mirror
/// `app/ratel/assets/design/character-xp-skills/public-profile-badge.html`
/// (Variant A — `.character-level-chip`) so the rules in
/// `app/ratel/assets/main.css` style it identically to the mockup.
///
/// Per spec Q5, only the level is exposed publicly — skill build, SP,
/// and XP totals never leak through this surface. The handler
/// `get_public_character_handler` enforces this server-side.
///
/// On any error (e.g. requested user has no character row yet, or the
/// username doesn't exist), the chip falls back to level 1.
#[component]
pub fn LevelChip(username: ReadSignal<String>) -> Element {
    let tr: LevelChipTranslate = use_translate();

    let resource = use_loader(move || async move {
        let level = get_public_character_handler(username())
            .await
            .map(|r| r.level)
            .unwrap_or(1);
        Ok::<i32, crate::common::Error>(level)
    })?;

    let level = resource();

    let sub_label = if level <= 1 {
        tr.just_joined
    } else {
        tr.ratel_character
    };

    rsx! {
        span {
            class: "character-level-chip",
            "data-level": "{level}",
            title: "{tr.level_label} {level}",
            span { class: "character-level-chip__label", "{tr.level_label}" }
            span { class: "character-level-chip__num", "{level}" }
            span { class: "character-level-chip__sub", "{sub_label}" }
        }
    }
}
