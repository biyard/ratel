use super::super::views::{format_points, RewardsPageTranslate};
use super::super::*;
use crate::common::services::PointTransactionResponse;
use crate::common::utils::time::time_ago;

pub fn transaction_item(
    tr: &RewardsPageTranslate,
    transaction: &PointTransactionResponse,
    idx: usize,
) -> Element {
    let is_received = transaction.transaction_type.eq_ignore_ascii_case("award");
    let time_ago_label = time_ago(transaction.created_at);
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
            class: "rounded border border-card-border p-5",
            div { class: "flex items-end justify-between w-full",
                div { class: "flex flex-col gap-0.5",
                    div { class: "flex items-center gap-2.5",
                        span { class: "{status_class}", "{status_label}" }
                        div { class: "flex items-center",
                            div { class: "w-5 h-5 rounded-full bg-primary mr-1" }
                            span { class: "text-[15px] font-medium text-text-primary",
                                "{amount_label}"
                            }
                        }
                    }
                    div { class: "flex items-center gap-1",
                        span { class: "text-sm font-semibold text-foreground-muted tracking-[0.5px]",
                            "{tr.from}"
                        }
                        div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                        span { class: "text-sm font-semibold text-text-primary tracking-[0.5px]",
                            "{description}"
                        }
                    }
                }
                div { class: "flex items-center gap-1",
                    span { class: "text-sm font-medium text-foreground-muted tracking-[0.5px]",
                        "{time_ago_label}"
                    }
                    lucide_dioxus::ExternalLink {
                        size: 18,
                        class: "[&>path]:stroke-foreground-muted [&>polyline]:stroke-foreground-muted [&>line]:stroke-foreground-muted",
                    }
                }
            }
        }
    }
}
