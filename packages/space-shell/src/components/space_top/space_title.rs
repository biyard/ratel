use crate::*;

#[component]
pub fn SpaceTitle(title: String) -> Element {
    rsx! {
        div { class: "text-[15px] font-bold text-white", {title} }
    }
}
