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
        div { class: "pt-10 border-t border-card-border",
            div { class: "flex flex-col gap-5 items-center w-full",
                // Exchange row: Points → Tokens
                div { class: "flex justify-between items-start w-full",
                    // Left: Points
                    div { class: "flex flex-col gap-0.5",
                        div { class: "flex gap-1 items-center",
                            div { class: "w-5 h-5 rounded-full bg-primary" }
                            span { class: "font-medium text-[15px] text-text-primary tracking-[0.5px]",
                                "{format_points(rewards.points)} P"
                            }
                        }
                        div { class: "flex gap-1 items-center",
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
                    div { class: "flex justify-center items-center self-stretch w-14",
                        div { class: "p-2.5 rounded-xl bg-[var(--web\\/card\\/bg2,#262626)]",
                            icons::arrows::CrossoverArrowsRight {
                                width: "24",
                                height: "24",
                                class: "[&>path]:stroke-icon-primary",
                            }
                        }
                    }

                    // Right: Tokens
                    div { class: "flex flex-col gap-0.5 items-end",
                        div { class: "flex gap-1 items-center",
                            span { class: "font-medium text-[15px] text-foreground-muted tracking-[0.5px]",
                                "{format_tokens(estimated_tokens)} {rewards.token_symbol}"
                            }
                            div { class: "w-5 h-5 rounded-full bg-primary" }
                        }
                        div { class: "flex gap-1 items-center",
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
                p { class: "text-xs font-medium text-center text-foreground-muted",
                    "{tr.swap_available_message}"
                }
            }
        }
    }
}
