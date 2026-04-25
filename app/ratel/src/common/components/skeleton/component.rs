use dioxus::prelude::*;

#[component]
pub fn Skeleton(#[props(extends=GlobalAttributes)] attributes: Vec<Attribute>) -> Element {
    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        div { class: "skeleton", ..attributes }
    }
}
