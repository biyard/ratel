use super::creator_page::*;
use crate::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DistributionMode {
    Top10RankOnly,
    HighScoreRandom,
    Mix,
}

#[component]
pub fn IncentivePoolContent(space_id: SpacePartition) -> Element {
    // FIXME: Use space_id when space-scoped data is added.
    let _ = space_id;
    let mut distribution_mode = use_signal(|| DistributionMode::Top10RankOnly);

    rsx! {
        div { class: "flex overflow-visible flex-col gap-5 self-start pb-6 w-full min-w-0 shrink-0 max-w-[1024px] max-tablet:gap-4 text-font-primary",
            h3 { class: "font-bold sp-dash-font-raleway text-[24px]/[28px] tracking-[-0.24px] text-font-primary",
                "Incentive Pool"
            }

            div { class: "p-4 w-full rounded-[12px] bg-card max-mobile:p-3",
                div { class: "flex gap-3 justify-between items-start",
                    div { class: "flex flex-col items-start min-w-0 gap-[10px]",
                        div { class: "flex justify-center items-center w-11 h-11 bg-violet-500 rounded-[10px]",
                            icons::ratel::Chest {
                                width: "24",
                                height: "24",
                                class: "text-font-primary [&>path]:fill-none [&>path]:stroke-current",
                            }
                        }
                        p { class: "font-bold leading-5 sp-dash-font-raleway text-[17px] tracking-[-0.18px] text-font-primary",
                            "Incentive Pool"
                        }
                        p { class: "w-full font-medium leading-4 sp-dash-font-raleway text-[12px] tracking-[0] text-card-meta",
                            "This reward pool runs on the Kaia mainnet and is funded by depositing USDT into the deployed reward contract. Rewards are distributed to eligible participants based on verified engagement and accumulated score. Higher scores increase reward probability, but distribution follows the predefined allocation rules."
                        }
                    }

                    button { class: "flex justify-center items-center w-6 h-6 text-web-font-neutral",
                        icons::validations::Extra {
                            width: "20",
                            height: "20",
                            class: "[&>circle]:fill-current",
                        }
                    }
                }
            }

            SectionCard {
                title: "Incentive Pool",
                title_class: "font-bold sp-dash-font-raleway text-[24px]/[28px] tracking-[-0.24px] text-font-primary",
                body_class: "flex flex-col gap-4 p-5 bg-card max-mobile:p-4",

                div { class: "flex flex-col gap-2 items-start",
                    p { class: "font-bold leading-5 sp-dash-font-raleway text-[17px] tracking-[-0.18px] text-font-primary",
                        "Incentive Pool Address"
                    }
                    div { class: "flex gap-2 items-center w-full max-tablet:flex-wrap",
                        div { class: "flex flex-1 items-center w-full min-w-0 h-11 border-gray-600 rounded-[8px] border-[0.5px] bg-web-input",
                            input {
                                class: "flex-1 px-3 min-w-0 h-full font-medium bg-transparent outline-none sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-font-primary",
                                value: "0xFdB7eDe7B3d5F9538315e163368Cd15a731A7A15",
                                readonly: true,
                            }

                            button { class: "flex justify-center items-center w-11 h-full shrink-0 rounded-r-[8px] text-web-font-neutral",
                                icons::arrows::ExpandPage {
                                    width: "20",
                                    height: "20",
                                    class: "[&>path]:stroke-current",
                                }
                            }
                        }

                        IconActionButton {
                            icons::notes_clipboard::Clipboard {
                                width: "24",
                                height: "24",
                                class: "[&>path]:stroke-current",
                            }
                        }
                        IconActionButton {
                            icons::arrows::Repost {
                                width: "24",
                                height: "24",
                                class: "[&>path]:stroke-current",
                            }
                        }
                    }
                }

                div { class: "grid grid-cols-2 w-full gap-[10px] max-tablet:grid-cols-1",
                    div { class: "flex flex-col gap-2 border rounded-[12px] border-separator bg-card p-[17px]",
                        div { class: "flex gap-2 justify-between items-center",
                            p { class: "font-bold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-neutral",
                                "Total Winners"
                            }
                            p { class: "flex justify-center items-center px-2 font-medium leading-4 border h-[25px] rounded-[100px] border-btn-primary-outline sp-dash-font-raleway text-[12px] text-btn-primary-bg",
                                "Rank Rate"
                            }
                        }

                        div { class: "flex flex-col gap-1 items-end ml-auto",
                            p { class: "font-bold sp-dash-font-raleway text-[36px] leading-[40px] tracking-[-0.72px] text-font-primary",
                                "10"
                            }
                            p { class: "font-semibold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-neutral",
                                "Peoples"
                            }
                        }
                    }

                    div { class: "flex flex-col gap-2 border rounded-[12px] border-separator bg-card p-[17px]",
                        p { class: "font-bold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-neutral",
                            "Total Deposit Amount"
                        }
                        div { class: "flex flex-col gap-1 items-end ml-auto",
                            p { class: "font-bold sp-dash-font-raleway text-[36px] leading-[40px] tracking-[-0.72px] text-font-primary",
                                "1,245"
                            }
                            p { class: "font-semibold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-neutral",
                                "USDT"
                            }
                        }
                    }
                }
            }

            SectionCard {
                title: "Deposit in Incentive Pool",
                title_class: "font-bold sp-dash-font-raleway text-[24px]/[28px] tracking-[-0.24px] text-font-primary",
                body_class: "flex flex-col items-start p-5 gap-[10px] bg-card max-mobile:p-4",

                div { class: "flex flex-col gap-2 justify-center items-start w-full",
                    p { class: "font-bold leading-5 sp-dash-font-raleway text-[17px] tracking-[-0.18px] text-font-primary",
                        "Incentive Token"
                    }
                    button { class: "flex justify-between items-center px-3 w-full h-11 border-gray-600 rounded-[8px] border-[0.5px] bg-web-input",
                        span { class: "font-medium sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-web-font-neutral",
                            "Select"
                        }
                        icons::arrows::ChevronDown {
                            width: "24",
                            height: "24",
                            class: "text-web-font-neutral [&>path]:stroke-current",
                        }
                    }
                    p { class: "font-normal text-gray-500 sp-dash-font-inter text-[12px] leading-[16px]",
                        "Token used for incentives distribution"
                    }
                }

                div { class: "flex flex-col gap-2 justify-center items-start w-full",
                    p { class: "font-bold leading-5 sp-dash-font-raleway text-[17px] tracking-[-0.18px] text-font-primary",
                        "Deposit Amount"
                    }
                    div { class: "relative w-full",
                        input {
                            class: "px-3 w-full h-12 font-medium text-right border-gray-600 rounded-[8px] border-[0.5px] bg-web-input pr-[68px] sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-font-primary",
                            value: "",
                            placeholder: "",
                        }
                        span { class: "absolute right-3 top-1/2 font-normal text-gray-500 -translate-y-1/2 pointer-events-none sp-dash-font-inter text-[12px] leading-[16px]",
                            "00.00"
                        }
                    }
                    p { class: "font-normal text-gray-500 sp-dash-font-inter text-[12px] leading-[16px]",
                        "It sends tokens to the generated incentive pool address."
                    }
                }
            }

            SectionCard {
                title: "Distribution",
                title_class: "font-semibold leading-5 text-center sp-dash-font-raleway text-[17px] tracking-[-0.18px] text-font-primary",
                body_class: "flex flex-col items-start p-5 gap-[10px] bg-card max-mobile:p-4",

                div { class: "flex flex-col gap-2 items-start w-full",
                    p { class: "font-semibold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-font-primary",
                        "Weight Sampling"
                    }

                    div { class: "grid grid-cols-3 gap-2 w-full max-tablet:grid-cols-1",
                        DistributionModeCard {
                            selected: distribution_mode() == DistributionMode::Top10RankOnly,
                            title: "Top 10 Rank Only".to_string(),
                            description: "Rank 100%".to_string(),
                            onclick: move |_| distribution_mode.set(DistributionMode::Top10RankOnly),
                        }
                        DistributionModeCard {
                            selected: distribution_mode() == DistributionMode::HighScoreRandom,
                            title: "High Score Random".to_string(),
                            description: "Ranking Lottery".to_string(),
                            onclick: move |_| distribution_mode.set(DistributionMode::HighScoreRandom),
                        }
                        DistributionModeCard {
                            selected: distribution_mode() == DistributionMode::Mix,
                            title: "Mix".to_string(),
                            description: "Rank 70% : Random 30%".to_string(),
                            onclick: move |_| distribution_mode.set(DistributionMode::Mix),
                        }
                    }
                }

                div { class: "flex gap-5 items-start p-5 w-full border border-gray-600 h-[102px] rounded-[12px] bg-neutral-800 max-tablet:h-auto",
                    icons::security::ShieldLock {
                        width: "18",
                        height: "18",
                        class: "mt-0.5 shrink-0 text-card-meta [&>path]:stroke-current",
                    }

                    div { class: "flex flex-col flex-1 gap-0.5 items-start min-w-0",
                        p { class: "w-full font-semibold leading-5 sp-dash-font-raleway text-[17px] tracking-[-0.18px] text-font-primary",
                            "Mainnet & funding"
                        }
                        p { class: "w-full font-medium leading-5 sp-dash-font-raleway text-[13px] tracking-[0] text-card-meta",
                            "· Kaia / ICP networks supported. Funding is external token send only."
                        }
                        p { class: "w-full font-medium leading-5 sp-dash-font-raleway text-[13px] tracking-[0] text-card-meta",
                            "· Ratel does not pull funds automatically, you send tokens to the generated treasury address."
                        }
                    }
                }

                div { class: "flex justify-end pt-3 w-full max-tablet:justify-stretch",
                    Button {
                        style: ButtonStyle::Secondary,
                        class: "font-semibold leading-6 w-[146px] max-tablet:w-full rounded-[10px] sp-dash-font-raleway text-[15px] tracking-[0.5px]",
                        "Confirm Setup"
                    }
                }
            }
        }
    }
}

#[component]
fn SectionCard(
    title: &'static str,
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
fn IconActionButton(children: Element) -> Element {
    rsx! {
        button { class: "flex justify-center items-center w-11 h-11 border shrink-0 rounded-[8px] border-btn-outline-outline bg-btn-outline-bg text-btn-outline-text",
            {children}
        }
    }
}

#[component]
fn DistributionModeCard(
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
            p { class: "font-bold sp-dash-font-raleway text-[15px] leading-[18px] tracking-[-0.16px] text-font-primary",
                "{title}"
            }
            p { class: "w-full font-medium leading-5 sp-dash-font-raleway text-[13px] tracking-[0] text-card-meta",
                "{description}"
            }
        }
    }
}

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role =
        use_loader(move || async move { Ok::<SpaceUserRole, Error>(SpaceUserRole::Creator) })?;

    if role() == SpaceUserRole::Creator {
        rsx! {
            CreatorPage { space_id }
        }
    } else {
        rsx! {
            div { class: "flex justify-center items-center w-full h-full text-font-primary",
                "No permission"
            }
        }
    }
}
