use crate::features::spaces::pages::actions::*;
use i18n::CreateActionModalTranslate;
mod i18n;

#[component]
pub fn CreateActionModal(space_id: SpacePartition) -> Element {
    let tr: CreateActionModalTranslate = use_translate();
    let nav = navigator();
    let mut layover = use_layover();

    let mut selected_type = use_signal(|| Some(SpaceActionType::Quiz));
    let mut is_creating = use_signal(|| false);

    let close_layover = {
        let mut layover = layover;
        let mut is_creating = is_creating;
        move |_| {
            is_creating.set(false);
            layover.close();
        }
    };

    let create_action = {
        let space_id = space_id.clone();
        move |_| {
            let space_id = space_id.clone();
            async move {
                let selected = selected_type();
                if selected.is_none() {
                    return;
                }

                let selected = selected.unwrap();
                if is_creating() {
                    return;
                }
                is_creating.set(true);
                match selected.create(space_id).await {
                    Ok(url) => {
                        is_creating.set(false);
                        nav.push(url);
                        layover.close();
                    }
                    Err(err) => {
                        error!("Failed to create action: {:?}", err);
                        is_creating.set(false);
                    }
                }
            }
        }
    };

    rsx! {

        div { class: "flex flex-col flex-1 min-h-0 bg-action-modal-bg",
            // Scrollable content area
            div { class: "flex overflow-y-auto flex-col gap-5 p-[1.875rem] grow",
                // 2x2 grid of action type options
                div { class: "grid grid-cols-2 gap-2.5",
                    ActionTypeOption {
                        test_id: "action-type-quiz".to_string(),
                        selected: selected_type() == Some(SpaceActionType::Quiz),
                        disabled: false,
                        onclick: move |_| selected_type.set(Some(SpaceActionType::Quiz)),
                        title: tr.quiz_title.to_string(),
                        caption: tr.quiz_caption.to_string(),
                        icon: rsx! {
                            icons::file::File {
                                width: "22",
                                height: "22",
                                class: "[&>path]:fill-none [&>path]:stroke-action-type-icon-stroke",
                            }
                        },
                    }
                    ActionTypeOption {
                        test_id: "action-type-poll".to_string(),
                        selected: selected_type() == Some(SpaceActionType::Poll),
                        disabled: false,
                        onclick: move |_| selected_type.set(Some(SpaceActionType::Poll)),
                        title: tr.poll_title.to_string(),
                        caption: tr.poll_caption.to_string(),
                        icon: rsx! {
                            icons::validations::Check {
                                width: "22",
                                height: "22",
                                class: "[&>path]:fill-none [&>path]:stroke-action-type-icon-stroke",
                            }
                        },
                    }
                    ActionTypeOption {
                        test_id: "action-type-discussion".to_string(),
                        selected: selected_type() == Some(SpaceActionType::TopicDiscussion),
                        disabled: false,
                        onclick: move |_| selected_type.set(Some(SpaceActionType::TopicDiscussion)),
                        title: tr.discussion_title.to_string(),
                        caption: tr.discussion_caption.to_string(),
                        icon: rsx! {
                            icons::chat::Discuss {
                                width: "22",
                                height: "22",
                                class: "[&>path]:fill-none [&>path]:stroke-action-type-icon-stroke",
                            }
                        },
                    }
                    ActionTypeOption {
                        test_id: "action-type-follow".to_string(),
                        selected: selected_type() == Some(SpaceActionType::Follow),
                        disabled: false,
                        onclick: move |_| selected_type.set(Some(SpaceActionType::Follow)),
                        title: tr.follow_title.to_string(),
                        caption: tr.follow_caption.to_string(),
                        icon: rsx! {
                            icons::user::UserGroup {
                                width: "22",
                                height: "22",
                                class: "[&>path]:fill-none [&>path]:stroke-action-type-icon-stroke",
                            }
                        },
                    }
                }

                // Preview section
                ActionPreview { selected: selected_type(), tr: tr.clone() }

            }

            // Bottom bar
            div { class: "flex gap-5 justify-end items-center px-5 border-t h-[5.25rem] border-card-border shrink-0",
                Button {
                    class: "hover:opacity-80 !px-5 !py-3 !text-[0.875rem]/[1rem] !font-bold !text-text-primary"
                        .to_string(),
                    style: ButtonStyle::Text,
                    onclick: close_layover,
                    {tr.back}
                }
                Button {
                    class: "hover:opacity-90 disabled:opacity-60 disabled:cursor-not-allowed !px-5 !py-3 !rounded-[0.625rem] !text-[0.875rem]/[1rem] !font-bold !bg-btn-primary-bg !text-btn-primary-text"
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
fn ActionPreview(selected: Option<SpaceActionType>, tr: CreateActionModalTranslate) -> Element {
    let content = match selected {
        Some(SpaceActionType::Poll) => rsx! {
            PollPreview {}
        },
        Some(SpaceActionType::Quiz) => rsx! {
            QuizPreview {}
        },
        Some(SpaceActionType::TopicDiscussion) => rsx! {
            DiscussionPreview {}
        },
        Some(SpaceActionType::Follow) => rsx! {
            FollowPreview {}
        },
        None => {
            return rsx! {
                SpaceCard {
                    class: "flex flex-col gap-5 border border-action-type-card-border !bg-action-type-card-bg !rounded-[0.75rem] !p-4"
                        .to_string(),
                    p { class: "font-medium text-text-primary text-[1.0625rem]/[1.25rem]",
                        {tr.preview_title}
                    }
                    div { class: "flex justify-center items-center w-full border h-[14.875rem] rounded-[0.75rem] border-action-type-card-border bg-action-preview-inner-bg",
                        p { class: "text-[0.875rem]/[1.25rem] text-foreground-muted",
                            {tr.preview_empty}
                        }
                    }
                }
            };
        }
    };

    rsx! {
        SpaceCard {
            class: "flex flex-col gap-3 border border-yellow-400/30 !bg-action-type-card-bg !rounded-[0.75rem] !p-4"
                .to_string(),
            p { class: "font-medium text-text-primary text-[1.0625rem]/[1.25rem]",
                {tr.preview_title}
            }
            div { class: "flex flex-col gap-3 p-4 w-full border opacity-90 pointer-events-none rounded-[0.75rem] border-action-type-card-border bg-action-preview-inner-bg",
                {content}
            }
        }
    }
}

// Mock poll viewer: time range + a sample question with radio options
#[component]
fn PollPreview() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3 w-full",
            div { class: "flex gap-2 items-center text-xs text-foreground-muted",
                span { "Mar 12, 2026 00:00" }
                span { "~" }
                span { "Mar 19, 2026 00:00" }
            }
            div { class: "p-3 rounded-lg border border-action-type-card-border",
                div { class: "mb-2 text-xs text-foreground-muted", "1 / 2" }
                p { class: "mb-2 text-sm font-medium text-text-primary",
                    "Which proposal do you support?"
                }
                div { class: "flex flex-col gap-1.5",
                    for option in ["Proposal A", "Proposal B", "Proposal C"] {
                        label { class: "flex gap-2 items-center text-sm text-action-preview-option-text",
                            div { class: "rounded-full border-2 size-4 border-action-preview-radio-border" }
                            "{option}"
                        }
                    }
                }
            }
            div { class: "py-2 text-sm font-medium text-center text-action-preview-submit-text rounded-lg bg-action-preview-submit-bg",
                "Submit"
            }
        }
    }
}

// Mock quiz viewer: time range + score + a sample question
#[component]
fn QuizPreview() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3 w-full",
            div { class: "flex justify-between items-center",
                div { class: "flex gap-2 items-center text-xs text-foreground-muted",
                    span { "Mar 12, 2026 00:00" }
                    span { "~" }
                    span { "Mar 19, 2026 00:00" }
                }
            }
            div { class: "flex gap-2 items-center text-xs text-foreground-muted",
                span { class: "font-medium", "Remaining submissions:" }
                span { "3" }
            }
            div { class: "p-3 rounded-lg border border-action-type-card-border",
                div { class: "mb-2 text-xs text-foreground-muted", "1 / 3" }
                p { class: "mb-2 text-sm font-medium text-text-primary",
                    "What is the governance threshold?"
                }
                div { class: "flex flex-col gap-1.5",
                    for (i , option) in ["51%", "67%", "75%", "90%"].iter().enumerate() {
                        label { class: "flex gap-2 items-center text-sm text-action-preview-option-text",
                            div {
                                class: format!(
                                    "size-4 rounded-full border-2 {}",
                                    if i == 1 {
                                        "border-yellow-400 bg-yellow-400/30"
                                    } else {
                                        "border-action-preview-radio-border"
                                    },
                                ),
                            }
                            "{option}"
                        }
                    }
                }
            }
        }
    }
}

