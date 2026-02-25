use common::models::space::SpaceCommon;

use crate::{components::SpaceStatusBadge, *};

#[derive(Clone, PartialEq)]
pub struct SpaceTopLabel {
    pub label: String,
    pub link: Option<NavigationTarget>,
}
#[component]
pub fn SpaceTop(
    labels: Vec<SpaceTopLabel>,
    space_status: Option<SpaceStatus>,
    show_participate_button: bool,
    on_participant: Option<EventHandler<()>>,
) -> Element {
    let tr: SpaceTopTranslates = use_translate();
    // let space_pk = space_id.clone();
    // let mut space = use_loader(move || get_space(space_id.clone()))?;
    // let space_data = space();

    //FIXME: Rotate Labels
    let title = labels.first().unwrap().label.clone();

    rsx! {
        div { class: "flex flex-row justify-between items-center px-[12px] py-[17.5px] min-h-[65px]",
            div { class: "flex flex-row w-full justify-start items-center gap-2.5",
                if let Some(space_status) = space_status {
                    SpaceStatusBadge { status: space_status }
                }

                SpaceTitle { title }
            }

            if show_participate_button {
                Button {
                    style: ButtonStyle::Primary,
                    onclick: move |_| {
                        if let Some(func) = &on_participant {
                            func.call(());
                        }
                    },
                    {tr.participant_button_label}
                }
            }
        }
    }
}

#[component]
pub fn SpaceTitle(title: String) -> Element {
    rsx! {
        div { class: "text-[15px] font-bold text-white", {title} }
    }
}

translate! {
    SpaceTopTranslates;

    participant_button_label: {
        en: "Participate",
        ko: "참여하기",
    },

    status_draft: {
        en: "Draft",
        ko: "초안",
    },
    status_in_progress: {
        en: "In Progress",
        ko: "진행 중",
    },
    status_started: {
        en: "Started",
        ko: "시작됨",
    },
    status_finished: {
        en: "Finished",
        ko: "종료",
    },
}
