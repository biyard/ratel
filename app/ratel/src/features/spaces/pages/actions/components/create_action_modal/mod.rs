use crate::features::spaces::pages::actions::actions::discussion::controllers::create_discussion;
use crate::features::spaces::pages::actions::actions::follow::controllers::create_follow;
use crate::features::spaces::pages::actions::actions::poll::controllers::create_poll;
use crate::features::spaces::pages::actions::actions::quiz::controllers::create_quiz;
use crate::features::spaces::pages::actions::*;
use crate::features::spaces::space_common::types::route::{
    space_action_discussion, space_action_follow, space_action_poll, space_action_quiz,
};
use i18n::CreateActionModalTranslate;
mod i18n;

#[component]
pub fn CreateActionModal(space_id: SpacePartition, has_follow: bool) -> Element {
    let tr: CreateActionModalTranslate = use_translate();
    let nav = navigator();
    let layover = use_layover();

    let mut selected_type = use_signal(|| None::<SpaceActionType>);
    let is_creating = use_signal(|| false);

    let close_layover = {
        let mut layover = layover;
        let mut is_creating = is_creating;
        move |_| {
            is_creating.set(false);
            layover.close();
        }
    };

    let create_action = {
        let nav = nav.clone();
        let space_id = space_id.clone();
        let layover = layover;

        let mut is_creating = is_creating;

        move |_| {
            if is_creating() {
                return;
            }

            let selected = selected_type();
            if selected.is_none() {
                return;
            }

            is_creating.set(true);
            let mut layover = layover;

            let nav = nav.clone();
            let space_id = space_id.clone();
            let mut is_creating = is_creating;

            match selected.unwrap() {
                SpaceActionType::Poll => {
                    spawn(async move {
                        match create_poll(space_id.clone()).await {
                            Ok(response) => match response.sk {
                                EntityType::SpacePoll(poll_id) => {
                                    is_creating.set(false);
                                    nav.push(space_action_poll(&space_id, &poll_id.into()));
                                    layover.close();
                                }
                                _ => {
                                    error!("Unexpected entity type from create_poll");
                                    is_creating.set(false);
                                }
                            },
                            Err(err) => {
                                error!("Failed to create poll: {:?}", err);
                                is_creating.set(false);
                            }
                        }
                    });
                }
                SpaceActionType::TopicDiscussion => {
                    spawn(async move {
                        match create_discussion(space_id.clone()).await {
                            Ok(response) => {
                                let discussion_pk: SpacePostEntityType =
                                    response.sk.try_into().unwrap_or_default();
                                is_creating.set(false);
                                nav.push(space_action_discussion(&space_id, &discussion_pk));
                                layover.close();
                            }
                            Err(err) => {
                                error!("Failed to create discussion: {:?}", err);
                                is_creating.set(false);
                            }
                        }
                    });
                }
                SpaceActionType::Follow => {
                    if has_follow {
                        is_creating.set(false);
                        return;
                    }
                    spawn(async move {
                        match create_follow(space_id.clone()).await {
                            Ok(_) => {
                                is_creating.set(false);
                                nav.push(space_action_follow(&space_id));
                                layover.close();
                            }
                            Err(err) => {
                                error!("Failed to create follow: {:?}", err);
                                is_creating.set(false);
                            }
                        }
                    });
                }
                SpaceActionType::Quiz => {
                    spawn(async move {
                        match create_quiz(space_id.clone()).await {
                            Ok(response) => {
                                is_creating.set(false);
                                nav.push(space_action_quiz(&space_id, &response.quiz_id));
                                layover.close();
                            }
                            Err(err) => {
                                error!("Failed to create quiz: {:?}", err);
                                is_creating.set(false);
                            }
                        }
                    });
                }
            }
        }
    };

    rsx! {

        div { class: "flex flex-col flex-1 min-h-0 bg-neutral-900 light:bg-neutral-200",
            // Scrollable content area
            div { class: "flex overflow-y-auto flex-col gap-5 p-[1.875rem] grow",
                // 2x2 grid of action type options
                div { class: "grid grid-cols-2 gap-2.5",
                    ActionTypeOption {
                        selected: selected_type() == Some(SpaceActionType::Quiz),
                        disabled: false,
                        onclick: move |_| selected_type.set(Some(SpaceActionType::Quiz)),
                        title: tr.quiz_title.to_string(),
                        caption: tr.quiz_caption.to_string(),
                        icon: rsx! {
                            icons::file::File {
                                width: "22",
                                height: "22",
                                class: "[&>path]:fill-none [&>path]:stroke-neutral-900",
                            }
                        },
                    }
                    ActionTypeOption {
                        selected: selected_type() == Some(SpaceActionType::Poll),
                        disabled: false,
                        onclick: move |_| selected_type.set(Some(SpaceActionType::Poll)),
                        title: tr.poll_title.to_string(),
                        caption: tr.poll_caption.to_string(),
                        icon: rsx! {
                            icons::validations::Check {
                                width: "22",
                                height: "22",
                                class: "[&>path]:fill-none [&>path]:stroke-neutral-900",
                            }
                        },
                    }
                    ActionTypeOption {
                        selected: selected_type() == Some(SpaceActionType::TopicDiscussion),
                        disabled: false,
                        onclick: move |_| selected_type.set(Some(SpaceActionType::TopicDiscussion)),
                        title: tr.discussion_title.to_string(),
                        caption: tr.discussion_caption.to_string(),
                        icon: rsx! {
                            icons::chat::Discuss {
                                width: "22",
                                height: "22",
                                class: "[&>path]:fill-none [&>path]:stroke-neutral-900",
                            }
                        },
                    }
                    if !has_follow {
                        ActionTypeOption {
                            selected: selected_type() == Some(SpaceActionType::Follow),
                            disabled: false,
                            onclick: move |_| selected_type.set(Some(SpaceActionType::Follow)),
                            title: tr.follow_title.to_string(),
                            caption: tr.follow_caption.to_string(),
                            icon: rsx! {
                                icons::user::UserGroup {
                                    width: "22",
                                    height: "22",
                                    class: "[&>path]:fill-none [&>path]:stroke-neutral-900",
                                }
                            },
                        }
                    }
                }

                // Sample preview section
                SpaceCard {
                    class: "flex flex-col gap-5 border border-neutral-700 light:border-neutral-300 !bg-neutral-800 light:!bg-white !rounded-[0.75rem] !p-4"
                        .to_string(),
                    p { class: "font-medium text-white text-[1.0625rem]/[1.25rem] light:text-neutral-900",
                        {tr.sample_title}
                    }
                    div { class: "w-full border h-[14.875rem] rounded-[0.75rem] border-neutral-700 light:border-neutral-300 bg-neutral-950/40 light:bg-neutral-100" }
                }

            }

            // Bottom bar
            div { class: "flex gap-5 justify-end items-center px-5 border-t h-[5.25rem] border-neutral-800 light:border-neutral-300 shrink-0",
                Button {
                    class: "hover:opacity-80 !px-5 !py-3 !text-[0.875rem]/[1rem] !font-bold !text-white light:!text-neutral-900"
                        .to_string(),
                    style: ButtonStyle::Text,
                    onclick: close_layover,
                    {tr.back}
                }
                Button {
                    class: "hover:opacity-90 disabled:opacity-60 disabled:cursor-not-allowed !px-5 !py-3 !rounded-[0.625rem] !text-[0.875rem]/[1rem] !font-bold !bg-yellow-400 light:!bg-yellow-500 !text-neutral-900"
                        .to_string(),
                    style: ButtonStyle::Text,
                    shape: ButtonShape::Square,
                    disabled: is_creating() || selected_type().is_none(),
                    onclick: create_action,
                    if is_creating() {
                        {tr.creating}
                    } else {
                        {tr.create}
                    }
                }
            }
        }
    }
}

