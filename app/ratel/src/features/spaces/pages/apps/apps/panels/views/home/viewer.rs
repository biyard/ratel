use super::*;

#[component]
pub fn ViewerPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: PanelsTranslate = use_translate();
    let nav = use_navigator();
    let _ = space_id;

    rsx! {

        div { class: "space-panels-arena",
            div { class: "spa-viewer",
                span { class: "spa-viewer__title", "{tr.viewer_no_access}" }
                button {
                    r#type: "button",
                    class: "spa-viewer__btn",
                    onclick: move |_| {
                        nav.go_back();
                    },
                    "{tr.viewer_back}"
                }
            }
        }
    }
}
