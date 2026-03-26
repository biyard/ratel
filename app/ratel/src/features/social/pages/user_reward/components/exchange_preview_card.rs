use super::super::{
    dto::RewardsResponse,
    views::{format_points, format_tokens, RewardsPageTranslate},
    *,
};

pub fn exchange_preview_card(
    tr: &RewardsPageTranslate,
    rewards: &RewardsResponse,
    estimated_tokens: f64,
) -> Element {
    rsx! {
        div { class: "border-t border-card-border pt-10",
            div { class: "flex flex-col items-center gap-5 w-full",
                // Exchange row: Points → Tokens
                div { class: "flex items-start justify-between w-full",
                    // Left: Points
                    div { class: "flex flex-col gap-0.5",
                        div { class: "flex items-center gap-1",
                            div { class: "w-5 h-5 rounded-full bg-primary" }
                            span { class: "text-[15px] font-medium text-text-primary tracking-[0.5px]",
                                "{format_points(rewards.points)} P"
                            }
                        }
                        div { class: "flex items-center gap-1",
                            span { class: "text-sm font-semibold text-foreground-muted tracking-[0.5px]",
                                "{tr.exchange_from}"
                            }
                            div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                            span { class: "text-sm font-semibold text-text-primary tracking-[0.5px]",
                                "{rewards.project_name} {tr.point}"
                            }
                        }
                    }

                    // Center: Exchange icon
                    div { class: "flex items-center justify-center self-stretch w-14",
                        div { class: "bg-[var(--web\\/card\\/bg2,#262626)] rounded-xl p-2.5",
                            icons::arrows::CrossoverArrowsRight {
                                width: "24",
                                height: "24",
                                class: "[&>path]:stroke-icon-primary",
                            }
                        }
                    }

                    // Right: Tokens
                    div { class: "flex flex-col items-end gap-0.5",
                        div { class: "flex items-center gap-1",
                            span { class: "text-[15px] font-medium text-foreground-muted tracking-[0.5px]",
                                "{format_tokens(estimated_tokens)} {rewards.token_symbol}"
                            }
                            div { class: "w-5 h-5 rounded-full bg-primary" }
                        }
                        div { class: "flex items-center gap-1",
                            span { class: "text-sm font-semibold text-foreground-muted tracking-[0.5px]",
                                "{tr.exchange_to}"
                            }
                            div { class: "w-5 h-5 rounded-full bg-foreground-muted" }
                            span { class: "text-sm font-semibold text-text-primary tracking-[0.5px]",
                                "{rewards.project_name} {tr.token}"
                            }
                        }
                    }
                }

                // Swap message
                p { class: "text-xs font-medium text-foreground-muted text-center",
                    "{tr.swap_available_message}"
                }
            }
        }
    }
}
