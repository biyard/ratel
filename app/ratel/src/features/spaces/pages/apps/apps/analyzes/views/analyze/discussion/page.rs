use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::space_common::hooks::use_space_role;

#[component]
pub fn SpaceAnalyzeDiscussionPage(
    space_id: ReadSignal<SpacePartition>,
    discussion_id: ReadSignal<SpacePostEntityType>,
) -> Element {
    let _ = space_id;
    let _ = discussion_id;
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let role = use_space_role()();

    if role != SpaceUserRole::Creator {
        return rsx! {};
    }

    rsx! {
        div { class: "flex w-full flex-col gap-5",
            h3 { class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-web-font-primary",
                {tr.discussion_section_title}
            }
            Card { class: "flex min-h-[320px] w-full items-center justify-center".to_string(),
                div { class: "text-center text-lg font-semibold text-text-secondary",
                    {tr.to_be_continue}
                }
            }
        }
    }
}
