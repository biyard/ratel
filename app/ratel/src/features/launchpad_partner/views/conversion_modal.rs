//! Point→token conversion entry (ratel side). Mirrors the launchpad demo
//! brand button: shows the balance and hands off to launchpad `/connect`,
//! which drives wallet connect, amount entry, round registration and token
//! issuance. ratel's only role is the (currently no-op) deduct callback.

use crate::common::*;
use crate::features::launchpad_partner::LaunchpadPartnerTranslate;
use crate::features::launchpad_partner::entry::launchpad_entry_url_handler;

fn format_with_commas(value: i64) -> String {
    let digits = value.abs().to_string();
    let mut out = String::new();
    for (idx, ch) in digits.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

#[component]
pub fn PointConversionModal(available_points: i64) -> Element {
    let tr: LaunchpadPartnerTranslate = use_translate();
    // Token-bearing `/connect` URL, built server-side (needs the shared
    // secret). Same value on SSR and after hydration → no href mismatch.
    let url_loader = use_loader(move || async move { launchpad_entry_url_handler().await })?;
    let url = url_loader().url;

    rsx! {
        div { class: "flex flex-col gap-4 w-[420px] max-w-full",
            h3 { class: "text-lg font-bold text-text-primary", "{tr.intro_title}" }
            p { class: "text-sm text-foreground-muted leading-relaxed", "{tr.intro_body}" }

            div { class: "flex justify-between items-center py-3 px-4 rounded-xl bg-card-bg",
                span { class: "text-sm font-semibold text-foreground-muted", "{tr.amount_balance_label}" }
                span { class: "text-base font-bold text-text-primary",
                    "{format_with_commas(available_points)} P"
                }
            }

            a {
                class: "w-full inline-flex items-center justify-center min-h-[44px] rounded-lg bg-primary text-btn-primary-text text-sm font-extrabold transition-opacity hover:opacity-80",
                href: "{url}",
                "{tr.go_convert}"
            }
            p { class: "text-xs text-foreground-muted text-center", "{tr.go_convert_hint}" }
        }
    }
}