// Mock discussion viewer: title + content + comment area
#[component]
fn DiscussionPreview() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3 w-full",
            h3 { class: "text-base font-bold text-text-primary",
                "Should we increase the staking reward?"
            }
            div { class: "flex gap-2 items-center text-xs text-foreground-muted",
                div { class: "rounded-full size-5 bg-action-preview-avatar-bg" }
                span { class: "font-medium", "alice.eth" }
            }
            p { class: "text-sm text-foreground-muted line-clamp-2",
                "I propose we increase the staking reward from 5% to 8% to incentivize long-term holders and strengthen network security..."
            }
            hr { class: "border-action-type-card-border" }
            p { class: "text-sm font-bold text-text-primary", "Comments (2)" }
            div { class: "flex flex-col gap-2",
                div { class: "p-2 rounded-lg border border-action-type-card-border bg-action-preview-item-bg",
                    div { class: "flex gap-2 items-center mb-1 text-xs",
                        div { class: "rounded-full size-4 bg-action-preview-avatar-bg" }
                        span { class: "font-medium text-text-primary",
                            "bob.eth"
                        }
                    }
                    p { class: "text-xs text-foreground-muted",
                        "I agree, higher rewards will attract more stakers."
                    }
                    div { class: "flex gap-3 items-center mt-1 text-xs text-foreground-muted",
                        span { "♡ 3" }
                        span { "Reply (1)" }
                    }
                }
            }
        }
    }
}

