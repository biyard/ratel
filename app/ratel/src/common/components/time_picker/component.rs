use dioxus::prelude::*;
use icons::time::Clock;

#[component]
pub fn TimePicker(
    #[props(default = 0)] hour: u8,
    #[props(default = 0)] minute: u8,
    #[props(default)] on_change: EventHandler<(u8, u8)>,
) -> Element {
    let mut is_open = use_signal(|| false);
    let mut selected_hour = use_signal(move || hour);
    let mut selected_minute = use_signal(move || minute);

    let display = format!("{:02}:{:02}", selected_hour(), selected_minute());

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "time-picker @max-sm:w-full",
            div {
                class: "time-picker-trigger @max-sm:w-full",
                onclick: move |e| {
                    e.stop_propagation();
                    is_open.toggle();
                },
                span { class: "text-center time-picker-display", "{display}" }
                Clock { width: "20", height: "20", class: "time-picker-icon" }
            }

            if is_open() {
                div {
                    class: "time-picker-backdrop",
                    onclick: move |_| is_open.set(false),
                }
                div { class: "time-picker-dropdown",
                    div { class: "time-picker-columns",
                        div { class: "time-picker-column",
                            div { class: "time-picker-column-label", "Hour" }
                            div { class: "time-picker-scroll",
                                for h in 0..24u8 {
                                    button {
                                        key: "h-{h}",
                                        class: "time-picker-cell",
                                        "data-selected": selected_hour() == h,
                                        onclick: move |_| {
                                            selected_hour.set(h);
                                            on_change.call((h, selected_minute()));
                                        },
                                        onmounted: move |e: MountedEvent| {
                                            if selected_hour() == h {
                                                spawn(async move {
                                                    let _ = e.scroll_to(ScrollBehavior::Instant).await;
                                                });
                                            }
                                        },
                                        "{h:02}"
                                    }
                                }
                            }
                        }

                        div { class: "time-picker-divider" }

                        div { class: "time-picker-column",
                            div { class: "time-picker-column-label", "Min" }
                            div { class: "time-picker-scroll",
                                for m in 0..60u8 {
                                    button {
                                        key: "m-{m}",
                                        class: "time-picker-cell",
                                        "data-selected": selected_minute() == m,
                                        onclick: move |_| {
                                            selected_minute.set(m);
                                            on_change.call((selected_hour(), m));
                                        },
                                        onmounted: move |e: MountedEvent| {
                                            if selected_minute() == m {
                                                spawn(async move {
                                                    let _ = e.scroll_to(ScrollBehavior::Instant).await;
                                                });
                                            }
                                        },
                                        "{m:02}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
