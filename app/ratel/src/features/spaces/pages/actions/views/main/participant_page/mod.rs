use super::*;
use dioxus_primitives::{ContentAlign, ContentSide};
use i18n::ParticipantActionPageTranslate;

mod i18n;

#[component]
pub fn ParticipantPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: ParticipantActionPageTranslate = use_translate();

    rsx! {
        div {
            id: "participant-action-page",
            class: "flex flex-col gap-5 items-start w-full text-web-font-primary",

            div { class: "flex flex-col gap-4 mx-auto w-full max-w-[1024px]",
                // Header: title + info tooltip
                div { class: "flex items-center gap-2",
                    h3 { {tr.title} }
                    Tooltip {
                        TooltipTrigger {
                            icons::help_support::Info {
                                width: "16",
                                height: "16",
                                class: "h-4 w-4 [&>path]:stroke-text-secondary [&>path]:fill-none [&>circle]:stroke-text-secondary [&>circle]:fill-current cursor-pointer",
                            }
                        }
                        TooltipContent {
                            side: ContentSide::Bottom,
                            align: ContentAlign::Start,
                            {tr.title_tooltip}
                        }
                    }
                }

                // Quest Map (Phase 4)
                QuestMap { space_id }
            }
        }
    }
}
