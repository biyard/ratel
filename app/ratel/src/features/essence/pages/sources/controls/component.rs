use crate::features::essence::pages::sources::*;
use crate::*;

#[component]
pub fn EssenceControls() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let mut hook = use_essence_sources();

    let is_mac = {
        #[cfg(all(feature = "web", not(feature = "server")))]
        {
            let platform = web_sys::window()
                .and_then(|w| w.navigator().platform().ok())
                .unwrap_or_default();
            platform.to_uppercase().contains("MAC")
        }
        #[cfg(not(all(feature = "web", not(feature = "server"))))]
        {
            false
        }
    };
    let kbd_label = if is_mac { "⌘ K" } else { "Ctrl K" };

    let current_status = hook.status_filter;

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        section { class: "essence-controls",
            label { class: "essence-search",
                svg {
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    circle { cx: "11", cy: "11", r: "8" }
                    line {
                        x1: "21",
                        y1: "21",
                        x2: "16.65",
                        y2: "16.65",
                    }
                }
                input {
                    r#type: "text",
                    placeholder: "{tr.search_placeholder}",
                    "data-essence-search-input": true,
                    value: "{hook.search_query}",
                    oninput: move |e: FormEvent| {
                        hook.search_query.set(e.value());
                    },
                }
                kbd { "{kbd_label}" }
            }

            div { class: "essence-filter-group",
                FilterOpt {
                    selected: current_status() == StatusFilter::All,
                    label: tr.filter_all.to_string(),
                    on_pick: move |_| hook.status_filter.set(StatusFilter::All),
                    icon: None,
                }
                FilterOpt {
                    selected: current_status() == StatusFilter::Active,
                    label: tr.filter_active.to_string(),
                    on_pick: move |_| hook.status_filter.set(StatusFilter::Active),
                    icon: rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "3",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            polyline { points: "20 6 9 17 4 12" }
                        }
                    },
                }
                FilterOpt {
                    selected: current_status() == StatusFilter::Paused,
                    label: tr.filter_paused.to_string(),
                    on_pick: move |_| hook.status_filter.set(StatusFilter::Paused),
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
                }
                FilterOpt {
                    selected: current_status() == StatusFilter::AiFlagged,
                    label: tr.filter_ai_flagged.to_string(),
                    on_pick: move |_| hook.status_filter.set(StatusFilter::AiFlagged),
                    icon: rsx! {
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            circle { cx: "12", cy: "12", r: "10" }
                            line {
                                x1: "12",
                                y1: "8",
                                x2: "12",
                                y2: "12",
                            }
                            line {
                                x1: "12",
                                y1: "16",
                                x2: "12.01",
                                y2: "16",
                            }
                        }
                    },
                }
            }

            select {
                class: "essence-sort",
                value: "{sort_value(hook.sort_order())}",
                onchange: move |e: FormEvent| {
                    hook.sort_order.set(sort_from_value(&e.value()));
                },
                option { value: "last-synced", "{tr.sort_last_synced}" }
                option { value: "last-edited", "{tr.sort_last_edited}" }
                option { value: "word-count", "{tr.sort_word_count}" }
                option { value: "quality", "{tr.sort_quality}" }
                option { value: "title", "{tr.sort_title}" }
            }
        }
    }
}

fn sort_value(s: SortOrder) -> &'static str {
    match s {
        SortOrder::LastSyncedDesc => "last-synced",
        SortOrder::LastEditedDesc => "last-edited",
        SortOrder::WordCountDesc => "word-count",
        SortOrder::QualityDesc => "quality",
        SortOrder::TitleAsc => "title",
    }
}

fn sort_from_value(v: &str) -> SortOrder {
    match v {
        "last-edited" => SortOrder::LastEditedDesc,
        "word-count" => SortOrder::WordCountDesc,
        "quality" => SortOrder::QualityDesc,
        "title" => SortOrder::TitleAsc,
        _ => SortOrder::LastSyncedDesc,
    }
}

#[component]
fn FilterOpt(
    selected: bool,
    label: String,
    on_pick: EventHandler<()>,
    #[props(default)] icon: Option<Element>,
) -> Element {
    rsx! {
        button {
            class: "essence-filter-opt",
            "aria-selected": selected,
            onclick: move |_| on_pick.call(()),
            if let Some(icon) = icon {
                {icon}
            }
            "{label}"
        }
    }
}
