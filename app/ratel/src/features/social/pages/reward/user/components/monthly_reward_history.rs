use super::super::{
    controllers::*,
    dto::RewardsResponse,
    views::{format_points, format_tokens},
    *,
};
use crate::common::services::PointTransactionResponse;
use chrono::Datelike;

translate! {
    MonthlyHistoryTranslate;

    swap_all: {
        en: "Swap all",
        ko: "전체 교환",
    },

    loading: {
        en: "Loading...",
        ko: "로딩 중...",
    },

    error: {
        en: "Error loading data",
        ko: "데이터 로딩 오류",
    },

    load_more: {
        en: "Load more",
        ko: "더 보기",
    },

    back: {
        en: "Back",
        ko: "뒤로",
    },

    accumulated_to: {
        en: "Accumulated to",
        ko: "누적 대상",
    },

    from: {
        en: "from",
        ko: "from",
    },

    received: {
        en: "Received",
        ko: "획득",
    },

    spent: {
        en: "Spent",
        ko: "사용",
    },

    empty: {
        en: "No transactions",
        ko: "거래 내역 없음",
    },

    empty_description: {
        en: "No point transactions for this month",
        ko: "이 달의 포인트 거래 내역이 없습니다",
    },
}

#[component]
pub fn MonthlyRewardHistory(
    username: ReadSignal<String>,
    month: ReadSignal<String>,
    on_back: EventHandler<()>,
) -> Element {
    let tr: MonthlyHistoryTranslate = use_translate();

    let rewards_resource =
        use_loader(
            move || async move { get_user_rewards_handler(username(), Some(month())).await },
        )?;

    let transactions_resource = use_loader(move || async move {
        list_user_transactions_handler(username(), Some(month()), None).await
    })?;

    let rewards_state = rewards_resource();
    let transactions_state = transactions_resource();

    let mut transactions = use_signal(move || transactions_resource().items);
    let mut next_bookmark = use_signal(move || transactions_resource().bookmark);
    let mut transactions_loaded = use_signal(|| false);
    let mut is_fetching_next = use_signal(|| false);

    let m = month();

    let date_range = format_month_date_range(&m);

    let rewards = rewards_resource();

    let estimated_tokens = if rewards.total_points > 0 {
        ((rewards.points as f64 / rewards.total_points as f64)
            * rewards.monthly_token_supply as f64)
            .round()
    } else {
        0.0
    };

    let has_next = next_bookmark().is_some();
    let is_fetching = *is_fetching_next.read();

    rsx! {
        Card { class: "w-full",
            // Header with date range
            div { class: "flex gap-2 items-center pb-5",
                button {
                    class: "cursor-pointer text-foreground-muted hover:text-text-primary",
                    onclick: move |_| on_back.call(()),
                    icons::arrows::ArrowLeft {
                        width: "20",
                        height: "20",
                        class: "[&>path]:stroke-icon-primary",
                    }
                }
                span { class: "font-bold text-[15px] text-text-primary tracking-[0.5px]",
                    "{date_range}"
                }
            }

            // Summary
            div { class: "flex flex-col gap-0.5 items-center py-5 border-b border-separator",
                div { class: "flex gap-1 items-center",
                    div { class: "w-5 h-5 rounded-full bg-primary" }
                    span { class: "text-xl font-bold text-text-primary tracking-[0.5px]",
                        "{format_points(rewards.points)} P"
                    }
                }
                div { class: "flex gap-1 items-center",
                    span { class: "text-sm font-medium text-foreground-muted", "{tr.accumulated_to}" }
                    div { class: "w-5 h-5 rounded-full bg-primary" }
                    span { class: "text-sm font-semibold text-text-primary",
                        "{rewards.project_name} point"
                    }
                }
            }

            // Exchange preview
            div { class: "py-5 border-b border-separator",
                div { class: "flex gap-4 justify-between items-center w-full",
                    div { class: "flex flex-col gap-0.5",
                        div { class: "flex gap-1 items-center",
                            div { class: "w-5 h-5 rounded-full bg-primary" }
                            span { class: "font-medium text-[15px] text-text-primary",
                                "{format_points(rewards.points)} P"
                            }
                        }
                        div { class: "flex gap-1 items-center",
                            span { class: "text-sm font-semibold text-foreground-muted",
                                "{tr.from}"
                            }
                            div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                            span { class: "text-sm font-semibold text-text-primary",
                                "{rewards.project_name}"
                            }
                        }
                    }

                    div { class: "p-2.5 rounded-xl bg-[var(--web\\/card\\/bg2,#262626)]",
                        icons::arrows::CrossoverArrowsRight {
                            width: "24",
                            height: "24",
                            class: "[&>path]:stroke-icon-primary",
                        }
                    }

                    div { class: "flex flex-col gap-0.5 items-end",
                        div { class: "flex gap-1 items-center",
                            span { class: "font-medium text-[15px] text-foreground-muted",
                                "{format_tokens(estimated_tokens)} {rewards.token_symbol}"
                            }
                            div { class: "w-5 h-5 rounded-full bg-primary" }
                        }
                        div { class: "flex gap-1 items-center",
                            span { class: "text-sm font-semibold text-foreground-muted",
                                "To"
                            }
                            div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                            span { class: "text-sm font-semibold text-text-primary",
                                "{rewards.project_name} Token"
                            }
                        }
                    }
                }

                div { class: "pt-5",
                    Button {
                        class: "w-full",
                        size: ButtonSize::Medium,
                        style: ButtonStyle::Primary,
                        "{tr.swap_all}"
                    }
                }
            }

            // Transaction list
            div { class: "flex flex-col gap-2.5 pt-5",
                for (idx, item) in transactions.read().iter().enumerate() {
                    TransactionRow {
                        key: "{item.created_at}-{idx}",
                        transaction: item.clone(),
                    }
                }

                if has_next {
                    div { class: "flex justify-center pt-2",
                        Button {
                            size: ButtonSize::Small,
                            style: ButtonStyle::Outline,
                            disabled: is_fetching,
                            onclick: {
                                move |_| {
                                    if *is_fetching_next.read() {
                                        return;
                                    }
                                    let Some(bookmark) = next_bookmark() else {
                                        return;
                                    };
                                    is_fetching_next.set(true);
                                    spawn(async move {
                                        let result = list_user_transactions_handler(
                                                username(),
                                                Some(month()),
                                                Some(bookmark),
                                            )
                                            .await;
                                        if let Ok(data) = result {
                                            let mut updated = transactions.read().clone();
                                            updated.extend(data.items);
                                            transactions.set(updated);
                                            next_bookmark.set(data.bookmark);
                                        }
                                        is_fetching_next.set(false);
                                    });
                                }
                            },
                            if is_fetching {
                                "{tr.loading}"
                            } else {
                                "{tr.load_more}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TransactionRow(transaction: PointTransactionResponse) -> Element {
    let tr: MonthlyHistoryTranslate = use_translate();
    let is_received = transaction.transaction_type.eq_ignore_ascii_case("award");
    let time_ago_label = crate::common::utils::time::time_ago(transaction.created_at);
    let description = transaction
        .description
        .clone()
        .unwrap_or_else(|| "Ratel".to_string());
    let amount_label = format!("{} P", format_points(transaction.amount));
    let status_label = if is_received { tr.received } else { tr.spent };
    let status_class = if is_received {
        "text-[15px] font-medium text-green-500"
    } else {
        "text-[15px] font-medium text-red-500"
    };

    rsx! {
        div { class: "p-5 rounded border border-[var(--web\\/card\\/stroke2,#262626)]",
            div { class: "flex justify-between items-end w-full",
                div { class: "flex flex-col gap-0.5",
                    div { class: "flex gap-2.5 items-center",
                        span { class: "{status_class}", "{status_label}" }
                        div { class: "flex gap-1 items-center",
                            div { class: "w-5 h-5 rounded-full bg-primary" }
                            span { class: "font-medium text-[15px] text-text-primary",
                                "{amount_label}"
                            }
                        }
                    }
                    div { class: "flex gap-1 items-center",
                        span { class: "text-sm font-semibold text-foreground-muted",
                            "{tr.from}"
                        }
                        div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                        span { class: "text-sm font-semibold text-text-primary", "{description}" }
                    }
                }
                div { class: "flex gap-1 items-center",
                    span { class: "text-sm font-medium text-foreground-muted", "{time_ago_label}" }
                    lucide_dioxus::ExternalLink {
                        size: 18,
                        class: "[&>path]:stroke-foreground-muted [&>polyline]:stroke-foreground-muted [&>line]:stroke-foreground-muted",
                    }
                }
            }
        }
    }
}

fn format_month_date_range(month: &str) -> String {
    let parts: Vec<&str> = month.split('-').collect();
    if parts.len() == 2 {
        let year: i32 = parts[0].parse().unwrap_or(2025);
        let mon: u32 = parts[1].parse().unwrap_or(1);
        let month_names = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        let name = month_names.get((mon - 1) as usize).unwrap_or(&"");
        let last_day = if mon == 12 {
            chrono::NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            chrono::NaiveDate::from_ymd_opt(year, mon + 1, 1)
        }
        .unwrap_or_default()
            - chrono::Duration::days(1);
        format!("{} 1 - {}", name, last_day.day())
    } else {
        month.to_string()
    }
}
