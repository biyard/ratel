use crate::common::{
    icons::{edit::Eye, home::Home1},
    models::space::SpaceCommon,
};

use crate::features::spaces::space_common::{
    components::{SpaceEndModal, SpaceStartModal, SpaceStatusBadge, SpaceVisibilityModal},
    controllers::update_space,
    hooks::{use_space, use_space_role},
    providers::{use_space_context, SpaceContextProvider},
    *,
};

const DEFAULT_LOGO: &str = "https://metadata.ratel.foundation/logos/logo.png";

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

    //FIXME: Rotate Labels
    let title = labels.first().unwrap().label.clone();
    let nav = use_navigator();
    let mut popup = use_popup();

    let mut ctx = use_space_context();
    let current_role = ctx.current_role();
    let real_role = ctx.role();
    let is_creator = real_role == SpaceUserRole::Creator;
    let can_preview = current_role == SpaceUserRole::Creator;
    let is_published = ctx.space().publish_state == SpacePublishState::Published;
    let is_in_progress = ctx.space().status == Some(SpaceStatus::InProgress);
    let is_started = ctx.space().status == Some(SpaceStatus::Started);
    let space_id = use_signal(|| ctx.space().id);
    let space_logo = {
        let logo = ctx.space().logo.clone();
        if logo.is_empty() {
            DEFAULT_LOGO.to_string()
        } else {
            logo
        }
    };

    rsx! {
        div { class: "flex flex-row justify-between items-center py-4 px-3 w-full min-h-16 shrink-0",
            div { class: "flex flex-row gap-2.5 justify-start items-center w-full min-w-0 max-tablet:flex-1",
                img {
                    src: "{space_logo}",
                    class: "hidden object-contain w-auto h-7 max-tablet:block",
                    alt: "Space logo",
                }

                if let Some(space_status) = space_status {
                    SpaceStatusBadge { status: space_status }
                }

                SpaceTitle { title }
            }

            div { class: "flex flex-row gap-2.5 justify-end items-center shrink-0 max-tablet:gap-1",
                Button {
                    style: ButtonStyle::Text,
                    shape: ButtonShape::Square,
                    class: "flex flex-row gap-1 justify-center items-center max-tablet:p-0 max-tablet:w-fit max-tablet:h-9",
                    onclick: move |_| {
                        nav.push("/");
                    },
                    Home1 { class: "w-4 h-4 [&>path]:stroke-icon-primary max-tablet:w-9 max-tablet:h-5" }
                    p { class: "max-tablet:hidden", {tr.go_home} }
                }

                if is_creator {
                    Button {
                        style: ButtonStyle::Outline,
                        shape: ButtonShape::Square,
                        class: "flex flex-row gap-1 justify-center items-center max-tablet:p-0 max-tablet:w-9 max-tablet:h-9 max-tablet:border-0",
                        onclick: move |_| {
                            ctx.toggle_role();
                        },
                        Eye { class: "w-4 h-4 [&>path]:stroke-icon-secondary [&>circle]:stroke-icon-secondary max-tablet:w-5 max-tablet:h-5 max-tablet:[&>path]:stroke-icon-primary max-tablet:[&>circle]:stroke-icon-primary" }
                        p { class: "max-tablet:hidden",
                            if can_preview {
                                {tr.preview}
                            } else {
                                {tr.design}
                            }
                        }
                    }

                    if !is_published {
                        Button {
                            shape: ButtonShape::Square,
                            class: "max-tablet:py-1 max-tablet:px-2 max-tablet:text-xs",
                            onclick: move |_| {
                                debug!("Publish button clicked. Current space status: {:?}", ctx.space().status);
                                let initial = ctx.space().visibility;
                                popup.open(rsx! {
                                    SpaceVisibilityModal {
                                        initial,
                                        on_confirm: move |visibility| async move {
                                            let space_id = ctx.space().id;
                                            update_space(
                                                    space_id,
                                                    controllers::UpdateSpaceRequest::Publish {
                                                        publish: true,
                                                        visibility,
                                                    },
                                                )
                                                .await;
                                            ctx.space.restart();
                                        },
                                    }
                                });
                            },
                            {tr.publish}
                        }
                    } else if is_in_progress {
                        Button {
                            shape: ButtonShape::Square,
                            class: "max-tablet:py-1 max-tablet:px-2 max-tablet:text-xs",
                            onclick: move |_| {
                                popup.open(rsx! {
                                    SpaceStartModal {
                                        space_id,
                                        on_started: move |_| {
                                            ctx.space.restart();
                                        },
                                    }
                                });
                            },
                            {tr.start}
                        }
                    } else if is_started {
                        Button {
                            shape: ButtonShape::Square,
                            class: "max-tablet:py-1 max-tablet:px-2 max-tablet:text-xs",
                            onclick: move |_| {
                                popup.open(rsx! {
                                    SpaceEndModal {
                                        space_id,
                                        on_ended: move |_| {
                                            ctx.space.restart();
                                        },
                                    }
                                });
                            },
                            {tr.finish}
                        }
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
    let mut toast = use_toast();

    let mut space = use_space();
    let role = use_space_role();

    let mut editing = use_signal(|| false);

    rsx! {
        Fragment {
            if editing() {
                Input {
                    onchange: move |evt: FormEvent| async move {
                        let value = evt.value();

                        match update_space(
                                space().id,
                                controllers::UpdateSpaceRequest::Title {
                                    title: value.clone(),
                                },
                            )
                            .await
                        {
                            Ok(_) => {
                                space
                                    .with_mut(move |space| {
                                        space.title = value;
                                    });
                            }
                            Err(e) => {
                                toast.error(e);
                            }
                        };
                    },
                    value: space().title,
                }
            } else {
                div {
                    class: "font-bold text-[15px] text-web-font-primary max-tablet:truncate max-tablet:text-sm",
                    onclick: move |_| {
                        if role().can_edit() {
                            editing.set(true);
                        }
                    },
                    {title}
                }
            }
        }
    }
}

translate! {
    SpaceTopTranslates;

    publish: {
        en: "Publish",
        ko: "게시하기",
    }

    start: {
        en: "Start",
        ko: "시작하기",
    }

    finish: {
        en: "Finish",
        ko: "종료하기",
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
