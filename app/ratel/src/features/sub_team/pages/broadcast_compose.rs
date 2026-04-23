use crate::*;
use crate::features::sub_team::SubTeamTranslate;

#[component]
pub fn TeamSubTeamBroadcastComposePage(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let _ = username;
    rsx! {
        div { class: "p-6 text-text-primary", "{tr.page_under_construction}" }
    }
}

#[component]
pub fn TeamSubTeamBroadcastEditPage(username: String, announcement_id: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let _ = username;
    let _ = announcement_id;
    rsx! {
        div { class: "p-6 text-text-primary", "{tr.page_under_construction}" }
    }
}
