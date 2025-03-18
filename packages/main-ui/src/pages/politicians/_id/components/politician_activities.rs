#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn PoliticianActivities(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { ..attributes,{children} }
    }
}
