#![allow(unused)]
use bdk::prelude::{
    by_components::icons::{arrows::DoubleArrowDown, chat::RoundBubble},
    *,
};
use dto::{
    ContentType,
    by_components::icons::{arrows::DoubleArrowUp, validations::Clear},
};

use crate::{
    components::{dropdown::Dropdown, icons::Badge, rich_text::RichText},
    services::backend_api::BackendApi,
};

use dto::File;

#[cfg(feature = "web")]
use crate::utils::file::handle_file_upload;

#[component]
pub fn CreateFeedBox(
    lang: Language,
    profile: String,
    nickname: String,
    onsend: EventHandler<(Vec<File>, ContentType, String)>,
    onclose: EventHandler<MouseEvent>,
) -> Element {
    let api: BackendApi = use_context();
    let mut minimize = use_signal(|| false);
    let tr: CreateFeedBoxTranslate = translate(&lang);

    let mut selected_value = use_signal(|| ContentType::Crypto);
    let mut content = use_signal(|| "".to_string());
    let mut feed_files: Signal<Vec<File>> = use_signal(|| vec![]);

    tracing::debug!("this line come: {:?}", feed_files());

    rsx! {
        div {
            class: "relative flex flex-col w-full justify-start items-start px-14 pt-15 pb-12 border border-t-6 border-primary gap-11 rounded-t-lg z-60",
            id: "create_feed",
            div { class: "flex flex-col w-full justify-start items-start gap-10 pb-12",
                div { class: " flex flex-row w-full justify-between items-center",
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

                    if !minimize() {
                        div { class: "flex flex-row w-fit justify-start items-center gap-20",
                            Dropdown {
                                class: "w-320 h-40 border border-border-primary rounded-lg placeholder-text-neutral-500 max-tablet:!hidden",
                                items: ContentType::variants(&lang),
                                onselect: move |value: String| {
                                    selected_value.set(value.parse::<ContentType>().unwrap());
                                },
                            }

                            div {
                                class: "cursor-pointer w-fit h-fit",
                                onclick: move |_| {
                                    minimize.set(true);
                                },
                                DoubleArrowDown {
                                    class: "[&>path]:stroke-white",
                                    width: "18",
                                    height: "18",
                                }
                            }
                        }
                    } else {
                        div { class: "flex flex-row w-fit justify-start items-center gap-20",
                            div {
                                class: "cursor-pointer w-fit h-fit",
                                onclick: move |_| {
                                    minimize.set(false);
                                },
                                DoubleArrowUp {
                                    class: "[&>path]:stroke-white",
                                    width: "18",
                                    height: "18",
                                }
                            }
                            div {
                                class: "cursor-pointer w-fit h-fit",
                                onclick: move |e| {
                                    onclose.call(e);
                                    minimize.set(false);
                                },
                                Clear {
                                    class: "[&>path]:stroke-white",
                                    width: "18",
                                    height: "18",
                                }
                            }
                        }
                    }
                }

                div {
                    class: format_args!(
                        "transition-all duration-300 overflow-hidden w-full {}",
                        if minimize() { "max-h-0 opacity-0" } else { "max-h-[600px] opacity-100" },
                    ),
                    Dropdown {
                        class: "w-full h-40 border border-border-primary rounded-lg placeholder-text-neutral-500 hidden max-tablet:!flex",
                        items: ContentType::variants(&lang),
                        onselect: move |value: String| {
                            selected_value.set(value.parse::<ContentType>().unwrap());
                        },
                    }

                    div { class: "flex flex-col w-full justify-start items-start gap-10 my-10",
                        for (i , file) in feed_files().iter().enumerate() {
                            FileBox {
                                file: file.clone(),
                                ondelete: move |_| {
                                    feed_files.with_mut(|v| v.remove(i));
                                },
                            }
                        }
                    }

                    RichText {
                        content: content(),
                        onchange: move |value| content.set(value),
                        change_location: true,
                        remove_border: true,
                        placeholder: tr.hint,
                        send_button: rsx! {
                            div {
                                class: "cursor-pointer p-8 bg-primary rounded-full",
                                onclick: move |_| {
                                    onsend.call((feed_files(), selected_value(), content()));
                                },
                                RoundBubble {
                                    width: "24",
                                    height: "24",
                                    fill: "none",
                                    class: "[&>path]:stroke-neutral-900 [&>line]:stroke-neutral-900",
                                }
                            }
                        },
                        onupload: move |ev: FormEvent| async move {
                            spawn(async move {
                                #[cfg(feature = "web")]
                                if let Some(file_engine) = ev.files() {
                                    let result = handle_file_upload(file_engine, api).await;
                                    tracing::debug!("file upload results: {:?}", result);
                                    if !result.is_empty() && result[0].clone().url.is_some() {
                                        feed_files.push(result[0].clone());
                                    }
                                }
                            });
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn FileBox(file: File, ondelete: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center px-10 py-5 border border-border-primary rounded-lg",
            div { class: "text-sm/16 font-semibold text-white", {file.name} }
            div {
                class: "cursor-pointer w-24 h-24",
                onclick: move |e| {
                    ondelete.call(e);
                },
                Clear {
                    width: "24",
                    height: "24",
                    fill: "none",
                    class: "[&>path]:stroke-white [&>line]:stroke-white",
                }
            }
        }
    }
}

translate! {
    CreateFeedBoxTranslate;

    hint: {
        ko: "Type here, Use Markdown, BB code, or HTML to format. Drag or paste images.",
        en: "Type here, Use Markdown, BB code, or HTML to format. Drag or paste images."
    }
}
