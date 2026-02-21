use crate::*;
use i18n::CreateActionModalTranslate;
use space_action_poll::controllers::create_poll;

mod i18n;

#[component]
pub fn CreateActionModal(space_id: SpacePartition) -> Element {
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

            is_creating.set(true);
            let mut layover = layover;

            let nav = nav.clone();
            let space_id = space_id.clone();
            let mut is_creating = is_creating;

            spawn(async move {
                match create_poll(space_id.clone()).await {
                    Ok(response) => match response.sk {
                        EntityType::SpacePoll(poll_id) => {
                            is_creating.set(false);

                            nav.push(Route::PollApp {
                                space_id: space_id.clone(),
                                rest: vec![poll_id.clone()],
                            });
                            layover.close();
                        }
                        _ => {
                            error!("Unexpected poll entity type returned from create_poll");
                            is_creating.set(false);
                        }
                    },
                    Err(err) => {
                        error!("Failed to create poll action: {:?}", err);
                        is_creating.set(false);
                    }
                }
            });
        }
    };

    rsx! {

        div { class: "flex flex-col flex-1 min-h-0 bg-neutral-900 light:bg-neutral-200",
            // Scrollable content area
            div { class: "flex flex-col gap-5 p-[1.875rem] overflow-y-auto grow",
                // 2x2 grid of action type options
                div { class: "grid grid-cols-2 gap-2.5",
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
                        selected: selected_type() == Some(SpaceActionType::StudyAndQuiz),
                        disabled: true,
                        onclick: move |_| {},
                        title: tr.quiz_title.to_string(),
                        caption: tr.quiz_caption.to_string(),
                        icon: rsx! {
                            icons::help_support::Help {
                                width: "22",
                                height: "22",
                                class: "[&>path]:fill-none [&>path]:stroke-neutral-900 [&>circle]:fill-neutral-900",
                            }
                        },
                    }
                    ActionTypeOption {
                        selected: selected_type() == Some(SpaceActionType::TopicDiscussion),
                        disabled: true,
                        onclick: move |_| {},
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
                                // ActionTypeOption {
                //     selected: selected_type() == Some(SpaceActionType::Follow),
                //     disabled: true,
                //     onclick: move |_| {},
                //     title: tr.follow_title.to_string(),
                //     caption: tr.follow_caption.to_string(),
                //     icon: rsx! {
                //         icons::user::User {
                //             width: "22",
                //             height: "22",
                //             class: "[&>path]:fill-none [&>path]:stroke-neutral-900",
                //         }
                //     },
                // }
                }

                // Sample preview section
                div { class: "flex flex-col gap-5 p-4 rounded-[0.75rem] border border-neutral-700 light:border-neutral-300 bg-neutral-800 light:bg-white",
                    p { class: "text-[1.0625rem]/[1.25rem] font-medium text-white light:text-neutral-900",
                        {tr.sample_title}
                    }
                    div { class: "w-full h-[14.875rem] rounded-[0.75rem] border border-neutral-700 light:border-neutral-300 bg-neutral-950/40 light:bg-neutral-100" }
                }

            }

            // Bottom bar
            div { class: "flex justify-end items-center gap-5 h-[5.25rem] px-5 border-t border-neutral-800 light:border-neutral-300 shrink-0",
                button {
                    class: "px-5 py-3 text-[0.875rem]/[1rem] font-bold text-white light:text-neutral-900 hover:opacity-80 transition-opacity",
                    onclick: close_layover,
                    {tr.back}
                }
                button {
                    class: "px-5 py-3 rounded-[0.625rem] text-[0.875rem]/[1rem] font-bold bg-yellow-400 light:bg-yellow-500 text-neutral-900 hover:opacity-90 transition-opacity disabled:opacity-60 disabled:cursor-not-allowed",
                    disabled: is_creating(),
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
        button {
            class: "flex items-center gap-2.5 p-2.5 rounded-[0.75rem] border hover:opacity-90 transition-opacity text-left",
            class: if selected { "border-yellow-400 bg-yellow-400/5" } else { "border-neutral-700 light:border-neutral-300 bg-neutral-800 light:bg-white" },
            class: if disabled { "opacity-40 cursor-not-allowed" },
            disabled,
            onclick: move |e| onclick.call(e),

            div { class: "flex justify-center items-center rounded-[0.625rem] size-11 bg-white light:bg-white shrink-0",
                {icon}

            }

            div { class: "flex flex-col items-start gap-1",
                p { class: "text-[1.0625rem]/[1.25rem] font-bold text-white light:text-neutral-900",
                    {title}
                }
                p { class: "text-[0.8125rem]/[1rem] font-semibold text-neutral-500 light:text-neutral-600",
                    {caption}
                }
            }
        }
    }
}
