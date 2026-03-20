use super::super::*;
use dioxus_primitives::{ContentAlign, ContentSide};

#[component]
pub fn PrerequisiteSetting(
    space_id: ReadSignal<SpacePartition>,
    action_id: ReadSignal<String>,
    action_setting: ReadSignal<SpaceAction>,
    #[props(default)] on_change: EventHandler<bool>,
) -> Element {
    let tr: PrerequisiteSettingTranslate = use_translate();
    let mut toast = crate::common::use_toast();
    let mut prerequisite = use_signal(move || action_setting().prerequisite);

    rsx! {
        Card {
            direction: CardDirection::Row,
            main_axis_align: MainAxisAlign::Between,
            cross_axis_align: CrossAxisAlign::Center,
            div { class: "flex gap-1 items-center",
                p { class: "font-semibold font-raleway text-[15px]/[18px] tracking-[-0.16px] text-web-font-primary",
                    {tr.prerequisite}
                }
                Tooltip {
                    TooltipTrigger {
                        icons::help_support::Info {
                            width: "14",
                            height: "14",
                            class: "cursor-help text-web-font-neutral [&>path]:stroke-current [&>circle]:fill-current [&>path]:fill-none",
                        }
                    }
                    TooltipContent { side: ContentSide::Bottom, align: ContentAlign::Start,
                        {tr.prerequisite_tooltip}
                    }
                }
            }
            Switch {
                active: prerequisite(),
                on_toggle: move |_| async move {
                    let new_val = !prerequisite();
                    let req = UpdateSpaceActionRequest::Prerequisite {
                        prerequisite: new_val,
                    };
                    match update_space_action(space_id(), action_id(), req).await {
                        Ok(_) => {
                            prerequisite.set(new_val);
                            toast.info(tr.prerequisite_updated.to_string());
                            on_change.call(new_val);
                        }
                        Err(e) => {
                            toast.error(e);
                        }
                    }
                },
            }
        }
    }
}

translate! {
    PrerequisiteSettingTranslate;

    prerequisite: {
        en: "Prerequisite",
        ko: "필수 참여",
    },
    prerequisite_tooltip: {
        en: "When enabled, participants must complete this action with requesting space participation",
        ko: "활성화하면 참여자가 스페이스에 참여신청 할 때, 이 액션을 먼저 완료해야 합니다.",
    },
    prerequisite_updated: {
        en: "Prerequisite setting updated.",
        ko: "필수 참여 설정이 업데이트되었습니다.",
    },
}
