use common::chrono::TimeZone;

use super::super::{controllers::PurchaseHistoryResponse, views::MembershipPageTranslate, *};

pub fn render_history(
    state: Option<&Result<PurchaseHistoryResponse>>,
    tr: &MembershipPageTranslate,
) -> Element {
    let tr: MembershipPageTranslate = use_translate();

    let Some(state) = state else {
        return rsx! {
            div { class: "flex justify-center py-8",
                div { class: "w-8 h-8 rounded-full border-b-2 animate-spin border-primary" }
            }
        };
    };

    let default_history = PurchaseHistoryResponse::default();
    let history = match state {
        Ok(data) => data,
        Err(_) => &default_history,
    };

    if history.items.is_empty() {
        return rsx! {
            div { class: "py-8 text-center text-text-secondary", "{tr.no_purchases}" }
        };
    }

    rsx! {
        div { class: "overflow-x-auto",
            table { class: "w-full",
                thead {
                    tr { class: "border-b border-card-border",
                        th { class: "py-3 px-2 text-sm font-semibold text-left text-text-secondary",
                            "{tr.transaction_type}"
                        }
                        th { class: "py-3 px-2 text-sm font-semibold text-left text-text-secondary",
                            "{tr.amount}"
                        }
                        th { class: "py-3 px-2 text-sm font-semibold text-left text-text-secondary",
                            "{tr.payment_id}"
                        }
                        th { class: "py-3 px-2 text-sm font-semibold text-left text-text-secondary",
                            "{tr.date}"
                        }
                    }
                }
                tbody {
                    for (idx , item) in history.items.iter().enumerate() {
                        tr {
                            key: "{idx}",
                            class: "border-b last:border-0 border-card-border",
                            td { class: "py-3 px-2 text-sm text-text-primary", "{item.tx_type}" }
                            td { class: "py-3 px-2 text-sm text-text-primary",
                                {format!("${}", item.amount)}
                            }
                            td { class: "py-3 px-2 font-mono text-xs text-text-secondary",
                                "{item.payment_id}"
                            }
                            td { class: "py-3 px-2 text-sm text-text-secondary",
                                "{format_date(item.created_at, tr.unlimited)}"
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn format_date(timestamp: i64, unlimited_label: &str) -> String {
    if timestamp == i64::MAX {
        return unlimited_label.to_string();
    }

    let dt = common::chrono::Utc.timestamp_millis_opt(timestamp).single();

    dt.map(|v| v.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| timestamp.to_string())
}
