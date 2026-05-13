use crate::common::*;

#[component]
pub fn Skeleton(#[props(extends=GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        div { class: "skeleton", ..attributes }
    }
}
