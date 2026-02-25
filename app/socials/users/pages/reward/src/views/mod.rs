use crate::components::{exchange_preview_card, points_summary_card, transaction_list};
use crate::controllers::{get_rewards_handler, list_point_transactions_handler};
use crate::dto::{PointTransactionResponse, RewardsResponse};
use crate::*;
use dioxus::prelude::*;

#[component]
pub fn Home(username: String) -> Element {
    let tr: RewardsPageTranslate = use_translate();

    let rewards_resource =
        use_server_future(move || async move { get_rewards_handler(None).await })?;
    let transactions_resource =
        use_server_future(
            move || async move { list_point_transactions_handler(None, None).await },
        )?;

    let rewards_state = rewards_resource.value();
    let transactions_state = transactions_resource.value();

    let mut transactions = use_signal(Vec::<PointTransactionResponse>::new);
    let mut next_bookmark = use_signal(|| Option::<String>::None);
    let mut transactions_loaded = use_signal(|| false);
    let mut transactions_error = use_signal(|| false);
    let mut is_fetching_next = use_signal(|| false);

    {
        let transactions_state = transactions_state.clone();
        let mut transactions = transactions.clone();
        let mut next_bookmark = next_bookmark.clone();
        let mut transactions_loaded = transactions_loaded.clone();
        let mut transactions_error = transactions_error.clone();

        use_effect(move || {
            if *transactions_loaded.read() {
                return;
            }

            let transaction_state = transactions_state.read();

            let Some(state) = transaction_state.as_ref() else {
                return;
            };

            match state {
                Ok(data) => {
                    transactions.set(data.items.clone());
                    next_bookmark.set(data.bookmark.clone());
                }
                Err(_) => {
                    transactions_error.set(true);
                }
            }

            transactions_loaded.set(true);
        });
    }

    if rewards_state.read().is_none() {
        return rsx! {
            div { class: "w-full max-w-desktop mx-auto px-4 py-8",
                div { class: "text-center text-text-primary", "{tr.loading}" }
            }
        };
    }

    let rewards = match rewards_state.read().as_ref() {
        Some(Ok(data)) => data.clone(),
        Some(Err(err)) => {
            return rsx! {
                div { class: "w-full max-w-desktop mx-auto px-4 py-8",
                    div { class: "bg-card-bg border border-card-border rounded-lg p-8",
                        div { class: "text-center text-destructive text-text-primary",
                            "{tr.error}: {err}"
                        }
                    }
                }
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
    let is_loading_transactions = !*transactions_loaded.read();
    let is_fetching_next_value = *is_fetching_next.read();
    let transactions_error_value = *transactions_error.read();
    let month = rewards.month.clone();

    let on_load_more = {
        let mut transactions = transactions.clone();
        let mut next_bookmark = next_bookmark.clone();
        let mut is_fetching_next = is_fetching_next.clone();
        let month = month.clone();

        move |_| {
            let month = month.clone();
            if *is_fetching_next.read() {
                return;
            }

            let Some(bookmark) = next_bookmark.read().clone() else {
                return;
            };

            is_fetching_next.set(true);
            spawn(async move {
                let result = list_point_transactions_handler(Some(month), Some(bookmark)).await;
                if let Ok(data) = result {
                    let mut updated = {
                        let current = transactions.read();
                        current.clone()
                    };
                    updated.extend(data.items);
                    transactions.set(updated);
                    next_bookmark.set(data.bookmark);
                }
                is_fetching_next.set(false);
            });
        }
    };

    rsx! {
        div { class: "w-full max-w-desktop mx-auto px-4 py-6",
            {points_summary_card(&tr, &rewards, estimated_tokens)}
            {exchange_preview_card(&tr, &rewards, estimated_tokens)}

            div { class: "mt-6",
                {
                    transaction_list(
                        &tr,
                        transactions.read().as_slice(),
                        is_loading_transactions,
                        transactions_error_value,
                    )
                }

                if has_next && !transactions_error_value {
                    button {
                        class: "mt-4 py-3 text-center text-sm font-medium text-text-primary hover:text-white transition-colors disabled:opacity-50",
                        onclick: on_load_more,
                        disabled: is_fetching_next_value,
                        if is_fetching_next_value {
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

fn format_points(points: i64) -> String {
    format_with_commas(points, None)
}

fn format_tokens(tokens: f64) -> String {
    let formatted = format!("{:.2}", tokens);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    format_with_commas_str(trimmed)
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

fn format_with_commas_str(value: &str) -> String {
    let (sign, raw) = if let Some(stripped) = value.strip_prefix('-') {
        ("-", stripped)
    } else {
        ("", value)
    };

    let mut parts = raw.split('.');
    let int_part = parts.next().unwrap_or("");
    let frac_part = parts.next();

    let mut out = String::new();
    for (idx, ch) in int_part.chars().rev().enumerate() {
        if idx > 0 && idx % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    let int_formatted: String = out.chars().rev().collect();

    if let Some(frac) = frac_part {
        if frac.is_empty() {
            format!("{}{}", sign, int_formatted)
        } else {
            format!("{}{}.{}", sign, int_formatted, frac)
        }
    } else {
        format!("{}{}", sign, int_formatted)
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

translate! {
    RewardsPageTranslate;

    title: {
        en: "This month's points",
        ko: "이번 달 포인트",
    },

    your_share: {
        en: "Your share",
        ko: "내 지분",
    },

    this_months_pool: {
        en: "This month's pool",
        ko: "이번 달 풀",
    },

    swap_available_message: {
        en: "Point-to-Token Swap will be available starting next month",
        ko: "포인트-토큰 스왑은 다음 달부터 가능합니다",
    },

    exchange_from: {
        en: "from",
        ko: "from",
    },

    exchange_to: {
        en: "To",
        ko: "To",
    },

    point: {
        en: "Point",
        ko: "Point",
    },

    token: {
        en: "Token",
        ko: "Token",
    },

    received: {
        en: "Received",
        ko: "획득",
    },

    spent: {
        en: "Spent",
        ko: "사용",
    },

    from: {
        en: "from",
        ko: "from",
    },

    empty: {
        en: "No transactions",
        ko: "거래 내역 없음",
    },

    empty_description: {
        en: "You have no point transactions yet",
        ko: "아직 포인트 거래 내역이 없습니다",
    },

    loading: {
        en: "Loading...",
        ko: "로딩 중...",
    },

    error: {
        en: "Error loading rewards",
        ko: "리워드 로딩 오류",
    },

    load_more: {
        en: "Load more",
        ko: "더 보기",
    },

    yours: {
        en: "Yours",
        ko: "내 지분",
    },
}
