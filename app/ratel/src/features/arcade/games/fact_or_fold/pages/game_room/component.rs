//! `FactFoldGameRoomPage` — the live-round shell.
//!
//! Owns the polling loop, the sidebar (global stage timeline + live
//! timer), and the view-stack dispatch. Each `RoundStatus` value maps
//! to exactly one sub-component, and a pair of `use_future` loops
//! refreshes every round-scoped loader every ~2.5s plus auto-ticks
//! when the client's wall clock crosses `stage_deadline_at`.

use crate::features::arcade::games::fact_or_fold::pages::game_room::{
    FactFoldRoomTranslate, FirstBetView, LiveDebateView, NewsRevealView, ReasoningRevealView,
    ReasoningWriteView, SettlementView,
};
use crate::features::arcade::games::fact_or_fold::{
    use_fact_fold_round_provider, RoundResponse, RoundStatus,
};
use crate::FactFoldRoundEntityType;
use crate::*;

/// Sidebar polling cadence — round + chat. Matches design doc § A2'
/// (v1 = 2~3s short polling).
const POLL_INTERVAL_MS: u64 = 2_500;

/// Heartbeat cadence — keeps the participant alive for the reconnect
/// grace window without spamming the endpoint.
const HEARTBEAT_INTERVAL_MS: u64 = 30_000;

#[component]
pub fn FactFoldGameRoomPage(round_id: ReadSignal<FactFoldRoundEntityType>) -> Element {
    let mut ctx = use_fact_fold_round_provider(round_id)?;
    let round = ctx.round;

    // Polling loop: refresh every loader + pull chat deltas + tick
    // the stage when the wall-clock passes the deadline.
    use_future(move || async move {
        loop {
            crate::common::utils::time::sleep(std::time::Duration::from_millis(POLL_INTERVAL_MS))
                .await;

            let r = (ctx.round)();
            let now = crate::common::utils::time::get_now_timestamp_millis();
            let past_deadline = r.stage_deadline_at.map(|d| now >= d).unwrap_or(false);
            let active = !matches!(r.status, RoundStatus::Waiting | RoundStatus::Settled);
            if active && past_deadline {
                let _ = ctx.tick(round_id()).await;
            }

            // Always re-pull state — `tick` already restarts the
            // loaders, but plain refreshes catch out-of-band changes
            // (other players' bets/rationales/chat).
            ctx.refresh_all();
            let _ = ctx.poll_chat(round_id()).await;
        }
    });

    // Heartbeat loop — separate cadence so we don't flood the
    // heartbeat endpoint on the polling tick.
    use_future(move || async move {
        loop {
            crate::common::utils::time::sleep(std::time::Duration::from_millis(
                HEARTBEAT_INTERVAL_MS,
            ))
            .await;
            let _ = ctx.heartbeat(round_id()).await;
        }
    });

    rsx! {
        SeoMeta { title: "Fact or Fold · Ratel Arcade" }
        div { class: "ff-room",
            // Top bar now lives in ArcadeLayout (wraps every /arcade
            // page). The round-status sub-line is folded into the
            // sidebar timer card instead so the brand bar stays
            // arcade-wide.

            div { class: "layout",
                aside { class: "sidebar",
                    StageTimeline { round: round() }
                    TimerCard { round: round() }
                }

                div { class: "view-stack",
                    {render_stage(&round())}
                }
            }
        }
    }
}

// ── Sidebar — stage timeline ────────────────────────────────────────

