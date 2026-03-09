use common::{
    icons::{edit::Eye, home::Home1},
    models::space::SpaceCommon,
};

use crate::{
    components::{SpaceStatusBadge, SpaceVisibilityModal},
    controllers::update_space,
    hooks::use_space_role,
    providers::{SpaceContextProvider, use_space_context},
    *,
};

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
    let nav = use_navigator();
    let mut popup = use_popup();
    let role = use_space_role();
    let mut ctx = use_space_context();
    let real_role = ctx.role();
    let is_creator = use_memo(move || ctx.role() == SpaceUserRole::Creator);
    let mut toast = use_toast();
    let can_preview = use_memo(move || {
        let current_role = ctx.current_role();

        current_role == SpaceUserRole::Creator
    });

    rsx! {
        div { class: "flex flex-row justify-between items-center py-4 px-3 min-h-16 shrink-0",
            div { class: "flex flex-row gap-2.5 justify-start items-center w-full",
                if let Some(space_status) = space_status {
                    SpaceStatusBadge { status: space_status }
                }

                SpaceTitle { title }
            }

            div { class: "flex flex-row gap-2.5 justify-end items-center shrink-0",
                Button {
                    style: ButtonStyle::Text,
                    shape: ButtonShape::Square,
                    class: "flex flex-row gap-1 justify-center items-center",
                    onclick: move |_| {
                        nav.push("/");
                    },
                    Home1 { class: "w-4 h-4 [&>path]:stroke-icon-primary" }
                    p { {tr.go_home} }
                }

                if is_creator() {
                    Button {
                        style: ButtonStyle::Outline,
                        shape: ButtonShape::Square,
                        class: "flex flex-row gap-1 justify-center items-center",
                        onclick: move |_| {
                            ctx.toggle_role();
                        },
                        Eye { class: "w-4 h-4 [&>path]:stroke-icon-secondary [&>circle]:stroke-icon-secondary" }
                        p {
                            if ctx.can_preview() {
                                {tr.preview}
                            } else {
                                {tr.design}
                            }
                        }
                    }

                    Button {
                        shape: ButtonShape::Square,
                        onclick: move |_| {
                            popup.open(rsx! {
                                SpaceVisibilityModal {
                                    on_confirm: move |visibility| async move {
                                        let space_id = ctx.space().id;
                                        update_space(
                                                space_id,
                                                controllers::UpdateSpaceRequest::Publish {
                                                    publish: true,
                                                    // FIXME: Pass actual content and visibility
                                                    visibility: SpaceVisibility::Public,
                                                },
                                            )
                                            .await;



                                    },

                                }
                            });
                        },
                        {tr.publish}
                    }
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
}

#[component]
pub fn SpaceTitle(title: String) -> Element {
    rsx! {
        div { class: "font-bold text-[15px] text-web-font-primary", {title} }
    }
}

translate! {
    SpaceTopTranslates;

    publish: {
        en: "Publish",
        ko: "게시하기",
    }

    preview: {
        en: "Preview",
        ko: "미리보기",
    }

    design: {
        en: "Design",
        ko: "설계하기",
    }

    go_home: {
        en: "Go Home",
        ko: "홈으로 이동",
    }

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
