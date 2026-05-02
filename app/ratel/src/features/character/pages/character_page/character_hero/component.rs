use crate::features::character::dto::CharacterResponse;
use crate::features::character::pages::character_page::CharacterPageTranslate;
use crate::*;

/// Hero panel: Level badge + XP bar + SP pill.
///
/// Class names mirror `app/ratel/assets/design/character-xp-skills/character-page.html`
/// exactly (`.character-hero`, `.character-hero__level`, etc.) so the
/// rules in `app/ratel/assets/main.css` style the component identically
/// to the mockup.
#[component]
pub fn CharacterHero(response: CharacterResponse) -> Element {
    let tr: CharacterPageTranslate = use_translate();

    let level = response.level;
    let next_level = level + 1;
    let total_xp = response.total_xp;
    let progress = response.xp_progress_in_level;
    let span = response.xp_span_of_level.max(1); // avoid div-by-zero
    let remaining = (span - progress).max(0);

    let pct = (progress as f64 / span as f64 * 100.0)
        .clamp(0.0, 100.0)
        .round() as i32;
    let fill_style = format!("width:{}%", pct);

    let unspent_sp = response.unspent_sp;
    let sp_empty = unspent_sp <= 0;
    let sp_hint = if sp_empty {
        tr.sp_hint_empty.to_string()
    } else if unspent_sp == 1 {
        tr.sp_hint_one_ready.to_string()
    } else {
        format!("{} {}", unspent_sp, tr.sp_hint_ready)
    };

    rsx! {
        section { class: "character-hero", aria_label: "Character summary",

            // Level badge
            div { class: "character-hero__level",
                span { class: "character-hero__level-label", "{tr.level_label}" }
                span {
                    class: "character-hero__level-num",
                    id: "hero-level",
                    "data-testid": "hero-level",
                    "{level}"
                }
            }

            // XP bar block
            div { class: "character-hero__xp",
                div { class: "character-hero__xp-meta",
                    span { class: "character-hero__xp-title", "{tr.xp_title}" }
                    span { class: "character-hero__xp-numbers",
                        em { id: "hero-xp-current", "{progress}" }
                        " / "
                        span { id: "hero-xp-needed", "{span}" }
                    }
                }
                div {
                    class: "character-hero__xp-bar",
                    role: "progressbar",
                    aria_valuemin: "0",
                    aria_valuemax: "100",
                    aria_valuenow: "{pct}",
                    div {
                        class: "character-hero__xp-fill",
                        id: "hero-xp-fill",
                        style: "{fill_style}",
                    }
                }
                div { class: "character-hero__xp-hint",
                    span { id: "hero-xp-to-next", "{remaining} {tr.xp_to_next} {next_level}" }
                    " · {tr.xp_total_earned}: "
                    span { id: "hero-xp-total", "data-testid": "hero-xp-total", "{total_xp}" }
                }
            }

            // SP pill
            div {
                class: "character-hero__sp",
                id: "hero-sp",
                "data-empty": sp_empty.then_some("true"),

                span { class: "character-hero__sp-label", "{tr.sp_label}" }
                span {
                    class: "character-hero__sp-value",
                    id: "hero-sp-value",
                    "data-testid": "hero-sp-value",
                    "{unspent_sp}"
                }
                span { class: "character-hero__sp-hint", id: "hero-sp-hint", "{sp_hint}" }
            }
        }
    }
}
