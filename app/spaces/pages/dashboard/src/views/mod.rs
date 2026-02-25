use crate::{api::fetch_dashboard_extensions, *};

mod candidate_page;
mod creator_page;
mod participant_page;
mod viewer_page;

use candidate_page::*;
use creator_page::*;
use participant_page::*;
use space_common::hooks::use_user_role;
use viewer_page::*;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let extension_loader = use_loader({
        let sid = space_id.clone();
        move || fetch_dashboard_extensions(sid.clone())
    })?;

    let extensions = extension_loader.read().clone();
    let user_role = use_user_role();

    match (user_role, extensions) {
        (role, exts) => match role {
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
