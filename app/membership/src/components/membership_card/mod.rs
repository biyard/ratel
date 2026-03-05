use crate::components::*;
use crate::*;
use common::components::{Button, ButtonStyle};

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
        Card { class: "flex flex-col w-full justify-start items-start min-h-140",
            div { class: "flex flex-col gap-5 h-full",
                h3 { class: "text-base md:text-lg lg:text-xl font-semibold text-text-primary",
                    {membership.name}
                }
                p { class: "text-[17px]/[20px] font-semibold text-text-primary",
                    {membership.description}
                }
                div { class: "flex flex-col flex-1 gap-3 text-[15px]/[20px] text-text-primary",
                    for feature in membership.features.iter() {
                        div {
                            key: "{feature}",
                            class: "flex items-center gap-3",
                            icons::validations::Check { class: "w-[13px] h-[13px] min-w-[13px] [&>path]:stroke-primary" }
                            "{feature}"
                        }
                    }
                }
                if let Some(price) = membership.price {
                    p { class: "text-[17px]/[20px] font-semibold text-text-primary",
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
        Card { class: "min-h-70 flex flex-col w-full justify-start items-start membership-plan-span-full",
            div { class: "flex flex-col gap-5 h-full w-full",
                h3 { class: "text-base md:text-lg lg:text-xl font-semibold text-text-primary",
                    {membership.name}
                }
                p { class: "text-[17px]/[20px] font-semibold text-text-primary",
                    {membership.description}
                }
                div { class: "flex flex-col flex-1 gap-3 text-[15px]/[20px] text-text-primary",
                    for feature in membership.features.iter() {
                        div {
                            key: "{feature}",
                            class: "flex items-center gap-3",
                            icons::validations::Check { class: "w-[13px] h-[13px] min-w-[13px] [&>path]:stroke-primary" }
                            "{feature}"
                        }
                    }
                }
                if let Some(price) = membership.price {
                    div { class: "flex justify-between items-center w-full",
                        p { class: "text-[17px]/[20px] font-semibold text-text-primary",
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
