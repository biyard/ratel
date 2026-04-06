use super::components::{exchange_preview_card, points_summary_card, transaction_list};
use super::controllers::{get_user_rewards_handler, list_user_transactions_handler};
use super::dto::RewardsResponse;
use super::*;
use crate::common::services::PointTransactionResponse;
use dioxus::prelude::*;

#[component]
pub fn Home(username: ReadSignal<String>) -> Element {
    let tr: RewardsPageTranslate = use_translate();

    let rewards_resource = use_server_future(use_reactive((&username,), |(name,)| async move {
        get_user_rewards_handler(name(), None).await
    }))?;

    let transactions_resource =
        use_server_future(use_reactive((&username,), |(name,)| async move {
            list_user_transactions_handler(name(), None, None).await
        }))?;

    let rewards_binding = rewards_resource.read();
    let rewards: RewardsResponse = rewards_binding
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned()
        .unwrap_or_default();

    let tx_binding = transactions_resource.read();
    let initial_transactions: super::controllers::ListTransactionsResponse = tx_binding
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned()
        .unwrap_or_default();

    let mut transactions = use_signal(Vec::<PointTransactionResponse>::new);
    let mut next_bookmark = use_signal(|| Option::<String>::None);
    let mut transactions_loaded = use_signal(|| false);
    let mut is_fetching_next = use_signal(|| false);

    use_effect(move || {
        if *transactions_loaded.read() {
            return;
        }
        transactions.set(initial_transactions.items.clone());
        next_bookmark.set(initial_transactions.bookmark.clone());
        transactions_loaded.set(true);
    });

    let estimated_tokens = if rewards.total_points > 0 {
        ((rewards.points as f64 / rewards.total_points as f64)
            * rewards.monthly_token_supply as f64)
            .round()
    } else {
        0.0
    };

    let has_next = next_bookmark.read().is_some();
    let is_fetching_next_value = *is_fetching_next.read();
    let month = rewards.month.clone();

    let on_load_more = move |_| {
        if *is_fetching_next.read() {
            return;
        }

        let Some(bookmark) = next_bookmark.read().clone() else {
            return;
        };

        let month = month.clone();
        let name = username();
        is_fetching_next.set(true);
        spawn(async move {
            let result =
                list_user_transactions_handler(name, Some(month), Some(bookmark)).await;
            if let Ok(data) = result {
                let mut updated = transactions.read().clone();
                updated.extend(data.items);
                transactions.set(updated);
                next_bookmark.set(data.bookmark);
            }
            is_fetching_next.set(false);
        });
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
                        false,
                        false,
                    )
                }

                if has_next {
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

pub fn format_points(points: i64) -> String {
    format_with_commas(points, None)
}

pub fn format_tokens(tokens: f64) -> String {
    let formatted = format!("{:.2}", tokens);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    format_with_commas_str(trimmed)
}

pub fn format_with_commas(value: i64, suffix: Option<&str>) -> String {
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

pub fn format_with_commas_str(value: &str) -> String {
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
