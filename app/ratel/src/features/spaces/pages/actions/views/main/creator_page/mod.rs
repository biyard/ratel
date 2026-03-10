use crate::features::spaces::pages::actions::components::{ActionCard, ActionSettingsModal, CreateActionModal};
use crate::features::spaces::pages::actions::controllers::list_actions;
use crate::features::spaces::pages::actions::*;
use i18n::CreatorActionPageTranslate;

mod i18n;

#[component]
pub fn CreatorActionPage(space_id: SpacePartition) -> Element {
    let tr: CreatorActionPageTranslate = use_translate();

    let layover = use_layover();

    let actions = use_loader({
        let space_id = space_id.clone();
        move || list_actions(space_id.clone())
    })?;

    let actions_for_subscription = actions.clone();
    let has_subscription = use_memo(move || {
        actions_for_subscription
            .iter()
            .any(|action| action.action_type == SpaceActionType::Subscription)
    });

    let on_settings_applied: EventHandler<()> = Callback::new({
        let mut actions = actions.clone();
        move |_| {
            actions.restart();
        }
    });

    let open_layover = {
        let mut layover = layover;
        let title = tr.layover_title.to_string();
        let space_id = space_id.clone();
        let has_subscription = has_subscription();
        move |_| {
            layover.open(
                "space-actions-layover".to_string(),
                title.clone(),
                rsx! {
                    CreateActionModal { space_id: space_id.clone(), has_subscription }
                },
                None,
            );
        }
    };

    let open_settings_layover = {
        let mut layover = layover;
        let actions = actions.clone();
        let space_id = space_id.clone();
        let on_applied = on_settings_applied.clone();
        move |_| {
            layover.open(
                "space-action-settings-layover".to_string(),
                String::new(),
                rsx! {
                    ActionSettingsModal {
                        space_id: space_id.clone(),
                        actions: actions().clone(),
                        on_applied: on_applied.clone(),
                    }
                },
                Some("!max-w-[337px] rounded-none border-l border-neutral-800 light:border-neutral-300 shadow-[0_8px_20px_0_rgba(20,26,62,0.25)] max-tablet:!max-w-full".to_string()),
            );
        }
    };

    rsx! {
        div {
            id: "creator-action-page",
            class: "flex w-full flex-col items-start gap-5 text-web-font-primary",

            div { class: "flex flex-col gap-2.5 mx-auto w-full max-w-[1024px]",
                div { class: "flex justify-between items-center w-full max-mobile:flex-col max-mobile:items-stretch max-mobile:gap-3",
                    h3 { {tr.title} }

                    Button {
                        size: ButtonSize::Medium,
                        style: ButtonStyle::Secondary,
                        shape: ButtonShape::Square,
                        class: "inline-flex border-transparent font-raleway hover:bg-web-btn-bg hover:border-transparent max-mobile:w-full",
                        onclick: open_settings_layover,
                        div { class: "flex flex-row gap-2.5 justify-center items-center",
                            icons::settings::Settings2 {
                                width: "16",
                                height: "16",
                                class: "[&>path]:fill-web-font-ab-bk [&>circle]:stroke-web-font-ab-bk",
                            }
                            span { {tr.button_settings_label} }
                        }
                    }
                }

                // Empty state card
                SpaceCard { class: "flex w-full flex-col items-center justify-center gap-5 border border-dashed border-neutral-800 light:border-neutral-300 !bg-neutral-900 light:!bg-neutral-100 !rounded-[0.75rem] !px-4 !pt-[0.625rem] !pb-5".to_string(),
                    icons::game::Thunder { class: "size-6 [&>path]:fill-none [&>path]:stroke-neutral-400 light:[&>path]:stroke-neutral-500" }
                    p { class: "font-medium text-[1.0625rem]/[1.25rem] text-font-primary",
                        {tr.no_actions_title}
                    }

                    Button {
                        //
                        style: ButtonStyle::Secondary,
                        onclick: open_layover,
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
                                space_id: space_id.clone(),
                            }
                        }
                    }
                }
            }

        }
    }
}
