use crate::components::{ActionCard, CreateActionModal};
use crate::controllers::list_actions;
use crate::*;
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
            );
        }
    };

    rsx! {
        div {
            id: "creator-action-page",
            class: "flex flex-col gap-5 items-start w-full text-web-font-primary",

            div { class: "flex flex-col gap-2.5 mx-auto w-full max-w-[1024px]",
                // Empty state card
                div { class: "flex flex-col gap-5 justify-center items-center px-4 pb-5 w-full border border-dashed py-[0.625rem] rounded-[0.75rem] bg-neutral-900 light:bg-neutral-100 border-neutral-800 light:border-neutral-300",
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
