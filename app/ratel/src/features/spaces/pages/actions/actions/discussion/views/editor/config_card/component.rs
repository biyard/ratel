use crate::features::spaces::pages::actions::actions::discussion::views::editor::DiscussionEditorTranslate;
use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::actions::components::{
    ActionDeleteButton, ActionDependencySelector, ActionRewardSetting, ActionStatusControl,
    PrerequisiteTile,
};

#[component]
pub fn ConfigCard() -> Element {
    let tr: DiscussionEditorTranslate = use_translate();
    let mut ctx = use_discussion_context();

    let space_id = ctx.space_id;
    let discussion_id = ctx.discussion_id;
    let action = ctx.discussion().space_action.clone();

    let action_id_str = discussion_id().to_string();
    let initial_prerequisite = action.prerequisite;
    let saved_credits = action.credits;
    let action_status = action.status.clone();
    let initial_depends_on = action.depends_on.clone();
    let initial_status = action.status.clone();
    let discussion_entity = use_memo(move || {
        crate::common::types::SpaceDiscussionEntityType(discussion_id().to_string())
    });

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

                // ── Participation & Rewards (TODO: not in discussion model) ─────
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
                        on_changed: move |_| ctx.discussion.restart(),
                    }
                }

                // ── AI Moderator (existing component, fully wired) ─────
                section { class: "section", "data-testid": "section-moderation",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_moderation_label}" }
                    }
                    crate::features::ai_moderator::AiModeratorSetting { space_id, discussion_id: discussion_entity }
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
                        on_changed: move |_| ctx.discussion.restart(),
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
                            action_id: discussion_id().to_string(),
                        }
                    }
                }
            }
        }
    }
}