#[component]
fn ActionTypeOption(
    selected: bool,
    disabled: bool,
    onclick: EventHandler<MouseEvent>,
    title: String,
    caption: String,
    icon: Element,
) -> Element {
    rsx! {
        SpaceCard {
            class: format!(
                "flex items-center gap-2.5 text-left !p-2.5 !rounded-[0.75rem] border transition-opacity {} {}",
                if selected {
                    "border-yellow-400 !bg-yellow-400/5"
                } else {
                    "border-neutral-700 light:border-neutral-300 !bg-neutral-800 light:!bg-white"
                },
                if disabled {
                    "opacity-40 cursor-not-allowed"
                } else {
                    "cursor-pointer hover:opacity-90"
                },
            ),
            onclick: move |e| {
                if disabled {
                    return;
                }
                onclick.call(e);
            },
            "aria-disabled": disabled.to_string(),

            div { class: "flex justify-center items-center bg-white rounded-[0.625rem] size-11 light:bg-white shrink-0",
                {icon}

            }

            div { class: "flex flex-col gap-1 items-start",
                p { class: "font-bold text-white text-[1.0625rem]/[1.25rem] light:text-neutral-900",
                    {title}
                }
                p { class: "font-semibold text-[0.8125rem]/[1rem] text-neutral-500 light:text-neutral-600",
                    {caption}
                }
            }
        }
    }
}
