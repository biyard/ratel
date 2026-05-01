use crate::features::character::dto::CharacterSkillResponse;
use crate::features::character::pages::character_page::CharacterPageTranslate;
use crate::features::character::types::SkillId;
use crate::*;

/// One skill card in the skill tree grid. Class names + element data
/// attributes mirror `app/ratel/assets/design/character-xp-skills/character-page.html`.
#[component]
pub fn SkillCard(
    response: CharacterSkillResponse,
    unspent_sp: i32,
    on_levelup: EventHandler<SkillId>,
) -> Element {
    let tr: CharacterPageTranslate = use_translate();

    let skill_id = response.skill_id;
    let skill_id_str = skill_id.as_str();
    let level = response.level;
    let max_level = response.max_level;
    let cost_opt = response.next_level_cost;
    let is_released = response.is_released;
    let is_maxed = level >= max_level;

    // Lookup-style copy. We can't dynamically index into the translate
    // struct, so a small match keeps it explicit per skill.
    let (name, sub, desc) = match skill_id {
        SkillId::MoneyTree => (
            tr.money_tree_name.to_string(),
            tr.money_tree_sub.to_string(),
            tr.money_tree_desc.to_string(),
        ),
        SkillId::Ranker => (
            tr.ranker_name.to_string(),
            tr.ranker_sub.to_string(),
            tr.ranker_desc.to_string(),
        ),
        SkillId::Influencer => (
            tr.influencer_name.to_string(),
            tr.influencer_sub.to_string(),
            tr.influencer_desc.to_string(),
        ),
        SkillId::Sweeper => (
            tr.sweeper_name.to_string(),
            tr.sweeper_sub.to_string(),
            tr.sweeper_desc.to_string(),
        ),
    };

    let multiplier_pct = level * 5;
    let next_pct = (level + 1) * 5;

    let pip_filled = (0..max_level)
        .map(|i| i < level)
        .collect::<Vec<_>>();

    let level_meta_value = format!("{} / {}", level, max_level);

    let testid_levelup = format!("skill-levelup-{}", skill_id_str);

    let cost_disabled = match cost_opt {
        Some(c) => unspent_sp < c,
        None => true,
    };

    rsx! {
        article {
            class: "skill-card",
            "data-skill": "{skill_id_str}",
            "data-locked": (!is_released).then_some("true"),

            div { class: "skill-card__head",
                span { class: "skill-card__icon", aria_hidden: "true", {skill_icon(skill_id)} }
                div { class: "skill-card__title-block",
                    div { class: "skill-card__name", "{name}" }
                    div { class: "skill-card__name-ko", "{sub}" }
                }
                if is_released {
                    span { class: "skill-card__multiplier", "+{multiplier_pct}%" }
                } else {
                    span { class: "skill-card__lock-tag", "{tr.coming_soon}" }
                }
            }

            p { class: "skill-card__desc", "{desc}" }

            div {
                class: "skill-card__pips",
                role: "meter",
                aria_valuemin: "0",
                aria_valuemax: "{max_level}",
                aria_valuenow: "{level}",
                for filled in pip_filled.iter() {
                    span {
                        class: "skill-card__pip",
                        "data-filled": (*filled).then_some("true"),
                    }
                }
            }

            footer { class: "skill-card__footer",
                div { class: "skill-card__level-meta",
                    if is_released {
                        span { class: "skill-card__level-meta-label", "{tr.level_meta_label}" }
                        span { class: "skill-card__level-meta-value", "{level_meta_value}" }
                        if is_maxed {
                            span { class: "skill-card__level-meta-next", "{tr.max_reached}" }
                        } else {
                            span { class: "skill-card__level-meta-next",
                                "{tr.next_boost}{next_pct}{tr.next_was}{multiplier_pct}%)"
                            }
                        }
                    } else {
                        span { class: "skill-card__level-meta-label", "{tr.status_meta_label}" }
                        span { class: "skill-card__level-meta-value", "{tr.not_released}" }
                    }
                }

                if is_released {
                    if is_maxed {
                        span { class: "skill-card__maxed",
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2.5",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                            "{tr.maxed_label}"
                        }
                    } else if let Some(cost) = cost_opt {
                        button {
                            class: "skill-card__levelup",
                            r#type: "button",
                            "data-testid": "{testid_levelup}",
                            disabled: cost_disabled,
                            onclick: move |_| on_levelup.call(skill_id),
                            span { "{tr.levelup_label}" }
                            span { class: "skill-card__levelup-cost", "{cost} {tr.sp_unit}" }
                        }
                    }
                } else {
                    span { class: "skill-card__locked-cta",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect {
                                x: "3",
                                y: "11",
                                width: "18",
                                height: "11",
                                rx: "2",
                            }
                            path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
                        }
                        span { "{tr.locked_label}" }
                    }
                }
            }
        }
    }
}

/// Per-skill icon SVG matching the HTML mockup shapes.
fn skill_icon(id: SkillId) -> Element {
    match id {
        SkillId::MoneyTree => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "12", cy: "8", r: "5" }
                path { d: "M12 13v8" }
                path { d: "M9 18h6" }
                path { d: "M9 8h6" }
            }
        },
        SkillId::Ranker => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                polyline { points: "17 11 12 6 7 11" }
                polyline { points: "17 18 12 13 7 18" }
            }
        },
        SkillId::Influencer => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M3 11v2a4 4 0 0 0 4 4h1l5 4V5L8 9H7a4 4 0 0 0-4 2Z" }
                path { d: "M16 8a5 5 0 0 1 0 8" }
            }
        },
        SkillId::Sweeper => rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M14 11 6 19" }
                path { d: "m21 4-7 7" }
                path { d: "M3 21h18" }
                path { d: "M5 16h6" }
            }
        },
    }
}
