use bdk::prelude::*;
use web_sys::window;

use crate::pages::components::BlackRoundedBox;

#[component]
pub fn ThreadContents(description: String) -> Element {
    use_effect(move || {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let style_tag = document.create_element("style").unwrap();
        style_tag.set_text_content(Some(
            r#"
            .rich-content h1 {
                font-size: 15px !important;
                font-weight: bold !important;
                color: #d4d4d4 !important;
                line-height: 20px !important;
            }
            .rich-content h2 {
                font-size: 15px !important;
                font-weight: bold !important;
                color: #d4d4d4 !important;
                line-height: 20px !important;
            }
            .rich-content p,
            .rich-content ul,
            .rich-content li,
            .rich-content div,
            .rich-content a {
                font-size: 15px !important;
                font-weight: normal !important;
                color: #d4d4d4 !important;
                line-height: 24px !important;
            }
            "#,
        ));
        document.head().unwrap().append_child(&style_tag).unwrap();
    });

    rsx! {
        div { class: "flex flex-row w-full",
            BlackRoundedBox {
                div {
                    class: "w-full rich-content",
                    dangerous_inner_html: description,
                }
            }
        }
    }
}
