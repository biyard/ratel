use crate::*;

#[component]
pub fn SuspenseBoundary(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        dioxus::prelude::SuspenseBoundary {
            fallback: |_| rsx! {
                LoadingIndicator {}
            },
            {children}
        }

    }
}
