use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::features::arcade::games::fact_or_fold::{
    use_fact_fold_round, BetResponse, BetSide, ChatMessagePayload, RoundParticipantSummary,
    RoundResponse, CHAT_TEXT_MAX_CHARS, FLIP_SLOT_LAST_MS,
};
use crate::features::auth::hooks::use_user_context;
use crate::*;

/// `LiveDebateView` — Stage 5. Free-text chat panel (read polled by
/// the page's polling future, write via ctx.post_chat) + a final-bet
/// flip bar that unlocks during the last `FLIP_SLOT_LAST_MS` of the
/// stage. The flip auto-cites the user marked in the previous reveal
/// stage (ctx.cited_user_pk).
#[component]
pub fn LiveDebateView() -> Element {
    let mut ctx = use_fact_fold_round();
    let round = ctx.round()?();
    let bets = ctx.bets()?();
    let participants = ctx.participants()?();
    let chat = (ctx.chat)();

    let user_ctx = use_user_context();
    let my_pk: UserPartition = UserPartition(user_ctx().user_id().unwrap_or_default());
    let my_bet = bets.items.iter().find(|b| b.user_pk == my_pk).cloned();

    let round_id_val = round.id.clone();

    rsx! {
        section { class: "view", "data-active": true, "data-view": "debate",
            ChatPanel {
                messages: chat.clone(),
                my_pk: my_pk.clone(),
                participants: participants.items.clone(),
                debate_started_at: round.stage_started_at.unwrap_or(0),
                on_post: move |text: String| {
                    let rid = round_id_val.clone();
                    async move {
                        let _ = ctx.post_chat(rid, text).await;
                    }
                },
            }

            FinalBetBar {
                round: round.clone(),
                my_bet,
                cited_user_pk: ctx.cited_user_pk,
            }
        }
    }
}

#[component]
fn ChatPanel(
    messages: Vec<ChatMessagePayload>,
    my_pk: UserPartition,
    participants: Vec<RoundParticipantSummary>,
    debate_started_at: i64,
    on_post: EventHandler<String>,
) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let mut draft = use_signal(String::new);

    let on_input = move |e: FormEvent| {
        let v = e.value();
        let truncated: String = v.chars().take(CHAT_TEXT_MAX_CHARS).collect();
        draft.set(truncated);
    };

    // Submit via <form onsubmit> so the browser fires the event AFTER
    // any in-flight IME composition (Korean / Japanese / Chinese input
    // methods) is committed. The old onkeydown(Enter) path raced with
    // the IME commit, which is what duplicated the last character.
    let on_submit = move |evt: FormEvent| {
        evt.prevent_default();
        let body = draft().trim().to_string();
        if body.is_empty() {
            return;
        }
        draft.set(String::new());
        on_post.call(body);
    };

    rsx! {
        div { class: "live-chat-card",
            div { class: "section-head",
                h2 { "{tr.chat_title}" }
                span { class: "sub", "{tr.chat_sub}" }
            }

            div { class: "live-chat-list",
                if messages.is_empty() {
                    div { class: "ff-room__placeholder", "{tr.chat_empty}" }
                }
                for msg in messages.iter() {
                    {
                        let is_me = msg.author_pk == my_pk;
                        let author_label = participant_display(&msg.author_pk, &participants);
                        let time_label = format_relative_time(msg.sent_at, debate_started_at);
                        let author_class = if is_me { "chat-msg-author me" } else { "chat-msg-author" };
                        rsx! {
                            div { key: "{msg.msg_id}", class: "chat-msg",
                                span { class: "{author_class}", "{author_label}" }
                                span { class: "chat-msg-time", "{time_label}" }
                                span { class: "chat-msg-body", "{msg.text}" }
                            }
                        }
                    }
                }
            }

            form { class: "chat-input-row", onsubmit: on_submit,
                input {
                    r#type: "text",
                    class: "chat-input",
                    placeholder: "{tr.chat_placeholder}",
                    maxlength: "{CHAT_TEXT_MAX_CHARS}",
                    value: "{draft()}",
                    oninput: on_input,
                }
                button {
                    r#type: "submit",
                    class: "btn btn-ghost",
                    style: "padding: 9px 14px; font-size: 12.5px",
                    "{tr.chat_send}"
                }
            }
        }
    }
}

