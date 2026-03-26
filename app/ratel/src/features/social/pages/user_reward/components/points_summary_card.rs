use super::super::{
    dto::RewardsResponse,
    views::{format_points, format_tokens, RewardsPageTranslate},
    *,
};

pub fn points_summary_card(
    tr: &RewardsPageTranslate,
    rewards: &RewardsResponse,
    estimated_tokens: f64,
) -> Element {
    let share_percentage = if rewards.total_points > 0 && rewards.points >= 0 {
        (rewards.points as f64 / rewards.total_points as f64) * 100.0
    } else {
        0.0
    };

    let share_label = format!("{:.2}", share_percentage);
    let bar_width = share_percentage.max(0.5);

    rsx! {
        div { class: "bg-card-bg rounded-xl px-4 py-5",
            div { class: "flex items-center justify-between mb-5",
                h2 { class: "text-base font-semibold text-text-primary tracking-[0.5px]",
                    "{tr.title}"
                }
                icons::arrows::ArrowUp {
                    width: "20",
                    height: "20",
                    class: "-rotate-90 [&>path]:stroke-white",
                }
            }

            div { class: "flex justify-between items-start mb-5",
                div { class: "flex flex-col gap-2",
                    span { class: "text-sm font-semibold text-foreground-muted tracking-[0.5px]",
                        "{tr.your_share}"
                    }
                    span { class: "text-xl font-bold text-text-primary",
                        "{format_points(rewards.points)} "
                        span { class: "text-[15px] font-medium", "P" }
                    }
                    div { class: "flex items-center gap-1",
                        icons::arrows::CrossoverArrowsRight {
                            width: "20",
                            height: "20",
                            class: "[&>path]:stroke-icon-primary",
                        }
                        span { class: "text-[15px] font-medium text-text-primary",
                            "{format_tokens(estimated_tokens)} {rewards.token_symbol}"
                        }
                        span { class: "text-xs font-medium text-text-primary",
                            " ({share_label}%)"
                        }
                    }
                }

                div { class: "flex flex-col items-end gap-2",
                    span { class: "text-sm font-semibold text-foreground-muted tracking-[0.5px]",
                        "{tr.this_months_pool}"
                    }
                    span { class: "text-xl font-bold text-foreground-muted",
                        "/ {format_tokens(rewards.monthly_token_supply as f64)} "
                        span { class: "text-[15px] font-medium",
                            "{rewards.token_symbol} "
                        }
                        span { class: "text-[12px] font-medium", "(100%)" }
                    }
                }
            }

            div { class: "relative h-9 bg-[var(--web\\/graph\\/bg,#262626)] rounded-[10px] overflow-hidden",
                div {
                    class: "absolute left-0 top-0 h-full bg-primary rounded-[10px] transition-all duration-300",
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
