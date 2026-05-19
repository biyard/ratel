use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::features::arcade::games::fact_or_fold::{
    use_fact_fold_round, RoundSubjectResponse, RoundParticipantSummary, RoundResponse,
    RoundStatus,
};
use crate::features::auth::hooks::use_user_context;
use crate::*;

/// `NewsRevealView` — Stage 1. Shows the public subject (text +
/// excerpt + source label + category/difficulty pills) alongside a
/// roster of the 4 participants. Stage auto-advances on timer; this
/// view is read-only.
#[component]
pub fn NewsRevealView() -> Element {
    let ctx = use_fact_fold_round();
    let subject = (ctx.subject)();
    let participants = (ctx.participants)();
    let round = (ctx.round)();

    let user_ctx = use_user_context();
    let my_pk: UserPartition = UserPartition(user_ctx().user_id().unwrap_or_default());

    rsx! {
        section { class: "view", "data-active": true, "data-view": "round",
            div { class: "round-grid",
                div {
                    NewsCard { subject: subject.clone() }
                    PlayersCard {
                        participants: participants.items.clone(),
                        round: round.clone(),
                        my_pk: my_pk.clone(),
                    }
                }
            }
        }
    }
}

#[component]
fn NewsCard(subject: RoundSubjectResponse) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let difficulty_label = render_difficulty(subject.difficulty);
    let primary_tag = subject
        .category_tags
        .first()
        .cloned()
        .unwrap_or_else(|| tr.news_pill_category_default.to_string());

    rsx! {
        div { class: "news-card",
            div { class: "news-source",
                span { "📰" }
                span { "{subject.source_label}" }
                span { class: "news-source-dot" }
                span { "{tr.news_source_lock}" }
            }
            h2 { class: "news-subject", "{subject.headline_text}" }
            p { class: "news-excerpt", "{subject.body_excerpt}" }
            div { class: "news-meta",
                span { class: "pill", "{primary_tag}" }
                span { class: "pill purple", "{tr.news_difficulty} {difficulty_label}" }
                span { class: "news-cta",
                    span { class: "pill teal", "{tr.news_cta_label}" }
                }
            }
        }
    }
}

#[component]
fn PlayersCard(
    participants: Vec<RoundParticipantSummary>,
    round: RoundResponse,
    my_pk: UserPartition,
) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let capacity = round.participant_pks.len().max(participants.len());
    let count = participants.len();
    let count_label = tr
        .players_card_count
        .replace("{$count}", &count.to_string())
        .replace("{$capacity}", &capacity.to_string());

    rsx! {
        div { class: "players-card",
            div { class: "players-head",
                div { class: "section-label", "{tr.players_card_title}" }
                span { style: "font-family: 'JetBrains Mono', monospace; font-size: 12px; color: var(--text-muted)",
                    "{count_label}"
                }
            }
            div { class: "players-list",
                for (idx, p) in participants.iter().enumerate() {
                    PlayerRow {
                        key: "{p.user_pk}",
                        participant: p.clone(),
                        round_status: round.status,
                        my_pk: my_pk.clone(),
                        avatar_variant: avatar_variant(idx),
                    }
                }
            }
        }
    }
}

#[component]
fn PlayerRow(
    participant: RoundParticipantSummary,
    round_status: RoundStatus,
    my_pk: UserPartition,
    avatar_variant: &'static str,
) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let is_me = participant.user_pk == my_pk;
    let row_class = if is_me { "player-row me" } else { "player-row" };
    let avatar_class = if avatar_variant.is_empty() {
        "p-avatar".to_string()
    } else {
        format!("p-avatar {avatar_variant}")
    };
    let display = if participant.display_name.is_empty() {
        participant.username.clone()
    } else {
        participant.display_name.clone()
    };
    let initials = initials(&display, &participant.username);
    let status_label = status_text(round_status, &tr);
    let (pill_label, pill_class) = pill_state(round_status, participant.forfeited, &tr);

    rsx! {
        div { class: "{row_class}",
            div { class: "{avatar_class}", "{initials}" }
            div { class: "p-info",
                div { class: "p-name",
                    "{display}"
                    if is_me {
                        span { class: "lb-you-badge", "{tr.players_you_badge}" }
                    }
                }
                div { class: "p-status", "{status_label}" }
            }
            div { class: "{pill_class}", "{pill_label}" }
        }
    }
}

// ── Helpers ─────────────────────────────────────────────────────────

/// `★★★☆☆` etc — 1..=5 stars rendering. Out-of-range falls back to
/// `?` so the row stays useful even if the operator sends bad data.
fn render_difficulty(d: i32) -> String {
    if !(1..=5).contains(&d) {
        return "?".to_string();
    }
    let filled = "★".repeat(d as usize);
    let empty = "☆".repeat((5 - d) as usize);
    format!("{filled}{empty}")
}

/// 4-color avatar palette aligned with the mockup's `.p-avatar.a{2..4}`
/// classes — index 0 is the unstyled `p-avatar`, then a2/a3/a4 cycle.
fn avatar_variant(idx: usize) -> &'static str {
    match idx % 4 {
        0 => "",
        1 => "a2",
        2 => "a3",
        _ => "a4",
    }
}

/// 2-char monogram from the participant's display name (falling back
/// to username, then to a `?` marker so we never render an empty
/// avatar).
fn initials(display: &str, username: &str) -> String {
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

fn status_text(s: RoundStatus, tr: &FactFoldRoomTranslate) -> &'static str {
    match s {
        RoundStatus::Waiting | RoundStatus::NewsReveal => tr.players_status_reading,
        RoundStatus::Bet => tr.players_status_bet_pending,
        RoundStatus::Rationale => tr.players_status_writing,
        RoundStatus::Reveal => tr.players_status_revealed,
        RoundStatus::Debate => tr.players_status_debating,
        RoundStatus::Settlement | RoundStatus::Settled => tr.players_status_done,
    }
}

fn pill_state(
    s: RoundStatus,
    forfeited: bool,
    tr: &FactFoldRoomTranslate,
) -> (&'static str, &'static str) {
    if forfeited {
        return (tr.players_pill_forfeited, "p-state-pill");
    }
    match s {
        RoundStatus::Settlement | RoundStatus::Settled => {
            (tr.players_pill_done, "p-state-pill done")
        }
        _ => (tr.players_pill_waiting, "p-state-pill waiting"),
    }
}
