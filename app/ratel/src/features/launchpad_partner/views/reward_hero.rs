//! Share-of-pool hero (arena style) — shows the holder's local points
//! against the current launchpad round's total registered points, the
//! round info, and (for convertible holders) the conversion button in
//! the bottom-right. The button is hidden unless the round is open.

use crate::common::*;
use crate::features::launchpad_partner::round_info::launchpad_round_info_handler;
use crate::features::launchpad_partner::views::PointConversionButton;

fn fmt_commas(value: i64) -> String {
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

#[component]
pub fn RewardHero(points: i64, #[props(default)] show_convert: bool) -> Element {
    let round_res = use_loader(move || async move { launchpad_round_info_handler().await })?;
    let round = round_res();

    let is_open = round.status == "open";
    // Show the convert entry only when the round is open AND the holder
    // actually has points to convert.
    let can_convert = show_convert && is_open && points > 0;

    rsx! {
        div { class: "hero",
            div { class: "hero__main",
                div { class: "hero__eyebrow",
                    span { class: "pulse" }
                    span { "이번 라운드 포인트" }
                    if round.has_round {
                        span { style: "color:var(--text-dim)", "·" }
                        strong { "{round.name}" }
                    }
                }
                div { class: "hero__points",
                    span { class: "hero__points-value", "{fmt_commas(points)}" }
                    span { class: "hero__points-unit", "포인트" }
                }
            }
            // Right column: the round box, and below it (outside the box,
            // still inside the hero) the convert button at the bottom-right.
            div { class: "flex flex-col gap-3",
                div { class: "hero__side",
                    if round.has_round {
                        div { class: "hero__token-label", "라운드" }
                        div { class: "hero__token-value",
                            strong { "{round.name}" }
                            small { "{round.status}" }
                        }
                        div { class: "hero__countdown",
                            div { class: "hero__countdown-text",
                                "참여 "
                                strong { "{fmt_commas(round.total_entries)}" }
                                " · 등록 "
                                strong { "{fmt_commas(round.total_points_registered)}" }
                                " pts"
                            }
                        }
                    } else {
                        div { class: "hero__token-label", "진행 중인 라운드 없음" }
                    }
                }

                // Convert entry — outside the round box, inside the hero,
                // bottom-right. Only when the round is open and points > 0.
                if can_convert {
                    div { class: "flex justify-end",
                        PointConversionButton { available_points: points }
                    }
                }
            }
        }
    }
}
