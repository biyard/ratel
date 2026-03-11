use crate::common::*;

#[component]
pub fn Card(
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "py-5 px-4 border rounded-[10px] bg-card-bg border-card-border {class}",
            ..attributes,
            {children}
        }
    }
}
