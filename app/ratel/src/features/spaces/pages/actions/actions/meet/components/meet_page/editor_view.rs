use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::*;

#[component]
pub fn MeetEditorView() -> Element {
    let tr: MeetActionTranslate = use_translate();

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }

        SeoMeta { title: "{tr.page_title}" }

        div { class: "meet-editor", "data-testid": "meet-editor-view",
            MeetModeToggle {}
            MeetDetailsCard {}
            MeetWhenCard {}
            MeetConfigCard {}
            MeetSubmitBar {}
        }
    }
}
