use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::features::arcade::games::fact_or_fold::{
    use_fact_fold_round, BetResponse, BetSide, RationaleResponse, RoundParticipantSummary,
};
use crate::features::auth::hooks::use_user_context;
use crate::*;

/// `ReasoningRevealView` — Stage 4. Camp distribution bar on top, 4
/// reveal cards (every player's rationale + their bet side), and a
/// quote button per card that records the player's "decisive" pick
/// in `ctx.cited_user_pk` so the live-debate flip auto-populates the
/// citation.
#[component]
pub fn ReasoningRevealView() -> Element {
    let ctx = use_fact_fold_round();
    let tr: FactFoldRoomTranslate = use_translate();
    let bets = ctx.bets()?();
    let rationales = ctx.rationales()?();
    let participants = ctx.participants()?();

    let user_ctx = use_user_context();
    let my_pk: UserPartition = UserPartition(user_ctx().user_id().unwrap_or_default());

    let cited_user_pk = ctx.cited_user_pk;

    let (real_bets, fake_bets): (Vec<&BetResponse>, Vec<&BetResponse>) = bets
        .items
        .iter()
        .partition(|b| matches!(active_side(b), BetSide::Real));
    let real_total: i64 = real_bets.iter().map(|b| b.amount_rp).sum();
    let fake_total: i64 = fake_bets.iter().map(|b| b.amount_rp).sum();

    rsx! {
        section { class: "view", "data-active": true, "data-view": "reveal",
            CampBar {
                real_count: real_bets.len(),
                real_total,
                fake_count: fake_bets.len(),
                fake_total,
                real_owners: real_bets.iter().map(|b| b.user_pk.clone()).collect(),
                fake_owners: fake_bets.iter().map(|b| b.user_pk.clone()).collect(),
                participants: participants.items.clone(),
            }

            div { class: "reveal-hint",
                strong { "{tr.reveal_hint_prefix}" }
                " {tr.reveal_hint_body}"
            }

            div { class: "reveal-grid",
                for (idx, p) in participants.items.iter().enumerate() {
                    {
                        let participant = p.clone();
                        let bet = bets.items.iter().find(|b| b.user_pk == participant.user_pk).cloned();
                        let rationale = rationales
                            .items
                            .iter()
                            .find(|r| r.user_pk == participant.user_pk)
                            .cloned();
                        rsx! {
                            RevealCard {
                                key: "{participant.user_pk}",
                                idx,
                                participant,
                                bet,
                                rationale,
                                my_pk: my_pk.clone(),
                                cited_user_pk,
                            }
                        }
                    }
                }
            }

            RevealCta { cited_user_pk, participants: participants.items.clone() }
        }
    }
}

#[component]
fn CampBar(
    real_count: usize,
    real_total: i64,
    fake_count: usize,
    fake_total: i64,
    real_owners: Vec<UserPartition>,
    fake_owners: Vec<UserPartition>,
    participants: Vec<RoundParticipantSummary>,
) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let real_label = tr
        .reveal_camp_count
        .replace("{$count}", &real_count.to_string())
        .replace("{$rp}", &real_total.to_string());
    let fake_label = tr
        .reveal_camp_count
        .replace("{$count}", &fake_count.to_string())
        .replace("{$rp}", &fake_total.to_string());
    let real_flex = real_total.max(1);
    let fake_flex = fake_total.max(1);

    rsx! {
        div { class: "camp-bar",
            div { class: "camp-side real", style: "flex: {real_flex}",
                div { class: "camp-side-head",
                    span { class: "camp-label", "{tr.reveal_camp_real_label}" }
                    span { class: "camp-count", "{real_label}" }
                }
                div { class: "camp-avatars", {camp_avatars(&real_owners, &participants)} }
            }
            div { class: "camp-vs", "{tr.reveal_camp_vs}" }
            div { class: "camp-side fake", style: "flex: {fake_flex}",
                div { class: "camp-side-head",
                    span { class: "camp-label", "{tr.reveal_camp_fake_label}" }
                    span { class: "camp-count", "{fake_label}" }
                }
                div { class: "camp-avatars", {camp_avatars(&fake_owners, &participants)} }
            }
        }
    }
}

