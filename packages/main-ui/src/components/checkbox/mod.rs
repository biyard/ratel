#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn Checkbox(id: String, onchange: EventHandler<bool>, children: Element) -> Element {
    let mut checked = use_signal(|| false);

    rsx! {
        div { class: "text-white text-sm/16 font-normal flex flex-row gap-10 items-start",

            div { class: "relative flex flex-row items-center justify-start gap-[6px] cursor-pointer",
                input {
                    id: "{id}",
                    r#type: "checkbox",
                    onchange: move |e| {
                        tracing::debug!("Checkbox changed {}", e.value());
                        checked.set(e.value() == "true".to_string());
                        onchange(checked());
                    },
                    class: "peer hidden",
                }
                label {
                    class: "border border-c-wg-50 rounded-[4px] peer-checked:bg-primary peer-checked:border-primary flex items-center justify-center w-18 h-18 cursor-pointer",
                    r#for: "{id}",
                    onclick: move |_| {},
                    div { visibility: if checked() { "visible" } else { "hidden" }, CheckboxIcon {} }
                }
            }
            {children}
        }
    }
}

#[component]
pub fn CheckboxIcon() -> Element {
    rsx! {
        svg {
            fill: "none",
            height: "9",
            view_box: "0 0 13 9",
            width: "13",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M1.91992 5L4.91992 8L11.9199 1",
                stroke: "#1A1A1A",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
            }
        }
    }
}
