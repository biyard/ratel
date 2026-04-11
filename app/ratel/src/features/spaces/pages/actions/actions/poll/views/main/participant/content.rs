use super::*;

#[component]
pub fn PollContent(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
    can_respond: bool,
) -> Element {
    rsx! {
        crate::features::spaces::pages::index::ActionPollViewer { space_id, poll_id, can_respond }
    }
}
