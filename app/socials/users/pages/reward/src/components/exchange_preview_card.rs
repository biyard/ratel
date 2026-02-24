use crate::{dto::RewardsResponse, views::RewardsPageTranslate, *};

pub fn exchange_preview_card(
    tr: &RewardsPageTranslate,
    rewards: &RewardsResponse,
    estimated_tokens: f64,
) -> Element {
    rsx! {
        div { class: "border-t border-bg pt-10",
            div { class: "flex flex-col items-center gap-5 p-4 rounded-lg bg-card-bg border border-card-border",
                div { class: "flex items-center justify-between gap-4 w-full",
                    div { class: "flex flex-col gap-0.5",
                        div { class: "flex items-center gap-1",
                            div { class: "w-5 h-5 rounded-full bg-primary" }
                            span { class: "text-[15px] font-medium text-white",
                                "{format_points(rewards.points)} P"
                            }
                        }
                        div { class: "flex items-start gap-1 flex-col",
                            span { class: "text-sm font-semibold text-text-primary",
                                "{tr.exchange_from}"
                            }
                            span { class: "text-sm font-semibold text-white",
                                "{rewards.project_name} {tr.point}"
                            }
                        }
                    }

                    div { class: "flex items-center justify-center",
                        icons::arrows::ArrowRight {
                            width: "24",
                            height: "24",
                            class: "text-primary [&>path]:stroke-primary",
                        }
                    }

                    div { class: "flex flex-col items-end gap-0.5",
                        div { class: "flex items-center gap-1",
                            span { class: "text-[15px] font-medium text-text-primary",
                                "{format_tokens(estimated_tokens)} {rewards.token_symbol}"
                            }
                            div { class: "w-5 h-5 rounded-full bg-primary" }
                        }
                        div { class: "flex items-end gap-2 flex-col",
                            span { class: "text-sm font-semibold text-text-primary",
                                "{tr.exchange_to}"
                            }
                            span { class: "text-sm font-semibold text-white",
                                "{rewards.project_name} {tr.token}"
                            }
                        }
                    }
                }

                p { class: "text-xs font-medium text-text-primary text-center",
                    "{tr.swap_available_message}"
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
