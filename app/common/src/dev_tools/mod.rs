mod toast_tools;

use dioxus::prelude::*;

#[component]
pub fn DevTools() -> Element {
    let mut expanded = use_signal(|| false);
    let is_expanded = *expanded.read();

    rsx! {
        div { class: "fixed right-4 bottom-4 z-[200] flex flex-col items-end gap-2",
            if is_expanded {
                div {
                    class: "flex flex-col gap-1 p-3 rounded-xl bg-[#1a1a2e] border border-[#2a2a3e] shadow-2xl min-w-[180px]",

                    // Header
                    div { class: "flex items-center justify-between mb-2 pb-2 border-b border-[#2a2a3e]",
                        span { class: "text-xs font-semibold text-[#8888aa] uppercase tracking-wider", "Dev Tools" }
                    }

                    // Toast section
                    toast_tools::ToastTools {}
                }
            }

            // FAB toggle button
            button {
                class: if is_expanded {
                    "w-12 h-12 rounded-full shadow-lg flex items-center justify-center text-white text-xl transition-all active:scale-90 bg-red-600 hover:bg-red-700 rotate-45"
                } else {
                    "w-12 h-12 rounded-full shadow-lg flex items-center justify-center text-white text-xl transition-all active:scale-90 bg-indigo-600 hover:bg-indigo-700"
                },
                onclick: move |_| expanded.set(!is_expanded),
                "+"
            }
        }
    }
}
