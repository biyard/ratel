use crate::features::essence::pages::sources::*;
use crate::*;

#[component]
pub fn EssenceBulkBar() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let hook = use_essence_sources();

    let count = use_memo(move || hook.selected_rows.read().len());
    let open = use_memo(move || count() > 0);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "essence-bulk", "data-open": open(),
            span { class: "essence-bulk__count",
                strong { "{count()}" }
                " {tr.bulk_selected_suffix}"
            }
            div { class: "essence-bulk__actions",
                BulkBtn {
                    label: tr.bulk_pause.to_string(),
                    on_click: move |_| hook.bulk_pause.call(()),
                    icon: rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            rect {
                                x: "6",
                                y: "4",
                                width: "4",
                                height: "16",
                            }
                            rect {
                                x: "14",
                                y: "4",
                                width: "4",
                                height: "16",
                            }
                        }
                    },
                    danger: false,
                }
                BulkBtn {
                    label: tr.bulk_reembed.to_string(),
                    on_click: move |_| hook.bulk_reembed.call(()),
                    icon: rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "23 4 23 10 17 10" }
                            path { d: "M20.49 15a9 9 0 1 1-2.12-9.36L23 10" }
                        }
                    },
                    danger: false,
                }
                BulkBtn {
                    label: tr.bulk_flag_ai.to_string(),
                    on_click: move |_| hook.bulk_flag_ai.call(()),
                    icon: rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                            polyline { points: "22 4 12 14.01 9 11.01" }
                        }
                    },
                    danger: false,
                }
                BulkBtn {
                    label: tr.bulk_remove.to_string(),
                    on_click: move |_| hook.bulk_remove.call(()),
                    icon: rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "3 6 5 6 21 6" }
                            path { d: "M19 6l-2 14a2 2 0 0 1-2 2H9a2 2 0 0 1-2-2L5 6" }
                        }
                    },
                    danger: true,
                }
            }
        }
    }
}

#[component]
fn BulkBtn(
    label: String,
    on_click: EventHandler<()>,
    icon: Element,
    #[props(default)] danger: bool,
) -> Element {
    let class_name = if danger {
        "essence-bulk-btn essence-bulk-btn--danger"
    } else {
        "essence-bulk-btn"
    };
    rsx! {
        button { class: "{class_name}", onclick: move |_| on_click.call(()),
            {icon}
            "{label}"
        }
    }
}
