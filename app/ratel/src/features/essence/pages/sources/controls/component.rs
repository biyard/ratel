use crate::features::essence::pages::sources::*;
use crate::*;

#[component]
pub fn EssenceControls() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let mut hook = use_essence_sources()?;

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

    rsx! {
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

            select {
                class: "essence-sort",
                value: "{sort_value(hook.sort_order())}",
                onchange: move |e: FormEvent| {
                    hook.set_sort.call(sort_from_value(&e.value()));
                },
                option { value: "last-edited", "{tr.sort_last_edited}" }
                option { value: "word-count", "{tr.sort_word_count}" }
                option { value: "title", "{tr.sort_title}" }
            }
        }
    }
}

fn sort_value(s: EssenceSort) -> &'static str {
    match s {
        EssenceSort::LastEditedDesc => "last-edited",
        EssenceSort::WordCountDesc => "word-count",
        EssenceSort::TitleAsc => "title",
    }
}

fn sort_from_value(v: &str) -> EssenceSort {
    match v {
        "word-count" => EssenceSort::WordCountDesc,
        "title" => EssenceSort::TitleAsc,
        _ => EssenceSort::LastEditedDesc,
    }
}
