use crate::common::{components::layover, *};
mod service;

pub use service::*;

#[component]
pub fn Layover() -> Element {
    let mut layover = use_layover();

    let router = use_context::<dioxus::router::RouterContext>();
    let current_path = router.full_route_string();
    let mut prev_path = use_signal(move || current_path.clone());

    use_effect(move || {
        let current = router.full_route_string();
        if current != *prev_path.read() {
            prev_path.set(current);
            layover.close();
        }
    });

    let config = layover.state();
    let (is_open, title, content, container_class) = match config {
        Some(c) => (true, c.title, c.content, c.container_class),
        None => (
            false,
            String::new(),
            rsx! {
                Fragment {}
            },
            None,
        ),
    };
    let onclose = move |_| layover.close();
    let panel_classes = format!(
        "absolute top-0 right-0 h-full w-full max-w-50% bg-neutral-900 light:bg-neutral-200 border-l border-neutral-800 light:border-neutral-300 rounded-l-[24px] overflow-hidden transition-transform duration-300 ease-in-out max-tablet:max-w-full max-tablet:rounded-none {} {}",
        if is_open {
            "translate-x-0"
        } else {
            "translate-x-full"
        },
        container_class.unwrap_or_default()
    );

    rsx! {
        div {
            class: "fixed inset-0 z-100",
            class: if !is_open { "pointer-events-none" },
            div {
                class: "absolute inset-0 bg-black backdrop-blur-sm transition-opacity duration-300",
                class: if is_open { "opacity-50" } else { "opacity-0 pointer-events-none" },
                onclick: onclose,
            }

            div {
                class: "{panel_classes}",
                onclick: move |e| e.stop_propagation(),

                div { class: "flex flex-col h-full",
                    if !title.is_empty() {
                        div { class: "flex flex-row gap-5 items-center px-5 h-16 border-b border-neutral-800 light:border-neutral-300 shrink-0",
                            button {
                                class: "flex justify-center items-center rounded-md size-6 bg-transparent hover:bg-neutral-800 light:hover:bg-neutral-300 transition-colors cursor-pointer",
                                onclick: onclose,
                                crate::common::icons::validations::Clear {
                                    width: "16",
                                    height: "16",
                                    class: "[&>path]:stroke-neutral-400 light:[&>path]:stroke-neutral-600",
                                }
                            }

                            h4 { class: "font-bold text-[20px]/[24px] tracking-[-0.2px] text-white light:text-neutral-900",
                                {title}
                            }
                        }
                    }

                    div { class: "flex flex-col flex-1 min-h-0", {content} }
                }
            }
        }
    }
}
