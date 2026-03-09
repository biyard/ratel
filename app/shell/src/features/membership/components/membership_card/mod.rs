use super::*;
use crate::features::membership::*;
use common::components::{Button, ButtonStyle};
use crate::features::membership::components::*;

#[component]
pub fn MembershipCard(
    membership: MembershipPlanItem,
    variant: &'static str,
    on_click: EventHandler<()>,
) -> Element {
    if variant == "horizontal" {
        return rsx! {
            MembershipHorizontalCard { membership, on_click }
        };
    }

    rsx! {
        MembershipVerticalCard { membership, on_click }
    }
}

#[component]
fn MembershipVerticalCard(membership: MembershipPlanItem, on_click: EventHandler<()>) -> Element {
    rsx! {
        Card { class: "flex flex-col justify-start items-start w-full min-h-140",
            div { class: "flex flex-col gap-5 h-full",
                h3 { class: "text-base font-semibold md:text-lg lg:text-xl text-text-primary",
                    {membership.name}
                }
                p { class: "font-semibold text-[17px]/[20px] text-text-primary",
                    {membership.description}
                }
                div { class: "flex flex-col flex-1 gap-3 text-[15px]/[20px] text-text-primary",
                    for feature in membership.features.iter() {
                        div {
                            key: "{feature}",
                            class: "flex gap-3 items-center",
                            icons::validations::Check { class: "w-[13px] h-[13px] min-w-[13px] [&>path]:stroke-primary" }
                            "{feature}"
                        }
                    }
                }
                if let Some(price) = membership.price {
                    p { class: "font-semibold text-[17px]/[20px] text-text-primary",
                        {price}
                    }
                }
                if let Some(btn) = membership.btn {
                    div { class: "flex justify-end w-full",
                        Button {
                            style: ButtonStyle::Secondary,
                            onclick: move |_| {
                                on_click.call(());
                            },
                            {btn}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MembershipHorizontalCard(membership: MembershipPlanItem, on_click: EventHandler<()>) -> Element {
    rsx! {
        Card { class: "flex flex-col justify-start items-start w-full min-h-70 membership-plan-span-full",
            div { class: "flex flex-col gap-5 w-full h-full",
                h3 { class: "text-base font-semibold md:text-lg lg:text-xl text-text-primary",
                    {membership.name}
                }
                p { class: "font-semibold text-[17px]/[20px] text-text-primary",
                    {membership.description}
                }
                div { class: "flex flex-col flex-1 gap-3 text-[15px]/[20px] text-text-primary",
                    for feature in membership.features.iter() {
                        div {
                            key: "{feature}",
                            class: "flex gap-3 items-center",
                            icons::validations::Check { class: "w-[13px] h-[13px] min-w-[13px] [&>path]:stroke-primary" }
                            "{feature}"
                        }
                    }
                }
                if let Some(price) = membership.price {
                    div { class: "flex justify-between items-center w-full",
                        p { class: "font-semibold text-[17px]/[20px] text-text-primary",
                            {price}
                        }
                        if let Some(btn) = membership.btn {
                            Button {
                                style: ButtonStyle::Secondary,
                                onclick: move |_| {
                                    on_click.call(());
                                },
                                {btn}
                            }
                        }
                    }
                }
            }
        }
    }
}
