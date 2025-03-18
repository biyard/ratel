#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn LabeledInput(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    label_name: String,
    placeholder: String,
    #[props(default = "50px".to_string())] height: String,
    #[props(default = "text".to_string())] r#type: String,

    oninput: EventHandler<String>,
    children: Element,
) -> Element {
    rsx! {
        div {..attributes,
            div { class: "w-full flex flex-col items-start gap-5",
                p { class: "text-c-cg-30 font-bold text-base/28", {label_name} }
                input {
                    class: "w-full outline-none h-full grow px-20 text-white text-base placeholder-c-neutral-600 font-medium border border-border-primary rounded-lg flex flex-row items-center justify-start",
                    style: "height: {height};",
                    r#type,
                    placeholder,
                    oninput: move |e| oninput(e.value()),
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn Labeled(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    label_name: String,
    #[props(default = "50px".to_string())] height: String,

    children: Element,
) -> Element {
    rsx! {
        div {..attributes,
            div { class: "w-full flex flex-col items-start gap-5",
                p { class: "text-c-cg-30 font-bold text-base/28", {label_name} }
                {children}
            }
        }
    }
}
