use crate::*;

use super::popup_service::use_popup;

#[component]
pub fn PopupZone() -> Element {
    let mut popup = use_popup();

    let config = match popup.config() {
        Some(c) => c,
        None => return rsx! {},
    };

    let backdrop_closable = config.backdrop_closable;
    let closable = config.closable;
    let overflow_class = if config.overflow {
        "overflow-visible"
    } else {
        "overflow-hidden"
    };
    let id = config.id.clone();
    let title = config.title.clone();
    let description = config.description.clone();
    let content = config.content.clone();

    rsx! {
        div {
            class: "flex fixed top-0 left-0 justify-center items-center w-screen h-screen bg-popup-background backdrop-blur-[10px] z-[101] bg-no-s",
            onclick: move |_| {
                if backdrop_closable {
                    popup.close();
                }
            },

            div {
                class: "relative rounded-[20px] p-[25px] min-w-[300px] max-mobile:!w-full max-mobile:!mx-[20px] bg-popover text-text-primary {overflow_class}",
                style: "box-shadow: 0px 0px 100px rgba(255, 206, 71, 0.25)",
                onclick: move |e| {
                    e.stop_propagation();
                },

                if closable {
                    button {
                        class: "group absolute top-[25px] right-[25px] rounded-sm cursor-pointer bg-transparent hover:bg-secondary",
                        onclick: move |_| {
                            popup.close();
                        },
                        icons::validations::Clear {
                            class: "[&>path]:stroke-neutral-80 group-hover:[&>path]:stroke-text-primary",
                        }
                    }
                }

                div {
                    id: "{id}",
                    class: "flex flex-col items-center justify-center gap-[25px]",

                    if let Some(title) = title {
                        div {
                            class: "text-[20px] font-bold text-text-primary max-tablet:mt-6",
                            "{title}"
                        }
                    }

                    if let Some(description) = description {
                        div {
                            class: "text-text-primary-muted",
                            "{description}"
                        }
                    }

                    {content}
                }
            }
        }
    }
}
