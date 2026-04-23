use crate::*;
use crate::features::sub_team::SubTeamTranslate;

#[component]
pub fn TeamSubTeamDeregisterPage(username: String, sub_team_id: String) -> Element {
    let tr: SubTeamTranslate = use_translate();
    let _ = username;
    let _ = sub_team_id;
    rsx! {
        div { class: "p-6 text-text-primary", "{tr.page_under_construction}" }
    }
}
