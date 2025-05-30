#![allow(non_snake_case)]
use bdk::prelude::*;
use dioxus_popup::{Close, PopupService};

#[component]
pub fn PopupZone() -> Element {
    let mut popup: PopupService = use_context();

    rsx! {
        div {
            class: format!(
                "{}",
                match popup.is_opened() {
                    true => {
                        "fixed top-0 left-0 w-screen h-screen bg-popup-background flex justify-center items-center backdrop-blur-[10px] z-[101]"
                    }
                    false => "hidden",
                },
            ),
            onclick: move |_| {
                popup.close();
            },
            if popup.is_opened() {
                div {
                    class: "relative rounded-[20px] p-25 min-w-300 max-mobile:!w-full max-mobile:!mx-20 bg-bg overflow-hidden",
                    style: "box-shadow: 0px 0px 100px rgba(255, 206, 71, 0.25);",
                    box_shadow: "0px 0px 100px rgba(255, 206, 71, 0.25);",
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    if (popup.close)() {
                        button {
                            class: "group absolute top-25 right-25 rounded-[4px] cursor-pointer bg-transparent hover:bg-secondary",
                            onclick: move |_| {
                                popup.close();
                            },
                            Close { class: "[&>path]:stroke-neutral-80 group-hover:[&>path]:stroke-text-primary" }
                        }
                    }
                    div {
                        id: popup.get_id(),
                        class: "flex flex-col items-center justify-center gap-[25px]",
                        match popup.get_title() {
                            Some(title) => {
                                rsx! {
                                    div { class: "text-[20px] font-bold text-white", "{title}" }
                                }
                            }
                            None => rsx! {},
                        }
                        {popup.render()}
                    }
                }
            }
        }
    }
}
