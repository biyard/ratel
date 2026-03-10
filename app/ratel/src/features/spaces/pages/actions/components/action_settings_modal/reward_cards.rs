use crate::features::spaces::pages::actions::*;

#[derive(Clone, PartialEq)]
pub struct RewardPreviewData {
    pub total_reward: i64,
    pub credits: i64,
    pub points: i64,
}

#[component]
pub fn RewardSummaryCard(
    reward: RewardPreviewData,
    action_point_reward: String,
    boost_unit: String,
) -> Element {
    rsx! {
        Card { class: "flex w-full min-h-[136px] flex-col gap-2 rounded-[8px] border-web-card-stroke bg-web-card-bg px-[17px] py-[17px] max-mobile:px-4 max-mobile:py-4",
            div { class: "flex items-start justify-between gap-3",
                div { class: "flex size-11 shrink-0 items-center justify-center rounded-[10px] bg-green-500 text-web-font-ab-bk",
                    icons::ratel::Clock {
                        width: "24",
                        height: "24",
                        class: "[&>circle]:fill-none",
                    }
                }

                div { class: "flex min-w-0 flex-1 flex-col items-end gap-1 text-right",
                    p { class: "w-full font-bold font-inter text-[24px]/[24px] text-web-font-primary",
                        {format_with_commas(reward.total_reward)}
                    }
                    p { class: "w-full font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-neutral",
                        {action_point_reward}
                    }
                }
            }

            div { class: "mt-auto flex items-center justify-between gap-3",
                div { class: "flex items-center gap-1.5",
                    span { class: "font-medium font-inter text-[12px]/[16px] text-web-font-primary",
                        "{reward.credits} {boost_unit}"
                    }
                    icons::validations::Clear {
                        width: "18",
                        height: "18",
                        class: "text-web-font-neutral [&>path]:stroke-current",
                    }
                }

                span { class: "font-semibold font-inter text-[12px]/[16px] text-web-font-primary",
                    {format_with_commas(reward.points)}
                }
            }
        }
    }
}

#[component]
pub fn RewardsCreditsCard(
    credit_usage: i64,
    remaining_credits: i64,
    credit_usage_label: String,
    remaining_credits_label: String,
    credit_unit_singular: String,
    credit_unit_plural: String,
) -> Element {
    rsx! {
        Card { class: "flex h-[84px] w-full flex-col gap-2 rounded-[8px] border-web-card-stroke bg-web-card-bg px-[17px] pt-[17px] pb-1 max-mobile:h-auto max-mobile:px-4 max-mobile:pt-4 max-mobile:pb-4",
            div { class: "flex items-center justify-between gap-3",
                p { class: "font-semibold font-raleway text-[13px]/[16px] tracking-[-0.14px] text-web-font-neutral",
                    {credit_usage_label}
                }
                p { class: "font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-primary",
                    {format_credit_amount(credit_usage, &credit_unit_singular, &credit_unit_plural)}
                }
            }

            div { class: "flex items-center justify-between gap-3",
                p { class: "font-semibold font-raleway text-[13px]/[16px] tracking-[-0.14px] text-web-font-neutral",
                    {remaining_credits_label}
                }
                p { class: "font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-green-500",
                    {
                        format_credit_amount(
                            remaining_credits,
                            &credit_unit_singular,
                            &credit_unit_plural,
                        )
                    }
                }
            }
        }
    }
}

#[component]
pub fn RewardsInfoCard(
    title: String,
    line_one: String,
    line_two: String,
    line_three: String,
    membership: String,
) -> Element {
    rsx! {
        div { class: "flex w-full items-start gap-5 rounded-[12px] border border-web-card-stroke3 bg-web-card-bg2 p-5 max-mobile:gap-4 max-mobile:p-4",
            icons::help_support::Info {
                width: "18",
                height: "18",
                class: "mt-0.5 shrink-0 text-web-font-neutral [&>path]:stroke-current",
            }

            div { class: "flex min-w-0 flex-1 flex-col gap-2",
                p { class: "font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-primary",
                    {title}
                }

                div { class: "flex flex-col text-[13px]/[20px] font-medium font-raleway text-web-font-body",
                    p { "• {line_one}" }
                    p { "• {line_two}" }
                    p {
                        "• {line_three} "
                        span { class: "underline", {membership} }
                    }
                }
            }
        }
    }
}

fn format_with_commas(value: i64) -> String {
    let negative = value.is_negative();
    let digits = value.abs().to_string();
    let mut formatted = String::with_capacity(digits.len() + (digits.len() / 3));

    for (idx, ch) in digits.chars().rev().enumerate() {
        if idx != 0 && idx % 3 == 0 {
            formatted.push(',');
        }
        formatted.push(ch);
    }

    let mut formatted: String = formatted.chars().rev().collect();
    if negative {
        formatted.insert(0, '-');
    }

    formatted
}

fn format_credit_amount(value: i64, singular: &str, plural: &str) -> String {
    let unit = if value.abs() == 1 { singular } else { plural };
    format!("{} {}", format_with_commas(value), unit)
}
