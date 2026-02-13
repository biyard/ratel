use crate::*;

mod candidate_page;
mod creator_page;
mod participant_page;
mod viewer_page;

use candidate_page::*;
use creator_page::*;
use participant_page::*;
use viewer_page::*;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let sid = space_id.to_string();

    let user_role = {
        let sid = sid.clone();
        use_resource(move || {
            let sid = sid.clone();
            async move { crate::api::get_user_role_in_space(sid).await }
        })
    };

    let extensions = {
        let sid = sid.clone();
        use_resource(move || {
            let sid = sid.clone();
            async move { crate::api::fetch_dashboard_extensions(sid).await }
        })
    };

    match (user_role(), extensions()) {
        (Some(Ok(role)), Some(Ok(exts))) => match role {
            SpaceUserRole::Creator => rsx! {
                CreatorPage { space_id: space_id.clone(), extensions: exts }
            },
            SpaceUserRole::Participant => rsx! {
                ParticipantPage { space_id: space_id.clone(), extensions: exts }
            },
            SpaceUserRole::Candidate => rsx! {
                CandidatePage { space_id: space_id.clone(), extensions: exts }
            },
            SpaceUserRole::Viewer => rsx! {
                ViewerPage { space_id: space_id.clone(), extensions: exts }
            },
        },
        _ => rsx! {
            div { class: "p-4", "Loading..." }
        },
    }
}
