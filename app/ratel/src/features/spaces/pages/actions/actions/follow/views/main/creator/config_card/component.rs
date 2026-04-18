use crate::features::spaces::pages::actions::actions::follow::views::main::creator::FollowCreatorTranslate;
use crate::features::spaces::pages::actions::actions::follow::*;
use crate::features::spaces::pages::actions::components::{ActionDeleteButton, ActionRewardSetting};
use crate::features::spaces::pages::actions::controllers::{
    UpdateSpaceActionRequest, update_space_action,
};

use crate::common::utils::time::{datetime_local_to_epoch_ms, epoch_ms_to_datetime_local};

#[component]
pub fn ConfigCard(
    space_id: ReadSignal<SpacePartition>,
    follow_id: ReadSignal<SpaceActionFollowEntityType>,
    started_at: i64,
    ended_at: i64,
    credits: u64,
    prerequisite: bool,
) -> Element {
    let tr: FollowCreatorTranslate = use_translate();
    let mut toast = use_toast();

    let action_id_str = follow_id().to_string();
    let saved_credits = credits;

    let mut prereq_follow = use_signal(|| prerequisite);

    let mut started_at_signal = use_signal(|| started_at);
    let mut ended_at_signal = use_signal(|| ended_at);

    let started_at_str = epoch_ms_to_datetime_local(started_at_signal());
    let ended_at_str = epoch_ms_to_datetime_local(ended_at_signal());

    let action_id_for_signal = action_id_str.clone();
    let action_id_signal: ReadSignal<String> =
        use_signal(move || action_id_for_signal.clone()).into();

    let action_id_for_schedule = action_id_str.clone();
    let mut save_schedule = use_callback(move |_: ()| {
        let start_ms = started_at_signal();
        let end_ms = ended_at_signal();
        if start_ms <= 0 || end_ms <= 0 {
            return;
        }
        let action_id = action_id_for_schedule.clone();
        spawn(async move {
            let req = UpdateSpaceActionRequest::Time {
                started_at: start_ms,
                ended_at: end_ms,
            };
            if let Err(err) = update_space_action(space_id(), action_id, req).await {
                error!("Failed to save follow schedule: {:?}", err);
                toast.error(err);
            }
        });
    });

    let action_id_for_prereq = action_id_str.clone();
    let toggle_prereq = move |_| {
        let new_val = !prereq_follow();
        prereq_follow.set(new_val);
        let action_id = action_id_for_prereq.clone();
        spawn(async move {
            let req = UpdateSpaceActionRequest::Prerequisite {
                prerequisite: new_val,
            };
            if let Err(err) = update_space_action(space_id(), action_id, req).await {
                error!("Failed to save prerequisite: {:?}", err);
                toast.error(err);
                prereq_follow.set(!new_val);
            }
        });
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

                // ── Schedule ─────
                section { class: "section", "data-testid": "section-schedule",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_schedule_label}" }
                    }
                    div { class: "grid-2",
                        div { class: "field",
                            label { class: "field__label", "{tr.schedule_starts_at}" }
                            input {
                                class: "input",
                                r#type: "datetime-local",
                                "data-testid": "schedule-start",
                                value: "{started_at_str}",
                                oninput: move |e| {
                                    if let Some(ms) = datetime_local_to_epoch_ms(&e.value()) {
                                        let old_start = started_at_signal();
                                        let old_end = ended_at_signal();
                                        if old_start > 0 && old_end > old_start {
                                            let gap = old_end - old_start;
                                            ended_at_signal.set(ms + gap);
                                        }
                                        started_at_signal.set(ms);
                                    }
                                },
                                onblur: move |_| save_schedule.call(()),
                            }
                        }
                        div { class: "field",
                            label { class: "field__label", "{tr.schedule_ends_at}" }
                            input {
                                class: "input",
                                r#type: "datetime-local",
                                "data-testid": "schedule-end",
                                value: "{ended_at_str}",
                                oninput: move |e| {
                                    if let Some(ms) = datetime_local_to_epoch_ms(&e.value()) {
                                        ended_at_signal.set(ms);
                                    }
                                },
                                onblur: move |_| save_schedule.call(()),
                            }
                        }
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
                    }
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
