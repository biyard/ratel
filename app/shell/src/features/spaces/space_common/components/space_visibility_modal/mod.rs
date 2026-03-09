use crate::features::spaces::space_common::*;
use common::components::Button;
use dioxus::prelude::*;

translate! {
    VisibilityModalTranslate;

    title: {
        en: "Post Visibility",
        ko: "게시물 공개 범위",
    },
    description: {
        en: "Choose who can see this post",
        ko: "이 게시물을 볼 수 있는 사람을 선택하세요",
    },
    public_label: {
        en: "Public",
        ko: "공개",
    },
    public_desc: {
        en: "Anyone can see this post",
        ko: "누구나 이 게시물을 볼 수 있습니다",
    },
    private_label: {
        en: "Private",
        ko: "비공개",
    },
    private_desc: {
        en: "Only you can see this post",
        ko: "나만 이 게시물을 볼 수 있습니다",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    confirm: {
        en: "Publish",
        ko: "게시",
    },
}

#[component]
pub fn SpaceVisibilityModal(
    #[props(default = SpaceVisibility::Private)] initial: SpaceVisibility,
    on_confirm: EventHandler<SpaceVisibility>,
    on_cancel: Option<EventHandler<MouseEvent>>,
) -> Element {
    let tr: VisibilityModalTranslate = use_translate();
    let mut selected = use_signal(move || initial.clone());
    let mut popup = use_popup();

    let is_public = matches!(*selected.read(), SpaceVisibility::Public);

    let public_card_class = if is_public {
        "flex items-center gap-4 p-4 rounded-xl border cursor-pointer transition-colors border-primary bg-primary/10"
    } else {
        "flex items-center gap-4 p-4 rounded-xl border cursor-pointer transition-colors border-border hover:border-text-tertiary"
    };

    let private_card_class = if !is_public {
        "flex items-center gap-4 p-4 rounded-xl border cursor-pointer transition-colors border-primary bg-primary/10"
    } else {
        "flex items-center gap-4 p-4 rounded-xl border cursor-pointer transition-colors border-border hover:border-text-tertiary"
    };

    let public_radio_class = if is_public {
        "w-5 h-5 rounded-full border-2 flex items-center justify-center border-primary"
    } else {
        "w-5 h-5 rounded-full border-2 flex items-center justify-center border-text-tertiary"
    };

    let private_radio_class = if !is_public {
        "w-5 h-5 rounded-full border-2 flex items-center justify-center border-primary"
    } else {
        "w-5 h-5 rounded-full border-2 flex items-center justify-center border-text-tertiary"
    };

    rsx! {
        div { class: "flex flex-col gap-6 max-w-full w-[400px]",
            div { class: "flex flex-col gap-1",
                div { class: "text-lg font-bold text-center text-text-primary", "{tr.title}" }
                div { class: "text-sm text-center text-text-secondary", "{tr.description}" }
            }

            div { class: "flex flex-col gap-3",
                // Public option
                button {
                    r#type: "button",
                    class: "{public_card_class}",
                    "data-testid": "public-option",
                    onclick: move |_| {
                        selected.set(SpaceVisibility::Public);
                    },
                    div { class: "flex justify-center items-center w-10 h-10 rounded-full bg-primary/15 shrink-0",
                        icons::internet_script::Internet { class: "w-5 h-5 [&>path]:stroke-primary [&>circle]:stroke-primary [&>ellipse]:stroke-primary [&>line]:stroke-primary" }
                    }
                    div { class: "flex flex-col gap-0.5 items-start",
                        span { class: "text-sm font-semibold text-text-primary", "{tr.public_label}" }
                        span { class: "text-xs text-text-secondary", "{tr.public_desc}" }
                    }
                    div { class: "ml-auto shrink-0",
                        div { class: "{public_radio_class}",
                            if is_public {
                                div { class: "w-2.5 h-2.5 rounded-full bg-primary" }
                            }
                        }
                    }
                }

                // Private option
                button {
                    r#type: "button",
                    class: "{private_card_class}",
                    "data-testid": "private-option",
                    onclick: move |_| {
                        selected.set(SpaceVisibility::Private);
                    },
                    div { class: "flex justify-center items-center w-10 h-10 rounded-full bg-primary/15 shrink-0",
                        icons::security::Lock1 { class: "w-5 h-5 [&>path]:stroke-primary [&>rect]:stroke-primary [&>circle]:stroke-primary" }
                    }
                    div { class: "flex flex-col gap-0.5 items-start",
                        span { class: "text-sm font-semibold text-text-primary", "{tr.private_label}" }
                        span { class: "text-xs text-text-secondary", "{tr.private_desc}" }
                    }
                    div { class: "ml-auto shrink-0",
                        div { class: "{private_radio_class}",
                            if !is_public {
                                div { class: "w-2.5 h-2.5 rounded-full bg-primary" }
                            }
                        }
                    }
                }
            }

            div { class: "flex gap-3 justify-end items-center",
                Button {
                    style: ButtonStyle::Outline,
                    shape: ButtonShape::Square,
                    onclick: move |e| {
                        if let Some(on_cancel) = on_cancel.as_ref() {
                            on_cancel.call(e);
                        }
                        popup.close();
                    },
                    "{tr.cancel}"
                }
                Button {
                    "aria-label": "Confirm visibility selection",
                    class: "min-w-[100px]",
                    onclick: move |_| {
                        on_confirm.call(selected());

                        popup.close();
                    },
                    "{tr.confirm}"
                }
            }
        }
    }
}