#[component]
fn RevealCard(
    idx: usize,
    participant: RoundParticipantSummary,
    bet: Option<BetResponse>,
    rationale: Option<RationaleResponse>,
    my_pk: UserPartition,
    cited_user_pk: Signal<Option<UserPartition>>,
) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let is_me = participant.user_pk == my_pk;
    let side = bet.as_ref().map(active_side).unwrap_or(BetSide::Real);
    let card_class = if is_me { "reveal-card me" } else { "reveal-card" };
    let side_attr = match side {
        BetSide::Real => "real",
        BetSide::Fake => "fake",
    };
    let avatar_variant = avatar_variant(idx);
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
    let initials = card_initials(&display, &participant.username);

    let bet_pill_label = bet
        .as_ref()
        .map(|b| {
            let side_label = match active_side(b) {
                BetSide::Real => "REAL",
                BetSide::Fake => "FAKE",
            };
            tr.reveal_card_bet_pill
                .replace("{$side}", side_label)
                .replace("{$rp}", &b.amount_rp.to_string())
        })
        .unwrap_or_default();
    let bet_pill_class = format!("reveal-card-bet {side_attr}");

    let rationale_text = rationale
        .as_ref()
        .map(|r| r.text.clone())
        .unwrap_or_default();

    let user_pk_for_cite = participant.user_pk.clone();
    let mut cited_signal = cited_user_pk;
    let marked = (cited_signal)().as_ref() == Some(&user_pk_for_cite);
    let on_quote = move |_| {
        let cur = (cited_signal)();
        if cur.as_ref() == Some(&user_pk_for_cite) {
            cited_signal.set(None);
        } else {
            cited_signal.set(Some(user_pk_for_cite.clone()));
        }
    };

    rsx! {
        div { class: "{card_class}", "data-side": side_attr,
            div { class: "reveal-card-head",
                div { class: "debate-card-author",
                    div { class: "{avatar_class}", "{initials}" }
                    div {
                        div { class: "p-name",
                            "{display}"
                            if is_me {
                                span { class: "lb-you-badge", "YOU" }
                            }
                        }
                    }
                }
                div { class: "{bet_pill_class}", "{bet_pill_label}" }
            }
            div { class: "reveal-card-text", "{rationale_text}" }
            if !is_me && bet.is_some() {
                div { class: "reveal-card-actions",
                    button {
                        class: "quote-btn",
                        "data-marked": "{marked}",
                        onclick: on_quote,
                        "{tr.reveal_quote_btn}"
                    }
                }
            }
        }
    }
}

#[component]
fn RevealCta(
    cited_user_pk: Signal<Option<UserPartition>>,
    participants: Vec<RoundParticipantSummary>,
) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let hint = match (cited_user_pk)() {
        Some(pk) => {
            let name = participants
                .iter()
                .find(|p| p.user_pk == pk)
                .map(|p| {
                    if p.display_name.is_empty() {
                        p.username.clone()
                    } else {
                        p.display_name.clone()
                    }
                })
                .unwrap_or_else(|| pk.0.clone());
            tr.reveal_cta_one_quote.replace("{$name}", &name)
        }
        None => tr.reveal_cta_no_quote.to_string(),
    };

    rsx! {
        div { class: "reveal-cta",
            span { class: "reveal-cta-hint", "{hint}" }
            span { class: "reveal-cta-hint", "{tr.reveal_cta_hint}" }
        }
    }
}

// ── Helpers ─────────────────────────────────────────────────────────

/// A bet that already flipped uses the post-flip side; otherwise the
/// original side. Reveal stage usually shows original side (flip slot
/// is in live debate), but rendering through this helper keeps the
/// view consistent if the player flips later.
fn active_side(bet: &BetResponse) -> BetSide {
    bet.flipped_to.unwrap_or(bet.side)
}

fn camp_avatars(
    owners: &[UserPartition],
    participants: &[RoundParticipantSummary],
) -> Element {
    rsx! {
        for pk in owners.iter() {
            {
                let owner = participants.iter().find(|p| &p.user_pk == pk);
                let display = owner
                    .map(|p| {
                        if p.display_name.is_empty() {
                            p.username.clone()
                        } else {
                            p.display_name.clone()
                        }
                    })
                    .unwrap_or_default();
                let initials = card_initials(&display, &display);
                let idx = participants.iter().position(|p| &p.user_pk == pk).unwrap_or(0);
                let variant = avatar_variant(idx);
                let class = if variant.is_empty() {
                    "p-avatar".to_string()
                } else {
                    format!("p-avatar {variant}")
                };
                rsx! {
                    div {
                        key: "{pk}",
                        class: "{class}",
                        style: "width: 28px; height: 28px; font-size: 11px",
                        "{initials}"
                    }
                }
            }
        }
    }
}

fn avatar_variant(idx: usize) -> &'static str {
    match idx % 4 {
        0 => "",
        1 => "a2",
        2 => "a3",
        _ => "a4",
    }
}

fn card_initials(display: &str, username: &str) -> String {
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
