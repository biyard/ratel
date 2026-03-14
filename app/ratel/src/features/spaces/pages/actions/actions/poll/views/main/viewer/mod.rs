use super::*;

#[component]
pub fn PollViewerPage(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    rsx! {
        PollContent { space_id, poll_id, can_respond: false }
    }
}
