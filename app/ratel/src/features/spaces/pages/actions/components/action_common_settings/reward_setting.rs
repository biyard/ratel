use super::super::*;
use dioxus_primitives::{ContentAlign, ContentSide};

#[cfg(feature = "membership")]
use crate::features::auth::hooks::use_user_membership;

#[component]
pub fn RewardSetting(
    space_action: ReadSignal<SpaceAction>,
    #[props(default)] on_change: EventHandler<u64>,
) -> Element {
    let tr: RewardSettingTranslate = use_translate();
    let mut enable_reward = use_signal(move || space_action().credits > 0);
    let mut credits = use_signal(move || space_action().credits);

    #[cfg(feature = "membership")]
    let membership = use_user_membership();
    #[cfg(not(feature = "membership"))]
    let membership: Option<()> = None;

    #[cfg(feature = "membership")]
    let is_paid = membership.as_ref().map_or(false, |m| m.is_paid());
    #[cfg(not(feature = "membership"))]
    let is_paid = false;

    #[cfg(feature = "membership")]
    let max_credits = membership
        .as_ref()
        .map_or(0, |m| m.max_credits_per_space as u64);
    #[cfg(not(feature = "membership"))]
    let max_credits = 0u64;

    #[cfg(feature = "membership")]
    let remaining_credits = membership
        .as_ref()
        .map_or(0, |m| m.remaining_credits as u64);
    #[cfg(not(feature = "membership"))]
    let remaining_credits = 0u64;

    let boost_multiplier = credits();
    let total_reward = credits() * 10_000;

    let label_class =
        "font-semibold font-raleway text-[13px]/[16px] tracking-[-0.14px] text-web-font-neutral";

    rsx! {
        Collapsible { open: enable_reward(),
            CollapsibleTrigger {
                r#as: move |_attrs: Vec<Attribute>| {
                    rsx! {
                        Card {
                            direction: CardDirection::Row,
                            main_axis_align: MainAxisAlign::Between,
                            cross_axis_align: CrossAxisAlign::Center,
                            class: if enable_reward() { "!rounded-b-none" } else { "" },
                            div { class: "flex gap-1 items-center",
                                p { class: "font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-primary",
                                    {tr.reward_setting}
                                }
                                Tooltip {
                                    TooltipTrigger {
                                        icons::help_support::Info {
                                            width: "14",
                                            height: "14",
                                            class: "cursor-help text-web-font-neutral [&>path]:stroke-current [&>circle]:fill-current [&>path]:fill-none",
                                        }
                                    }
                                    TooltipContent { side: ContentSide::Bottom, align: ContentAlign::Start,
                                        {tr.reward_setting_tooltip}
                                    }
                                }
                            }
                            if is_paid {
                                Switch {
                                    active: enable_reward(),
                                    on_toggle: move |_| {
                                        let new_enabled = !enable_reward();
                                        enable_reward.set(new_enabled);
                                        let new_credits = if new_enabled { 1 } else { 0 };
                                        credits.set(new_credits);
                                        on_change.call(new_credits);
                                    },
                                }
                            } else {
                                UnlockButton {}
                            }
                        }
                    }
                },
            }
            CollapsibleContent {
                Card { class: "gap-1.5 w-full rounded-t-none!",
                    div { class: "flex flex-col gap-5 w-full tablet:flex-row tablet:gap-5",
                        // Default Reward column
                        div { class: "flex flex-col flex-1 gap-2.5",
                            p { class: label_class, {tr.default_reward} }
                            Card {
                                direction: CardDirection::Row,
                                main_axis_align: MainAxisAlign::Between,
                                cross_axis_align: CrossAxisAlign::Center,
                                variant: CardVariant::Outlined,
                                class: "gap-2 w-full",

                                div { class: "flex flex-col gap-3 justify-between items-start",
                                    div { class: "flex justify-center items-center text-white bg-green-500 size-11 shrink-0 rounded-[10px]",
                                        icons::ratel::Clock {
                                            width: "24",
                                            height: "24",
                                            class: "[&>circle]:fill-none",
                                        }
                                    }

                                    span { class: "font-medium font-inter text-[12px]/[16px] text-web-font-primary",
                                        "x {boost_multiplier} {tr.boost_arrow}"
                                    }
                                }

                                div { class: "flex flex-col justify-between h-full text-right items-between",
                                    p { class: "w-full font-bold font-inter text-[24px]/[24px] text-web-font-primary",
                                        {format_number(total_reward)}
                                    }
                                    p { class: "w-full font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-neutral",
                                        {tr.action_point_reward}
                                    }
                                }
                            }
                        }

                        // Boost Settings column
                        div { class: "flex flex-col flex-1 gap-2.5",
                            p { class: label_class, {tr.boost_multiplier_settings} }
                            Card { class: "gap-3",
                                div { class: "flex gap-3 justify-between items-center w-full",
                                    p { class: label_class, {tr.credit_usage} }
                                    Input {
                                        r#type: InputType::Number,
                                        class: "font-semibold text-right !w-[150px] font-raleway text-[15px]/[18px] tracking-[-0.16px]",
                                        value: "{credits()}",
                                        oninput: move |evt: FormEvent| {
                                            let val = evt.value().parse::<u64>().unwrap_or(0);
                                            let clamped = if max_credits > 0 { val.min(max_credits) } else { val };
                                            credits.set(clamped);
                                            on_change.call(clamped);
                                        },
                                    }
                                }
                                if max_credits > 0 {
                                    div { class: "flex gap-3 justify-between items-center w-full",
                                        div { class: "flex gap-1 items-center",
                                            p { class: label_class, {tr.max_credits} }
                                            Tooltip {
                                                TooltipTrigger {
                                                    icons::help_support::Info {
                                                        width: "14",
                                                        height: "14",
                                                        class: "cursor-help text-web-font-neutral [&>path]:stroke-current [&>circle]:fill-current [&>path]:fill-none",
                                                    }
                                                }
                                                TooltipContent {
                                                    side: ContentSide::Bottom,
                                                    align: ContentAlign::Start,
                                                    {tr.max_credits_tooltip}
                                                }
                                            }
                                        }
                                        p { class: "font-semibold text-web-font-neutral font-raleway text-[15px]/[18px] tracking-[-0.16px]",
                                            {format_number(max_credits)}
                                        }
                                    }
                                }
                                div { class: "flex gap-3 justify-between items-center w-full",
                                    p { class: label_class, {tr.remaining_credits} }
                                    p { class: "font-semibold text-green-500 font-raleway text-[15px]/[18px] tracking-[-0.16px]",
                                        {format_number(remaining_credits)}
                                    }
                                }
                            }
                        }
                    }

                    // Info notice box
                    div { class: "flex gap-5 items-start p-5 w-full border rounded-[12px] border-web-card-stroke3 bg-web-card-bg2",
                        icons::help_support::Info {
                            width: "18",
                            height: "18",
                            class: "mt-0.5 shrink-0 text-web-font-neutral [&>path]:stroke-current [&>circle]:fill-current [&>path]:fill-none",
                        }
                        div { class: "flex flex-col flex-1 gap-2 min-w-0",
                            p { class: "font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-primary",
                                {tr.point_boost}
                            }
                            ul { class: "pl-4 font-medium list-disc text-[13px]/[20px] font-raleway text-web-font-body",
                                li { {tr.point_boost_line_one} }
                                li { {tr.point_boost_line_two} }
                                li { {tr.point_boost_line_three} }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn UnlockButton() -> Element {
    let tr: UnlockButtonTranslate = use_translate();
    let nav = use_navigator();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let username = user_ctx
        .read()
        .user
        .as_ref()
        .map(|u| u.username.clone())
        .unwrap_or_default();

    rsx! {
        button {
            class: "inline-flex gap-1.5 items-center py-1.5 px-3 text-xs font-semibold rounded-full transition-all bg-primary/10 text-primary hover:bg-primary/20",
            onclick: move |_| {
                let username = username.clone();
                // FIXME: unlock with modal
                nav.push(format!("/{username}/memberships"));
            },
            icons::security::Lock1 { width: "14", height: "14", class: "[&>path]:stroke-current" }
            {tr.unlock}
        }
    }
}

use crate::*;

fn format_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

translate! {
    RewardSettingTranslate;

    reward_setting: {
        en: "Reward",
        ko: "보상",
    },
    reward_setting_tooltip: {
        en: "Set Ratel Reward points for participants who complete this action. Use boost credits to multiply the base reward.",
        ko: "이 액션을 완료한 참여자에게 지급할 Ratel 보상 포인트를 설정하세요. 부스트 크레딧을 사용하면 기본 보상을 배율로 늘릴 수 있습니다.",
    },
    default_reward: {
        en: "Default Reward",
        ko: "기본 보상",
    },
    action_point_reward: {
        en: "Action Point Reward",
        ko: "액션 포인트 보상",
    },
    boost_arrow: {
        en: "Boost →",
        ko: "부스트 →",
    },
    boost_prefix: {
        en: "Boost",
        ko: "부스트",
    },
    boost_multiplier_settings: {
        en: "Boost Multiplier Settings",
        ko: "부스트 배율 설정",
    },
    credit_usage: {
        en: "Credit Usage",
        ko: "크레딧 사용량",
    },
    max_credits: {
        en: "Max Credits",
        ko: "최대 크레딧",
    },
    max_credits_tooltip: {
        en: "Upgrade your membership to increase max credits per space.",
        ko: "멤버십을 업그레이드하면 스페이스당 최대 크레딧이 증가합니다.",
    },
    remaining_credits: {
        en: "Remaining Credits",
        ko: "잔여 크레딧",
    },
    point_boost: {
        en: "Point & Boost",
        ko: "포인트 & 부스트",
    },
    point_boost_line_one: {
        en: "Points are rewarded to participants when they complete the action.",
        ko: "포인트는 참여자가 액션을 완료하면 지급됩니다.",
    },
    point_boost_line_two: {
        en: "Boost multiplies the base reward points by the set multiplier.",
        ko: "부스트는 기본 보상 포인트에 설정된 배율을 곱합니다.",
    },
    point_boost_line_three: {
        en: "Credits are consumed when boost is applied to an action.",
        ko: "액션에 부스트를 적용하면 크레딧이 소모됩니다.",
    },
}

translate! {
    UnlockButtonTranslate;

    unlock: {
        en: "Unlock",
        ko: "잠금 해제",
    },
}
