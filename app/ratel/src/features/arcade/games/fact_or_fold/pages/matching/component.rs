//! `FactFoldMatchingPage` — `/arcade/games/fact-or-fold/matching/:round_id`.
//! Pre-round waiting room. Polls /lobby every 2s, renders 4 seat
//! slots reflecting current participants, and auto-redirects to the
//! game room the moment the round transitions out of `Waiting`. The
//! caller's joined round lives in the URL so the page survives a
//! refresh and doesn't depend on the in-process `lobby.current_round`
//! pointer (which `join_lobby_handler` clears the moment capacity
//! fills, see PR comment thread).

use crate::features::arcade::games::fact_or_fold::pages::matching::{
    use_fact_fold_matching_provider, FactFoldMatchingTranslate,
};
use crate::features::arcade::games::fact_or_fold::{LobbyResponse, RoundStatus};
use crate::features::auth::hooks::use_user_context;
use crate::FactFoldRoundEntityType;
use crate::*;

const POLL_INTERVAL_MS: u64 = 2_000;

#[component]
pub fn FactFoldMatchingPage(round_id: ReadSignal<FactFoldRoundEntityType>) -> Element {
    let mut ctx = use_fact_fold_matching_provider()?;
    let nav = use_navigator();
    let lobby_loader = ctx.lobby()?;
    let lobby = lobby_loader();

    // Polling — refresh lobby every 2s so the slot grid + progress
    // bar track other players joining/leaving in near real-time.
    use_future(move || async move {
        loop {
            crate::common::utils::time::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS))
                .await;
            ctx.lobby_refresh.with_mut(|n| *n += 1);
        }
    });

    // Auto-redirect:
    //   - Lobby still tracks my round AND it's no longer Waiting → game room.
    //   - Lobby tracks a different round (capacity filled, lobby pointer
    //     advanced) or has been cleared → jump to my game room.
    //   - Lobby still tracks my round and it's Waiting → stay put.
    let my_round_id = round_id();
    match lobby.current_round.as_ref() {
        Some(round) if round.id == my_round_id => {
            if !matches!(round.status, RoundStatus::Waiting) {
                nav.push(Route::FactFoldGameRoomPage { round_id: my_round_id });
            }
        }
        _ => {
            nav.push(Route::FactFoldGameRoomPage { round_id: my_round_id });
        }
    }

    rsx! {
        SeoMeta { title: "Matching · Fact or Fold" }
        section { class: "ff-matching",
            MatchingShell { lobby }
        }
    }
}

#[component]
fn MatchingShell(lobby: LobbyResponse) -> Element {
    let tr: FactFoldMatchingTranslate = use_translate();
    let user_ctx = use_user_context();
    let my_pk = user_ctx().user_id().unwrap_or_default();
    let my_display = user_ctx()
        .user
        .as_ref()
        .map(|u| {
            if u.display_name.is_empty() {
                u.username.clone()
            } else {
                u.display_name.clone()
            }
        })
        .unwrap_or_else(|| "you".to_string());

    let round = lobby.current_round.clone();
    let capacity = lobby.round_capacity.max(1) as usize;
    let participants: Vec<String> = round
        .as_ref()
        .map(|r| r.participant_pks.iter().map(|p| p.0.clone()).collect())
        .unwrap_or_default();
    let current = participants.len();
    let ready = round
        .as_ref()
        .map(|r| !matches!(r.status, RoundStatus::Waiting))
        .unwrap_or(false);

    let eyebrow_label = if ready {
        tr.eyebrow_full.to_string()
    } else if round.is_some() {
        tr.eyebrow_waiting.replace("{$capacity}", &capacity.to_string())
    } else {
        tr.eyebrow_no_round.to_string()
    };

    rsx! {
        div { class: "matching-eyebrow",
            span { class: "matching-eyebrow-dot" }
            span { "{eyebrow_label}" }
        }

        h1 { class: "matching-title", "{tr.title}" }
        p { class: "matching-subtitle", "{tr.subtitle}" }

        BuyinNote { buy_in_chips: lobby.buy_in_chips }

        MatchingSlots {
            participants: participants.clone(),
            my_pk: my_pk.clone(),
            my_display,
            capacity,
        }

        ProgressBar { current, capacity }
        HintsStrip {}
        CancelRow {}
    }
}

#[component]
fn BuyinNote(buy_in_chips: i64) -> Element {
    let tr: FactFoldMatchingTranslate = use_translate();
    let body = tr.buyin_note.replace("{$chips}", &buy_in_chips.to_string());
    rsx! {
        div { class: "matching-buyin-note", "{body}" }
    }
}

