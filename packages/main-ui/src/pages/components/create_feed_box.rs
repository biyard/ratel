use bdk::prelude::{
    by_components::{
        icons::{arrows::DoubleArrowDown, chat::RoundBubble},
        rich_texts::RichText,
    },
    *,
};

use crate::{
    components::{dropdown::Dropdown, icons::Badge},
    pages::controller::ContentType,
};

#[component]
pub fn CreateFeedBox(
    lang: Language,
    profile: String,
    nickname: String,
    onsend: EventHandler<(ContentType, String)>,
) -> Element {
    let tr: CreateFeedBoxTranslate = translate(&lang);

    let mut selected_value = use_signal(|| ContentType::Crypto);
    let mut content = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "relative flex flex-col w-full justify-start items-start px-14 pt-15 pb-12 border border-t-6 border-primary gap-11 rounded-t-lg",
            id: "create_feed",
            div { class: "flex flex-col w-full justify-start items-start gap-10 pb-50",
                div { class: "flex flex-row w-full justify-between items-center",
                    div { class: "flex flex-row w-fit justify-start items-center gap-10",
                        img {
                            class: "w-24 h-24 rounded-full object-cover",
                            src: profile,
                        }
                        div { class: "flex flex-row w-fit justify-start items-center gap-4",
                            div { class: "font-semibold text-lg/25 text-white", {nickname} }
                            Badge { width: "20", height: "20" }
                        }
                    }

                    div { class: "flex flex-row w-fit justify-start items-center gap-20",
                        Dropdown {
                            class: "w-320 h-40 border border-border-primary rounded-lg placeholder-text-neutral-500",
                            items: ContentType::variants(&lang),
                            onselect: move |value: String| {
                                selected_value.set(value.parse::<ContentType>().unwrap());
                            },
                        }

                        DoubleArrowDown {
                            class: "[&>path]:stroke-white",
                            width: "18",
                            height: "18",
                        }
                    }
                }

                RichText {
                    content: content(),
                    onchange: move |value| content.set(value),
                    change_location: true,
                    remove_border: true,
                    placeholder: tr.hint,
                }
            }

            div { class: "absolute bottom-10 right-10",
                div {
                    class: "cursor-pointer p-8 bg-primary rounded-full",
                    onclick: move |_| {
                        onsend.call((selected_value(), content()));
                    },
                    RoundBubble {
                        width: "24",
                        height: "24",
                        fill: "none",
                        class: "[&>path]:stroke-neutral-900 [&>line]:stroke-neutral-900",
                    }
                }
            }
        }
    }
}

translate! {
    CreateFeedBoxTranslate;

    hint: {
        ko: "Type here...",
        en: "Type here..."
    }
}
