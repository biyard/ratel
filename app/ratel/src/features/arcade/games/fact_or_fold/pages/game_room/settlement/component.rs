use crate::features::arcade::games::fact_or_fold::controllers::settlement::{
    SettleRoundResponse, SettlementBreakdown,
};
use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::features::arcade::games::fact_or_fold::{
    use_fact_fold_round, BetResponse, BetSide, RationaleResponse, RoundHeadlineResponse,
    RoundParticipantSummary, Verdict,
};
use crate::features::auth::hooks::use_user_context;
use crate::FactFoldRoundEntityType;
use crate::*;

/// `SettlementView` — Stage 6 (Settled). Surfaces the verdict +
/// per-player breakdown + the caller's own formula + Essence opt-in.
#[component]
pub fn SettlementView() -> Element {
    let ctx = use_fact_fold_round();
    let tr: FactFoldRoomTranslate = use_translate();
    let headline = (ctx.headline)();
    let settlement_opt = (ctx.settlement)();
    let participants = (ctx.participants)();
    let bets = (ctx.bets)();
    let rationales = (ctx.rationales)();
    let round = (ctx.round)();

    let user_ctx = use_user_context();
    let my_pk = user_ctx().user_pk().unwrap_or_default();

    let my_rationale = rationales
        .items
        .iter()
        .find(|r| r.user_pk == my_pk)
        .cloned();

    rsx! {
        section { class: "view", "data-active": true, "data-view": "result",
            RevealBanner { headline: headline.clone() }

            if let Some(settlement) = settlement_opt {
                div { class: "result-grid",
                    ResultTable {
                        settlement: settlement.clone(),
                        bets: bets.items.clone(),
                        participants: participants.items.clone(),
                        my_pk: my_pk.clone(),
                    }
                    div {
                        MySettlementCard {
                            settlement: settlement.clone(),
                            my_pk: my_pk.clone(),
                        }
                        EssenceCard {
                            round_id: round.id.clone(),
                            my_rationale: my_rationale.clone(),
                        }
                    }
                }
                SettlementExitRow { round_id: round.id.clone() }
            } else {
                div { class: "ff-room__placeholder", "{tr.reveal_pending}" }
            }
        }
    }
}

#[component]
fn SettlementExitRow(round_id: FactFoldRoundEntityType) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let mut ctx = use_fact_fold_round();
    let nav = use_navigator();
    let mut submitting = use_signal(|| false);

    let on_exit = move |_| {
        let rid = round_id.clone();
        async move {
            if submitting() {
                return;
            }
            submitting.set(true);
            // Best-effort cleanup: even if the DELETE fails (e.g. another
            // player already wiped the transcript) we still send the user
            // home so they don't get stuck on the settlement screen.
            let _ = ctx.exit_round(rid).await;
            nav.push(Route::ArcadeHomePage {});
        }
    };

    let label = if submitting() {
        tr.exit_to_home_busy
    } else {
        tr.exit_to_home_label
    };

    rsx! {
        div { class: "settlement-exit-row",
            button {
                class: "btn btn-primary",
                disabled: submitting(),
                onclick: on_exit,
                "{label}"
            }
        }
    }
}

#[component]
fn RevealBanner(headline: RoundHeadlineResponse) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let (verdict_label, verdict_class) = match headline.verdict {
        Some(Verdict::Real) => (tr.reveal_verdict_real, "reveal-verdict real"),
        Some(Verdict::Fake) => (tr.reveal_verdict_fake, "reveal-verdict fake"),
        None => ("—", "reveal-verdict"),
    };

    rsx! {
        div { class: "reveal-banner",
            div { class: "reveal-label", "{tr.reveal_banner_label}" }
            div { class: "{verdict_class}", "{verdict_label}" }
            p { class: "reveal-headline", "{headline.reveal_summary}" }
            if !headline.reveal_sources.is_empty() {
                div { class: "reveal-source",
                    "{tr.reveal_source_label} "
                    for (idx , src) in headline.reveal_sources.iter().enumerate() {
                        if idx > 0 {
                            " · "
                        }
                        a { href: "{src.url}", "{src.label}" }
                    }
                }
            }
        }
    }
}