#[component]
fn FinalBetBar(
    round: RoundResponse,
    my_bet: Option<BetResponse>,
    cited_user_pk: Signal<Option<UserPartition>>,
) -> Element {
    let mut ctx = use_fact_fold_round();
    let tr: FactFoldRoomTranslate = use_translate();
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let deadline = round.stage_deadline_at.unwrap_or(0);
    let remaining_ms = (deadline - now).max(0);

    let flip_open = remaining_ms <= FLIP_SLOT_LAST_MS && remaining_ms > 0;
    let countdown_seconds = (remaining_ms as f64 / 1000.0).ceil() as i64;
    let already_flipped = my_bet.as_ref().map(|b| b.flipped_to.is_some()).unwrap_or(false);
    let has_cite = (cited_user_pk)().is_some();

    let body_text = if already_flipped {
        tr.final_already_flipped
    } else if !flip_open {
        tr.final_text_locked
    } else if !has_cite {
        tr.final_text_open_no_cite
    } else {
        tr.final_text_open_ready
    };

    let round_id_val = round.id.clone();
    let bet_side = my_bet.as_ref().map(|b| b.side);

    let on_flip = move |_| {
        let Some(side) = bet_side else { return };
        let flipped_side = match side {
            BetSide::Real => BetSide::Fake,
            BetSide::Fake => BetSide::Real,
        };
        let Some(cite) = (cited_user_pk)() else { return };
        let rid = round_id_val.clone();
        spawn(async move {
            let _ = ctx.flip_bet(rid, flipped_side, cite).await;
        });
    };
    let on_keep = move |_| {
        // No server call needed; flip-slot defaults to "keep" until
        // the deadline passes.
    };

    let countdown_text = if remaining_ms == 0 {
        "—".to_string()
    } else {
        format!("−{countdown_seconds}s")
    };

    let flip_disabled = !flip_open || already_flipped || !has_cite || my_bet.is_none();
    let bar_class = if flip_open { "final-bet-bar" } else { "final-bet-bar hidden" };

    rsx! {
        div { class: "{bar_class}",
            div { class: "final-bet-tag", "{tr.final_tag}" }
            div { class: "final-bet-text", "{body_text}" }
            button {
                class: "mini-btn",
                disabled: !flip_open || already_flipped,
                onclick: on_keep,
                "{tr.final_btn_keep}"
            }
            button {
                class: "mini-btn pink",
                disabled: flip_disabled,
                onclick: on_flip,
                "{tr.final_btn_flip}"
            }
            div { class: "final-bet-countdown", "{countdown_text}" }
        }
    }
}

// ── Helpers ─────────────────────────────────────────────────────────

fn participant_display(pk: &UserPartition, participants: &[RoundParticipantSummary]) -> String {
    let p = participants.iter().find(|x| &x.user_pk == pk);
    let Some(p) = p else {
        return short_pk(&pk.0);
    };
    let src = if !p.display_name.is_empty() {
        &p.display_name
    } else if !p.username.is_empty() {
        &p.username
    } else {
        &pk.0
    };
    src.chars()
        .filter(|c| c.is_alphanumeric())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

fn short_pk(pk: &str) -> String {
    pk.chars().take(2).collect::<String>().to_uppercase()
}

/// `MM:SS` since the debate stage started — bounded to 99:59 for the
/// unlikely case of a round that's been alive longer than the timer
/// (e.g., the local clock disagrees with the server).
fn format_relative_time(sent_at: i64, debate_started_at: i64) -> String {
    if debate_started_at == 0 {
        return "--:--".to_string();
    }
    let secs = ((sent_at - debate_started_at) / 1000).clamp(0, 99 * 60 + 59);
    let mm = secs / 60;
    let ss = secs % 60;
    format!("{mm:02}:{ss:02}")
}
