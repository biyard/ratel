use crate::features::essence::pages::sources::*;
use crate::*;

#[component]
pub fn EssenceBulkBar() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let mut hook = use_essence_sources()?;

    let selected = hook.selected_rows;
    let count = use_memo(move || selected.read().len());
    let open = use_memo(move || count() > 0);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "essence-bulk", "data-open": open(),
            span { class: "essence-bulk__count",
                strong { "{count()}" }
                " {tr.bulk_selected_suffix}"
            }
            div { class: "essence-bulk__actions",
                button {
                    class: "essence-bulk-btn essence-bulk-btn--danger",
                    onclick: move |_| {
                        hook.bulk_remove.call();
                    },
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
                    "{tr.bulk_remove}"
                }
            }
        }
    }
}
