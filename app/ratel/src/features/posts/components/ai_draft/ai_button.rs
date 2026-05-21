use crate::common::*;
use crate::features::posts::components::ai_draft::i18n::AiDraftTranslate;

#[component]
pub fn AiDraftButton(on_click: EventHandler<()>) -> Element {
    let tr: AiDraftTranslate = use_translate();
    rsx! {
        button {
            class: "ai-btn",
            r#type: "button",
            aria_label: "{tr.button_label}",
            "data-testid": "ai-draft-button",
            onclick: move |_| on_click.call(()),

            svg {
                class: "ai-btn__sparkle",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path { d: "M12 3l1.6 4.4L18 9l-4.4 1.6L12 15l-1.6-4.4L6 9l4.4-1.6L12 3z" }
                path { d: "M19 14l.8 2.2L22 17l-2.2.8L19 20l-.8-2.2L16 17l2.2-.8L19 14z" }
            }
            span { "{tr.button_label}" }
            span { class: "ai-btn__badge", "{tr.pro_badge}" }
        }
    }
}
