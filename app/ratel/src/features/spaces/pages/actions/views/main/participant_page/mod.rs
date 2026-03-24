use super::*;
use dioxus_primitives::{ContentAlign, ContentSide};
use i18n::ParticipantActionPageTranslate;

mod i18n;

#[component]
pub fn ParticipantPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: ParticipantActionPageTranslate = use_translate();

    let actions = use_loader(move || async move { list_actions(space_id()).await })?;

    rsx! {
        div {
            id: "participant-action-page",
            class: "flex flex-col gap-5 items-start w-full text-web-font-primary",

            div { class: "flex flex-col gap-2.5 mx-auto w-full max-w-[1024px]",
                div { class: "flex items-center gap-2",
                    h3 { {tr.title} }
                    Tooltip {
                        TooltipTrigger {
                            icons::help_support::Info {
                                width: "16",
                                height: "16",
                                class: "h-4 w-4 [&>path]:stroke-text-secondary [&>path]:fill-none [&>circle]:stroke-text-secondary [&>circle]:fill-none cursor-pointer",
                            }
                        }
                        TooltipContent {
                            side: ContentSide::Bottom,
                            align: ContentAlign::Start,
                            {tr.title_tooltip}
                        }
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
