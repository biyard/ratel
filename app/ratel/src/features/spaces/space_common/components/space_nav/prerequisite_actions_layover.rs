use super::*;
use crate::features::spaces::pages::actions::components::ActionCard;
use crate::features::spaces::pages::actions::types::SpaceActionSummary;

#[component]
pub fn PrerequisiteActionsLayover(
    space_id: SpacePartition,
    actions: Vec<SpaceActionSummary>,
) -> Element {
    let tr: PrerequisiteActionsLayoverTranslate = use_translate();
    let mut layover = use_layover();

    rsx! {
        div { class: "flex flex-col w-full h-full bg-neutral-900 light:bg-neutral-200 text-web-font-primary",
            // Header
            div { class: "flex gap-3 items-center py-4 px-5 border-b border-neutral-800 light:border-neutral-300",
                div { class: "flex flex-col gap-1",
                    h4 { class: "font-bold text-[18px]/[22px] text-text-primary", {tr.title} }
                    p { class: "text-[13px]/[18px] text-neutral-400 light:text-neutral-600",
                        {tr.description}
                    }
                }
            }

            // Action list
            div { class: "flex overflow-y-auto flex-col flex-1 gap-2.5 p-5",
                for action in actions.iter() {
                    ActionCard {
                        key: "{action.action_id}",
                        action: action.clone(),
                        space_id: space_id.clone(),
                    }
                }
            }

            // Footer button
            div { class: "flex justify-end items-center py-4 px-5 border-t border-neutral-800 light:border-neutral-300",
                Button {
                    style: ButtonStyle::Primary,
                    size: ButtonSize::Small,
                    onclick: move |_| {
                        layover.close();
                    },
                    {tr.close}
                }
            }
        }
    }
}

translate! {
    PrerequisiteActionsLayoverTranslate;

    title: {
        en: "Required Actions",
        ko: "필수 액션",
    },
    description: {
        en: "Please complete the following actions to fully participate in this space.",
        ko: "이 스페이스에 완전히 참여하려면 다음 액션을 완료해주세요.",
    },
    close: {
        en: "Close",
        ko: "닫기",
    },
}
