use bdk::prelude::*;

#[component]
pub fn SideRoundedBox(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col px-16 py-20 rounded-[10px] bg-footer",
            div { class: "flex flex-col w-[218px]", {children} }
        }
    }
}
