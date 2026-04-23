use crate::features::spaces::pages::actions::actions::quiz::*;
use crate::features::spaces::pages::actions::actions::quiz::views::main::creator::QuizCreatorTranslate;
use crate::features::spaces::pages::actions::components::{
    ActionDeleteButton, ActionDependencySelector, ActionRewardSetting, ActionStatusControl,
    PrerequisiteTile,
};

#[component]
pub fn ConfigCard() -> Element {
    let tr: QuizCreatorTranslate = use_translate();
    let mut ctx = use_space_quiz_context();

    let space_id = ctx.space_id;
    let quiz_id = ctx.quiz_id;
    let quiz = ctx.quiz.read().clone();
    let action_id_str = quiz_id().to_string();

    let initial_prerequisite = quiz.space_action.prerequisite;
    let saved_credits = quiz.space_action.credits;
    let action_status = quiz.space_action.status.clone();
    let initial_depends_on = quiz.space_action.depends_on.clone();
    let initial_status = quiz.space_action.status.clone();

    let action_id_for_signal = action_id_str.clone();
    let action_id_signal: ReadSignal<String> =
        use_signal(move || action_id_for_signal.clone()).into();

    rsx! {
        section { class: "pager__page", "data-page": "2",
            article { class: "page-card", "data-testid": "page-card-config",
                header { class: "page-card__head",
                    div { class: "page-card__title-wrap",
                        span { class: "page-card__num", "{tr.card_index_3}" }
                        div {
                            h1 { class: "page-card__title", "{tr.card_config_title}" }
                            div { class: "page-card__subtitle", "{tr.card_config_subtitle}" }
                        }
                    }
                }

                // ── Dependencies (other actions a user must finish first) ─────
                section { class: "section", "data-testid": "section-dependencies",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_dependencies_label}" }
                    }
                    ActionDependencySelector {
                        space_id,
                        action_id: action_id_signal,
                        initial_depends_on,
                    }
                }

                // ── Participation & Rewards (TODO: most fields not in model) ─────
                section { class: "section", "data-testid": "section-participation",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_participation_label}" }
                    }
                    // Reward — uses shared ActionRewardSetting (membership + boost UI)
                    ActionRewardSetting {
                        space_id,
                        action_id: action_id_signal,
                        saved_credits,
                        action_status: action_status.clone(),
                    }
                    // Prerequisite — shared HTML-first tile; writes via update_space_action::Prerequisite
                    PrerequisiteTile {
                        space_id,
                        action_id: action_id_signal,
                        initial_prerequisite,
                        on_changed: move |_| ctx.quiz.restart(),
                    }
                }

                // ── Status (publish / close lifecycle) ─────
                section { class: "section", "data-testid": "section-status",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_status_label}" }
                    }
                    ActionStatusControl {
                        space_id,
                        action_id: action_id_signal,
                        initial_status: initial_status.clone(),
                        on_changed: move |_| ctx.quiz.restart(),
                    }
                }

                // ── Danger zone ─────
                section {
                    class: "section section--danger",
                    "data-testid": "section-danger",
                    div { class: "section__head",
                        span {
                            class: "section__label",
                            style: "color:var(--accent-red);opacity:0.85",
                            "{tr.section_danger_label}"
                        }
                    }
                    div { style: "display:flex;align-items:center;justify-content:space-between;gap:16px;flex-wrap:wrap",
                        div { style: "flex:1;min-width:220px",
                            div { style: "font-size:13px;font-weight:600;color:var(--qc-text-primary);margin-bottom:4px",
                                "{tr.delete_quiz_title}"
                            }
                            div { style: "font-size:12px;color:var(--qc-text-muted)",
                                "{tr.delete_quiz_desc}"
                            }
                        }
                        ActionDeleteButton {
                            space_id: space_id(),
                            action_id: quiz_id().to_string(),
                        }
                    }
                }
            }
        }
    }
}
