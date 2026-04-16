use crate::common::config::Environment;
use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::actions::actions::poll::views::main::creator::PollCreatorTranslate;
use crate::features::spaces::pages::actions::components::{ActionDeleteButton, ActionRewardSetting};
use crate::features::spaces::pages::actions::controllers::{
    UpdateSpaceActionRequest, update_space_action,
};

fn epoch_ms_to_datetime_local(ms: i64) -> String {
    if ms <= 0 {
        return String::new();
    }
    let secs = ms / 1000;
    let nanos = ((ms % 1000) * 1_000_000) as u32;
    let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nanos).unwrap_or_default();
    dt.format("%Y-%m-%dT%H:%M").to_string()
}

fn datetime_local_to_epoch_ms(s: &str) -> Option<i64> {
    chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
        .ok()
        .map(|dt| dt.and_utc().timestamp_millis())
}

#[component]
pub fn ConfigCard() -> Element {
    let tr: PollCreatorTranslate = use_translate();
    let mut ctx = use_space_poll_context();
    let mut toast = use_toast();

    let space_id = ctx.space_id;
    let poll_id = ctx.poll_id;
    let poll = ctx.poll.read().clone();
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        poll.space_action.started_at,
    );

    let show_encrypted_upload = Environment::default() != Environment::Production;

    let action_id_str = poll_id().to_string();
    let mut response_editable = use_signal(|| poll.response_editable);
    let mut encrypted_upload = use_signal(|| poll.encrypted_upload_enabled);
    let mut prereq_follow = use_signal(|| poll.space_action.prerequisite);
    let saved_credits = poll.space_action.credits;

    let mut started_at_signal = use_signal(|| poll.started_at);
    let mut ended_at_signal = use_signal(|| poll.ended_at);

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
                Ok(_) => ctx.poll.restart(),
                Err(err) => {
                    error!("Failed to save prerequisite: {:?}", err);
                    toast.error(err);
                    prereq_follow.set(!new_val);
                }
            }
        });
    };

    let started_at = epoch_ms_to_datetime_local(started_at_signal());
    let ended_at = epoch_ms_to_datetime_local(ended_at_signal());

    let save_schedule = move || {
        let start_ms = started_at_signal();
        let end_ms = ended_at_signal();
        if start_ms <= 0 || end_ms <= 0 {
            return;
        }
        spawn(async move {
            let req = UpdatePollRequest::Time {
                started_at: start_ms,
                ended_at: end_ms,
            };
            if let Err(err) = update_poll(space_id(), poll_id(), req).await {
                error!("Failed to save poll schedule: {:?}", err);
                toast.error(err);
            } else {
                ctx.poll.restart();
            }
        });
    };

    let mut toggle_response_editable = move |_| {
        let next = !response_editable();
        response_editable.set(next);
        spawn(async move {
            let req = UpdatePollRequest::ResponseEditable {
                response_editable: next,
            };
            if let Err(err) = update_poll(space_id(), poll_id(), req).await {
                error!("Failed to save response_editable: {:?}", err);
                toast.error(err);
            } else {
                ctx.poll.restart();
            }
        });
    };

    let toggle_encrypted_upload = move |_| {
        let next = !encrypted_upload();
        encrypted_upload.set(next);
        // When encrypted upload is enabled, response editing is forced off server-side.
        if next {
            response_editable.set(false);
        }
        spawn(async move {
            let req = UpdatePollRequest::CanisterUploadEnabled {
                canister_upload_enabled: next,
            };
            if let Err(err) = update_poll(space_id(), poll_id(), req).await {
                error!("Failed to save encrypted_upload: {:?}", err);
                toast.error(err);
            } else {
                ctx.poll.restart();
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

                // ── Schedule (TODO: not wired to backend yet) ─────
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
                                value: "{started_at}",
                                oninput: move |e| {
                                    if let Some(ms) = datetime_local_to_epoch_ms(&e.value()) {
                                        started_at_signal.set(ms);
                                    }
                                },
                                onblur: move |_| save_schedule(),
                            }
                        }
                        div { class: "field",
                            label { class: "field__label", "{tr.schedule_ends_at}" }
                            input {
                                class: "input",
                                r#type: "datetime-local",
                                "data-testid": "schedule-end",
                                value: "{ended_at}",
                                oninput: move |e| {
                                    if let Some(ms) = datetime_local_to_epoch_ms(&e.value()) {
                                        ended_at_signal.set(ms);
                                    }
                                },
                                onblur: move |_| save_schedule(),
                            }
                        }
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

                // ── Voting rules (response editable + encrypted upload) ─────
                section { class: "section", "data-testid": "section-voting-rules",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_voting_rules_label}" }
                    }
                    div {
                        class: "switch",
                        role: "switch",
                        tabindex: "0",
                        "aria-checked": response_editable(),
                        "aria-disabled": encrypted_upload(),
                        "data-testid": "poll-response-editable",
                        onclick: move |evt| {
                            if !encrypted_upload() {
                                toggle_response_editable(evt);
                            }
                        },
                        span { class: "switch__track",
                            span { class: "switch__thumb" }
                        }
                        span {
                            span { class: "switch__label", "{tr.voting_response_editable_label}" }
                            span { class: "switch__sub", "{tr.voting_response_editable_sub}" }
                        }
                    }
                    if show_encrypted_upload {
                        div {
                            class: "switch",
                            role: "switch",
                            tabindex: "0",
                            "aria-checked": encrypted_upload(),
                            "data-testid": "poll-encrypted-upload",
                            onclick: toggle_encrypted_upload,
                            span { class: "switch__track",
                                span { class: "switch__thumb" }
                            }
                            span {
                                span { class: "switch__label", "{tr.voting_encrypted_label}" }
                                span { class: "switch__sub", "{tr.voting_encrypted_sub}" }
                            }
                        }
                    }
                }

                // ── Danger zone ─────
                if !locked {
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
                                    "{tr.delete_poll_title}"
                                }
                                div { style: "font-size:12px;color:var(--qc-text-muted)",
                                    "{tr.delete_poll_desc}"
                                }
                            }
                            ActionDeleteButton {
                                space_id: space_id(),
                                action_id: poll_id().to_string(),
                            }
                        }
                    }
                }
            }
        }
    }
}
