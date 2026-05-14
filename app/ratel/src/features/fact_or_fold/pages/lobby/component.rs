use crate::features::fact_or_fold::hooks::{UseFactFoldLobby, use_fact_fold_lobby_provider};
use crate::features::fact_or_fold::types::{LobbyResponse, RoundStatus};
use crate::*;

use super::i18n::FactFoldLobbyTranslate;

/// `/fact-or-fold` — player entry point. Shows the featured round
/// + a join/leave CTA driven by `LobbyResponse.can_join` and
/// `already_joined`. Round play (stage 2+) lands in PR4.
#[component]
pub fn FactFoldLobbyPage() -> Element {
    let UseFactFoldLobby { state } = use_fact_fold_lobby_provider()?;
    let snapshot = state();
    let tr: FactFoldLobbyTranslate = use_translate();

    rsx! {
        SeoMeta { title: "Fact or Fold · Ratel Arcade" }
        section { class: "ff-lobby",
            div { class: "ff-lobby__brand",
                div { class: "ff-lobby__brand-logo", "R" }
                div { class: "ff-lobby__brand-text",
                    div { class: "ff-lobby__brand-name", "{tr.brand}" }
                    div { class: "ff-lobby__brand-sub", "{tr.brand_sub}" }
                }
            }

            // Featured card — title + tagline + meta strip + CTA
            div { class: "ff-lobby__featured",
                LobbyTag { snapshot: snapshot.clone() }
                h1 { class: "ff-lobby__title", "Fact or Fold" }
                p { class: "ff-lobby__tagline", "{tr.tagline}" }

                LobbyMeta { snapshot: snapshot.clone() }

                LobbyCta { snapshot: snapshot.clone() }
            }

            // Help / spec callouts
            div { class: "ff-lobby__rules",
                strong { "{tr.rules_title}" }
                ul {
                    li { "{tr.rule_capacity}" }
                    li { "{tr.rule_insider}" }
                    li { "{tr.rule_stake}" }
                    li { "{tr.rule_settle}" }
                }
            }
        }
    }
}

#[component]
fn LobbyTag(snapshot: LobbyResponse) -> Element {
    let tr: FactFoldLobbyTranslate = use_translate();
    let (label, variant) = if let Some(r) = snapshot.current_round.as_ref() {
        match r.status {
            RoundStatus::Waiting => (tr.tag_waiting, "waiting"),
            RoundStatus::Settled => (tr.tag_settled, "settled"),
            _ => (tr.tag_in_progress, "live"),
        }
    } else if snapshot.headline_available {
        (tr.tag_open, "open")
    } else {
        (tr.tag_closed, "closed")
    };
    rsx! {
        div { class: "ff-lobby__tag", "data-variant": variant,
            span { class: "ff-lobby__tag-dot" }
            span { "{label}" }
        }
    }
}

#[component]
fn LobbyMeta(snapshot: LobbyResponse) -> Element {
    let tr: FactFoldLobbyTranslate = use_translate();
    let waiting_count = snapshot
        .current_round
        .as_ref()
        .map(|r| r.participant_pks.len())
        .unwrap_or(0);
    rsx! {
        div { class: "ff-lobby__meta",
            MetaCell {
                label: "{tr.meta_waiting}",
                value: "{waiting_count} / {snapshot.round_capacity}",
            }
            MetaCell {
                label: "{tr.meta_min_bet}",
                value: format!("{} RP+", snapshot.min_bet_rp),
            }
            MetaCell {
                label: "{tr.meta_round_time}",
                value: "{tr.meta_round_time_value}",
            }
            MetaCell { label: "{tr.meta_cycle}", value: "{tr.meta_cycle_value}" }
        }
    }
}

#[component]
fn MetaCell(label: String, value: String) -> Element {
    rsx! {
        div { class: "ff-lobby__meta-cell",
            div { class: "ff-lobby__meta-label", "{label}" }
            div { class: "ff-lobby__meta-value", "{value}" }
        }
    }
}

#[component]
fn LobbyCta(snapshot: LobbyResponse) -> Element {
    let tr: FactFoldLobbyTranslate = use_translate();
    let mut ctx = use_fact_fold_lobby_provider()?;
    let mut submitting = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    let on_join = move |_| async move {
        submitting.set(true);
        error_msg.set(None);
        if let Err(e) = ctx.join().await {
            error_msg.set(Some(format!("{e}")));
        }
        submitting.set(false);
    };

    let on_leave = move |_| async move {
        submitting.set(true);
        error_msg.set(None);
        if let Err(e) = ctx.leave().await {
            error_msg.set(Some(format!("{e}")));
        }
        submitting.set(false);
    };

    rsx! {
        div { class: "ff-lobby__cta",
            if let Some(err) = error_msg() {
                div { class: "ff-lobby__error", "{err}" }
            }

            if snapshot.already_joined {
                Button {
                    style: ButtonStyle::Outline,
                    size: ButtonSize::Medium,
                    disabled: submitting(),
                    onclick: on_leave,
                    "{tr.cta_leave}"
                }
                span { class: "ff-lobby__status-line", "{tr.status_already_joined}" }
            } else if snapshot.can_join {
                Button {
                    style: ButtonStyle::Primary,
                    size: ButtonSize::Medium,
                    disabled: submitting(),
                    onclick: on_join,
                    "{tr.cta_join}"
                }
                span { class: "ff-lobby__status-line",
                    if let Some(r) = snapshot.current_round.as_ref() {
                        "{tr.status_can_join_existing} ({r.participant_pks.len()} / {snapshot.round_capacity})"
                    } else {
                        "{tr.status_can_join_new}"
                    }
                }
            } else if !snapshot.headline_available && snapshot.current_round.is_none() {
                Button {
                    style: ButtonStyle::Outline,
                    size: ButtonSize::Medium,
                    disabled: true,
                    "{tr.cta_disabled}"
                }
                span { class: "ff-lobby__status-line ff-lobby__status-line--muted",
                    "{tr.status_no_headline}"
                }
            } else {
                Button {
                    style: ButtonStyle::Outline,
                    size: ButtonSize::Medium,
                    disabled: true,
                    "{tr.cta_in_progress}"
                }
                span { class: "ff-lobby__status-line ff-lobby__status-line--muted",
                    "{tr.status_round_in_progress}"
                }
            }
        }
    }
}
