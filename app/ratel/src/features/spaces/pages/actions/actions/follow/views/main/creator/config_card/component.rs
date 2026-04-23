use crate::features::spaces::pages::actions::actions::follow::views::main::creator::FollowCreatorTranslate;
use crate::features::spaces::pages::actions::actions::follow::*;
use crate::features::spaces::pages::actions::components::{
    ActionDeleteButton, ActionDependencySelector, ActionRewardSetting, ActionStatusControl,
    PrerequisiteTile,
};
use crate::features::spaces::pages::actions::models::SpaceAction;

#[component]
pub fn ConfigCard(
    space_id: ReadSignal<SpacePartition>,
    follow_id: ReadSignal<SpaceActionFollowEntityType>,
    action_setting: ReadSignal<SpaceAction>,
) -> Element {
    let tr: FollowCreatorTranslate = use_translate();

    let action = action_setting();
    let action_id_str = follow_id().to_string();
    let saved_credits = action.credits;
    let action_status = action.status.clone();
    let initial_prerequisite = action.prerequisite;
    let initial_depends_on = action.depends_on.clone();
    let initial_status = action.status.clone();

    let action_id_for_signal = action_id_str.clone();
    let action_id_signal: ReadSignal<String> =
        use_signal(move || action_id_for_signal.clone()).into();

    rsx! {
        section { class: "pager__page", "data-page": "1",
            article { class: "page-card", "data-testid": "page-card-config",
                header { class: "page-card__head",
                    div { class: "page-card__title-wrap",
                        span { class: "page-card__num", "{tr.card_index_2}" }
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

                // ── Participation & Rewards ─────
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
                    PrerequisiteTile {
                        space_id,
                        action_id: action_id_signal,
                        initial_prerequisite,
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
                                "{tr.delete_title}"
                            }
                            div { style: "font-size:12px;color:var(--qc-text-muted)",
                                "{tr.delete_desc}"
                            }
                        }
                        ActionDeleteButton {
                            space_id: space_id(),
                            action_id: follow_id().to_string(),
                        }
                    }
                }
            }
        }
    }
}
