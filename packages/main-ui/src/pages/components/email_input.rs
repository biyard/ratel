#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn EmailInput() -> Element {
    let mut email = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col space-y-2 text-white",
            label { class: "text-sm font-medium", "Email" }
            div { class: "flex items-center border border-gray-600 rounded-lg bg-black px-3",
                input {
                    class: "bg-transparent outline-none flex-1 px-2 py-2 text-white placeholder-gray-500",
                    r#type: "email",
                    placeholder: "🖂 Input your mail",
                    value: email(),
                    oninput: move |e| email.set(e.value()),
                }
                button {
                    class: "bg-white text-black font-medium px-3 py-1 rounded-r-lg hover:bg-gray-200 focus:outline-none",
                    onclick: move |_| {
                        btracing::info!("Subscribed by {}", email());
                    },
                    "Subscribe"
                }
            }
        }
    }
}
