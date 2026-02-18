use crate::*;

#[component]
pub fn SpaceTop() -> Element {
    rsx! {
        div { class: "flex flex-row justify-between items-center px-3 min-h-[65px]",
            "Space top"
        }
    }
}
