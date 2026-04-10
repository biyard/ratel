use super::*;

#[component]
pub fn PollViewerPage(
    space_id: ReadSignal<SpacePartition>,
    poll_id: ReadSignal<SpacePollEntityType>,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full",
            PollContent { space_id, poll_id, can_respond: false }
        }
    }
}
