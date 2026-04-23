use crate::*;
use crate::features::sub_team::SubTeamTranslate;

#[component]
pub fn TeamSubTeamDocComposePage(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let _ = username;
    rsx! {
        div { class: "p-6 text-text-primary", "{tr.page_under_construction}" }
    }
}

#[component]
pub fn TeamSubTeamDocEditPage(username: String, doc_id: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let _ = username;
    let _ = doc_id;
    rsx! {
        div { class: "p-6 text-text-primary", "{tr.page_under_construction}" }
    }
}
