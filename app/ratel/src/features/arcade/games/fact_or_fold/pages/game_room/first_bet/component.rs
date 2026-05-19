use crate::features::arcade::games::fact_or_fold::pages::game_room::FactFoldRoomTranslate;
use crate::features::arcade::games::fact_or_fold::{
    use_fact_fold_round, BetResponse, BetSide,
};
use crate::features::auth::hooks::use_user_context;
use crate::*;

const DEFAULT_BET_RP: i64 = 300;
const MIN_BET_RP: i64 = 100;
const MAX_BET_RP: i64 = 1_000;
const BET_STEP_RP: i64 = 50;

/// `FirstBetView` — Stage 2. Shows the bet form (REAL/FAKE + RP
/// slider) plus the insider statement card when the caller is the
/// insider. Once the caller submits, the form flips to a "locked"
/// confirmation panel.
#[component]
pub fn FirstBetView() -> Element {
    let mut ctx = use_fact_fold_round();
    let round = ctx.round()?();
    let bets = ctx.bets()?();
    let insider = ctx.insider()?();

    let user_ctx = use_user_context();
    let my_pk: UserPartition = UserPartition(user_ctx().user_id().unwrap_or_default());

    // Caller's existing bet — server returns only the caller's row
    // during the Bet stage, but we still match on user_pk to be safe.
    let my_bet: Option<BetResponse> = bets
        .items
        .iter()
        .find(|b| b.user_pk == my_pk)
        .cloned();

    let round_id_val = round.id.clone();

    rsx! {
        section { class: "view", "data-active": true, "data-view": "bet",
            div { class: "live-bet-grid",
                div {
                    if let Some(statement) = insider.statement.as_ref() {
                        InsiderCard { statement: statement.clone() }
                    }
                    if let Some(b) = my_bet.as_ref() {
                        BetLockedCard { bet: b.clone() }
                    } else {
                        BetForm {
                            on_submit: move |(side, amount)| {
                                let rid = round_id_val.clone();
                                async move {
                                    let _ = ctx.place_bet(rid, side, amount).await;
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn InsiderCard(statement: String) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    rsx! {
        div { class: "insider-card", style: "margin-bottom: 18px",
            div { class: "insider-head",
                div { class: "insider-icon", "⚐" }
                div { class: "insider-title", "{tr.insider_title}" }
            }
            div { class: "insider-body", "{statement}" }
            div { class: "insider-foot", "{tr.insider_tip}" }
        }
    }
}

#[component]
fn BetForm(on_submit: EventHandler<(BetSide, i64)>) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let mut side = use_signal(|| BetSide::Real);
    let mut amount = use_signal(|| DEFAULT_BET_RP);
    let mut submitting = use_signal(|| false);

    let on_slider = move |e: FormEvent| {
        if let Ok(v) = e.value().parse::<i64>() {
            amount.set(v.clamp(MIN_BET_RP, MAX_BET_RP));
        }
    };

    let on_real = move |_| side.set(BetSide::Real);
    let on_fake = move |_| side.set(BetSide::Fake);

    let on_confirm = move |_| async move {
        if submitting() {
            return;
        }
        submitting.set(true);
        on_submit.call((side(), amount()));
        submitting.set(false);
    };

    let real_selected = matches!(side(), BetSide::Real);
    let fake_selected = matches!(side(), BetSide::Fake);

    rsx! {
        div { class: "bet-card",
            h3 { class: "card-title", "{tr.bet_card_title}" }
            p { class: "card-sub", "{tr.bet_card_sub}" }

            div { class: "bet-toggle",
                div {
                    class: "bet-option",
                    "data-side": "real",
                    "data-selected": "{real_selected}",
                    onclick: on_real,
                    div { class: "bet-option-icon", "◎" }
                    div { class: "bet-option-label", "{tr.bet_option_real_label}" }
                    div { class: "bet-option-sub", "{tr.bet_option_real_sub}" }
                }
                div {
                    class: "bet-option",
                    "data-side": "fake",
                    "data-selected": "{fake_selected}",
                    onclick: on_fake,
                    div { class: "bet-option-icon", "⊘" }
                    div { class: "bet-option-label", "{tr.bet_option_fake_label}" }
                    div { class: "bet-option-sub", "{tr.bet_option_fake_sub}" }
                }
            }

            div { class: "rp-slider-wrap",
                div { class: "rp-slider-head",
                    span { class: "rp-slider-label", "{tr.bet_slider_label}" }
                    span { class: "rp-slider-value", "{amount()} RP" }
                }
                input {
                    r#type: "range",
                    class: "rp-slider",
                    min: "{MIN_BET_RP}",
                    max: "{MAX_BET_RP}",
                    step: "{BET_STEP_RP}",
                    value: "{amount()}",
                    oninput: on_slider,
                }
                div { class: "rp-slider-ticks",
                    span { "100" }
                    span { "300" }
                    span { "500" }
                    span { "700" }
                    span { "1000" }
                }
            }

            div { class: "submit-row",
                button {
                    class: "btn btn-primary",
                    disabled: submitting(),
                    onclick: on_confirm,
                    "{tr.bet_submit}"
                }
                span { style: "font-size: 12px; color: var(--text-muted)", "{tr.bet_submit_hint}" }
            }
        }
    }
}

#[component]
fn BetLockedCard(bet: BetResponse) -> Element {
    let tr: FactFoldRoomTranslate = use_translate();
    let side_label = match bet.side {
        BetSide::Real => "REAL",
        BetSide::Fake => "FAKE",
    };
    let side_class = match bet.side {
        BetSide::Real => "pill gold",
        BetSide::Fake => "pill pink",
    };

    rsx! {
        div { class: "bet-card",
            h3 { class: "card-title", "{tr.bet_already_placed_title}" }
            p { class: "card-sub", "{tr.bet_already_placed_body}" }
            div { class: "submit-row",
                span { class: "{side_class}", "{side_label}" }
                span { class: "rp-slider-value", "{bet.amount_rp} RP" }
            }
        }
    }
}
