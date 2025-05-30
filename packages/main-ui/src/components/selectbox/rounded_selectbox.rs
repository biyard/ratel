use bdk::prelude::*;

#[component]
pub fn RoundedSelectbox(selected: bool, onchange: EventHandler<MouseEvent>) -> Element {
    rsx! {
        if selected {
            div {
                class: "cursor-pointer w-24 h-24 rounded-full bg-yellow-500 flex items-center justify-center flex-shrink-0",
                onclick: move |e| {
                    onchange.call(e);
                },
                CheckIcon {}
            }
        } else {
            div {
                onclick: move |e| {
                    onchange.call(e);
                },
                class: "cursor-pointer w-24 h-24 bg-transparent border-2 border-neutral-500 rounded-full",
            }
        }
    }
}

#[component]
pub fn CheckIcon() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "black",
            stroke_width: "3",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: "w-12 h-12",
            path { d: "M5 13l4 4L19 7" }
        }
    }
}
