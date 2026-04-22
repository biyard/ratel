use crate::common::components::date_picker::{DateAndTimePicker, DateTimeRange};
use crate::features::spaces::pages::actions::actions::discussion::views::editor::DiscussionEditorTranslate;
use crate::features::spaces::pages::actions::actions::discussion::*;
use crate::features::spaces::pages::actions::components::{ActionDeleteButton, ActionRewardSetting};
use crate::features::spaces::pages::actions::controllers::{
    UpdateSpaceActionRequest, update_space_action,
};

#[component]
pub fn ConfigCard() -> Element {
    let tr: DiscussionEditorTranslate = use_translate();
    let mut ctx = use_discussion_context();
    let mut toast = use_toast();

    let space_id = ctx.space_id;
    let discussion_id = ctx.discussion_id;
    let action = ctx.discussion().space_action.clone();

    let action_id_str = discussion_id().to_string();
    let mut prereq_follow = use_signal(|| action.prerequisite);
    let saved_credits = action.credits;
    let discussion_entity = use_memo(move || {
        crate::common::types::SpaceDiscussionEntityType(discussion_id().to_string())
    });

    let initial_started_at = action.started_at;
    let initial_ended_at = action.ended_at;

    let action_id_for_signal = action_id_str.clone();
    let action_id_signal: ReadSignal<String> =
        use_signal(move || action_id_for_signal.clone()).into();

    let action_id_for_prereq = action_id_str.clone();
    let toggle_prereq = move |_| {
        let new_val = !prereq_follow();
        prereq_follow.set(new_val);
        let action_id = action_id_for_prereq.clone();
        spawn(async move {
            let req = UpdateSpaceActionRequest::Prerequisite {
                prerequisite: new_val,
            };
            match update_space_action(space_id(), action_id, req).await {
                Ok(_) => ctx.discussion.restart(),
                Err(err) => {
                    error!("Failed to save prerequisite: {:?}", err);
                    toast.error(err);
                    prereq_follow.set(!new_val);
                }
            }
        });
    };

    let on_schedule_change = move |range: DateTimeRange| async move {
        let (Some(start_date), Some(end_date)) = (range.start_date, range.end_date) else {
            return;
        };
        let start_ms = range
            .timezone
            .local_to_utc_millis(start_date, range.start_hour, range.start_minute);
        let end_ms = range
            .timezone
            .local_to_utc_millis(end_date, range.end_hour, range.end_minute);
        if start_ms <= 0 || end_ms <= 0 {
            return;
        }
        let req = UpdateDiscussionRequest {
            title: None,
            html_contents: None,
            category_name: None,
            started_at: Some(start_ms),
            ended_at: Some(end_ms),
            files: None,
        };
        if let Err(err) = update_discussion(space_id(), discussion_id(), req).await {
            error!("Failed to save discussion schedule: {:?}", err);
            toast.error(err);
        } else {
            ctx.discussion.restart();
        }
    };

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

                section { class: "section", "data-testid": "section-schedule",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_schedule_label}" }
                    }
                    DateAndTimePicker {
                        initial_started_at: Some(initial_started_at),
                        initial_ended_at: Some(initial_ended_at),
                        on_change: on_schedule_change,
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
                        started_at: initial_started_at,
                    }
                    // Prerequisite — wired via update_space_action::Prerequisite
                    div { class: "tile", "data-testid": "tile-prereq",
                        span { class: "tile__label", "{tr.tile_prereq}" }
                        div { class: "tile__row",
                            span { style: "font-size:13px;color:var(--qc-text-muted)",
                                "{tr.tile_prereq_label}"
                            }
                            span {
                                class: "switch",
                                role: "switch",
                                tabindex: "0",
                                "aria-checked": prereq_follow(),
                                onclick: toggle_prereq,
                                span { class: "switch__track",
                                    span { class: "switch__thumb" }
                                }
                            }
                        }
                    }
                }

                // ── AI Moderator (existing component, fully wired) ─────
                section { class: "section", "data-testid": "section-moderation",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_moderation_label}" }
                    }
                    crate::features::ai_moderator::AiModeratorSetting { space_id, discussion_id: discussion_entity }
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
