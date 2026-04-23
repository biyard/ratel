use crate::*;
use crate::features::sub_team::SubTeamTranslate;

#[component]
pub fn TeamBylawsPage(username: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let _ = username;
    rsx! {
        div { class: "p-6 text-text-primary", "{tr.page_under_construction}" }
    }
}
