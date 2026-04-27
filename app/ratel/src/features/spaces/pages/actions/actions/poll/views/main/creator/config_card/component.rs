use crate::features::spaces::pages::actions::actions::poll::views::main::creator::PollCreatorTranslate;
use crate::features::spaces::pages::actions::actions::poll::*;
use crate::features::spaces::pages::actions::components::{
    ActionDeleteButton, ActionDependencySelector, ActionRewardSetting, ActionStatusControl,
    PrerequisiteTile,
};

#[component]
pub fn ConfigCard() -> Element {
    let tr: PollCreatorTranslate = use_translate();
    let mut ctx = use_space_poll_context();
    let mut toast = use_toast();

    let space_id = ctx.space_id;
    let poll_id = ctx.poll_id;
    let poll = ctx.poll.read().clone();
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let _locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        poll.space_action.status.as_ref(),
    );

    let action_id_str = poll_id().to_string();
    let mut response_editable = use_signal(|| poll.response_editable);
    let mut encrypted_upload = use_signal(|| poll.encrypted_upload_enabled);
    let initial_prerequisite = poll.space_action.prerequisite;
    let saved_credits = poll.space_action.credits;
    let action_status = poll.space_action.status.clone();
    let initial_depends_on = poll.space_action.depends_on.clone();
    let initial_status = poll.space_action.status.clone();

    let action_id_for_signal = action_id_str.clone();
    let action_id_signal: ReadSignal<String> =
        use_signal(move || action_id_for_signal.clone()).into();

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
                        on_changed: move |_| ctx.poll.restart(),
                    }
                }

                // ── Voting rules (response editable + encrypted upload) ─────
                section { class: "section", "data-testid": "section-voting-rules",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_voting_rules_label}" }
                    }
                    div { class: "setting-row", "data-testid": "poll-response-editable",
                        div { class: "setting-row__text",
                            span { class: "setting-row__label", "{tr.voting_response_editable_label}" }
                            span { class: "setting-row__sub", "{tr.voting_response_editable_sub}" }
                        }
                        crate::common::components::Switch {
                            active: response_editable(),
                            disabled: encrypted_upload(),
                            on_toggle: toggle_response_editable,
                            label: tr.voting_response_editable_label.to_string(),
                        }
                    }
                    div { class: "setting-row", "data-testid": "poll-encrypted-upload",
                        div { class: "setting-row__text",
                            span { class: "setting-row__label", "{tr.voting_encrypted_label}" }
                            span { class: "setting-row__sub", "{tr.voting_encrypted_sub}" }
                        }
                        crate::common::components::Switch {
                            active: encrypted_upload(),
                            on_toggle: toggle_encrypted_upload,
                            label: tr.voting_encrypted_label.to_string(),
                        }
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
                        on_changed: move |_| ctx.poll.restart(),
                    }
                }

                // ── Danger zone ─────
                if !_locked {
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
