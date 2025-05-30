use bdk::prelude::*;

#[component]
pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-1/4 p-4 border-r border-gray-800",
            div { class: "mb-4", "Office of Rep. Oregon, Unite State" }
            div { class: "text-yellow-400", "Tier: Diamond 💎" }
            ul {
                class: "mt-6 space-y-2 text-gray-300",
                li { "Team Threads" }
                li { "Team Profile" }
                li { "Manage Team" }
                li { "Settings" }
            }
        }
    })
}