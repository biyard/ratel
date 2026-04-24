use crate::features::spaces::pages::index::*;

#[component]
pub fn QuestEditButton(action_id: String, on_edit: EventHandler<()>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        button {
            aria_label: "{tr.edit}",
            class: "quest-card__edit-btn",
            "data-testid": "quest-edit-btn-{action_id}",
            onclick: move |e: Event<MouseData>| {
                e.stop_propagation();
                on_edit.call(());
            },
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "1.8",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" }
                polyline { points: "14 2 14 8 20 8" }
                line {
                    x1: "16",
                    x2: "8",
                    y1: "13",
                    y2: "13",
                }
                line {
                    x1: "16",
                    x2: "8",
                    y1: "17",
                    y2: "17",
                }
                line {
                    x1: "10",
                    x2: "8",
                    y1: "9",
                    y2: "9",
                }
            }
        }
    }
}
