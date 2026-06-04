#[allow(unused)]
mod i18n;

use super::components::RewardHistorySection;
use super::controllers::get_user_rewards_handler;
use super::*;
use crate::common::*;
use crate::features::launchpad_partner::views::{LaunchpadTokenCard, RewardHero};

pub use i18n::UserRewardsTranslate;

// Kept so sibling reward components keep compiling (they consume these).
translate! {
    RewardsPageTranslate;

    title: { en: "This month's points", ko: "이번 달 포인트" },
    your_share: { en: "Your share", ko: "내 지분" },
    this_months_pool: { en: "This month's pool", ko: "이번 달 풀" },
    swap_available_message: { en: "Point-to-Token Swap will be available starting next month", ko: "포인트-토큰 스왑은 다음 달부터 가능합니다" },
    exchange_from: { en: "from", ko: "from" },
    exchange_to: { en: "To", ko: "To" },
    point: { en: "Point", ko: "Point" },
    token: { en: "Token", ko: "Token" },
    received: { en: "Received", ko: "획득" },
    spent: { en: "Spent", ko: "사용" },
    from: { en: "from", ko: "from" },
    empty: { en: "No transactions", ko: "거래 내역 없음" },
    empty_description: { en: "You have no point transactions yet", ko: "아직 포인트 거래 내역이 없습니다" },
    loading: { en: "Loading...", ko: "로딩 중..." },
    error: { en: "Error loading rewards", ko: "리워드 로딩 오류" },
    load_more: { en: "Load more", ko: "더 보기" },
    yours: { en: "Yours", ko: "내 지분" },
    past_months: { en: "Reward History", ko: "리워드 히스토리" },
    past_months_preparing: { en: "Token claim is being prepared", ko: "토큰 클레임 준비 중입니다" },
    past_months_empty: { en: "No past month rewards yet", ko: "아직 지난 달 리워드가 없습니다" },
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

/// Rewards page (scope-A): the balance is the local `User.points`. The page
/// shows only the point balance, the conversion entry, and the reward
/// history — no console-fed charts / monthly pool / token estimates.
#[component]
pub fn Home(username: ReadSignal<String>) -> Element {
    let tr: UserRewardsTranslate = use_translate();
    let nav = use_navigator();

    let rewards_resource =
        use_loader(move || async move { get_user_rewards_handler(username(), None).await })?;
    let points = rewards_resource().points;

    rsx! {
        div { class: "rewards-arena",
            div { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    button {
                        class: "back-btn",
                        "aria-label": tr.back,
                        onclick: move |_| {
                            nav.go_back();
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "15 18 9 12 15 6" }
                        }
                    }
                    div { class: "topbar-title",
                        span { class: "topbar-title__main", "{tr.page_title}" }
                    }
                }
            }

            div { class: "page",
                // Share-of-pool hero: my points (User.points) vs the current
                // launchpad round total + round info + (bottom-right) the
                // conversion button when the round is open. Isolated so the
                // round lookup doesn't block the rest of the page.
                SuspenseBoundary {
                    RewardHero { points, show_convert: true }
                }

                // Token holdings (launchpad-backed on-chain balance),
                // isolated so the lookup doesn't block the page.
                SuspenseBoundary { LaunchpadTokenCard {} }

                RewardHistorySection { username }
            }
        }
    }
}
