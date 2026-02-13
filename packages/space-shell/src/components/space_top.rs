use crate::*;

#[component]
pub fn SpaceTop() -> Element {
    rsx! {
        div { class: "h-[65px] px-3 flex flex-row items-center justify-between", "Space top" }
    }
}
