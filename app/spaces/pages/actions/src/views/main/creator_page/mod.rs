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

    let open_layover = {
        let mut layover = layover;
        let title = tr.layover_title.to_string();
        let space_id = space_id.clone();
        move |_| {
            layover.open(
                "space-actions-layover".to_string(),
                title.clone(),
                rsx! {
                    CreateActionModal { space_id: space_id.clone() }
                },
            );
        }
    };

    rsx! {
        div {
            id: "creator-action-page",
            class: "flex flex-col gap-5 items-start w-full text-font-primary",

            div { class: "flex flex-col gap-2.5 w-full max-w-[1024px] mx-auto",
                h3 { {tr.title} }

                // Empty state card
                div { class: "flex flex-col gap-5 justify-center items-center py-[0.625rem] pb-5 px-4 w-full border border-dashed rounded-[0.75rem] bg-neutral-900 light:bg-neutral-100 border-neutral-800 light:border-neutral-300",
                    icons::game::Thunder { class: "size-6 [&>path]:fill-none [&>path]:stroke-neutral-400 light:[&>path]:stroke-neutral-500" }
                    p { class: "text-[1.0625rem]/[1.25rem] font-medium text-font-primary",
                        {tr.no_actions_title}
                    }

                    Button {
                        class: "inline-flex gap-2 justify-center items-center bg-white light:bg-white text-neutral-900 hover:opacity-90 transition-opacity",
                        style: ButtonStyle::Secondary,
                        onclick: open_layover,
                        div { class: "flex flex-row w-full justify-center items-center gap-2",
                            icons::validations::AddCircle {
                                width: "24",
                                height: "24",
                                class: "[&>circle]:fill-none [&>circle]:stroke-neutral-900 [&>path]:stroke-neutral-900",
                            }
                            span { {tr.button_add_action_label} }
                        }

                    }

                    p { class: "text-[0.75rem]/[1rem] font-semibold text-neutral-400 light:text-neutral-600 text-center",
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
