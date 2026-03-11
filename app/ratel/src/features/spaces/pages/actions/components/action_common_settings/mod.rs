use super::*;

#[component]
pub fn ActionCommonSettings(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { id: "setting-tab", ..attributes, {children} }
    }
}
