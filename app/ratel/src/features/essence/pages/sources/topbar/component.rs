use crate::features::essence::pages::sources::*;
use crate::*;

#[component]
pub fn EssenceTopbar() -> Element {
    let tr: EssenceSourcesTranslate = use_translate();
    let nav = use_navigator();

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        div { class: "essence-topbar",
            div { class: "essence-topbar__left",
                button {
                    class: "essence-topbar__back",
                    aria_label: "{tr.back_label}",
                    onclick: move |_| {
                        nav.go_back();
                    },
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        polyline { points: "15 18 9 12 15 6" }
                    }
                }
                div { class: "essence-topbar__title",
                    span { class: "essence-topbar__eyebrow", "{tr.topbar_eyebrow}" }
                    span { class: "essence-topbar__main", "{tr.topbar_main}" }
                }
            }
        }
    }
}
