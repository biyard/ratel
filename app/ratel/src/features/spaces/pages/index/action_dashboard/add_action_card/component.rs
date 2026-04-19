use crate::features::spaces::pages::index::*;

#[component]
pub fn AddActionCard(on_click: EventHandler<()>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        button {
            aria_label: "{tr.create_action}",
            class: "quest-card quest-card--add",
            "data-testid": "admin-add-action-card",
            "data-type": "add",
            onclick: move |_| {
                on_click.call(());
            },
            div { class: "add-card__plus",
                svg {
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    line {
                        x1: "12",
                        x2: "12",
                        y1: "5",
                        y2: "19",
                    }
                    line {
                        x1: "5",
                        x2: "19",
                        y1: "12",
                        y2: "12",
                    }
                }
            }
            div { class: "add-card__title", "{tr.new_action}" }
            div { class: "add-card__desc", "{tr.new_action_desc}" }
        }
    }
}
