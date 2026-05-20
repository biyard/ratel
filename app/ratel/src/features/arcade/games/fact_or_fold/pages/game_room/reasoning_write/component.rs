use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::features::arcade::games::fact_or_fold::{
    use_fact_fold_round, BetResponse, BetSide, RationaleResponse, RoundParticipantSummary,
    RATIONALE_TEXT_MAX_CHARS,
};
use crate::features::auth::hooks::use_user_context;
use crate::*;

/// `ReasoningWriteView` — Stage 3. Left column hosts the rationale
/// textarea + char counter; right column shows tips + per-player
/// "submitted" / "writing" pulses + (for the insider) an extra
/// strategy hint.
#[component]
pub fn ReasoningWriteView() -> Element {
    let mut ctx = use_fact_fold_round();
    let tr: FactFoldRoomTranslate = use_translate();
    let round = ctx.round()?();
    let bets = ctx.bets()?();
    let rationales = ctx.rationales()?();
    let participants = ctx.participants()?();
    let insider = ctx.insider()?();

    let user_ctx = use_user_context();
    let my_pk: UserPartition = UserPartition(user_ctx().user_id().unwrap_or_default());

    let my_bet = bets.items.iter().find(|b| b.user_pk == my_pk).cloned();
    let my_rationale = rationales
        .items
        .iter()
        .find(|r| r.user_pk == my_pk)
        .cloned();

    let round_id_val = round.id.clone();

    rsx! {
        section { class: "view", "data-active": true, "data-view": "reason",
            div { class: "bet-grid",
                div {
                    if let Some(bet) = my_bet.as_ref() {
                        if let Some(rationale) = my_rationale.as_ref() {
                            ReasonSubmittedCard {
                                rationale: rationale.clone(),
                                bet: bet.clone(),
                            }
                        } else {
                            ReasonWriteCard {
                                bet: bet.clone(),
                                on_submit: move |text: String| {
                                    let rid = round_id_val.clone();
                                    async move {
                                        let _ = ctx.submit_rationale(rid, text).await;
                                    }
                                },
                            }
                        }
                    } else {
                        div { class: "reason-live-card",
                            div { class: "reason-warn", "{tr.reason_no_bet_warning}" }
                        }
                    }
                }
                div {
                    ReasonSidePanel {
                        participants: participants.items.clone(),
                        rationales: rationales.items.clone(),
                        my_pk: my_pk.clone(),
                        is_insider: insider.statement.is_some(),
                    }
                }
            }
        }
    }
}

#[component]
fn ReasonWriteCard(bet: BetResponse, on_submit: EventHandler<String>) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let mut text = use_signal(String::new);
    let mut submitting = use_signal(|| false);

    let on_input = move |e: FormEvent| text.set(e.value());

    let on_confirm = move |_| async move {
        if submitting() {
            return;
        }
        let body = text();
        let len = body.chars().count();
        if body.trim().is_empty() || len > RATIONALE_TEXT_MAX_CHARS {
            return;
        }
        submitting.set(true);
        on_submit.call(body);
        submitting.set(false);
    };

    let len = text().chars().count();
    let counter_class = if len > 0 && len <= RATIONALE_TEXT_MAX_CHARS {
        "char-counter ok"
    } else {
        "char-counter warn"
    };
    let prompt_text = render_prompt(&bet, &tr);
    let pink_or_gold_class = match bet.side {
        BetSide::Real => "gold-mark",
        BetSide::Fake => "pink-mark",
    };
    let bet_side_label = match bet.side {
        BetSide::Real => "REAL",
        BetSide::Fake => "FAKE",
    };

    rsx! {
        div { class: "reason-live-card",
            div { class: "reason-prompt",
                div { class: "reason-prompt-label", "{tr.reason_prompt_label}" }
                div { class: "reason-prompt-text",
                    span { class: "{pink_or_gold_class}", "{bet_side_label}" }
                    " · {bet.amount_rp} RP — "
                    "{prompt_text}"
                }
            }

            div { class: "reason-wrap",
                div { class: "reason-head",
                    span { class: "reason-label", "{tr.reason_textarea_label}" }
                    span { class: "{counter_class}", "{len} / {RATIONALE_TEXT_MAX_CHARS}" }
                }
                textarea {
                    class: "reason-textarea",
                    placeholder: "{tr.reason_textarea_placeholder}",
                    oninput: on_input,
                    value: "{text()}",
                }
                div { class: "reason-warn", "{tr.reason_warn}" }
            }

            div { class: "submit-row",
                button {
                    class: "btn btn-primary",
                    disabled: submitting() || len == 0 || len > RATIONALE_TEXT_MAX_CHARS,
                    onclick: on_confirm,
                    "{tr.reason_submit}"
                }
                span { style: "font-size: 12px; color: var(--text-muted)", "{tr.reason_submit_hint}" }
            }
        }
    }
}

