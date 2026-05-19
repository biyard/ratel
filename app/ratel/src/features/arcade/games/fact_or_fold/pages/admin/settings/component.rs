use crate::features::arcade::games::fact_or_fold::hooks::{
    UseFactFoldAdminSettings, use_fact_fold_admin_settings_provider,
};
use crate::features::arcade::games::fact_or_fold::types::{
    FactOrFoldSettingsResponse, UpdateFactOrFoldSettingsRequest,
};
use crate::*;

use super::i18n::FactFoldAdminSettingsTranslate;

/// `/admin/fact-or-fold/settings` — read + edit the admin-tunable
/// game parameters singleton. Backed by the PR1 settings endpoints.
///
/// Sections mirror the design mockup but cover only the fields the
/// PR1 backend currently models. Mockup-only knobs (auto-publish KST
/// time, auto-backfill, Essence policy, danger-zone actions) are
/// noted inline as deferred so reviewers can map mockup → spec.
#[component]
pub fn FactFoldAdminSettingsPage() -> Element {
    let ctx = use_fact_fold_admin_settings_provider()?;
    let settings = ctx.settings()?;
    let initial = settings();

    rsx! {
        SeoMeta { title: "Settings · Fact or Fold" }
        SettingsForm { initial }
    }
}

/// Inner form that owns one signal per editable field. Splitting it
/// out from the page keeps the loader render cycle separate from the
/// per-field reactivity.
#[component]
fn SettingsForm(initial: FactOrFoldSettingsResponse) -> Element {
    let tr: FactFoldAdminSettingsTranslate = use_translate();
    let mut ctx = use_fact_fold_admin_settings_provider()?;

    let round_capacity = use_signal(|| initial.round_capacity);
    let stage_news_reveal_sec = use_signal(|| initial.stage_news_reveal_sec);
    let stage_bet_sec = use_signal(|| initial.stage_bet_sec);
    let stage_rationale_sec = use_signal(|| initial.stage_rationale_sec);
    let stage_reveal_sec = use_signal(|| initial.stage_reveal_sec);
    let stage_debate_sec = use_signal(|| initial.stage_debate_sec);
    let min_bet_rp = use_signal(|| initial.min_bet_rp);
    let max_bet_rp = use_signal(|| initial.max_bet_rp);
    let correct_side_multiplier_bps = use_signal(|| initial.correct_side_multiplier_bps);
    let insider_correct_bonus_bps = use_signal(|| initial.insider_correct_bonus_bps);
    let influence_bonus_bps = use_signal(|| initial.influence_bonus_bps);
    let new_user_signup_rp = use_signal(|| initial.new_user_signup_rp);
    let reconnect_grace_sec = use_signal(|| initial.reconnect_grace_sec);
    let queue_low_alert_days = use_signal(|| initial.queue_low_alert_days);

    let mut submitting = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut saved_at = use_signal(|| Option::<i64>::None);

    let on_save = move |_| async move {
        submitting.set(true);
        error_msg.set(None);
        let patch = UpdateFactOrFoldSettingsRequest {
            round_capacity: Some(round_capacity()),
            stage_news_reveal_sec: Some(stage_news_reveal_sec()),
            stage_bet_sec: Some(stage_bet_sec()),
            stage_rationale_sec: Some(stage_rationale_sec()),
            stage_reveal_sec: Some(stage_reveal_sec()),
            stage_debate_sec: Some(stage_debate_sec()),
            min_bet_rp: Some(min_bet_rp()),
            max_bet_rp: Some(max_bet_rp()),
            correct_side_multiplier_bps: Some(correct_side_multiplier_bps()),
            insider_correct_bonus_bps: Some(insider_correct_bonus_bps()),
            influence_bonus_bps: Some(influence_bonus_bps()),
            new_user_signup_rp: Some(new_user_signup_rp()),
            reconnect_grace_sec: Some(reconnect_grace_sec()),
            queue_low_alert_days: Some(queue_low_alert_days()),
        };
        match ctx.save(patch).await {
            Ok(_) => {
                saved_at.set(Some(crate::common::utils::time::get_now_timestamp_millis()));
            }
            Err(e) => error_msg.set(Some(format!("{e}"))),
        }
        submitting.set(false);
    };

    rsx! {
        form {
            class: "ff-settings",
            onsubmit: move |e| {
                e.prevent_default();
            },
            // Section 01 — round shape + stage timing
            SettingsSection {
                title: "{tr.section_round_title}",
                sub: "{tr.section_round_sub}",
                IntRow {
                    label: "{tr.round_capacity}",
                    desc: "{tr.round_capacity_desc}",
                    suffix: "{tr.unit_people}",
                    value: round_capacity,
                }
                IntRow {
                    label: "{tr.stage_news_reveal_sec}",
                    desc: "{tr.stage_sec_desc}",
                    suffix: "{tr.unit_sec}",
                    value: stage_news_reveal_sec,
                }
                IntRow {
                    label: "{tr.stage_bet_sec}",
                    desc: "{tr.stage_sec_desc}",
                    suffix: "{tr.unit_sec}",
                    value: stage_bet_sec,
                }
                IntRow {
                    label: "{tr.stage_rationale_sec}",
                    desc: "{tr.stage_sec_desc}",
                    suffix: "{tr.unit_sec}",
                    value: stage_rationale_sec,
                }
                IntRow {
                    label: "{tr.stage_reveal_sec}",
                    desc: "{tr.stage_sec_desc}",
                    suffix: "{tr.unit_sec}",
                    value: stage_reveal_sec,
                }
                IntRow {
                    label: "{tr.stage_debate_sec}",
                    desc: "{tr.stage_sec_desc}",
                    suffix: "{tr.unit_sec}",
                    value: stage_debate_sec,
                }
            }

            // Section 02 — RP economy
            SettingsSection {
                title: "{tr.section_economy_title}",
                sub: "{tr.section_economy_sub}",
                BigIntRow {
                    label: "{tr.min_bet_rp}",
                    desc: "{tr.min_bet_rp_desc}",
                    suffix: "{tr.unit_rp}",
                    value: min_bet_rp,
                }
                BigIntRow {
                    label: "{tr.max_bet_rp}",
                    desc: "{tr.max_bet_rp_desc}",
                    suffix: "{tr.unit_rp}",
                    value: max_bet_rp,
                }
                BpsRow {
                    label: "{tr.correct_multiplier}",
                    desc: "{tr.correct_multiplier_desc}",
                    value: correct_side_multiplier_bps,
                }
                BpsRow {
                    label: "{tr.insider_bonus}",
                    desc: "{tr.insider_bonus_desc}",
                    value: insider_correct_bonus_bps,
                }
                BpsRow {
                    label: "{tr.influence_bonus}",
                    desc: "{tr.influence_bonus_desc}",
                    value: influence_bonus_bps,
                }
                BigIntRow {
                    label: "{tr.signup_rp}",
                    desc: "{tr.signup_rp_desc}",
                    suffix: "{tr.unit_rp}",
                    value: new_user_signup_rp,
                }
            }

            // Section 03 — insider (D1: TRUTH-KNOWER 1명 고정)
            SettingsSection {
                title: "{tr.section_insider_title}",
                sub: "{tr.section_insider_sub}",
                p { class: "ff-settings__note", "{tr.insider_note}" }
            }

            // Section 04 — operations
            SettingsSection { title: "{tr.section_ops_title}", sub: "{tr.section_ops_sub}",
                IntRow {
                    label: "{tr.reconnect_grace}",
                    desc: "{tr.reconnect_grace_desc}",
                    suffix: "{tr.unit_sec}",
                    value: reconnect_grace_sec,
                }
                IntRow {
                    label: "{tr.queue_alert}",
                    desc: "{tr.queue_alert_desc}",
                    suffix: "{tr.unit_day}",
                    value: queue_low_alert_days,
                }
            }

            // Deferred — link rest of mockup to spec gaps so reviewers
            // know what's intentional vs missed.
            div { class: "ff-settings__deferred",
                strong { "{tr.deferred_title}" }
                ul {
                    li { "{tr.deferred_auto_publish}" }
                    li { "{tr.deferred_auto_backfill}" }
                    li { "{tr.deferred_essence_policy}" }
                    li { "{tr.deferred_danger_zone}" }
                }
            }

            // Save bar
            div { class: "ff-settings__actions",
                if let Some(err) = error_msg() {
                    span { class: "ff-settings__error", "{err}" }
                }
                if saved_at().is_some() && error_msg().is_none() {
                    span { class: "ff-settings__saved", "{tr.saved}" }
                }
                button {
                    class: "btn btn--primary",
                    disabled: submitting(),
                    onclick: on_save,
                    if submitting() {
                        "{tr.saving}"
                    } else {
                        "{tr.save}"
                    }
                }
            }
        }
    }
}

