mod service;

use crate::common::hooks::use_scroll_lock;
use crate::common::*;

pub use service::*;

#[component]
pub fn PopupZone() -> Element {
    let mut popup = use_popup();

    let config = popup.config();
    use_scroll_lock(config.is_some());

    let config = match config {
        Some(c) => c,
        None => return rsx! {},
    };

    let backdrop_closable = config.backdrop_closable;
    let closable = config.closable;
    // Default popups cap their height to the viewport; the box itself stays
    // fixed (so the close button pins to its top-right) while the inner content
    // scrolls when it is taller than the viewport (e.g. the membership purchase
    // / login / signup forms on mobile). Popups that opt into
    // `with_overflow(true)` (e.g. ones containing dropdowns that must escape the
    // box) keep `overflow-visible` and do not scroll.
    let (box_overflow, content_overflow) = if config.overflow {
        ("overflow-visible", "")
    } else {
        (
            "max-h-[90dvh] overflow-hidden",
            "overflow-y-auto overflow-x-hidden min-h-0",
        )
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
                class: "flex relative flex-col rounded-[20px] p-[25px] min-w-[300px] max-mobile:!w-full max-mobile:!mx-[20px] bg-popover text-text-primary {box_overflow}",
                style: "box-shadow: 0px 0px 100px rgba(255, 206, 71, 0.25)",
                onclick: move |e| {
                    e.stop_propagation();
                },

                if closable {
                    button {
                        class: "absolute bg-transparent rounded-sm cursor-pointer group top-[25px] right-[25px] hover:bg-secondary",
                        onclick: move |_| {
                            popup.close();
                        },
                        crate::common::icons::validations::Clear { class: "[&>path]:stroke-icon-primary group-hover:[&>path]:stroke-icon-primary" }
                    }
                }

                div {
                    id: "{id}",
                    class: "flex flex-col items-center gap-[25px] w-full {content_overflow}",

                    if let Some(title) = title {
                        div { class: "font-bold text-[20px] text-text-primary", "{title}" }
                    }

                    if let Some(description) = description {
                        div { class: "text-text-primary-muted", "{description}" }
                    }

                    {content}
                }
            }
        }
    }
}
