#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn InputWithButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    placeholder: String,
    btn_name: String,
    #[props(default = "text".to_string())] r#type: String,

    onsubmit: EventHandler<String>,
) -> Element {
    let mut value = use_signal(|| "".to_string());

    rsx! {
        div { class: "w-full max-w-604 h-50 flex flex-row items-center rounded-lg overflow-hidden",
            input {
                class: "outline-none h-full grow px-20 text-white text-base placeholder-c-wg-30 border border-c-wg-70 border rounded-l-lg flex flex-row items-center justify-start",
                r#type,
                value: value(),
                placeholder,
                oninput: move |e| value.set(e.value()),
            }
            button {
                class: "h-full bg-white text-black text-sm font-bold px-23 hover:bg-gray-200 focus:outline-none rounded-r-lg flex items-center justify-center",
                onclick: move |_| {
                    onsubmit(value());
                },
                {btn_name}
            }
        }
    }
}
