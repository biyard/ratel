use bdk::prelude::{by_components::icons::chat::RoundBubble, *};

use crate::components::rich_text::RichText;

#[component]
pub fn CreateReplyBox(id: String, lang: Language, onsend: EventHandler<String>) -> Element {
    let mut content = use_signal(|| "".to_string());
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start px-14 py-12 border border-primary z-60 rounded-lg",
            RichText {
                id,
                content: content(),
                onchange: move |value| content.set(value),
                change_location: true,
                remove_border: true,
                placeholder: "Type here, Use Markdown, BB code, or HTML to format. Drag or paste images."
                    .to_string(),
                send_button: rsx! {
                    div {
                        class: "cursor-pointer p-8 bg-primary rounded-full",
                        onclick: move |_| {
                            onsend.call(content());
                        },
                        RoundBubble {
                            width: "24",
                            height: "24",
                            fill: "none",
                            class: "[&>path]:stroke-white [&>line]:stroke-white",
                        }
                    }
                },
                onupload: move |_| {},
            }
        }
    }
}
