#![allow(non_snake_case)]
use bdk::prelude::*;

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
        div { class: "w-full max-w-604 h-50 flex flex-row items-center rounded-lg overflow-hidden max-[900px]:!h-full max-[900px]:gap-20 max-[900px]:!flex-col",
            input {
                class: "outline-none h-full grow px-20 text-white text-base placeholder-c-wg-30 border border-c-wg-70 rounded-l-lg flex flex-row items-center justify-start max-[900px]:!w-full min-h-50 max-[900px]:rounded-[8px]",
                r#type,
                value: value(),
                placeholder,
                oninput: move |e| value.set(e.value()),
            }
            button {
                class: "h-full bg-white text-black text-sm font-bold px-23 hover:bg-gray-200 focus:outline-none rounded-r-lg flex items-center justify-center max-[900px]:w-full max-[900px]:max-w-300 max-[900px]:!h-50 max-[900px]:rounded-[8px] text-[15px]",
                onclick: move |_| {
                    onsubmit(value());
                },
                {btn_name}
            }
        }
    }
}