#[component]
fn MatchingSlots(
    participants: Vec<String>,
    my_pk: String,
    my_display: String,
    capacity: usize,
) -> Element {
    let tr: FactFoldMatchingTranslate = use_translate();

    rsx! {
        div { class: "matching-slots",
            for idx in 0..capacity {
                {
                    let pk = participants.get(idx).cloned();
                    let seat_label = tr.slot_seat.replace("{$n}", &(idx + 1).to_string());
                    match pk {
                        Some(pk) if pk == my_pk => rsx! {
                            FilledSlot {
                                key: "seat-{idx}",
                                idx,
                                seat_label,
                                pk: pk.clone(),
                                display: my_display.clone(),
                                is_me: true,
                            }
                        },
                        Some(pk) => rsx! {
                            FilledSlot {
                                key: "seat-{idx}",
                                idx,
                                seat_label,
                                pk: pk.clone(),
                                display: short_display(&pk),
                                is_me: false,
                            }
                        },
                        None => rsx! {
                            EmptySlot { key: "seat-{idx}", idx, seat_label }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn FilledSlot(
    idx: usize,
    seat_label: String,
    pk: String,
    display: String,
    is_me: bool,
) -> Element {
    let tr: FactFoldMatchingTranslate = use_translate();
    let state = if is_me { "me" } else { "filled" };
    let avatar_variant = avatar_variant(idx);
    let avatar_class = if avatar_variant.is_empty() {
        "matching-slot-avatar".to_string()
    } else {
        format!("matching-slot-avatar {avatar_variant}")
    };
    let initials = initials(&display);
    let _ = pk;

    let slot_testid = if is_me {
        "ff-matching-slot-self".to_string()
    } else {
        format!("ff-matching-slot-{idx}")
    };

    rsx! {
        div {
            class: "matching-slot",
            "data-testid": "{slot_testid}",
            "data-state": state,
            div { class: "matching-slot-num", "{seat_label}" }
            div { class: "{avatar_class}", "{initials}" }
            div { class: "matching-slot-name",
                "{display}"
                if is_me {
                    span { class: "you-tag", "{tr.you_tag}" }
                }
            }
            div { class: "matching-slot-pill", "{tr.slot_pill_ready}" }
        }
    }
}

#[component]
fn EmptySlot(idx: usize, seat_label: String) -> Element {
    let tr: FactFoldMatchingTranslate = use_translate();
    let _ = idx;
    rsx! {
        div { class: "matching-slot", "data-state": "empty",
            div { class: "matching-slot-num", "{seat_label}" }
            div { class: "matching-slot-avatar", "··" }
            div { class: "matching-slot-placeholder", "{tr.slot_empty}" }
            div { class: "matching-slot-pill", "{tr.slot_pill_empty}" }
        }
    }
}

#[component]
fn ProgressBar(current: usize, capacity: usize) -> Element {
    let tr: FactFoldMatchingTranslate = use_translate();
    let pct = if capacity == 0 {
        0
    } else {
        ((current as f32 / capacity as f32) * 100.0).clamp(0.0, 100.0) as i32
    };
    let count_label = tr
        .progress_count
        .replace("{$current}", &current.to_string())
        .replace("{$capacity}", &capacity.to_string());
    let fill_style = format!("width: {pct}%");

    rsx! {
        div { class: "matching-progress",
            div { class: "matching-progress-head",
                span { "{tr.progress_label}" }
                span { class: "matching-progress-count", "{count_label}" }
            }
            div { class: "matching-progress-bar",
                div { class: "matching-progress-fill", style: "{fill_style}" }
            }
        }
    }
}

#[component]
fn HintsStrip() -> Element {
    let tr: FactFoldMatchingTranslate = use_translate();
    rsx! {
        div { class: "matching-hints",
            div { class: "matching-hint",
                span { class: "matching-hint-icon", "◇" }
                span {
                    strong { "{tr.hint_timing_strong}" }
                    "{tr.hint_timing_body}"
                }
            }
            div { class: "matching-hint",
                span { class: "matching-hint-icon", "⚐" }
                span {
                    strong { "{tr.hint_insider_strong}" }
                    "{tr.hint_insider_body}"
                }
            }
            div { class: "matching-hint",
                span { class: "matching-hint-icon", "◆" }
                span {
                    strong { "{tr.hint_persuade_strong}" }
                    "{tr.hint_persuade_body}"
                }
            }
        }
    }
}

#[component]
fn CancelRow() -> Element {
    let tr: FactFoldMatchingTranslate = use_translate();
    let mut ctx = use_fact_fold_matching_provider()?;
    let nav = use_navigator();
    let mut submitting = use_signal(|| false);
    let mut error = use_signal(|| Option::<String>::None);

    let on_cancel = move |_| async move {
        if submitting() {
            return;
        }
        submitting.set(true);
        error.set(None);
        match ctx.leave().await {
            Ok(_) => {
                nav.push(Route::ArcadeHomePage {});
            }
            Err(_) => {
                error.set(Some(tr.leave_error.to_string()));
            }
        }
        submitting.set(false);
    };

    rsx! {
        div { class: "matching-actions",
            button {
                class: "btn btn-ghost",
                "data-testid": "ff-matching-cancel",
                style: "padding: 10px 18px; font-size: 13px",
                disabled: submitting(),
                onclick: on_cancel,
                "{tr.cancel_btn}"
            }
            span { class: "matching-actions-hint", "{tr.cancel_hint}" }
        }
        if let Some(err) = error() {
            div {
                class: "reason-warn",
                style: "margin-top: 10px; max-width: 520px",
                "{err}"
            }
        }
    }
}

// ── Helpers ─────────────────────────────────────────────────────────

fn avatar_variant(idx: usize) -> &'static str {
    match idx % 4 {
        0 => "",
        1 => "a2",
        2 => "a3",
        _ => "a4",
    }
}

fn initials(display: &str) -> String {
    if display.is_empty() {
        return "?".to_string();
    }
    display
        .chars()
        .filter(|c| c.is_alphanumeric())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

/// `USER#abc123def…` → `AB`. Fallback only — the matching page
/// doesn't have a per-participant display-name lookup yet (the
/// /lobby endpoint returns pks only). Once /participants is wired
/// up through here, this falls away.
fn short_display(pk: &str) -> String {
    let stripped = pk.strip_prefix("USER#").unwrap_or(pk);
    stripped.chars().take(6).collect::<String>().to_lowercase()
}
