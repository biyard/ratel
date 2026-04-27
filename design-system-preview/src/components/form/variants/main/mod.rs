use dioxus::prelude::*;
use dioxus_primitives_core::checkbox::{Checkbox, CheckboxIndicator};
#[component]
pub fn Demo() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        form {
            class: "form-example",
            onsubmit: move |e| {
                tracing::info!("{:?}", e.values());
            },
            Checkbox { id: "tos-check", name: "tos-check",
                CheckboxIndicator { "+" }
            }
            label { r#for: "tos-check", "I agree to the terms presented." }
            br {}
            button { r#type: "submit", "Submit" }
        }
    }
}
