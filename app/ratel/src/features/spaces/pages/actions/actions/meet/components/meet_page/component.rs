use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::actions::meet::*;
use crate::features::spaces::space_common::hooks::use_space_role;
use crate::*;

#[component]
pub fn MeetActionPage(space_id: SpacePartition, meet_id: SpaceMeetEntityType) -> Element {
    let space_id_sig: ReadSignal<SpacePartition> = use_signal(|| space_id.clone()).into();
    let meet_id_sig: ReadSignal<SpaceMeetEntityType> = use_signal(|| meet_id.clone()).into();

    // Provision the UseMeet controller context for descendant components.
    let _ = use_meet(space_id_sig, meet_id_sig)?;

    let role = use_space_role()();
    let is_admin = role.is_admin();

    rsx! {
        if is_admin {
            MeetEditorView {}
        } else {
            MeetViewerView {}
        }
    }
}