#[component]
fn SettingsSection(title: String, sub: String, children: Element) -> Element {
    rsx! {
        section { class: "ff-settings__section",
            header { class: "ff-settings__section-head",
                span { class: "ff-settings__section-title", "{title}" }
                span { class: "ff-settings__section-sub", "{sub}" }
            }
            div { class: "ff-settings__panel", {children} }
        }
    }
}

#[component]
fn IntRow(label: String, desc: String, suffix: String, value: Signal<i32>) -> Element {
    let mut value = value;
    rsx! {
        div { class: "ff-settings__row",
            div { class: "ff-settings__row-text",
                div { class: "ff-settings__label", "{label}" }
                div { class: "ff-settings__desc", "{desc}" }
            }
            div { class: "ff-settings__control",
                input {
                    class: "ff-settings__input",
                    r#type: "number",
                    value: "{value}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<i32>() {
                            value.set(v);
                        }
                    },
                }
                span { class: "ff-settings__suffix", "{suffix}" }
            }
        }
    }
}

#[component]
fn BigIntRow(label: String, desc: String, suffix: String, value: Signal<i64>) -> Element {
    let mut value = value;
    rsx! {
        div { class: "ff-settings__row",
            div { class: "ff-settings__row-text",
                div { class: "ff-settings__label", "{label}" }
                div { class: "ff-settings__desc", "{desc}" }
            }
            div { class: "ff-settings__control",
                input {
                    class: "ff-settings__input",
                    r#type: "number",
                    value: "{value}",
                    oninput: move |e| {
                        if let Ok(v) = e.value().parse::<i64>() {
                            value.set(v);
                        }
                    },
                }
                span { class: "ff-settings__suffix", "{suffix}" }
            }
        }
    }
}

/// Basis-points input — UI shows the human "1.6×" label while the
/// underlying signal stores the raw bps integer (10000 = 1.0×).
#[component]
fn BpsRow(label: String, desc: String, value: Signal<i32>) -> Element {
    let mut value = value;
    let multiplier_str = format!("{:.2}", (value() as f64) / 10_000.0);
    rsx! {
        div { class: "ff-settings__row",
            div { class: "ff-settings__row-text",
                div { class: "ff-settings__label", "{label}" }
                div { class: "ff-settings__desc", "{desc}" }
            }
            div { class: "ff-settings__control",
                input {
                    class: "ff-settings__input",
                    r#type: "number",
                    step: "0.01",
                    value: "{multiplier_str}",
                    oninput: move |e| {
                        if let Ok(f) = e.value().parse::<f64>() {
                            value.set((f * 10_000.0).round() as i32);
                        }
                    },
                }
                span { class: "ff-settings__suffix", "×" }
            }
        }
    }
}
