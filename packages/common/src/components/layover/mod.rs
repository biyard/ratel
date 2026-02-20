use crate::*;
mod service;

pub use service::*;

#[component]
pub fn Layover(is_open: bool, #[props(default)] title: String, children: Element) -> Element {
    let mut layover = use_layover();
    let panel_transform = if is_open {
        "translate-x-0"
    } else {
        "translate-x-full"
    };
    let backdrop_class = if is_open {
        "opacity-50"
    } else {
        "opacity-0 pointer-events-none"
    };
    let container_pointer = if is_open { "" } else { "pointer-events-none" };
    let onclose = move |_| layover.close();
    rsx! {
        div { class: "fixed inset-0 z-100 {container_pointer}",

            // Backdrop with blur
            div {
                class: "absolute inset-0 bg-black backdrop-blur-sm transition-opacity duration-300 {backdrop_class}",
                onclick: onclose,
            }

            // Right-side slide panel
            div {
                class: "absolute top-0 right-0 h-full w-full max-w-[50%] bg-popover overflow-y-auto transition-transform duration-300 ease-in-out {panel_transform}",
                onclick: move |e| e.stop_propagation(),

                div { class: "flex flex-col gap-4 p-8 h-full",
                    // Header: title + close button
                    if !title.is_empty() {
                        div { class: "flex flex-row justify-between items-center w-full",
                            h3 { class: "text-font-primary", {title} }
                            button {
                                class: "bg-transparent cursor-pointer hover:bg-gray-700 rounded-sm p-1",
                                onclick: onclose,
                                icons::validations::Clear { class: "[&>path]:stroke-font-primary" }
                            }
                        }
                    }

                    {children}
                }
            }
        }
    }
}
