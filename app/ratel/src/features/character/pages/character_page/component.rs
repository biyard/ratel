use super::*;
use crate::common::components::SeoMeta;
use crate::components::{RatelArenaTopbar, RatelArenaTopbarSection};
use crate::features::character::hooks::{use_character, UseCharacter};
use crate::*;

/// Top-level Character page (`/me/character`).
///
/// Reads from the [`UseCharacter`] controller (XP, level, SP, skills) and
/// renders the hero panel + skill grid. Class names and element IDs are
/// preserved verbatim from
/// `app/ratel/assets/design/character-xp-skills/character-page.html` so
/// the global CSS in `app/ratel/assets/main.css` styles both identically.
#[component]
pub fn CharacterPage() -> Element {
    let tr: CharacterPageTranslate = use_translate();
    let UseCharacter {
        character,
        mut level_up_action,
    } = use_character()?;

    let response = character();
    let skills = response.skills.clone();
    let unspent_sp = response.unspent_sp;

    rsx! {
        SeoMeta { title: "{tr.page_title}" }

        div { class: "character-arena",
            RatelArenaTopbar { active: Some(RatelArenaTopbarSection::Character) }

            main { class: "character-page", id: "character-page",

                CharacterHero { response: response.clone() }

                header { class: "section-header",
                    h2 { class: "section-header__title", "{tr.skill_tree_title}" }
                    span { class: "section-header__hint", "{tr.skill_tree_hint}" }
                }

                div { class: "skill-grid",
                    for skill in skills.into_iter() {
                        SkillCard {
                            key: "{skill.skill_id.as_str()}",
                            response: skill,
                            unspent_sp,
                            on_levelup: move |id| level_up_action.call(id),
                        }
                    }
                }
            }
        }
    }
}
