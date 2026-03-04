use crate::views::RewardsPageTranslate;
use crate::*;
use common::services::PointTransactionResponse;

pub fn transaction_item(
    tr: &RewardsPageTranslate,
    transaction: &PointTransactionResponse,
    idx: usize,
) -> Element {
    let is_received = transaction.transaction_type.eq_ignore_ascii_case("award");
    let time_ago = format_time_ago(transaction.created_at);
    let description = transaction
        .description
        .clone()
        .unwrap_or_else(|| "Ratel".to_string());
    let amount_label = if is_received {
        format!("{} P", format_points(transaction.amount))
    } else {
        format!("-{} P", format_points(transaction.amount))
    };
    let status_label = if is_received { tr.received } else { tr.spent };
    let status_class = if is_received {
        "text-[15px] font-medium text-green-500"
    } else {
        "text-[15px] font-medium text-red-500"
    };

    rsx! {
        div {
            key: "{transaction.created_at}-{idx}",
            class: "rounded border border-card-border bg-card-bg px-4 py-4",
            div { class: "flex items-center justify-between w-full",
                div { class: "flex flex-col gap-0.5",
                    div { class: "flex items-center gap-2.5",
                        span { class: "{status_class}", "{status_label}" }
                        div { class: "flex items-center",
                            div { class: "w-5 h-5 rounded-full bg-primary mr-1" }
                            span { class: "text-[15px] font-medium text-white", "{amount_label}" }
                        }
                    }
                    div { class: "flex items-center gap-2.5",
                        span { class: "text-sm font-semibold text-text-primary", "{tr.from}" }
                        div { class: "flex items-center gap-1",
                            div { class: "w-3 h-3 rounded-full bg-bg" }
                            span { class: "text-sm font-semibold text-white", "{description}" }
                        }
                    }
                }
                div { class: "flex items-center gap-1",
                    span { class: "text-sm font-medium text-text-primary", "{time_ago}" }
                }
            }
        }
    }
}

fn format_points(points: i64) -> String {
    format_with_commas(points, None)
}

fn format_with_commas(value: i64, suffix: Option<&str>) -> String {
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
    if let Some(suffix) = suffix {
        format!("{}{}{}", sign, formatted, suffix)
    } else {
        format!("{}{}", sign, formatted)
    }
}

fn format_time_ago(timestamp_millis: i64) -> String {
    let now = chrono::Utc::now().timestamp_millis();
    let diff = now - timestamp_millis;

    if diff < 60 * 1000 {
        format!("{}s ago", diff / 1000)
    } else if diff < 3600 * 1000 {
        format!("{}m ago", diff / 1000 / 60)
    } else if diff < 86400 * 1000 {
        format!("{}h ago", diff / 1000 / 3600)
    } else if diff < 604800 * 1000 {
        format!("{}d ago", diff / 1000 / 86400)
    } else if diff < 31536000 * 1000 {
        format!("{}w ago", diff / 1000 / 604800)
    } else {
        format!("{}y ago", diff / 1000 / 31536000)
    }
}
