mod i18n;

pub use i18n::*;

use crate::components::MembershipTier;
use common::components::{Button, ButtonStyle};
use common::components::Card;
use crate::*;
use common::chrono::TimeZone;

#[derive(Clone, Debug, PartialEq)]
pub struct MembershipReceiptData {
    pub tx_id: String,
    pub membership: MembershipTier,
    pub amount: i64,
    pub duration_days: i64,
    pub credits: i64,
    pub paid_at: i64,
}

fn tier_label(tier: MembershipTier) -> &'static str {
    match tier {
        MembershipTier::Free => "Free",
        MembershipTier::Pro => "Pro",
        MembershipTier::Max => "Max",
        MembershipTier::Vip => "VIP",
        MembershipTier::Enterprise => "Enterprise",
    }
}

fn format_with_commas(value: i64) -> String {
    let sign = if value < 0 { "-" } else { "" };
    let digits = value.abs().to_string();
    let mut out = String::new();
    for (idx, ch) in digits.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    let formatted: String = out.chars().rev().collect();
    format!("{}{}", sign, formatted)
}

fn format_date_time(timestamp_millis: i64) -> String {
    let dt = common::chrono::Local
        .timestamp_millis_opt(timestamp_millis)
        .single();
    dt.map(|v| v.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| timestamp_millis.to_string())
}

#[component]
pub fn MembershipReceiptModal(receipt: MembershipReceiptData) -> Element {
    let mut popup = use_popup();
    let tr: MembershipReceiptTranslate = use_translate();
    let receipt_days = tr.receipt_days;
    let amount_text = use_memo(move || format!("₩{}", format_with_commas(receipt.amount)));
    let duration_text = use_memo(move || format!("{} {}", receipt.duration_days, receipt_days));
    let paid_at_text = use_memo(move || format_date_time(receipt.paid_at));
    let tx_id = use_memo(move || {
        if receipt.tx_id.len() > 16 {
            format!("{}...", &receipt.tx_id[..16])
        } else {
            receipt.tx_id.clone()
        }
    });

    rsx! {
        div { class: "w-[420px]",
            div { class: "flex flex-col gap-5",
                div { class: "text-center",
                    div { class: "mb-4 text-6xl", "✓" }
                    h4 { class: "text-lg md:text-xl lg:text-2xl font-semibold text-primary mb-2",
                        {tr.receipt_title}
                    }
                    p { class: "text-sm text-text-secondary", {tr.receipt_thank_you} }
                }

                Card {
                    div { class: "flex flex-col gap-4",
                        div { class: "flex items-center justify-between",
                            p { class: "text-sm font-medium text-text-secondary",
                                {tr.receipt_transaction_id}
                            }
                            p { class: "text-sm text-text-primary font-mono", {tx_id()} }
                        }

                        div { class: "h-px bg-border" }

                        div { class: "flex items-center justify-between",
                            p { class: "text-sm font-medium text-text-secondary",
                                {tr.membership_label}
                            }
                            p { class: "text-sm font-semibold text-text-primary",
                                {tier_label(receipt.membership)}
                            }
                        }

                        div { class: "flex items-center justify-between",
                            p { class: "text-sm font-medium text-text-secondary",
                                {tr.receipt_amount}
                            }
                            h5 { class: "text-base md:text-lg lg:text-xl font-semibold text-primary",
                                {amount_text()}
                            }
                        }

                        div { class: "flex items-center justify-between",
                            p { class: "text-sm font-medium text-text-secondary",
                                {tr.receipt_duration}
                            }
                            p { class: "text-sm text-text-primary", {duration_text()} }
                        }

                        div { class: "flex items-center justify-between",
                            p { class: "text-sm font-medium text-text-secondary",
                                {tr.receipt_credits}
                            }
                            p { class: "text-sm text-text-primary",
                                {format_with_commas(receipt.credits)}
                            }
                        }

                        div { class: "h-px bg-border" }

                        div { class: "flex items-center justify-between",
                            p { class: "text-sm font-medium text-text-secondary",
                                {tr.receipt_paid_at}
                            }
                            p { class: "text-sm text-text-primary", {paid_at_text()} }
                        }
                    }
                }

                Button {
                    style: ButtonStyle::Primary,
                    onclick: move |_| {
                        popup.close();
                    },
                    {tr.receipt_close}
                }
            }
        }
    }
}