// Mock follow viewer: user list
#[component]
fn FollowPreview() -> Element {
    rsx! {
        div { class: "flex flex-col gap-3 w-full",
            p { class: "text-sm font-bold text-text-primary", "Followers" }
            div { class: "flex flex-col gap-2",
                for (name , handle) in [("Alice", "@alice.eth"), ("Bob", "@bob.eth"), ("Charlie", "@charlie.eth")] {
                    div { class: "flex gap-3 items-center p-2 rounded-lg border border-action-type-card-border bg-action-preview-item-bg",
                        div { class: "rounded-full size-8 bg-action-preview-avatar-bg" }
                        div { class: "flex flex-col",
                            span { class: "text-sm font-medium text-text-primary",
                                "{name}"
                            }
                            span { class: "text-xs text-foreground-muted", "{handle}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ActionTypeOption(
    test_id: String,
    selected: bool,
    disabled: bool,
    onclick: EventHandler<MouseEvent>,
    title: String,
    caption: String,
    icon: Element,
) -> Element {
    let testid = format!("action-type-{}", title.to_lowercase().replace(' ', "-"));
    rsx! {
        SpaceCard {
            "data-testid": "{test_id}",
            class: format!(
                "flex items-center gap-2.5 text-left !p-2.5 !rounded-[0.75rem] border transition-opacity {} {}",
                if selected {
                    "border-yellow-400 !bg-yellow-400/5"
                } else {
                    "border-action-type-card-border !bg-action-type-card-bg"
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

            div { class: "flex justify-center items-center bg-white rounded-[0.625rem] size-11 shrink-0",
                {icon}

            }

            div { class: "flex flex-col gap-1 items-start",
                p { class: "font-bold text-text-primary text-[1.0625rem]/[1.25rem]",
                    {title}
                }
                p { class: "font-semibold text-[0.8125rem]/[1rem] text-action-type-card-caption",
                    {caption}
                }
            }
        }
    }
}
