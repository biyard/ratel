use crate::features::spaces::pages::actions::actions::meet::*;
use crate::*;

#[component]
pub fn MeetActionPage(
    space_id: SpacePartition,
    meet_id: SpaceMeetEntityType,
) -> Element {
    let _ = (space_id, meet_id);
    rsx! {
        div { class: "meet-action-page placeholder",
            SeoMeta { title: "Meet" }
            "Meet action page (Phase 1 scaffold)"
        }
    }
}
