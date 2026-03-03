use crate::components::MembershipPlanItem;
use crate::*;

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
        div { class: "min-h-140 rounded-[10px] bg-card-bg border border-card-border px-4 py-5 flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col gap-5 h-full",
                h3 { class: "text-base md:text-lg lg:text-xl font-semibold text-text-primary",
                    {membership.name}
                }
                p { class: "text-[17px]/[20px] font-semibold text-text-primary",
                    {membership.description}
                }
                div { class: "flex flex-col flex-1 gap-3 text-[15px]/[20px] text-text-primary",
                    for feature in membership.features.iter() {
                        div { class: "flex items-center gap-3",
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
                        button {
                            class: "inline-flex gap-2.5 justify-center items-center py-1.5 px-4 h-auto text-xs font-bold whitespace-nowrap rounded-full transition-all outline-none bg-btn-secondary-bg text-btn-secondary-text border-btn-secondary-outline hover:bg-btn-secondary-hover-bg hover:text-btn-secondary-hover-text",
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
        div {
            class: "min-h-70 rounded-[10px] bg-card-bg border border-card-border px-4 py-5 flex flex-col w-full justify-start items-start",
            style: "grid-column: 1 / -1;",
            div { class: "flex flex-col gap-5 h-full w-full",
                h3 { class: "text-base md:text-lg lg:text-xl font-semibold text-text-primary",
                    {membership.name}
                }
                p { class: "text-[17px]/[20px] font-semibold text-text-primary",
                    {membership.description}
                }
                div { class: "flex flex-col flex-1 gap-3 text-[15px]/[20px] text-text-primary",
                    for feature in membership.features.iter() {
                        div { class: "flex items-center gap-3",
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
                            button {
                                class: "inline-flex gap-2.5 justify-center items-center py-1.5 px-4 h-auto text-xs font-bold whitespace-nowrap rounded-full transition-all outline-none bg-btn-secondary-bg text-btn-secondary-text border-btn-secondary-outline hover:bg-btn-secondary-hover-bg hover:text-btn-secondary-hover-text",
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
