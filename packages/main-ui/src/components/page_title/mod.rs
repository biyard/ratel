#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn PageHeader(title: String, children: Element) -> Element {
    rsx! {
        div { class: "w-full flex flex-row justify-between items-center",
            div { class: "flex justify-center items-center text-[20px] font-semibold leading-none py-[0px] align-middle",
                "{title}"
            }
            {children}
        }
    }
}

#[component]
pub fn PageHeaderTail(children: Element, onclick: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "flex justify-center items-center py-[5px] px-[10px] transition-all rounded-[5px] text-[14px] font-semibold hover:bg-[#424563] cursor-pointer gap-[5px]",
            onclick: move |_| { onclick(()) },
            {children}
        }
    }
}
