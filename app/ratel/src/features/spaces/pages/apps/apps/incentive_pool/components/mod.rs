use crate::features::spaces::pages::apps::apps::incentive_pool::*;

#[component]
pub(crate) fn SummaryStatCard(
    title: String,
    badge: Option<String>,
    value: String,
    unit: String,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 border rounded-[12px] border-separator bg-card p-[17px]",
            div { class: "flex gap-2 justify-between items-center",
                p { class: "font-bold font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-neutral",
                    "{title}"
                }
                if let Some(badge_text) = badge {
                    p { class: "flex justify-center items-center px-2 font-medium leading-4 border h-[25px] rounded-[100px] border-btn-primary-outline font-raleway text-[12px] text-btn-primary-bg",
                        "{badge_text}"
                    }
                }
            }

            div { class: "flex flex-col gap-1 items-end ml-auto",
                p { class: "font-bold font-raleway text-[36px] leading-[40px] tracking-[-0.72px] text-web-font-primary",
                    "{value}"
                }
                p { class: "font-semibold font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-neutral",
                    "{unit}"
                }
            }
        }
    }
}

#[component]
pub(crate) fn SectionCard(
    title: String,
    title_class: &'static str,
    body_class: &'static str,
    children: Element,
) -> Element {
    rsx! {
        div { class: "w-full rounded-[12px] bg-card",
            div { class: "flex justify-between items-center self-stretch px-5 py-4 border-b rounded-t-[12px] border-separator bg-card",
                p { class: "{title_class}", "{title}" }
            }
            div { class: "{body_class}", {children} }
        }
    }
}

#[component]
pub(crate) fn IconActionButton(
    disabled: bool,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx! {
        button {
            class: "flex justify-center items-center w-11 h-11 border shrink-0 rounded-[8px] border-btn-outline-outline bg-btn-outline-bg text-btn-outline-text disabled:opacity-50",
            disabled,
            onclick: move |evt| onclick.call(evt),
            {children}
        }
    }
}

#[component]
pub(crate) fn DistributionModeCard(
    selected: bool,
    title: String,
    description: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let card_class = if selected {
        "flex w-full shrink-0 flex-col items-start gap-1 self-stretch rounded-[12px] border border-btn-primary-outline bg-btn-primary-bg/5 p-[17px] text-left"
    } else {
        "flex w-full shrink-0 flex-col items-start gap-1 self-stretch rounded-[12px] border border-separator bg-transparent p-[17px] text-left"
    };

    rsx! {
        button { class: "{card_class}", onclick: move |evt| onclick.call(evt),
            p { class: "font-bold font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-primary",
                "{title}"
            }
            p { class: "w-full font-medium leading-5 font-raleway text-[13px] tracking-[0] text-card-meta",
                "{description}"
            }
        }
    }
}
