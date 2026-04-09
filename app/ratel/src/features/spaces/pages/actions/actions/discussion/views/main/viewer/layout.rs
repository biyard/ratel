use crate::features::spaces::pages::actions::actions::discussion::*;

#[component]
pub fn NotionLayout(html_contents: String) -> Element {
    rsx! {
        super::DiscussionContentBody { html_contents }
    }
}
