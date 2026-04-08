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
        "absolute top-0 right-0 h-full bg-layover-bg border-l border-card-border rounded-l-[24px] overflow-hidden transition-transform duration-300 ease-in-out max-tablet:max-w-full max-tablet:rounded-none {} {} shadow-[0_8px_20px_0_rgba(20,26,62,0.25)]",
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
                class: "absolute inset-0 bg-black transition-opacity duration-300 backdrop-blur-sm",
                class: if is_open { "opacity-50" } else { "opacity-0 pointer-events-none" },
                onclick: onclose,
            }

            div {
                class: "{panel_classes}",
                onclick: move |e| e.stop_propagation(),

                div { class: "flex flex-col h-full",
                    if !title.is_empty() {
                        div { class: "flex flex-row gap-5 items-center px-5 h-16 border-b border-card-border shrink-0",
                            button {
                                class: "flex justify-center items-center bg-transparent rounded-md transition-colors cursor-pointer size-6 hover:bg-layover-close-btn-hover-bg",
                                onclick: onclose,
                                crate::common::icons::validations::Clear {
                                    width: "16",
                                    height: "16",
                                    class: "[&>path]:stroke-foreground-muted",
                                }
                            }

                            h4 { class: "font-bold text-text-primary text-[20px]/[24px] tracking-[-0.2px]",
                                {title}
                            }
                        }
                    }

                    div { class: "flex flex-col flex-1 min-h-0 overflow-y-auto overscroll-contain",
                        {content}
                    }
                }
            }
        }
    }
}