#[component]
fn ReasonSubmittedCard(rationale: RationaleResponse, bet: BetResponse) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let bet_side_label = match bet.side {
        BetSide::Real => "REAL",
        BetSide::Fake => "FAKE",
    };
    let bet_class = match bet.side {
        BetSide::Real => "pill gold",
        BetSide::Fake => "pill pink",
    };

    rsx! {
        div { class: "reason-live-card",
            div { class: "reason-prompt",
                div { class: "reason-prompt-label", "{tr.reason_submitted_title}" }
                div { class: "reason-prompt-text",
                    span { class: "{bet_class}", "{bet_side_label}" }
                    " · {bet.amount_rp} RP"
                }
            }
            div { class: "reason-wrap",
                div { class: "reason-head",
                    span { class: "reason-label", "{tr.reason_textarea_label}" }
                }
                // Read-only: render the submitted text as a styled
                // block instead of a textarea so the player sees it
                // but can't edit (server doesn't accept resubmission).
                div {
                    class: "reason-textarea",
                    style: "background: rgba(0,0,0,0.18); white-space: pre-wrap;",
                    "{rationale.text}"
                }
            }
            div {
                class: "reason-warn",
                style: "background: rgba(110,237,216,0.08); color: var(--teal); border-color: rgba(110,237,216,0.25)",
                "{tr.reason_submitted_body}"
            }
        }
    }
}

#[component]
fn ReasonSidePanel(
    participants: Vec<RoundParticipantSummary>,
    rationales: Vec<RationaleResponse>,
    my_pk: UserPartition,
    is_insider: bool,
) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();

    rsx! {
        div { class: "bet-summary",
            h3 { class: "card-title", "{tr.reason_tips_title}" }
            div { style: "font-size: 13px; color: var(--text-muted); line-height: 1.7; margin-bottom: 14px",
                "{tr.reason_tips_body}"
            }

            div { class: "section-label", "{tr.reason_others_label}" }
            div { class: "player-pulses",
                for (idx, p) in participants.iter().enumerate() {
                    if p.user_pk != my_pk {
                        {
                            let submitted = rationales.iter().any(|r| r.user_pk == p.user_pk);
                            let chip_class = if submitted {
                                "pulse-chip done"
                            } else {
                                "pulse-chip thinking"
                            };
                            let state_label = if submitted {
                                tr.reason_pulse_submitted
                            } else {
                                tr.reason_pulse_writing
                            };
                            let initials = pulse_initials(&p.display_name, &p.username);
                            let avatar_style = pulse_avatar_style(idx);
                            rsx! {
                                div { key: "{p.user_pk}", class: "{chip_class}",
                                    div { class: "pulse-chip-avatar", style: "{avatar_style}", "{initials}" }
                                    div { class: "pulse-chip-state", "{state_label}" }
                                }
                            }
                        }
                    }
                }
            }

            if is_insider {
                div { style: "margin-top: 18px; padding: 12px 14px; background: var(--purple-soft); border: 1px solid rgba(167,139,250,0.3); border-radius: var(--r-md); font-size: 12.5px; color: var(--text-muted); line-height: 1.6",
                    strong { style: "color: var(--purple)", "INSIDER: " }
                    "{tr.reason_insider_hint}"
                }
            }
        }
    }
}

// ── Helpers ─────────────────────────────────────────────────────────

fn render_prompt(bet: &BetResponse, tr: &FactFoldRoomTranslate) -> String {
    let template = match bet.side {
        BetSide::Real => tr.reason_prompt_text_real,
        BetSide::Fake => tr.reason_prompt_text_fake,
    };
    template.replace("{$amount}", &bet.amount_rp.to_string())
}

fn pulse_initials(display: &str, username: &str) -> String {
    let src = if !display.is_empty() {
        display
    } else if !username.is_empty() {
        username
    } else {
        "?"
    };
    src.chars()
        .filter(|c| c.is_alphanumeric())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

fn pulse_avatar_style(idx: usize) -> &'static str {
    match idx % 4 {
        0 => "background: linear-gradient(135deg, #4f3aaf, #7d3aaf)",
        1 => "background: linear-gradient(135deg, #3aaf7d, #6eedd8); color: #042a1f",
        2 => "background: linear-gradient(135deg, #af3a7d, #db2780)",
        _ => "background: linear-gradient(135deg, #af7d3a, #fcb300); color: #1a0a00",
    }
}
