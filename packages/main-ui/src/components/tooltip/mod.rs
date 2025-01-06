use dioxus::prelude::*;

#[component]
pub fn Tooltip(
    #[props(into)] inner_class: Option<String>,
    text: String,
    children: Element,
    #[props(default = "black".to_string())] bg_color: String,
    #[props(default = true)] arrow: bool,
) -> Element {
    let mut show_tooltip = use_signal(|| false);

    rsx! {
        div {
            class: "relative inline-block",
            
            div {
                class: "relative",
                onmouseenter: move |_| show_tooltip.set(true),
                onmouseleave: move |_| show_tooltip.set(false),

                {children.clone()},

                div {
                    class: "absolute z-50 left-1/2 -translate-x-1/2 bottom-full mb-2",
                    style: if (show_tooltip)() { "display: block" } else { "display: none" },
                    div {
                        class: if let Some(class) = inner_class { class } 
                            else { "bg-[{bg_color}] text-white px-[16px] py-[12px] rounded-lg text-sm max-w-[500px] leading-[1.5]".into() },
                        p {
                            class: "whitespace-pre-wrap break-words",
                            style: "word-break: keep-all",
                            "{text}"
                        }
                    }
                    if arrow {
                        div {
                            class: "absolute left-1/2 transform -translate-x-1/2 top-full w-0 h-0 border-l-8 border-l-transparent border-r-8 border-r-transparent border-t-8 border-t-[{bg_color}]",
                        }
                    }
                }
            }
        }
    }
}