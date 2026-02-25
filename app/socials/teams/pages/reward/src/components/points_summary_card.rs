use crate::{dto::TeamRewardsResponse, views::TeamRewardsTranslate, *};

pub fn points_summary_card(
    tr: &TeamRewardsTranslate,
    rewards: &TeamRewardsResponse,
    estimated_tokens: f64,
) -> Element {
    let share_percentage = if rewards.total_points > 0 && rewards.team_points >= 0 {
        (rewards.team_points as f64 / rewards.total_points as f64) * 100.0
    } else {
        0.0
    };

    let share_label = format!("{:.2}", share_percentage);
    let bar_width = share_percentage.max(0.5);

    rsx! {
        div { class: "bg-card-bg border border-card-border rounded-xl p-5",
            div { class: "flex items-center justify-between mb-5",
                h2 { class: "text-base font-semibold text-text-primary tracking-wide",
                    {tr.title}
                }
                icons::arrows::ArrowUp {
                    width: "20",
                    height: "20",
                    class: "text-white [&>path]:stroke-white",
                }
            }

            div { class: "flex justify-between items-start mb-5",
                div { class: "flex flex-col gap-2",
                    span { class: "text-sm font-semibold text-text-primary", {tr.your_share} }
                    span { class: "text-xl font-bold text-text-primary",
                        "{format_points(rewards.team_points)} P"
                    }
                    div { class: "flex items-center gap-1",
                        icons::arrows::CrossoverArrowsRight {
                            width: "20",
                            height: "20",
                            class: "text-icon-primary [&>path]:stroke-icon-primary",
                        }
                        span { class: "text-[15px] font-medium text-text-primary",
                            "{format_tokens(estimated_tokens)} {rewards.token_symbol} ({share_label}%)"
                        }
                    }
                }

                div { class: "flex flex-col items-end gap-2",
                    span { class: "text-sm font-semibold text-text-primary", {tr.this_months_pool} }
                    span { class: "text-xl font-bold text-text-primary",
                        "{format_tokens(rewards.monthly_token_supply as f64)} {rewards.token_symbol} (100%)"
                    }
                }
            }

            div { class: "relative h-9 bg-neutral-300 dark:bg-neutral-500 rounded-lg overflow-hidden",
                div {
                    class: "absolute left-0 top-0 h-full bg-primary rounded-lg transition-all duration-300",
                    style: format!("width: {}%", bar_width),
                }
                div { class: "absolute left-3 top-1/2 -translate-y-1/2 text-sm font-bold text-white",
                    "{tr.yours} {share_label}%"
                }
                div { class: "absolute right-3 top-1/2 -translate-y-1/2 text-sm font-bold text-white",
                    "100%"
                }
            }
        }
    }
}

fn format_tokens(tokens: f64) -> String {
    let formatted = format!("{:.2}", tokens);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    format_with_commas_str(trimmed)
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
