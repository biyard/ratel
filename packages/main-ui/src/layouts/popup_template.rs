#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_popup::PopupService;

use crate::{components::icons, theme::Theme};

#[component]
pub fn PopupZone() -> Element {
    let theme: Theme = use_context();
    let theme = theme.get_data();
    let mut popup: PopupService = use_context();
    let mut hover_close = use_signal(|| false);

    rsx! {
        div {
            class: format!("{}", match popup.is_opened() {
                true => "fixed top-0 left-0 w-screen h-screen bg-black bg-opacity-50 flex justify-center items-center backdrop-blur-[4px] bg-black/25 z-[101]",
                false => "hidden"
            }),
            onclick: move |_| {
                popup.close();
            },
            if popup.is_opened() {
                div {
                    class: "relative bg-[{theme.primary06}] rounded-[12px] border-[{theme.primary11}] border-[1px] p-[25px] min-w-[350px]",
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    div {
                        class: format!("absolute top-[25px] right-[25px] rounded-[4px] cursor-pointer {}", if hover_close() { format!("bg-[{}]", theme.background) } else { "".to_string() }),
                        onclick: move |_| {
                            popup.close();
                        },
                        onmouseenter: move |_| {
                            hover_close.set(true);
                        },
                        onmouseleave: move |_| {
                            hover_close.set(false);
                        },
                        icons::Close {
                            color: if hover_close() {
                                theme.primary03.as_str()
                            } else {
                                "white"
                            }
                        }
                    }
                    div {
                        id: popup.get_id(),
                        class: "flex flex-col items-center justify-center gap-[25px]",
                        match popup.get_title() {
                            Some(title) => {
                                rsx! {
                                    div {
                                        class: "text-[20px] font-bold text-white",
                                        "{title}"
                                    }
                                }
                            }
                            None => rsx! {}
                        }
                        {popup.render()}
                    }
                }
            }
        }
    }
}
