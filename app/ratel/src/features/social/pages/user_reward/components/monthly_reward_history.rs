use chrono::Datelike;
use super::super::{
    controllers::*,
    dto::RewardsResponse,
    views::{format_points, format_tokens},
    *,
};
use crate::common::services::PointTransactionResponse;

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
    username: String,
    month: String,
    on_back: EventHandler<()>,
) -> Element {
    let tr: MonthlyHistoryTranslate = use_translate();
    let month_clone = month.clone();
    let username_clone = username.clone();
    let username_clone2 = username.clone();
    let month_clone2 = month.clone();

    let rewards_resource = use_server_future(move || {
        let username = username_clone.clone();
        let month = month_clone.clone();
        async move { get_user_rewards_handler(username, Some(month)).await }
    })?;

    let transactions_resource = use_server_future(move || {
        let username = username_clone2.clone();
        let month = month_clone2.clone();
        async move { list_user_transactions_handler(username, Some(month), None).await }
    })?;

    let rewards_state = rewards_resource.value();
    let transactions_state = transactions_resource.value();

    let mut transactions = use_signal(Vec::<PointTransactionResponse>::new);
    let mut next_bookmark = use_signal(|| Option::<String>::None);
    let mut transactions_loaded = use_signal(|| false);
    let mut is_fetching_next = use_signal(|| false);

    {
        let transactions_state = transactions_state.clone();
        let mut transactions = transactions.clone();
        let mut next_bookmark = next_bookmark.clone();
        let mut transactions_loaded = transactions_loaded.clone();

        use_effect(move || {
            if *transactions_loaded.read() {
                return;
            }

            let transaction_state = transactions_state.read();
            let Some(state) = transaction_state.as_ref() else {
                return;
            };

            if let Ok(data) = state {
                transactions.set(data.items.clone());
                next_bookmark.set(data.bookmark.clone());
            }
            transactions_loaded.set(true);
        });
    }

    let date_range = format_month_date_range(&month);

    let rewards = match rewards_state.read().as_ref() {
        Some(Ok(data)) => data.clone(),
        Some(Err(_)) => {
            return rsx! {
                div { class: "text-center text-destructive py-8", "{tr.error}" }
            };
        }
        None => RewardsResponse::default(),
    };

    let estimated_tokens = if rewards.total_points > 0 {
        ((rewards.points as f64 / rewards.total_points as f64)
            * rewards.monthly_token_supply as f64)
            .round()
    } else {
        0.0
    };

    let has_next = next_bookmark.read().is_some();
    let is_fetching = *is_fetching_next.read();

    rsx! {
        Card { class: "w-full",
            // Header with date range
            div { class: "flex items-center gap-2 pb-5",
                button {
                    class: "text-foreground-muted hover:text-text-primary cursor-pointer",
                    onclick: move |_| on_back.call(()),
                    icons::arrows::ArrowLeft {
                        width: "20",
                        height: "20",
                        class: "[&>path]:stroke-icon-primary",
                    }
                }
                span { class: "text-[15px] font-bold text-text-primary tracking-[0.5px]",
                    "{date_range}"
                }
            }

            // Summary
            div { class: "flex flex-col gap-0.5 items-center py-5 border-b border-separator",
                div { class: "flex items-center gap-1",
                    div { class: "w-5 h-5 rounded-full bg-primary" }
                    span { class: "text-xl font-bold text-text-primary tracking-[0.5px]",
                        "{format_points(rewards.points)} P"
                    }
                }
                div { class: "flex items-center gap-1",
                    span { class: "text-sm font-medium text-foreground-muted", "{tr.accumulated_to}" }
                    div { class: "w-5 h-5 rounded-full bg-primary" }
                    span { class: "text-sm font-semibold text-text-primary",
                        "{rewards.project_name} point"
                    }
                }
            }

            // Exchange preview
            div { class: "py-5 border-b border-separator",
                div { class: "flex items-center justify-between gap-4 w-full",
                    div { class: "flex flex-col gap-0.5",
                        div { class: "flex items-center gap-1",
                            div { class: "w-5 h-5 rounded-full bg-primary" }
                            span { class: "text-[15px] font-medium text-text-primary",
                                "{format_points(rewards.points)} P"
                            }
                        }
                        div { class: "flex items-center gap-1",
                            span { class: "text-sm font-semibold text-foreground-muted", "{tr.from}" }
                            div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                            span { class: "text-sm font-semibold text-text-primary", "{rewards.project_name}" }
                        }
                    }

                    div { class: "bg-[var(--web\\/card\\/bg2,#262626)] rounded-xl p-2.5",
                        icons::arrows::CrossoverArrowsRight {
                            width: "24",
                            height: "24",
                            class: "[&>path]:stroke-icon-primary",
                        }
                    }

                    div { class: "flex flex-col items-end gap-0.5",
                        div { class: "flex items-center gap-1",
                            span { class: "text-[15px] font-medium text-foreground-muted",
                                "{format_tokens(estimated_tokens)} {rewards.token_symbol}"
                            }
                            div { class: "w-5 h-5 rounded-full bg-primary" }
                        }
                        div { class: "flex items-center gap-1",
                            span { class: "text-sm font-semibold text-foreground-muted", "To" }
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
                for (idx , item) in transactions.read().iter().enumerate() {
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
                                let username = username.clone();
                                let month = month.clone();
                                move |_| {
                                    let username = username.clone();
                                    let month = month.clone();
                                    if *is_fetching_next.read() {
                                        return;
                                    }
                                    let Some(bookmark) = next_bookmark.read().clone() else {
                                        return;
                                    };
                                    is_fetching_next.set(true);
                                    spawn(async move {
                                        let result = list_user_transactions_handler(
                                            username, Some(month), Some(bookmark),
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
        div { class: "border border-[var(--web\\/card\\/stroke2,#262626)] rounded p-5",
            div { class: "flex items-end justify-between w-full",
                div { class: "flex flex-col gap-0.5",
                    div { class: "flex items-center gap-2.5",
                        span { class: "{status_class}", "{status_label}" }
                        div { class: "flex items-center gap-1",
                            div { class: "w-5 h-5 rounded-full bg-primary" }
                            span { class: "text-[15px] font-medium text-text-primary", "{amount_label}" }
                        }
                    }
                    div { class: "flex items-center gap-1",
                        span { class: "text-sm font-semibold text-foreground-muted", "{tr.from}" }
                        div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                        span { class: "text-sm font-semibold text-text-primary", "{description}" }
                    }
                }
                div { class: "flex items-center gap-1",
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
        let name = month_names
            .get((mon - 1) as usize)
            .unwrap_or(&"");
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
