use super::*;
use dioxus_primitives::{ContentAlign, ContentSide};
use i18n::CreatorActionPageTranslate;

mod i18n;

#[component]
pub fn CreatorActionPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: CreatorActionPageTranslate = use_translate();
    let mut layover = use_layover();
    let mut actions = use_loader(move || async move { list_actions(space_id()).await })?;

    rsx! {
        div {
            id: "creator-action-page",
            class: "flex flex-col gap-5 items-start w-full text-web-font-primary",

            div { class: "flex flex-col gap-2.5 mx-auto w-full max-w-[1024px]",
                div { class: "flex justify-between items-center w-full max-mobile:flex-col max-mobile:items-stretch max-mobile:gap-3",
                    div { class: "flex items-center gap-2",
                        h3 { {tr.title} }
                        Tooltip {
                            TooltipTrigger {
                                icons::help_support::Info { width: "16", height: "16", class: "h-4 w-4 [&>path]:stroke-text-secondary [&>circle]:fill-text-secondary cursor-help" }
                            }
                            TooltipContent { side: ContentSide::Bottom, align: ContentAlign::Start,
                                {tr.title_tooltip}
                            }
                        }
                    }

                    Button {
                        size: ButtonSize::Medium,
                        style: ButtonStyle::Secondary,
                        shape: ButtonShape::Square,
                        class: "inline-flex border-transparent hover:border-transparent font-raleway max-mobile:w-full hover:bg-web-btn-bg",
                        onclick: move |_| {
                            layover
                                .open(
                                    "space-action-settings-layover".to_string(),
                                    String::new(),
                                    rsx! {
                                        ActionSettingsModal {
                                            space_id: space_id(),
                                            actions: actions(),
                                            on_applied: move |_| {
                                                actions.restart();
                                            },
                                        }
                                    },
                                )
                                .set_size(LayoverSize::Small);
                        },
                        div { class: "flex flex-row gap-2.5 justify-center items-center",
                            icons::settings::Settings2 {
                                width: "16",
                                height: "16",
                                class: "[&>path]:fill-web-font-ab-bk [&>circle]:stroke-web-font-ab-bk [&>circle]:fill-none",
                            }
                            span { {tr.button_settings_label} }
                        }
                    }
                }

                // Empty state card
                SpaceCard {
                    class: "flex flex-col gap-5 justify-center items-center w-full border border-dashed border-neutral-800 light:border-neutral-300 !bg-neutral-900 light:!bg-neutral-100 !rounded-[0.75rem] !px-4 !pt-[0.625rem] !pb-5"
                        .to_string(),
                    icons::game::Thunder { class: "size-6 [&>path]:fill-none [&>path]:stroke-neutral-400 light:[&>path]:stroke-neutral-500" }
                    p { class: "font-medium text-[1.0625rem]/[1.25rem] text-font-primary",
                        {tr.no_actions_title}
                    }

                    Button {
                        style: ButtonStyle::Secondary,
                        onclick: move |_| {
                            layover
                                .open(
                                    "space-actions-layover".to_string(),
                                    tr.layover_title.to_string(),
                                    rsx! {
                                        CreateActionModal { space_id: space_id() }
                                    },
                                )
                                .set_size(LayoverSize::Half);
                        },
                        div { class: "flex flex-row gap-2 justify-center items-center w-full",
                            icons::validations::AddCircle {
                                width: "24",
                                height: "24",
                                class: "[&>circle]:fill-none [&>circle]:stroke-neutral-900 [&>path]:stroke-neutral-900",
                            }
                            span { {tr.button_add_action_label} }
                        }
                    
                    }

                    p { class: "font-semibold text-center text-[0.75rem]/[1rem] text-neutral-400 light:text-neutral-600",
                        {tr.no_actions_description}
                    }
                }

                // Action cards grid
                if !actions.is_empty() {
                    div { class: "grid grid-cols-1 gap-2.5 w-full tablet:grid-cols-2",
                        for action in actions.iter() {
                            ActionCard {
                                key: "{action.action_id}",
                                action: action.clone(),
                                space_id: space_id(),
                            }
                        }
                    }
                }
            }
        
        }
    }
}
