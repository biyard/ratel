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
        div { class: "w-full max-w-604 h-50 flex flex-row items-center rounded-lg overflow-hidden",
            input {
                class: "outline-none h-full grow px-20 text-white text-base placeholder-c-wg-30 border border-c-wg-70 rounded-l-lg flex flex-row items-center justify-start",
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

#[component]
pub fn MobileInputWithButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    placeholder: String,
    btn_name: String,
    #[props(default = "text".to_string())] r#type: String,

    onsubmit: EventHandler<String>,
) -> Element {
    let mut value = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col gap-[20px]",
            div { class: "w-full min-w-[300px] h-[50px] px-[20px] border-[1px] flex items-center rounded-[10px]",
                input {
                    class: "w-full flex items-center justify-start text-white text-[15px] outline-none",
                    r#type,
                    value: value(),
                    placeholder,
                    oninput: move |e| value.set(e.value()),
                }
            }
            div { class: "w-[300px] h-[48px] px-[40px] py-[20px] bg-[#ffffff] rounded-[10px] cursor-pointer",
                button {
                    class: "w-full h-full text-black text-[15px] font-bold flex items-center justify-center",
                    onclick: move |_| {
                        onsubmit(value());
                    },
                    {btn_name}
                }
            }
        }
    }
}
