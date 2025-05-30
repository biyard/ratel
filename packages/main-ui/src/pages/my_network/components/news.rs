use bdk::prelude::*;

#[component]
pub fn NewsRightPanel(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-1/4 p-4 border-l border-gray-800",
            h3 { class: "text-lg font-bold", "News" }
            div { class: "mt-4 text-sm text-gray-400", "Ratel Launches Digital Asset Policy Comparison Feature Ahead of 2025 Election" }
            h3 { class: "mt-6 text-lg font-bold", "Add to your feed" }
            div { class: "mt-2 text-sm", "Donald Trump" }
            div { class: "text-sm", "Elon Musk" }
        }
    })
}