#[component]
fn ResultTable(
    settlement: SettleRoundResponse,
    bets: Vec<BetResponse>,
    participants: Vec<RoundParticipantSummary>,
    my_pk: String,
) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let round_buy_in = 0; // chips_out delta vs buy_in is shown in the
                          // detail card; here we just show chips_out
                          // signed by win/lose for visual cue.

    rsx! {
        div { class: "result-table",
            div { class: "section-head",
                h2 { "{tr.result_table_title}" }
                span { class: "sub", "{tr.result_table_sub}" }
            }

            for (idx , row) in settlement.outcomes.iter().enumerate() {
                {
                    let participant = participants.iter().find(|p| p.user_pk == row.user_pk).cloned();
                    let bet = bets.iter().find(|b| b.user_pk == row.user_pk).cloned();
                    rsx! {
                        ResultRow {
                            key: "{row.user_pk}",
                            idx,
                            row: row.clone(),
                            participant,
                            bet,
                            is_me: row.user_pk == my_pk,
                            round_buy_in,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ResultRow(
    idx: usize,
    row: SettlementBreakdown,
    participant: Option<RoundParticipantSummary>,
    bet: Option<BetResponse>,
    is_me: bool,
    round_buy_in: i64,
) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let display = participant
        .as_ref()
        .map(|p| {
            if p.display_name.is_empty() {
                p.username.clone()
            } else {
                p.display_name.clone()
            }
        })
        .unwrap_or_else(|| row.user_pk.clone());
    let initials = participant
        .as_ref()
        .map(|p| {
            let src = if !p.display_name.is_empty() {
                &p.display_name
            } else if !p.username.is_empty() {
                &p.username
            } else {
                "?"
            };
            src.chars()
                .filter(|c| c.is_alphanumeric())
                .take(2)
                .collect::<String>()
                .to_uppercase()
        })
        .unwrap_or_default();
    let avatar_variant = avatar_variant(idx);
    let avatar_class = if avatar_variant.is_empty() {
        "p-avatar".to_string()
    } else {
        format!("p-avatar {avatar_variant}")
    };
    let judgement = if row.won {
        if row.insider_bonus > 0 {
            tr.result_judgement_insider_won
        } else {
            tr.result_judgement_won
        }
    } else {
        tr.result_judgement_lost
    };
    let (side_label, side_class, side_pill_class) = bet
        .as_ref()
        .map(|b| {
            let s = b.flipped_to.unwrap_or(b.side);
            let label = match s {
                BetSide::Real => "REAL",
                BetSide::Fake => "FAKE",
            };
            let cls = match s {
                BetSide::Real => "result-bet-side real",
                BetSide::Fake => "result-bet-side fake",
            };
            (
                format!("{label} {}", b.amount_rp),
                cls,
                "result-bet-side",
            )
        })
        .unwrap_or((
            "—".to_string(),
            "result-bet-side",
            "result-bet-side",
        ));
    let _ = side_pill_class;

    let delta = row.chips_out - round_buy_in;
    let (delta_label, delta_class) = if delta > 0 {
        (format!("+{delta}"), "result-delta up")
    } else if delta < 0 {
        (format!("{delta}"), "result-delta down")
    } else {
        ("±0".to_string(), "result-delta")
    };

    rsx! {
        div { class: "result-row",
            div { class: "{avatar_class}", "{initials}" }
            div { class: "result-meta",
                div { class: "result-name",
                    "{display}"
                    if is_me {
                        span { class: "lb-you-badge", style: "margin-left: 6px", "YOU" }
                    }
                    if row.insider_bonus > 0 {
                        span { class: "insider-truth-badge", style: "margin-left: 6px", "⚐ INSIDER" }
                    }
                }
                div { class: "result-judgement", "{judgement}" }
            }
            div { class: "{side_class}", "{side_label}" }
            div { class: "{delta_class}", "{delta_label}" }
        }
    }
}

#[component]
fn MySettlementCard(settlement: SettleRoundResponse, my_pk: String) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let mine = settlement
        .outcomes
        .iter()
        .find(|o| o.user_pk == my_pk)
        .cloned();
    let Some(o) = mine else {
        return rsx! {};
    };
    let total = o.chips_out;

    rsx! {
        div { class: "essence-card",
            h3 { class: "card-title", "{tr.my_settlement_title}" }
            FormulaRow { label: tr.my_settle_base.to_string(), value: o.base_refund }
            FormulaRow { label: tr.my_settle_correct.to_string(), value: o.correct_bonus }
            FormulaRow { label: tr.my_settle_pool.to_string(), value: o.pool_share }
            FormulaRow { label: tr.my_settle_influence.to_string(), value: o.influence_bonus }
            FormulaRow { label: tr.my_settle_insider.to_string(), value: o.insider_bonus }
            div { class: "formula-row total",
                span { class: "lbl", "{tr.my_settle_total}" }
                span { class: "val", "+{total} RP" }
            }
        }
    }
}

#[component]
fn FormulaRow(label: String, value: i64) -> Element {
    rsx! {
        div { class: "formula-row",
            span { class: "lbl", "{label}" }
            span { class: "val",
                if value >= 0 {
                    "+{value} RP"
                } else {
                    "{value} RP"
                }
            }
        }
    }
}

#[component]
fn EssenceCard(
    round_id: FactFoldRoundEntityType,
    my_rationale: Option<RationaleResponse>,
) -> Element {
    let mut ctx = use_fact_fold_round();
    let tr: FactFoldRoomTranslate = use_translate();
    let mut submitting = use_signal(|| false);

    let Some(rationale) = my_rationale.clone() else {
        return rsx! {};
    };
    let already_registered = rationale.essence_registered;
    let eligible = rationale.essence_eligible;

    let on_register = move |_| {
        let rid = round_id.clone();
        async move {
            if submitting() || already_registered || !eligible {
                return;
            }
            submitting.set(true);
            let _ = ctx.register_essence(rid).await;
            submitting.set(false);
        }
    };

    rsx! {
        div { class: "essence-card", style: "margin-top: 18px",
            div { class: "section-head",
                h2 { "{tr.essence_title}" }
                span { class: "sub", "{tr.essence_sub}" }
            }

            div { class: "essence-list",
                div { class: "essence-item",
                    div {
                        class: "essence-checkbox",
                        "data-checked": "{already_registered || eligible}",
                        if already_registered { "✓" } else { "" }
                    }
                    div {
                        div { class: "essence-meta",
                            span { class: "pill", "{tr.essence_my_rationale_label}" }
                            if !eligible {
                                span { class: "pill pink", "{tr.essence_ineligible}" }
                            }
                        }
                        div { class: "essence-text", "{rationale.text}" }
                    }
                }
            }

            div { class: "essence-foot",
                span { class: "essence-foot-label" }
                button {
                    class: "btn btn-primary",
                    disabled: submitting() || already_registered || !eligible,
                    onclick: on_register,
                    if already_registered {
                        "{tr.essence_registered}"
                    } else {
                        "{tr.essence_register}"
                    }
                }
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
