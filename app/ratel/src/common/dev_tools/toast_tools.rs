use dioxus::prelude::*;

use crate::common::providers::use_toast;

#[component]
pub fn ToastTools() -> Element {
    let mut toast = use_toast();
    let mut count = use_signal(|| 0u32);

    rsx! {
        div { class: "flex flex-col gap-1",
            span { class: "text-[10px] font-medium text-[#6666aa] uppercase tracking-wider mb-1", "Toast" }
            ToolButton {
                label: "Info",
                color: "bg-blue-600 hover:bg-blue-700",
                onclick: move |_| {
                    let n = *count.read();
                    count.set(n + 1);
                    toast.info(format!("Info toast #{n}"));
                },
            }
            ToolButton {
                label: "Warn",
                color: "bg-yellow-600 hover:bg-yellow-700",
                onclick: move |_| {
                    let n = *count.read();
                    count.set(n + 1);
                    toast.warn(format!("Warning toast #{n}"));
                },
            }
            ToolButton {
                label: "Error",
                color: "bg-red-600 hover:bg-red-700",
                onclick: move |_| {
                    let n = *count.read();
                    count.set(n + 1);
                    toast.error(crate::common::Error::Internal);
                },
            }
            ToolButton {
                label: "With Link",
                color: "bg-purple-600 hover:bg-purple-700",
                onclick: move |_| {
                    let n = *count.read();
                    count.set(n + 1);
                    toast.info(format!("Toast with link #{n}")).with_link("https://example.com");
                },
            }
        }
    }
}

#[component]
fn ToolButton(label: &'static str, color: &'static str, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "py-1.5 px-3 text-xs font-medium text-white rounded-md transition-all active:scale-95 text-left {color}",
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}
