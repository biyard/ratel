use crate::*;

#[component]
pub fn SuspenseBoundary(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        dioxus::prelude::SuspenseBoundary {
            fallback: |_| rsx! {
                div { class: "flex justify-center items-center w-full h-full min-h-screen bg-background",
                    LoadingIndicator {}
                }
            },
            {children}
        }
    }
}
