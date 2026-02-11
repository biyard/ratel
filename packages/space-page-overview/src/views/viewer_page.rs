use super::*;
use common::components::TiptapEditor;

#[component]
pub fn ViewerPage(space_id: SpacePartition) -> Element {
    let mut content = use_signal(|| "<p>TESTESTES<span>TSETSE</span></p>".to_string());

    rsx! {
        TiptapEditor {
            class: "w-full h-200",
            content: content(),
            editable: true,
            placeholder: "Type here...",
            on_content_change: move |html: String| {
                content.set(html);
            },
        }
    }
}
