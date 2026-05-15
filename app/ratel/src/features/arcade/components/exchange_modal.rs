//! Exchange modal — RP → chip conversion. Mounted by ArcadeLayout
//! and toggled by the chip-balance widget. v1 only allows RP→chip;
//! the chip→RP direction surfaces as a disabled hint.

use crate::features::arcade::hooks::use_arcade_wallet;
use crate::features::arcade::i18n::ArcadeExchangeModalTranslate;
use crate::*;

const DEFAULT_RP_INPUT: i64 = 500;

#[component]
pub fn ArcadeExchangeModal(open: bool, on_close: EventHandler<()>) -> Element {
    let tr: ArcadeExchangeModalTranslate = use_translate();
    let mut wallet = use_arcade_wallet();
    let state = (wallet.state)();
    let ratio_bps = state.rp_to_chip_ratio_bps.max(0) as i64;

    let mut rp_input = use_signal(|| DEFAULT_RP_INPUT);
    let mut error = use_signal(|| Option::<String>::None);
    let mut summary = use_signal(|| Option::<String>::None);
    let mut submitting = use_signal(|| false);

    let chips_preview = preview_chips(rp_input(), ratio_bps);
    let ratio_label = tr
        .ratio_value
        .replace("{$chips}", &preview_chips(1, ratio_bps).to_string());

    let on_input = move |e: FormEvent| {
        if let Ok(v) = e.value().parse::<i64>() {
            rp_input.set(v.max(0));
            error.set(None);
        }
    };

    let on_confirm = move |_| async move {
        if submitting() {
            return;
        }
        let amount = rp_input();
        if amount <= 0 {
            error.set(Some(tr.error_amount.to_string()));
            return;
        }
        submitting.set(true);
        error.set(None);
        summary.set(None);
        match wallet.convert(amount).await {
            Ok(res) => {
                let msg = tr
                    .success_summary
                    .replace("{$balance}", &res.balance_after.to_string());
                summary.set(Some(msg));
            }
            Err(e) => {
                error.set(Some(format!("{e}")));
            }
        }
        submitting.set(false);
    };

    if !open {
        return rsx! {};
    }

    rsx! {
        div { class: "ff-arcade__modal-scrim", onclick: move |_| on_close.call(()),
            div {
                class: "ff-arcade__modal-card",
                onclick: move |e| e.stop_propagation(),
                div { class: "section-head",
                    h2 { "{tr.title}" }
                    button {
                        class: "btn btn-ghost",
                        style: "padding: 6px 12px; font-size: 12px",
                        onclick: move |_| on_close.call(()),
                        "{tr.cancel_btn}"
                    }
                }
                p {
                    style: "font-size: 13px; color: var(--text-muted); line-height: 1.6; margin: 0 0 18px",
                    "{tr.subtitle}"
                }

                div { class: "ff-arcade__modal-row",
                    span { class: "ff-arcade__modal-label", "{tr.ratio_label}" }
                    span { class: "ff-arcade__modal-value", "{ratio_label}" }
                }

                div { class: "rp-slider-wrap",
                    div { class: "rp-slider-head",
                        span { class: "rp-slider-label", "{tr.input_label}" }
                        span { class: "rp-slider-value", "{rp_input()} RP" }
                    }
                    input {
                        r#type: "number",
                        class: "ff-arcade__modal-input",
                        min: "0",
                        step: "100",
                        value: "{rp_input()}",
                        oninput: on_input,
                    }
                }

                div { class: "ff-arcade__modal-row",
                    span { class: "ff-arcade__modal-label", "{tr.receive_label}" }
                    span { class: "ff-arcade__modal-value ff-arcade__modal-value--accent",
                        {tr.receive_value.replace("{$chips}", &chips_preview.to_string())}
                    }
                }

                if let Some(err) = error() {
                    div { class: "reason-warn", style: "margin-top: 10px", "{err}" }
                }
                if let Some(msg) = summary() {
                    div { class: "ff-arcade__modal-success", "{msg}" }
                }

                div { class: "submit-row",
                    button {
                        class: "btn btn-primary",
                        disabled: submitting() || rp_input() <= 0,
                        onclick: on_confirm,
                        "{tr.confirm_btn}"
                    }
                    span {
                        style: "font-size: 11px; color: var(--text-faint); font-family: 'JetBrains Mono', monospace",
                        "{tr.redeem_disabled_note}"
                    }
                }
            }
        }
    }
}

/// Apply the operator's bps ratio. `10_000 = 1×`, so `chips = rp *
/// (ratio_bps / 10_000)`. Integer math: round half-down.
fn preview_chips(rp_amount: i64, ratio_bps: i64) -> i64 {
    if rp_amount <= 0 || ratio_bps <= 0 {
        return 0;
    }
    (rp_amount.saturating_mul(ratio_bps)) / 10_000
}
