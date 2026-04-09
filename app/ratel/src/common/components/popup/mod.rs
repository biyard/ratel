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
            class: "flex fixed inset-0 justify-center items-center bg-black/60 backdrop-blur-sm z-[101]",
            onclick: move |_| {
                if backdrop_closable {
                    popup.close();
                }
            },

            div {
                class: "relative rounded-2xl border p-[25px] min-w-[300px] max-mobile:!w-full max-mobile:!mx-[20px] bg-[image:var(--glass-surface-primary)] backdrop-blur-[var(--glass-blur)] border-white/8 shadow-[var(--depth-lg)] text-text-primary {overflow_class}",
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
                    class: "flex flex-col justify-center items-center gap-[25px]",

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