#[component]
fn StageTimeline(round: RoundResponse) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let stages: [(RoundStatus, &str, &str); 6] = [
        (
            RoundStatus::NewsReveal,
            tr.stage_news_reveal_name,
            tr.stage_news_reveal_time,
        ),
        (RoundStatus::Bet, tr.stage_bet_name, tr.stage_bet_time),
        (
            RoundStatus::Rationale,
            tr.stage_rationale_name,
            tr.stage_rationale_time,
        ),
        (
            RoundStatus::Reveal,
            tr.stage_reveal_name,
            tr.stage_reveal_time,
        ),
        (
            RoundStatus::Debate,
            tr.stage_debate_name,
            tr.stage_debate_time,
        ),
        (
            RoundStatus::Settlement,
            tr.stage_settlement_name,
            tr.stage_settlement_time,
        ),
    ];

    let current_idx = stage_index(round.status);

    rsx! {
        div { class: "stage-timeline stage-timeline-global", id: "globalTimeline",
            for (idx , (_, name , time)) in stages.iter().enumerate() {
                {
                    let state = if let Some(curr) = current_idx {
                        if idx < curr {
                            "done"
                        } else if idx == curr {
                            "active"
                        } else {
                            "upcoming"
                        }
                    } else {
                        "upcoming"
                    };
                    let dot_label = if state == "done" {
                        "✓".to_string()
                    } else {
                        (idx + 1).to_string()
                    };
                    rsx! {
                        div { class: "stage-step", "data-state": state,
                            div { class: "stage-dot", "{dot_label}" }
                            div { class: "stage-label",
                                div { class: "stage-label-name", "{name}" }
                                div { class: "stage-label-time", "{time}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ── Sidebar — live timer ────────────────────────────────────────────

#[component]
fn TimerCard(round: RoundResponse) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();

    // The round loader only refreshes every POLL_INTERVAL_MS (~2.5s),
    // so without a local tick the timer jumps in chunks. This signal
    // re-renders the card every second; compute_timer recomputes from
    // wall-clock vs `stage_deadline_at` each time.
    let mut tick = use_signal(|| 0u32);
    use_future(move || async move {
        loop {
            crate::common::utils::time::sleep(std::time::Duration::from_secs(1)).await;
            tick.with_mut(|t| *t = t.wrapping_add(1));
        }
    });
    let _ = tick();

    let TimerState {
        stage_name,
        deadline_label,
        time_label,
        sub_label,
        frac_remaining,
    } = compute_timer(&round, &tr);

    // Ring stroke-dashoffset: full circle is 377, we offset by the
    // fraction *spent* so the visible arc shrinks as time runs out.
    let dash_offset = 377.0 * (1.0 - frac_remaining.clamp(0.0, 1.0));
    let dash_offset_str = format!("{dash_offset:.1}");

    rsx! {
        div { class: "timer-card",
            div { class: "section-label", "{tr.timer_section_label}" }
            div { class: "timer-stage", "{stage_name}" }
            div { class: "timer-deadline", "{deadline_label}" }
            div { class: "timer-ring",
                svg { width: 140, height: 140, view_box: "0 0 140 140",
                    defs {
                        linearGradient {
                            id: "timerGradSide",
                            x1: "0%",
                            y1: "0%",
                            x2: "100%",
                            y2: "100%",
                            stop { offset: "0%", stop_color: "#fcb300" }
                            stop { offset: "100%", stop_color: "#db2780" }
                        }
                    }
                    circle { cx: 70, cy: 70, r: 60, class: "timer-ring-bg" }
                    circle {
                        cx: 70,
                        cy: 70,
                        r: 60,
                        class: "timer-ring-fg",
                        stroke_dasharray: "377",
                        stroke_dashoffset: "{dash_offset_str}",
                        style: "stroke: url(#timerGradSide)",
                    }
                }
                div { class: "timer-display",
                    div {
                        div { class: "timer-display-time", "{time_label}" }
                        div { class: "timer-display-sub", "{sub_label}" }
                    }
                }
            }
        }
    }
}

// ── View-stack dispatch ─────────────────────────────────────────────

fn render_stage(round: &RoundResponse) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    match round.status {
        RoundStatus::Waiting => rsx! {
            section { class: "view", "data-active": true,
                div { class: "ff-room__placeholder", "{tr.waiting_for_players}" }
            }
        },
        RoundStatus::NewsReveal => rsx! { NewsRevealView {} },
        RoundStatus::Bet => rsx! { FirstBetView {} },
        RoundStatus::Rationale => rsx! { ReasoningWriteView {} },
        RoundStatus::Reveal => rsx! { ReasoningRevealView {} },
        RoundStatus::Debate => rsx! { LiveDebateView {} },
        RoundStatus::Settlement => rsx! {
            section { class: "view", "data-active": true,
                div { class: "ff-room__placeholder", "{tr.settling}" }
            }
        },
        RoundStatus::Settled => rsx! { SettlementView {} },
    }
}

// ── Helpers ─────────────────────────────────────────────────────────

/// Map `RoundStatus` to its index on the 6-step timeline. `Waiting`
/// isn't on the bar so return `None`; `Settled` lands on the
/// terminal step.
fn stage_index(s: RoundStatus) -> Option<usize> {
    match s {
        RoundStatus::NewsReveal => Some(0),
        RoundStatus::Bet => Some(1),
        RoundStatus::Rationale => Some(2),
        RoundStatus::Reveal => Some(3),
        RoundStatus::Debate => Some(4),
        RoundStatus::Settlement | RoundStatus::Settled => Some(5),
        RoundStatus::Waiting => None,
    }
}

/// Live brand sub-line — round id + status badge. Falls back to a
/// neutral label while we don't have a round id yet.
fn brand_sub(round: &RoundResponse) -> String {
    let status = match round.status {
        RoundStatus::Waiting => "WAITING",
        RoundStatus::NewsReveal
        | RoundStatus::Bet
        | RoundStatus::Rationale
        | RoundStatus::Reveal
        | RoundStatus::Debate => "LIVE",
        RoundStatus::Settlement => "SETTLING",
        RoundStatus::Settled => "SETTLED",
    };
    let id_part = round.id.0.clone();
    if id_part.is_empty() {
        format!("Fact or Fold · {status}")
    } else {
        format!("Fact or Fold · {id_part} · {status}")
    }
}

struct TimerState {
    stage_name: String,
    deadline_label: String,
    time_label: String,
    sub_label: String,
    frac_remaining: f32,
}

/// Compute everything the live-timer card displays. Pulled out so the
/// component body stays declarative.
fn compute_timer(round: &RoundResponse, tr: &FactFoldRoomTranslate) -> TimerState {
    let stage_name = stage_name(round.status, tr).to_string();
    let next_label = next_stage_name(round.status, tr);

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let started = round.stage_started_at.unwrap_or(0);
    let deadline = round.stage_deadline_at.unwrap_or(0);

    if deadline == 0 || matches!(round.status, RoundStatus::Waiting) {
        return TimerState {
            stage_name,
            deadline_label: String::new(),
            time_label: "—".to_string(),
            sub_label: tr.timer_waiting_sub.to_string(),
            frac_remaining: 0.0,
        };
    }
    if matches!(round.status, RoundStatus::Settled) {
        return TimerState {
            stage_name,
            deadline_label: String::new(),
            time_label: "—".to_string(),
            sub_label: tr.timer_done_sub.to_string(),
            frac_remaining: 0.0,
        };
    }

    let remaining_ms = (deadline - now).max(0);
    let total_ms = (deadline - started).max(1);
    let frac = remaining_ms as f32 / total_ms as f32;
    let seconds = (remaining_ms as f64 / 1000.0).ceil() as i64;
    let time_label = format!("{seconds:02}s");
    let deadline_label = if next_label.is_empty() {
        String::new()
    } else {
        format!("{} {}", tr.timer_next, next_label)
    };

    TimerState {
        stage_name,
        deadline_label,
        time_label,
        sub_label: tr.timer_remaining_sub.to_string(),
        frac_remaining: frac,
    }
}

fn stage_name(s: RoundStatus, tr: &FactFoldRoomTranslate) -> &'static str {
    match s {
        RoundStatus::Waiting | RoundStatus::NewsReveal => tr.stage_news_reveal_name,
        RoundStatus::Bet => tr.stage_bet_name,
        RoundStatus::Rationale => tr.stage_rationale_name,
        RoundStatus::Reveal => tr.stage_reveal_name,
        RoundStatus::Debate => tr.stage_debate_name,
        RoundStatus::Settlement | RoundStatus::Settled => tr.stage_settlement_name,
    }
}

fn next_stage_name(s: RoundStatus, tr: &FactFoldRoomTranslate) -> &'static str {
    match s {
        RoundStatus::NewsReveal => tr.stage_bet_name,
        RoundStatus::Bet => tr.stage_rationale_name,
        RoundStatus::Rationale => tr.stage_reveal_name,
        RoundStatus::Reveal => tr.stage_debate_name,
        RoundStatus::Debate => tr.stage_settlement_name,
        _ => "",
    }
}
